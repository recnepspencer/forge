use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

type Sequence = Vec<u32>;

pub(super) fn validate(root: &Path, manifest: &toml::Value) -> Result<(), String> {
    let unicode = super::table(manifest, "unicode")?;
    let emoji = super::table(manifest, "emoji")?;
    let test = read_text(root.join(super::table_string(unicode, "emoji_test")?))?;
    let sequences = read_text(root.join(super::table_string(unicode, "emoji_sequences")?))?;
    let zwj = read_text(root.join(super::table_string(unicode, "emoji_zwj_sequences")?))?;
    let variations = read_text(root.join("unicode/ucd/emoji/emoji-variation-sequences.txt"))?;
    require_version_17(&test)?;
    require_version_17(&sequences)?;
    require_version_17(&zwj)?;
    for (status, field) in [
        ("fully-qualified", "fully_qualified"),
        ("minimally-qualified", "minimally_qualified"),
        ("unqualified", "unqualified"),
        ("component", "components"),
    ] {
        require_count(emoji, field, emoji_test_count(&test, status))?;
    }
    for (kind, field) in [
        ("Basic_Emoji", "basic_sequence_records"),
        ("Emoji_Keycap_Sequence", "keycap_sequence_records"),
        ("RGI_Emoji_Flag_Sequence", "flag_sequence_records"),
        ("RGI_Emoji_Tag_Sequence", "tag_sequence_records"),
        ("RGI_Emoji_Modifier_Sequence", "modifier_sequence_records"),
    ] {
        require_count(emoji, field, sequence_count(&sequences, kind))?;
    }
    require_count(
        emoji,
        "zwj_sequence_records",
        sequence_count(&zwj, "RGI_Emoji_ZWJ_Sequence"),
    )?;
    require_count(
        emoji,
        "variation_sequence_records",
        data_line_count(&variations),
    )?;
    require_exact_rgi_set(&test, &sequences, &zwj)?;
    require_complete_variation_pairs(&variations)
}

pub(super) fn require_exact_rgi_set(test: &str, sequences: &str, zwj: &str) -> Result<(), String> {
    let test_set = emoji_test_set(test)?;
    let mut corpus_set = sequence_set(
        sequences,
        &[
            "Basic_Emoji",
            "Emoji_Keycap_Sequence",
            "RGI_Emoji_Flag_Sequence",
            "RGI_Emoji_Tag_Sequence",
            "RGI_Emoji_Modifier_Sequence",
        ],
    )?;
    for sequence in sequence_set(zwj, &["RGI_Emoji_ZWJ_Sequence"])? {
        if !corpus_set.insert(sequence) {
            return Err("RGI emoji sequence is duplicated across source corpora".to_owned());
        }
    }
    if corpus_set != test_set {
        return Err("Unicode 17 RGI emoji corpus sets disagree".to_owned());
    }
    Ok(())
}

fn emoji_test_set(source: &str) -> Result<BTreeSet<Sequence>, String> {
    let mut sequences = BTreeSet::new();
    for line in source.lines().filter(data_line) {
        let fields: Vec<_> = line.split(';').collect();
        let status = fields
            .get(1)
            .and_then(|field| field.split_whitespace().next())
            .ok_or("emoji-test status missing")?;
        if matches!(status, "fully-qualified" | "component") {
            let sequence = parse_sequence(fields[0])?;
            if !sequences.insert(sequence) {
                return Err("emoji-test contains a duplicate RGI sequence".to_owned());
            }
        }
    }
    Ok(sequences)
}

fn sequence_set(source: &str, allowed: &[&str]) -> Result<BTreeSet<Sequence>, String> {
    let allowed: BTreeSet<_> = allowed.iter().copied().collect();
    let mut sequences = BTreeSet::new();
    for line in source.lines().filter(data_line) {
        let fields: Vec<_> = line.split(';').collect();
        let kind = fields
            .get(1)
            .map(|field| field.trim())
            .ok_or("emoji sequence class missing")?;
        if !allowed.contains(kind) {
            return Err(format!("unexpected emoji sequence class: {kind}"));
        }
        for sequence in parse_sequence_or_range(fields[0])? {
            if !sequences.insert(sequence) {
                return Err("emoji sequence corpus contains a duplicate".to_owned());
            }
        }
    }
    Ok(sequences)
}

fn parse_sequence_or_range(field: &str) -> Result<Vec<Sequence>, String> {
    let field = field.trim();
    if let Some((start, end)) = field.split_once("..") {
        let start = parse_scalar(start)?;
        let end = parse_scalar(end)?;
        if start > end {
            return Err("emoji sequence range is reversed".to_owned());
        }
        return Ok((start..=end).map(|scalar| vec![scalar]).collect());
    }
    Ok(vec![parse_sequence(field)?])
}

fn parse_sequence(field: &str) -> Result<Sequence, String> {
    let sequence: Result<Vec<_>, _> = field.split_whitespace().map(parse_scalar).collect();
    let sequence = sequence?;
    if sequence.is_empty() {
        return Err("emoji sequence is empty".to_owned());
    }
    Ok(sequence)
}

fn parse_scalar(value: &str) -> Result<u32, String> {
    let scalar = u32::from_str_radix(value.trim(), 16)
        .map_err(|_| format!("emoji scalar is not hexadecimal: {value}"))?;
    char::from_u32(scalar)
        .map(|_| scalar)
        .ok_or_else(|| format!("emoji scalar is not Unicode: {value}"))
}

pub(super) fn require_complete_variation_pairs(source: &str) -> Result<(), String> {
    let mut selectors: BTreeMap<u32, u8> = BTreeMap::new();
    for line in source.lines().filter(data_line) {
        let fields: Vec<_> = line.split(';').collect();
        let sequence = parse_sequence(fields.first().copied().unwrap_or_default())?;
        if sequence.len() != 2 || !matches!(sequence[1], 0xFE0E | 0xFE0F) {
            return Err("emoji variation sequence is malformed".to_owned());
        }
        let bit = if sequence[1] == 0xFE0E { 1 } else { 2 };
        let entry = selectors.entry(sequence[0]).or_default();
        if *entry & bit != 0 {
            return Err("emoji variation sequence is duplicated".to_owned());
        }
        *entry |= bit;
    }
    if selectors.values().any(|selectors| *selectors != 3) {
        return Err("emoji text/presentation variation pair is incomplete".to_owned());
    }
    Ok(())
}

fn emoji_test_count(source: &str, status: &str) -> usize {
    source
        .lines()
        .filter(data_line)
        .filter(|line| {
            line.split(';')
                .nth(1)
                .is_some_and(|value| value.trim().starts_with(status))
        })
        .count()
}

fn sequence_count(source: &str, kind: &str) -> usize {
    source
        .lines()
        .filter(data_line)
        .filter(|line| {
            line.split(';')
                .nth(1)
                .is_some_and(|value| value.trim() == kind)
        })
        .count()
}

fn data_line_count(source: &str) -> usize {
    source.lines().filter(data_line).count()
}

fn data_line(line: &&str) -> bool {
    line.as_bytes().first().is_some_and(u8::is_ascii_hexdigit)
}

fn require_count(
    contract: &toml::value::Table,
    field: &str,
    observed: usize,
) -> Result<(), String> {
    let observed = i64::try_from(observed).map_err(|_| "emoji count overflow".to_owned())?;
    if super::table_integer(contract, field)? != observed {
        return Err(format!("emoji corpus count drifted: {field}"));
    }
    Ok(())
}

fn require_version_17(source: &str) -> Result<(), String> {
    source
        .lines()
        .any(|line| line.trim() == "# Version: 17.0")
        .then_some(())
        .ok_or_else(|| "emoji corpus Unicode version drifted".to_owned())
}

fn read_text(path: PathBuf) -> Result<String, String> {
    fs::read_to_string(path).map_err(|error| error.to_string())
}
