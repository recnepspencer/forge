use std::{fs, path::PathBuf};

pub fn generate(inputs: &[PathBuf; 4]) -> String {
    let line = property_ranges(&inputs[0], None)
        .into_iter()
        .map(|(start, end, value)| format!("    (0x{start:X}, 0x{end:X}, LineClass::{value}),\n"))
        .collect::<String>();
    let (categories, assigned) = unicode_categories(&inputs[1]);
    let categories = categories
        .into_iter()
        .map(|(start, end, value)| {
            format!("    (0x{start:X}, 0x{end:X}, GeneralCategory::{value}),\n")
        })
        .collect::<String>();
    let assigned = assigned
        .into_iter()
        .map(|(start, end)| format!("    (0x{start:X}, 0x{end:X}),\n"))
        .collect::<String>();
    let east_asian = property_ranges(&inputs[2], None)
        .into_iter()
        .filter(|(_, _, value)| matches!(*value, "F" | "W" | "H"))
        .map(|(start, end, _)| format!("    (0x{start:X}, 0x{end:X}),\n"))
        .collect::<String>();
    let extended_pictographic = property_ranges(&inputs[3], Some("Extended_Pictographic"))
        .into_iter()
        .map(|(start, end, _)| format!("    (0x{start:X}, 0x{end:X}),\n"))
        .collect::<String>();
    format!(
        "const LINE_CLASSES: &[(u32, u32, LineClass)] = &[\n{line}];\n\
         const GENERAL_CATEGORIES: &[(u32, u32, GeneralCategory)] = &[\n{categories}];\n\
         const ASSIGNED_RANGES: &[(u32, u32)] = &[\n{assigned}];\n\
         const EAST_ASIAN_RANGES: &[(u32, u32)] = &[\n{east_asian}];\n\
         const EXTENDED_PICTOGRAPHIC_RANGES: &[(u32, u32)] = &[\n{extended_pictographic}];\n"
    )
}

fn property_ranges(path: &PathBuf, selected: Option<&str>) -> Vec<(u32, u32, &'static str)> {
    fs::read_to_string(path)
        .expect("read Unicode property data")
        .lines()
        .filter_map(|line| {
            let data = line.split('#').next().unwrap_or("").trim();
            let (range, value) = data.split_once(';')?;
            let value = value.trim();
            if selected.is_some_and(|selected| value != selected) {
                return None;
            }
            let (start, end) = code_range(range.trim());
            let value = Box::leak(value.to_owned().into_boxed_str());
            Some((start, end, &*value))
        })
        .collect()
}

fn unicode_categories(path: &PathBuf) -> (Vec<(u32, u32, &'static str)>, Vec<(u32, u32)>) {
    let mut special = Vec::new();
    let mut assigned = Vec::new();
    let mut pending = None;
    for line in fs::read_to_string(path).expect("read UnicodeData").lines() {
        let fields = line.split(';').collect::<Vec<_>>();
        let code = u32::from_str_radix(fields[0], 16).expect("UnicodeData code");
        let name = fields[1];
        let category = fields[2];
        if name.ends_with(", First>") {
            pending = Some((code, category));
            continue;
        }
        let (start, category) = if name.ends_with(", Last>") {
            pending.take().expect("UnicodeData range start")
        } else {
            (code, category)
        };
        push_range(&mut assigned, start, code);
        if matches!(category, "Mn" | "Mc" | "Pi" | "Pf") {
            let category = Box::leak(category.to_owned().into_boxed_str());
            push_property(&mut special, start, code, &*category);
        }
    }
    (special, assigned)
}

fn push_range(ranges: &mut Vec<(u32, u32)>, start: u32, end: u32) {
    if let Some(last) = ranges.last_mut().filter(|last| last.1 + 1 == start) {
        last.1 = end;
    } else {
        ranges.push((start, end));
    }
}

fn push_property(
    ranges: &mut Vec<(u32, u32, &'static str)>,
    start: u32,
    end: u32,
    value: &'static str,
) {
    if let Some(last) = ranges
        .last_mut()
        .filter(|last| last.1 + 1 == start && last.2 == value)
    {
        last.1 = end;
    } else {
        ranges.push((start, end, value));
    }
}

fn code_range(value: &str) -> (u32, u32) {
    let mut bounds = value.split("..");
    let start = u32::from_str_radix(bounds.next().expect("range start"), 16).expect("hex start");
    let end = bounds
        .next()
        .map_or(start, |end| u32::from_str_radix(end, 16).expect("hex end"));
    (start, end)
}
