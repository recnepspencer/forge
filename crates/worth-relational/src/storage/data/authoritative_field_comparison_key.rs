use serde::{Deserialize, Serialize};
use worth_foundational::facade::AspectValue;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct AuthoritativeFieldComparisonKey {
    canonical_value_bytes: Vec<u8>,
}

impl AuthoritativeFieldComparisonKey {
    pub fn from_aspect_value(value: &AspectValue) -> Self {
        Self {
            canonical_value_bytes: canonical_aspect_value_bytes(value),
        }
    }

    pub fn canonical_value_bytes(&self) -> &[u8] {
        &self.canonical_value_bytes
    }

    pub fn owned_allocation_capacity_bytes(&self) -> u64 {
        self.canonical_value_bytes.capacity() as u64
    }
}

pub fn authoritative_aspect_value_field_comparison_key(
    value: &AspectValue,
) -> AuthoritativeFieldComparisonKey {
    AuthoritativeFieldComparisonKey::from_aspect_value(value)
}

fn canonical_aspect_value_bytes(value: &AspectValue) -> Vec<u8> {
    crate::aspect_wire::encode_aspect_value(value)
}

#[cfg(test)]
mod tests {
    use worth_foundational::facade::{AspectValue, InternedString};

    use super::AuthoritativeFieldComparisonKey;

    #[test]
    fn comparison_key_preserves_aspect_value_family_even_when_display_collides() {
        let int_key = AuthoritativeFieldComparisonKey::from_aspect_value(&AspectValue::Int64(1));
        let string_key = AuthoritativeFieldComparisonKey::from_aspect_value(&AspectValue::String(
            InternedString::Raw("1".to_string()),
        ));

        assert_ne!(int_key, string_key);
        assert_ne!(
            int_key.canonical_value_bytes(),
            string_key.canonical_value_bytes()
        );
    }

    #[test]
    fn comparison_key_reports_owned_capacity_instead_of_initialized_length() {
        let mut canonical_value_bytes = Vec::with_capacity(128);
        canonical_value_bytes.extend_from_slice(b"small");
        let key = AuthoritativeFieldComparisonKey {
            canonical_value_bytes,
        };

        assert_eq!(key.canonical_value_bytes().len(), 5);
        assert_eq!(key.owned_allocation_capacity_bytes(), 128);
        assert_ne!(
            key.owned_allocation_capacity_bytes(),
            key.canonical_value_bytes().len() as u64,
            "initialized length cannot stand in for an owned allocation's capacity"
        );
    }
}
