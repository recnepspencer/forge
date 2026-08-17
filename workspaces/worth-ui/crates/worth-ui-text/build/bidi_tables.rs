use std::{fs, path::Path};

pub fn generate(derived: &Path, brackets: &Path) -> String {
    let mut explicit = Vec::new();
    let mut defaults = Vec::new();
    for line in fs::read_to_string(derived)
        .expect("read DerivedBidiClass")
        .lines()
    {
        let trimmed = line.trim();
        let missing = trimmed.strip_prefix("# @missing:");
        let data = missing.unwrap_or(trimmed.split('#').next().unwrap_or("").trim());
        if data.is_empty() || (missing.is_none() && data.starts_with('#')) {
            continue;
        }
        let Some((range, class)) = data.split_once(';') else {
            continue;
        };
        let record = (code_range(range.trim()), class_alias(class.trim()));
        if missing.is_some() {
            defaults.push(record)
        } else {
            explicit.push(record)
        }
    }
    explicit.sort_by_key(|record| record.0 .0);
    let mut bracket_rows = Vec::new();
    for line in fs::read_to_string(brackets)
        .expect("read BidiBrackets")
        .lines()
    {
        let data = line.split('#').next().unwrap_or("").trim();
        if data.is_empty() {
            continue;
        }
        let fields = data.split(';').map(str::trim).collect::<Vec<_>>();
        if fields.len() != 3 {
            continue;
        }
        let code = u32::from_str_radix(fields[0], 16).expect("bracket code");
        let paired = u32::from_str_radix(fields[1], 16).expect("paired bracket code");
        let is_open = fields[2] == "o";
        let opening = if is_open { code } else { paired };
        bracket_rows.push((
            code,
            if opening == 0x2329 { 0x3008 } else { opening },
            is_open,
        ));
    }
    let ranges = explicit.into_iter().map(range_source).collect::<String>();
    let missing = defaults.into_iter().map(range_source).collect::<String>();
    let brackets = bracket_rows
        .into_iter()
        .map(|(code, opening, is_open)| format!("    (0x{code:X}, 0x{opening:X}, {is_open}),\n"))
        .collect::<String>();
    format!(
        "const EXPLICIT_BIDI: &[(u32, u32, unicode_bidi::BidiClass)] = &[\n{ranges}];\n\
         const DEFAULT_BIDI: &[(u32, u32, unicode_bidi::BidiClass)] = &[\n{missing}];\n\
         const BIDI_BRACKETS: &[(u32, u32, bool)] = &[\n{brackets}];\n"
    )
}

fn code_range(value: &str) -> (u32, u32) {
    let mut bounds = value.split("..");
    let start = u32::from_str_radix(bounds.next().expect("range start"), 16).expect("hex start");
    let end = bounds
        .next()
        .map_or(start, |end| u32::from_str_radix(end, 16).expect("hex end"));
    (start, end)
}

fn class_alias(value: &str) -> &'static str {
    match value {
        "Left_To_Right" => "L",
        "Right_To_Left" => "R",
        "Arabic_Letter" => "AL",
        "European_Terminator" => "ET",
        "L" => "L",
        "R" => "R",
        "AL" => "AL",
        "EN" => "EN",
        "ES" => "ES",
        "ET" => "ET",
        "AN" => "AN",
        "CS" => "CS",
        "B" => "B",
        "S" => "S",
        "WS" => "WS",
        "ON" => "ON",
        "BN" => "BN",
        "NSM" => "NSM",
        "LRE" => "LRE",
        "LRO" => "LRO",
        "RLE" => "RLE",
        "RLO" => "RLO",
        "PDF" => "PDF",
        "LRI" => "LRI",
        "RLI" => "RLI",
        "FSI" => "FSI",
        "PDI" => "PDI",
        other => panic!("unsupported bidi class {other}"),
    }
}

fn range_source(((start, end), class): ((u32, u32), &str)) -> String {
    format!("    (0x{start:X}, 0x{end:X}, unicode_bidi::BidiClass::{class}),\n")
}
