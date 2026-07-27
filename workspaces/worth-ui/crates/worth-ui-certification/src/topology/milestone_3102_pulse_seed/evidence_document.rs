use std::fs;
use std::path::Path;

pub(super) fn load_toml(path: &Path) -> Result<toml::Value, String> {
    let text = fs::read_to_string(path)
        .map_err(|error| format!("{} should be readable: {error}", path.display()))?;
    text.parse::<toml::Value>()
        .map_err(|error| format!("{} should be valid TOML: {error}", path.display()))
}

pub(super) fn load_json(path: &Path) -> Result<serde_json::Value, String> {
    let text = fs::read_to_string(path)
        .map_err(|error| format!("{} should be readable: {error}", path.display()))?;
    serde_json::from_str(&text)
        .map_err(|error| format!("{} should be valid JSON: {error}", path.display()))
}

pub(super) fn toml_rows<'a>(
    document: &'a toml::Value,
    key: &str,
) -> Result<&'a [toml::Value], String> {
    document
        .get(key)
        .and_then(toml::Value::as_array)
        .map(Vec::as_slice)
        .ok_or_else(|| format!("Phase 1 evidence should contain [[{key}]] rows"))
}

pub(super) fn toml_text<'a>(row: &'a toml::Value, key: &str) -> Result<&'a str, String> {
    row.get(key)
        .and_then(toml::Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| format!("Phase 1 evidence field `{key}` should be non-empty text"))
}

pub(super) fn toml_texts<'a>(row: &'a toml::Value, key: &str) -> Result<Vec<&'a str>, String> {
    row.get(key)
        .and_then(toml::Value::as_array)
        .ok_or_else(|| format!("Phase 1 evidence field `{key}` should be an array"))?
        .iter()
        .map(|value| {
            value
                .as_str()
                .filter(|text| !text.trim().is_empty())
                .ok_or_else(|| format!("Phase 1 evidence `{key}` entries should be text"))
        })
        .collect()
}

pub(super) fn row_ids(document: &toml::Value, key: &str) -> Result<Vec<String>, String> {
    toml_rows(document, key)?
        .iter()
        .map(|row| toml_text(row, "id").map(str::to_owned))
        .collect()
}

pub(super) fn require_exact_ids(
    document: &toml::Value,
    key: &str,
    expected: &[&str],
) -> Result<(), String> {
    let mut actual = row_ids(document, key)?;
    actual.sort();
    actual.dedup();
    let mut expected = expected
        .iter()
        .map(|id| (*id).to_owned())
        .collect::<Vec<_>>();
    expected.sort();
    if actual == expected {
        return Ok(());
    }
    Err(format!(
        "Phase 1 `{key}` ids should be exactly {expected:?}; found {actual:?}"
    ))
}
