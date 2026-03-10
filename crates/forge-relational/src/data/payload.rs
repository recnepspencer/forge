use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
}

impl From<Value> for RecordPayload {
    fn from(value: Value) -> Self {
        Self::StructuredJson(value)
    }
}
