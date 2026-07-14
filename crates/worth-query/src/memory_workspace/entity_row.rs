use std::collections::BTreeMap;
use worth_foundational::facade::{AspectKey, AspectValue, CanonicalFieldPath, FieldKey};

use super::WorthQueryEntityIdentity;

#[derive(Debug, Clone, PartialEq)]
pub struct WorthQueryEntity {
    identity: WorthQueryEntityIdentity,
    row: WorthQueryEntityRow,
}

#[derive(Debug, Clone, PartialEq)]
enum WorthQueryEntityRow {
    AspectProjection {
        aspect_values: BTreeMap<AspectKey, AspectValue>,
        field_values: BTreeMap<CanonicalFieldPath, AspectValue>,
    },
}

impl WorthQueryEntity {
    pub(crate) fn from_aspect_projection(
        identity: WorthQueryEntityIdentity,
        aspect_values: BTreeMap<AspectKey, AspectValue>,
        field_values: BTreeMap<CanonicalFieldPath, AspectValue>,
    ) -> Self {
        Self {
            identity,
            row: WorthQueryEntityRow::AspectProjection {
                aspect_values,
                field_values,
            },
        }
    }

    pub fn from_native_field_values(
        identity: WorthQueryEntityIdentity,
        field_values: BTreeMap<CanonicalFieldPath, AspectValue>,
    ) -> Self {
        Self {
            identity,
            row: WorthQueryEntityRow::AspectProjection {
                aspect_values: BTreeMap::new(),
                field_values,
            },
        }
    }

    pub fn identity(&self) -> &WorthQueryEntityIdentity {
        &self.identity
    }

    pub fn aspect_value(&self, aspect_key: &AspectKey) -> Option<&AspectValue> {
        match &self.row {
            WorthQueryEntityRow::AspectProjection { aspect_values, .. } => {
                aspect_values.get(aspect_key)
            }
        }
    }

    pub fn aspect_values(&self) -> Box<dyn Iterator<Item = (&AspectKey, &AspectValue)> + '_> {
        match &self.row {
            WorthQueryEntityRow::AspectProjection { aspect_values, .. } => {
                Box::new(aspect_values.iter())
            }
        }
    }

    pub(crate) fn native_field_values(
        &self,
    ) -> Box<dyn Iterator<Item = (&CanonicalFieldPath, &AspectValue)> + '_> {
        match &self.row {
            WorthQueryEntityRow::AspectProjection { field_values, .. } => {
                Box::new(field_values.iter())
            }
        }
    }

    pub fn scalar_value_at(&self, field_path: &CanonicalFieldPath) -> Option<&AspectValue> {
        match &self.row {
            WorthQueryEntityRow::AspectProjection { field_values, .. } => {
                field_values.get(field_path)
            }
        }
    }

    pub fn terminal_field_value_projection(&self) -> BTreeMap<String, AspectValue> {
        match &self.row {
            WorthQueryEntityRow::AspectProjection { field_values, .. } => field_values
                .iter()
                .map(|(field_path, value)| {
                    (
                        terminal_projection_from_field_path(field_path),
                        value.clone(),
                    )
                })
                .collect(),
        }
    }

    pub(crate) fn terminal_result_digest_parts(&self) -> Vec<String> {
        match &self.row {
            WorthQueryEntityRow::AspectProjection {
                aspect_values,
                field_values,
            } => aspect_values
                .iter()
                .map(|(aspect_key, value)| format!("aspect:{}={value:?}", aspect_key.as_str()))
                .chain(field_values.iter().map(|(field_path, value)| {
                    format!(
                        "field:{}={value:?}",
                        terminal_projection_from_field_path(field_path)
                    )
                }))
                .collect(),
        }
    }
}

fn terminal_projection_from_field_path(field_path: &CanonicalFieldPath) -> String {
    field_path
        .fields()
        .iter()
        .map(FieldKey::as_str)
        .collect::<Vec<_>>()
        .join(".")
}
