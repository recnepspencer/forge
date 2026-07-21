use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

pub(crate) fn canonicalize_json(value: Value) -> Result<(Value, Vec<u8>), serde_json::Error> {
    let canonical_value = canonicalize_value(value);
    let canonical_bytes = serde_json::to_vec(&canonical_value)?;
    Ok((canonical_value, canonical_bytes))
}

fn canonicalize_value(value: Value) -> Value {
    match value {
        Value::Array(values) => Value::Array(values.into_iter().map(canonicalize_value).collect()),
        Value::Object(values) => {
            let mut rows = values.into_iter().collect::<Vec<_>>();
            rows.sort_by(|left, right| left.0.cmp(&right.0));
            let mut canonical = Map::new();
            for (name, value) in rows {
                canonical.insert(name, canonicalize_value(value));
            }
            Value::Object(canonical)
        }
        scalar => scalar,
    }
}

pub(crate) fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    super::hex_digest(digest.as_slice())
}
