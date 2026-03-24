use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PayloadClass {
    StructuredJson,
    OpaqueBytes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PayloadEncoding {
    JsonDocument,
    RawBytes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PayloadCompatibility {
    Compatible,
    StructuredOnly,
    OpaqueOnly,
    Incompatible,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PayloadPolicy {
    pub default_class: PayloadClass,
    pub allow_opaque_bytes: bool,
}

impl Default for PayloadPolicy {
    fn default() -> Self {
        Self {
            default_class: PayloadClass::StructuredJson,
            allow_opaque_bytes: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecordPayload {
    StructuredJson(Value),
    OpaqueBytes(Vec<u8>),
}

impl RecordPayload {
    pub fn payload_class(&self) -> PayloadClass {
        match self {
            Self::StructuredJson(_) => PayloadClass::StructuredJson,
            Self::OpaqueBytes(_) => PayloadClass::OpaqueBytes,
        }
    }

    pub fn as_json(&self) -> Option<&Value> {
        match self {
            Self::StructuredJson(value) => Some(value),
            Self::OpaqueBytes(_) => None,
        }
    }

    pub fn canonicalized(&self) -> Self {
        match self {
            Self::StructuredJson(value) => Self::StructuredJson(canonicalize_json(value)),
            Self::OpaqueBytes(bytes) => Self::OpaqueBytes(bytes.clone()),
        }
    }
}

impl From<Value> for RecordPayload {
    fn from(value: Value) -> Self {
        Self::StructuredJson(canonicalize_json(&value))
    }
}

pub fn canonicalize_json(value: &Value) -> Value {
    match value {
        Value::Null => Value::Null,
        Value::Bool(boolean) => Value::Bool(*boolean),
        Value::Number(number) => Value::Number(number.clone()),
        Value::String(text) => Value::String(text.clone()),
        Value::Array(values) => Value::Array(values.iter().map(canonicalize_json).collect()),
        Value::Object(entries) => {
            let mut ordered = serde_json::Map::new();
            let mut keys = entries.keys().cloned().collect::<Vec<_>>();
            keys.sort();
            for key in keys {
                let value = entries
                    .get(&key)
                    .expect("object key collected from map must exist");
                ordered.insert(key, canonicalize_json(value));
            }
            Value::Object(ordered)
        }
    }
}
