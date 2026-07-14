use std::collections::BTreeMap;

use worth_foundational::facade::{AspectValue, CanonicalFieldPath, FieldKey};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WorthQueryRetainedFieldPath {
    path: CanonicalFieldPath,
}

impl WorthQueryRetainedFieldPath {
    pub fn from_canonical_field_path(path: CanonicalFieldPath) -> Self {
        Self { path }
    }

    pub fn canonical_field_path(&self) -> &CanonicalFieldPath {
        &self.path
    }

    pub(crate) fn terminal_projection_for_boundary(&self) -> String {
        self.path
            .fields()
            .iter()
            .map(FieldKey::as_str)
            .collect::<Vec<_>>()
            .join(".")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryRetainedMaterializedRow {
    scalar_values: BTreeMap<WorthQueryRetainedFieldPath, AspectValue>,
}

impl WorthQueryRetainedMaterializedRow {
    pub(crate) fn from_scalar_values(
        scalar_values: BTreeMap<WorthQueryRetainedFieldPath, AspectValue>,
    ) -> Result<Self, String> {
        if scalar_values.is_empty() {
            return Err("retained materialized row requires at least one scalar value".to_string());
        }
        Ok(Self { scalar_values })
    }

    pub fn field_value_at(&self, field_path: &WorthQueryRetainedFieldPath) -> Option<&AspectValue> {
        self.scalar_values.get(field_path)
    }

    pub fn scalar_values(
        &self,
    ) -> impl Iterator<Item = (&WorthQueryRetainedFieldPath, &AspectValue)> {
        self.scalar_values.iter()
    }

    pub(in crate::runtime) fn terminal_digest_parts(&self) -> Vec<String> {
        self.scalar_values
            .iter()
            .map(|(field_path, value)| {
                format!(
                    "{}={}",
                    field_path.terminal_projection_for_boundary(),
                    super::retained_scalar_values::retained_scalar_value_digest_text(value)
                )
            })
            .collect()
    }
}
