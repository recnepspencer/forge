use std::fmt::Write;
use std::path::PathBuf;

use serde_json::Value;
use sha2::{Digest, Sha256};

const KEY_BYTES: usize = 32;
const BLOCK_BYTES: usize = 64;

pub(super) fn validate(artifact: &Value) -> Result<(), String> {
    let tag = artifact
        .get("runner_authentication")
        .and_then(Value::as_str)
        .ok_or_else(|| "dependency artifact omits runner authentication".to_owned())?;
    let mut unsigned = artifact.clone();
    unsigned
        .as_object_mut()
        .ok_or_else(|| "dependency artifact is not an object".to_owned())?
        .remove("runner_authentication");
    validate_with_key(&unsigned, tag, &runner_key()?)
}

fn validate_with_key(value: &Value, tag: &str, key: &[u8]) -> Result<(), String> {
    let expected = hmac_sha256(key, canonical_json(value).as_bytes());
    let expected = expected.iter().fold(String::new(), |mut output, byte| {
        write!(output, "{byte:02x}").expect("string writes are infallible");
        output
    });
    let mismatch = expected
        .as_bytes()
        .iter()
        .zip(tag.as_bytes())
        .fold(expected.len() ^ tag.len(), |difference, (left, right)| {
            difference | usize::from(left ^ right)
        });
    (mismatch == 0)
        .then_some(())
        .ok_or_else(|| "dependency artifact lacks runner provenance".to_owned())
}

fn runner_key() -> Result<Vec<u8>, String> {
    let identity = runner_key_identity()?;
    let metadata = std::fs::symlink_metadata(&identity)
        .map_err(|error| format!("cannot inspect ledger runner key: {error}"))?;
    if metadata.file_type().is_symlink() {
        return Err("ledger runner key cannot be a symbolic link".to_owned());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if metadata.permissions().mode() & 0o077 != 0 {
            return Err("ledger runner key permissions are not private".to_owned());
        }
    }
    let repository = super::source_digest::repository_root()
        .canonicalize()
        .map_err(|error| error.to_string())?;
    let resolved = identity
        .canonicalize()
        .map_err(|error| format!("cannot read ledger runner key: {error}"))?;
    if resolved.starts_with(repository) {
        return Err("ledger runner key must remain outside the repository".to_owned());
    }
    let key = std::fs::read(resolved).map_err(|error| error.to_string())?;
    (key.len() == KEY_BYTES)
        .then_some(key)
        .ok_or_else(|| "ledger runner key has an invalid length".to_owned())
}

fn runner_key_identity() -> Result<PathBuf, String> {
    #[cfg(target_os = "windows")]
    let base = std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .or_else(|| {
            std::env::var_os("USERPROFILE")
                .map(PathBuf::from)
                .map(|home| home.join("AppData/Local"))
        });
    #[cfg(not(target_os = "windows"))]
    let base = std::env::var_os("XDG_STATE_HOME")
        .map(PathBuf::from)
        .or_else(|| {
            std::env::var_os("HOME")
                .map(PathBuf::from)
                .map(|home| home.join(".local/state"))
        });
    base.map(|base| base.join("Worth/ledger-runner/ledger-runner-hmac-v1.key"))
        .ok_or_else(|| "cannot locate ledger runner key".to_owned())
}

fn hmac_sha256(key: &[u8], message: &[u8]) -> [u8; 32] {
    let mut block = [0_u8; BLOCK_BYTES];
    if key.len() > BLOCK_BYTES {
        block[..32].copy_from_slice(&Sha256::digest(key));
    } else {
        block[..key.len()].copy_from_slice(key);
    }
    let mut inner = [0x36_u8; BLOCK_BYTES];
    let mut outer = [0x5c_u8; BLOCK_BYTES];
    for ((inner, outer), key) in inner.iter_mut().zip(&mut outer).zip(block) {
        *inner ^= key;
        *outer ^= key;
    }
    let mut digest = Sha256::new();
    digest.update(inner);
    digest.update(message);
    let inner = digest.finalize();
    let mut digest = Sha256::new();
    digest.update(outer);
    digest.update(inner);
    digest.finalize().into()
}

fn canonical_json(value: &Value) -> String {
    match value {
        Value::Null => "null".to_owned(),
        Value::Bool(value) => value.to_string(),
        Value::Number(value) => value.to_string(),
        Value::String(value) => canonical_string(value),
        Value::Array(values) => format!(
            "[{}]",
            values
                .iter()
                .map(canonical_json)
                .collect::<Vec<_>>()
                .join(",")
        ),
        Value::Object(values) => {
            let mut fields = values.iter().collect::<Vec<_>>();
            fields.sort_unstable_by_key(|(key, _)| *key);
            format!(
                "{{{}}}",
                fields
                    .into_iter()
                    .map(|(key, value)| format!(
                        "{}:{}",
                        canonical_string(key),
                        canonical_json(value)
                    ))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        }
    }
}

fn canonical_string(value: &str) -> String {
    let mut output = String::from("\"");
    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\u{8}' => output.push_str("\\b"),
            '\t' => output.push_str("\\t"),
            '\n' => output.push_str("\\n"),
            '\u{c}' => output.push_str("\\f"),
            '\r' => output.push_str("\\r"),
            character if (' '..='~').contains(&character) => output.push(character),
            character if u32::from(character) <= 0xffff => {
                write!(output, "\\u{:04x}", u32::from(character)).unwrap();
            }
            character => {
                let scalar = u32::from(character) - 0x1_0000;
                write!(
                    output,
                    "\\u{:04x}\\u{:04x}",
                    0xd800 + (scalar >> 10),
                    0xdc00 + (scalar & 0x3ff)
                )
                .unwrap();
            }
        }
    }
    output.push('"');
    output
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{canonical_json, validate_with_key};

    #[test]
    fn canonical_authentication_rejects_content_or_tag_substitution() {
        let value = json!({"z": "😀\n", "a": [true, 3]});
        assert_eq!(
            canonical_json(&value),
            r#"{"a":[true,3],"z":"\ud83d\ude00\n"}"#
        );
        let key = [0x0b; 20];
        let tag = "69ab6c68d61a3b85453ab106b356640d2d3f8d4945c67781423b7a8d48565a02";
        validate_with_key(&json!("Hi There"), tag, &key).unwrap();
        assert!(validate_with_key(&json!("Hi There!"), tag, &key).is_err());
        assert!(validate_with_key(&json!("Hi There"), "0", &key).is_err());
    }
}
