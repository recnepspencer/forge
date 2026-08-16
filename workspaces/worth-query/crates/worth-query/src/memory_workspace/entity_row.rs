use std::collections::BTreeMap;
use worth_foundational::facade::{
    prepare_aspect_value_identity_basis, prepare_struct_aspect_value_identity_basis, AspectKey,
    AspectValue, CanonicalFieldPath, FieldKey, StructAspectValue,
};

use super::{
    WorthQueryEntityIdentity, WorthQueryEntityNativeReplacement,
    WorthQueryEntityNativeReplacementValue,
};

#[derive(Debug, Clone, PartialEq)]
pub struct WorthQueryEntity {
    identity: WorthQueryEntityIdentity,
    row: WorthQueryEntityRow,
}

#[derive(Debug, Clone, PartialEq)]
enum WorthQueryEntityRow {
    AspectProjection {
        aspect_values: BTreeMap<AspectKey, AspectValue>,
        struct_aspect_values: BTreeMap<AspectKey, StructAspectValue>,
        field_values: BTreeMap<CanonicalFieldPath, AspectValue>,
    },
}

impl WorthQueryEntity {
    pub(crate) fn from_aspect_projection(
        identity: WorthQueryEntityIdentity,
        aspect_values: BTreeMap<AspectKey, AspectValue>,
        struct_aspect_values: BTreeMap<AspectKey, StructAspectValue>,
        field_values: BTreeMap<CanonicalFieldPath, AspectValue>,
    ) -> Self {
        Self {
            identity,
            row: WorthQueryEntityRow::AspectProjection {
                aspect_values,
                struct_aspect_values,
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
                struct_aspect_values: BTreeMap::new(),
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

    pub fn struct_aspect_value(&self, aspect_key: &AspectKey) -> Option<&StructAspectValue> {
        match &self.row {
            WorthQueryEntityRow::AspectProjection {
                struct_aspect_values,
                ..
            } => struct_aspect_values.get(aspect_key),
        }
    }

    pub fn struct_aspect_values(
        &self,
    ) -> Box<dyn Iterator<Item = (&AspectKey, &StructAspectValue)> + '_> {
        match &self.row {
            WorthQueryEntityRow::AspectProjection {
                struct_aspect_values,
                ..
            } => Box::new(struct_aspect_values.iter()),
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

    pub(crate) fn with_native_field_values(
        mut self,
        values: impl IntoIterator<Item = (CanonicalFieldPath, AspectValue)>,
    ) -> Self {
        let WorthQueryEntityRow::AspectProjection { field_values, .. } = &mut self.row;
        field_values.extend(values);
        self
    }

    pub(crate) fn replace_native_values(
        mut self,
        replacements: impl IntoIterator<Item = WorthQueryEntityNativeReplacement>,
    ) -> Self {
        let WorthQueryEntityRow::AspectProjection {
            aspect_values,
            struct_aspect_values,
            field_values,
        } = &mut self.row;
        for replacement in replacements {
            replace_native_value(
                aspect_values,
                struct_aspect_values,
                field_values,
                replacement,
            );
        }
        self
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
                struct_aspect_values,
                field_values,
            } => aspect_values
                .iter()
                .map(|(aspect_key, value)| {
                    format!(
                        "aspect:{}={}",
                        aspect_key.as_str(),
                        prepare_aspect_value_identity_basis(value).as_str()
                    )
                })
                .chain(field_values.iter().map(|(field_path, value)| {
                    format!(
                        "field:{}={}",
                        terminal_projection_from_field_path(field_path),
                        prepare_aspect_value_identity_basis(value).as_str()
                    )
                }))
                .chain(struct_aspect_values.iter().map(|(aspect_key, value)| {
                    format!(
                        "struct-aspect:{}={}",
                        aspect_key.as_str(),
                        prepare_struct_aspect_value_identity_basis(value).as_str()
                    )
                }))
                .collect(),
        }
    }
}

fn replace_native_value(
    aspect_values: &mut BTreeMap<AspectKey, AspectValue>,
    struct_aspect_values: &mut BTreeMap<AspectKey, StructAspectValue>,
    field_values: &mut BTreeMap<CanonicalFieldPath, AspectValue>,
    replacement: WorthQueryEntityNativeReplacement,
) {
    let WorthQueryEntityNativeReplacement {
        aspect,
        field,
        canonical_paths,
        replace_aspect_storage,
        value,
    } = replacement;
    let field_is_none = field.is_none();
    for path in &canonical_paths {
        field_values.remove(path);
    }
    if replace_aspect_storage {
        match field {
            Some(field) => {
                aspect_values.remove(&aspect);
                let mut fields = retain_sibling_fields(struct_aspect_values, &aspect, &field);
                if let WorthQueryEntityNativeReplacementValue::Scalar(value) = &value {
                    fields.push((field, value.clone()));
                }
                if !fields.is_empty() {
                    struct_aspect_values.insert(
                        aspect.clone(),
                        StructAspectValue::new(fields)
                            .expect("retained native fields remain canonical"),
                    );
                }
            }
            None => {
                aspect_values.remove(&aspect);
                struct_aspect_values.remove(&aspect);
            }
        }
    }
    match value {
        WorthQueryEntityNativeReplacementValue::Scalar(value) => {
            if field_is_none && replace_aspect_storage {
                aspect_values.insert(aspect, value.clone());
            }
            for path in canonical_paths {
                field_values.insert(path, value.clone());
            }
        }
        WorthQueryEntityNativeReplacementValue::Struct(value) => {
            struct_aspect_values.insert(aspect, value);
        }
        WorthQueryEntityNativeReplacementValue::Absent => {}
    }
}

fn retain_sibling_fields(
    struct_aspect_values: &mut BTreeMap<AspectKey, StructAspectValue>,
    aspect: &AspectKey,
    replaced_field: &FieldKey,
) -> Vec<(FieldKey, AspectValue)> {
    struct_aspect_values
        .remove(aspect)
        .into_iter()
        .flat_map(|value| {
            value
                .fields()
                .filter(|(key, _)| *key != replaced_field)
                .map(|(key, value)| (key.clone(), value.clone()))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn terminal_projection_from_field_path(field_path: &CanonicalFieldPath) -> String {
    field_path
        .fields()
        .iter()
        .map(FieldKey::as_str)
        .collect::<Vec<_>>()
        .join(".")
}
