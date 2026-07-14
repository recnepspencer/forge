use std::path::PathBuf;

pub struct CargoArtifactMessage {
    pub target_name: String,
    pub filenames: Vec<PathBuf>,
}

pub fn parse(input: &str) -> Option<CargoArtifactMessage> {
    if string_field(input, "reason")?.0 != "compiler-artifact" {
        return None;
    }
    let target = object_field(input, "target")?;
    let target_name = string_field(target, "name")?.0;
    let kinds = string_array_field(target, "kind")?;
    if !kinds
        .iter()
        .any(|kind| kind == "lib" || kind == "proc-macro")
    {
        return None;
    }
    let filenames = string_array_field(input, "filenames")?
        .into_iter()
        .map(PathBuf::from)
        .collect();
    Some(CargoArtifactMessage {
        target_name,
        filenames,
    })
}

fn object_field<'a>(input: &'a str, field: &str) -> Option<&'a str> {
    let start = value_start(input, field)?;
    let bytes = input.as_bytes();
    if bytes.get(start) != Some(&b'{') {
        return None;
    }
    let end = matching_delimiter(bytes, start, b'{', b'}')?;
    input.get(start..=end)
}

fn string_field(input: &str, field: &str) -> Option<(String, usize)> {
    let mut cursor = value_start(input, field)?;
    parse_string(input.as_bytes(), &mut cursor).map(|value| (value, cursor))
}

fn string_array_field(input: &str, field: &str) -> Option<Vec<String>> {
    let mut cursor = value_start(input, field)?;
    let bytes = input.as_bytes();
    if bytes.get(cursor) != Some(&b'[') {
        return None;
    }
    cursor += 1;
    let mut values = Vec::new();
    loop {
        skip_whitespace(bytes, &mut cursor);
        match bytes.get(cursor)? {
            b']' => return Some(values),
            b'"' => values.push(parse_string(bytes, &mut cursor)?),
            _ => return None,
        }
        skip_whitespace(bytes, &mut cursor);
        match bytes.get(cursor)? {
            b',' => cursor += 1,
            b']' => return Some(values),
            _ => return None,
        }
    }
}

fn value_start(input: &str, field: &str) -> Option<usize> {
    let needle = format!("\"{field}\"");
    let field_start = input.find(&needle)?;
    let bytes = input.as_bytes();
    let mut cursor = field_start + needle.len();
    skip_whitespace(bytes, &mut cursor);
    if bytes.get(cursor) != Some(&b':') {
        return None;
    }
    cursor += 1;
    skip_whitespace(bytes, &mut cursor);
    Some(cursor)
}

fn matching_delimiter(bytes: &[u8], start: usize, open: u8, close: u8) -> Option<usize> {
    let mut depth = 0_u32;
    let mut cursor = start;
    let mut in_string = false;
    let mut escaped = false;
    while let Some(byte) = bytes.get(cursor).copied() {
        if in_string {
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                in_string = false;
            }
        } else if byte == b'"' {
            in_string = true;
        } else if byte == open {
            depth = depth.checked_add(1)?;
        } else if byte == close {
            depth = depth.checked_sub(1)?;
            if depth == 0 {
                return Some(cursor);
            }
        }
        cursor += 1;
    }
    None
}

fn parse_string(bytes: &[u8], cursor: &mut usize) -> Option<String> {
    if bytes.get(*cursor) != Some(&b'"') {
        return None;
    }
    *cursor += 1;
    let mut value = String::new();
    while let Some(byte) = bytes.get(*cursor).copied() {
        *cursor += 1;
        match byte {
            b'"' => return Some(value),
            b'\\' => value.push(parse_escape(bytes, cursor)?),
            byte if byte.is_ascii() => value.push(char::from(byte)),
            _ => return None,
        }
    }
    None
}

fn parse_escape(bytes: &[u8], cursor: &mut usize) -> Option<char> {
    let escaped = bytes.get(*cursor).copied()?;
    *cursor += 1;
    match escaped {
        b'"' => Some('"'),
        b'\\' => Some('\\'),
        b'/' => Some('/'),
        b'b' => Some('\u{0008}'),
        b'f' => Some('\u{000c}'),
        b'n' => Some('\n'),
        b'r' => Some('\r'),
        b't' => Some('\t'),
        b'u' => parse_unicode_escape(bytes, cursor),
        _ => None,
    }
}

fn parse_unicode_escape(bytes: &[u8], cursor: &mut usize) -> Option<char> {
    let digits = bytes.get(*cursor..cursor.checked_add(4)?)?;
    *cursor += 4;
    let encoded = std::str::from_utf8(digits).ok()?;
    char::from_u32(u32::from_str_radix(encoded, 16).ok()?)
}

fn skip_whitespace(bytes: &[u8], cursor: &mut usize) {
    while bytes.get(*cursor).is_some_and(u8::is_ascii_whitespace) {
        *cursor += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn parses_library_artifact_with_windows_path() {
        let message = parse(
            r#"{"reason":"compiler-artifact","target":{"kind":["lib"],"name":"forge-store-certification"},"filenames":["C:\\target\\libforge_store_certification.rlib"]}"#,
        )
        .expect("compiler artifact parses");

        assert_eq!(message.target_name, "forge-store-certification");
        assert_eq!(
            message.filenames[0].to_string_lossy(),
            r"C:\target\libforge_store_certification.rlib"
        );
    }
}
