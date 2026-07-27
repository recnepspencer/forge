use std::fs;
use std::path::Path;

use serde_json::Value as JsonValue;
use toml::Value as TomlValue;

pub(super) fn load_text(path: &Path) -> Result<String, String> {
    fs::read_to_string(path)
        .map_err(|error| format!("`{}` should be readable: {error}", path.display()))
}

pub(super) fn load_toml(path: &Path) -> Result<TomlValue, String> {
    let text = load_text(path)?;
    text.parse::<TomlValue>()
        .map_err(|error| format!("`{}` should parse as TOML: {error}", path.display()))
}

pub(super) fn load_json(path: &Path) -> Result<JsonValue, String> {
    let text = load_text(path)?;
    serde_json::from_str(&text)
        .map_err(|error| format!("`{}` should parse as JSON: {error}", path.display()))
}

pub(super) fn canonical_fingerprint(text: &str) -> String {
    let canonical = text.replace("\r\n", "\n").replace('\r', "\n");
    let mut hash = 14_695_981_039_346_656_037_u64;
    for byte in canonical.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(1_099_511_628_211);
    }
    format!("{hash:016x}")
}

pub(super) fn toml_rows<'a>(
    document: &'a TomlValue,
    family: &str,
) -> Result<&'a Vec<TomlValue>, String> {
    document
        .get(family)
        .and_then(TomlValue::as_array)
        .ok_or_else(|| format!("Phase 1 inventory should contain `[[{family}]]` rows"))
}

pub(super) fn toml_text<'a>(value: &'a TomlValue, field: &str) -> Result<&'a str, String> {
    value
        .get(field)
        .and_then(TomlValue::as_str)
        .filter(|text| !text.trim().is_empty())
        .ok_or_else(|| format!("Phase 1 TOML field `{field}` should contain text"))
}

pub(super) fn toml_texts<'a>(value: &'a TomlValue, field: &str) -> Result<Vec<&'a str>, String> {
    value
        .get(field)
        .and_then(TomlValue::as_array)
        .ok_or_else(|| format!("Phase 1 TOML field `{field}` should be an array"))?
        .iter()
        .map(|entry| {
            entry
                .as_str()
                .filter(|text| !text.trim().is_empty())
                .ok_or_else(|| format!("Phase 1 TOML field `{field}` entries should be text"))
        })
        .collect()
}

pub(super) fn json_object<'a>(value: &'a JsonValue, field: &str) -> Result<&'a JsonValue, String> {
    value
        .get(field)
        .filter(|entry| entry.is_object())
        .ok_or_else(|| format!("Phase 1 baseline should contain object `{field}`"))
}

pub(super) fn json_text<'a>(value: &'a JsonValue, field: &str) -> Result<&'a str, String> {
    value
        .get(field)
        .and_then(JsonValue::as_str)
        .filter(|text| !text.trim().is_empty())
        .ok_or_else(|| format!("Phase 1 baseline field `{field}` should contain text"))
}

pub(super) fn json_integer(value: &JsonValue, field: &str) -> Result<i64, String> {
    value
        .get(field)
        .and_then(JsonValue::as_i64)
        .ok_or_else(|| format!("Phase 1 baseline field `{field}` should be an integer"))
}

pub(super) fn json_number(value: &JsonValue, field: &str) -> Result<f64, String> {
    value
        .get(field)
        .and_then(JsonValue::as_f64)
        .filter(|number| number.is_finite() && *number >= 0.0)
        .ok_or_else(|| format!("Phase 1 baseline field `{field}` should be a finite number"))
}
