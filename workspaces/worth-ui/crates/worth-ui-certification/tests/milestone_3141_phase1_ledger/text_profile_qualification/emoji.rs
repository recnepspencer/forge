use std::fs;
use std::path::{Path, PathBuf};

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
    require_count(emoji, "variation_sequence_records", data_line_count(&variations))?;
    require_representative_sequences(&sequences, &zwj, &variations)
}

fn require_representative_sequences(
    sequences: &str,
    zwj: &str,
    variations: &str,
) -> Result<(), String> {
    for (source, sequence) in [
        (sequences, "0023 FE0F 20E3"),
        (sequences, "1F1FA 1F1F8"),
        (sequences, "1F3F4 E0067 E0062 E0065 E006E E0067 E007F"),
        (sequences, "1F44B 1F3FD"),
        (zwj, "1F468 200D 1F469 200D 1F467 200D 1F466"),
        (variations, "2764 FE0E"),
        (variations, "2764 FE0F"),
    ] {
        if !source.lines().any(|line| line.starts_with(sequence)) {
            return Err(format!("qualified emoji sequence missing: {sequence}"));
        }
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
