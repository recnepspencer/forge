use std::collections::BTreeMap;
use std::fmt;

use serde::de::{MapAccess, SeqAccess, Visitor};
use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ExternalSerdeDiagnosticProjectionValue {
    Null,
    Bool(bool),
    Unsigned(u64),
    Signed(i64),
    String(String),
    Array(Vec<ExternalSerdeDiagnosticProjectionValue>),
    Object(BTreeMap<String, ExternalSerdeDiagnosticProjectionValue>),
}

impl Serialize for ExternalSerdeDiagnosticProjectionValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Null => serializer.serialize_none(),
            Self::Bool(value) => serializer.serialize_bool(*value),
            Self::Unsigned(value) => serializer.serialize_u64(*value),
            Self::Signed(value) => serializer.serialize_i64(*value),
            Self::String(value) => serializer.serialize_str(value),
            Self::Array(values) => {
                let mut sequence = serializer.serialize_seq(Some(values.len()))?;
                for value in values {
                    sequence.serialize_element(value)?;
                }
                sequence.end()
            }
            Self::Object(fields) => {
                let mut map = serializer.serialize_map(Some(fields.len()))?;
                for (key, value) in fields {
                    map.serialize_entry(key, value)?;
                }
                map.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for ExternalSerdeDiagnosticProjectionValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ExternalSerdeDiagnosticProjectionVisitor)
    }
}

struct ExternalSerdeDiagnosticProjectionVisitor;

impl<'de> Visitor<'de> for ExternalSerdeDiagnosticProjectionVisitor {
    type Value = ExternalSerdeDiagnosticProjectionValue;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a diagnostic serde projection value")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
        Ok(ExternalSerdeDiagnosticProjectionValue::Bool(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
        Ok(ExternalSerdeDiagnosticProjectionValue::Signed(value))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
        Ok(ExternalSerdeDiagnosticProjectionValue::Unsigned(value))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E> {
        Ok(ExternalSerdeDiagnosticProjectionValue::String(
            value.to_string(),
        ))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(ExternalSerdeDiagnosticProjectionValue::String(
            value.to_string(),
        ))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E> {
        Ok(ExternalSerdeDiagnosticProjectionValue::String(value))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E> {
        Ok(ExternalSerdeDiagnosticProjectionValue::Null)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(ExternalSerdeDiagnosticProjectionValue::Null)
    }

    fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut values = Vec::new();
        while let Some(value) = sequence.next_element()? {
            values.push(value);
        }
        Ok(ExternalSerdeDiagnosticProjectionValue::Array(values))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut fields = BTreeMap::new();
        while let Some((key, value)) = map.next_entry()? {
            fields.insert(key, value);
        }
        Ok(ExternalSerdeDiagnosticProjectionValue::Object(fields))
    }
}
