use forge_foundational::facade::{AspectValue, InternedString};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct AuthoritativeFieldComparisonKey {
    canonical_value_bytes: Vec<u8>,
    display_value: String,
}

impl AuthoritativeFieldComparisonKey {
    pub fn from_aspect_value(value: &AspectValue) -> Self {
        Self {
            canonical_value_bytes: canonical_aspect_value_bytes(value),
            display_value: display_value_for_aspect_value(value),
        }
    }

    pub fn canonical_value_bytes(&self) -> &[u8] {
        &self.canonical_value_bytes
    }

    pub fn display_value(&self) -> &str {
        &self.display_value
    }
}

pub fn authoritative_aspect_value_field_comparison_key(
    value: &AspectValue,
) -> AuthoritativeFieldComparisonKey {
    AuthoritativeFieldComparisonKey::from_aspect_value(value)
}

fn canonical_aspect_value_bytes(value: &AspectValue) -> Vec<u8> {
    crate::aspect_wire::encode_aspect_value(value)
        .unwrap_or_else(|error| unsupported_aspect_value_comparison_bytes(error.detail()))
}

fn unsupported_aspect_value_comparison_bytes(detail: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.push(0xff);
    bytes.extend_from_slice(&(detail.len() as u32).to_le_bytes());
    bytes.extend_from_slice(detail.as_bytes());
    bytes
}

fn display_value_for_aspect_value(value: &AspectValue) -> String {
    match value {
        AspectValue::Null => "null".to_string(),
        AspectValue::Bool(value) => value.to_string(),
        AspectValue::Int8(value) => value.to_string(),
        AspectValue::Int16(value) => value.to_string(),
        AspectValue::Int32(value) => value.to_string(),
        AspectValue::Int64(value) => value.to_string(),
        AspectValue::UInt8(value) => value.to_string(),
        AspectValue::UInt16(value) => value.to_string(),
        AspectValue::UInt32(value) => value.to_string(),
        AspectValue::UInt64(value) => value.to_string(),
        AspectValue::Float32(value) => format!("f32-bits:{}", value.bits()),
        AspectValue::Float64(value) => format!("f64-bits:{}", value.bits()),
        AspectValue::Decimal(value) => value.as_str().to_string(),
        AspectValue::BigInt(value) => value.as_str().to_string(),
        AspectValue::Rational(value) => {
            format!(
                "{}/{}",
                value.numerator.as_str(),
                value.denominator.as_str()
            )
        }
        AspectValue::String(value) => interned_string_field_comparison_key(value),
        AspectValue::Bytes(value) => format!("bytes-ref:{}", value.0),
        AspectValue::Uuid(value) => value.iter().map(|byte| format!("{byte:02x}")).collect(),
        AspectValue::Date(value) => value.days_from_unix_epoch.to_string(),
        AspectValue::Time(value) => value.nanos_since_midnight.to_string(),
        AspectValue::Timestamp(value) => value.micros_since_unix_epoch.to_string(),
        AspectValue::TimestampTz(value) => format!(
            "{}:{}",
            value.utc_micros_since_unix_epoch, value.offset_minutes
        ),
        AspectValue::EntityRef(value) => format!(
            "entity:{}:{}:{}",
            value.partition_id.0, value.local_slot.0, value.generation.0
        ),
        AspectValue::ContentRef(value) => format!("content-ref:{}", value.0),
    }
}

fn interned_string_field_comparison_key(value: &InternedString) -> String {
    match value {
        InternedString::Raw(value) => value.clone(),
        InternedString::Symbol(symbol) => format!("symbol:{}", symbol.0),
    }
}

#[cfg(test)]
mod tests {
    use forge_foundational::facade::{AspectValue, InternedString};

    use super::AuthoritativeFieldComparisonKey;

    #[test]
    fn comparison_key_preserves_aspect_value_family_even_when_display_collides() {
        let int_key = AuthoritativeFieldComparisonKey::from_aspect_value(&AspectValue::Int64(1));
        let string_key = AuthoritativeFieldComparisonKey::from_aspect_value(&AspectValue::String(
            InternedString::Raw("1".to_string()),
        ));

        assert_eq!(int_key.display_value(), string_key.display_value());
        assert_ne!(int_key, string_key);
        assert_ne!(
            int_key.canonical_value_bytes(),
            string_key.canonical_value_bytes()
        );
    }
}
