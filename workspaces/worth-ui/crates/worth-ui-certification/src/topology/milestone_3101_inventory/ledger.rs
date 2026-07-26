use std::fs;
use std::path::Path;

pub(super) fn load(path: &Path) -> Result<toml::Value, String> {
    let text = fs::read_to_string(path)
        .map_err(|error| format!("{} should be readable: {error}", path.display()))?;
    text.parse::<toml::Value>()
        .map_err(|error| format!("{} should be valid TOML: {error}", path.display()))
}

pub(super) fn tables<'a>(
    document: &'a toml::Value,
    key: &str,
) -> Result<&'a Vec<toml::Value>, String> {
    document
        .get(key)
        .and_then(toml::Value::as_array)
        .ok_or_else(|| format!("ledger should contain [[{key}]] rows"))
}

pub(super) fn text<'a>(row: &'a toml::Value, key: &str) -> Result<&'a str, String> {
    row.get(key)
        .and_then(toml::Value::as_str)
        .ok_or_else(|| format!("ledger row should contain string `{key}`"))
}

pub(super) fn optional_text<'a>(row: &'a toml::Value, key: &str) -> Option<&'a str> {
    row.get(key).and_then(toml::Value::as_str)
}

pub(super) fn integer(row: &toml::Value, key: &str) -> Result<i64, String> {
    row.get(key)
        .and_then(toml::Value::as_integer)
        .ok_or_else(|| format!("ledger row should contain integer `{key}`"))
}

pub(super) fn strings<'a>(row: &'a toml::Value, key: &str) -> Result<Vec<&'a str>, String> {
    row.get(key)
        .and_then(toml::Value::as_array)
        .ok_or_else(|| format!("ledger row should contain array `{key}`"))?
        .iter()
        .map(|value| {
            value
                .as_str()
                .ok_or_else(|| format!("`{key}` entries should be strings"))
        })
        .collect()
}

pub(super) fn fingerprint(text: impl AsRef<[u8]>) -> String {
    let mut hash = 14_695_981_039_346_656_037_u64;
    for byte in text.as_ref() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(1_099_511_628_211);
    }
    format!("{hash:016x}")
}
