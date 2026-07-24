use std::collections::BTreeMap;
use std::sync::Arc;

use worth_foundational::facade::{
    prepare_aspect_value_identity_basis, prepare_struct_aspect_value_identity_basis, AspectKey,
    AspectValue, CanonicalFieldPath, FieldKey, StructAspectValue,
};

#[derive(Debug, PartialEq)]
pub struct WorthQueryGraphReadRow {
    entity_identity: Arc<str>,
    aspect_values: BTreeMap<AspectKey, AspectValue>,
    struct_aspect_values: BTreeMap<AspectKey, StructAspectValue>,
    field_values: BTreeMap<CanonicalFieldPath, AspectValue>,
}

impl WorthQueryGraphReadRow {
    pub fn new(
        entity_identity: impl Into<Arc<str>>,
        aspect_values: BTreeMap<AspectKey, AspectValue>,
        struct_aspect_values: BTreeMap<AspectKey, StructAspectValue>,
        field_values: BTreeMap<CanonicalFieldPath, AspectValue>,
    ) -> Result<Self, WorthQueryGraphReadRowConstructionDenial> {
        let entity_identity = entity_identity.into();
        if entity_identity.trim().is_empty() {
            return Err(WorthQueryGraphReadRowConstructionDenial::EmptyEntityIdentity);
        }
        Ok(Self {
            entity_identity,
            aspect_values,
            struct_aspect_values,
            field_values,
        })
    }

    pub fn from_native_fields(
        entity_identity: impl Into<Arc<str>>,
        field_values: BTreeMap<CanonicalFieldPath, AspectValue>,
    ) -> Result<Self, WorthQueryGraphReadRowConstructionDenial> {
        Self::new(
            entity_identity,
            BTreeMap::new(),
            BTreeMap::new(),
            field_values,
        )
    }

    pub fn entity_identity(&self) -> &str {
        &self.entity_identity
    }

    pub fn aspect_value(&self, key: &AspectKey) -> Option<&AspectValue> {
        self.aspect_values.get(key)
    }

    pub fn struct_aspect_value(&self, key: &AspectKey) -> Option<&StructAspectValue> {
        self.struct_aspect_values.get(key)
    }

    pub fn field_value(&self, path: &CanonicalFieldPath) -> Option<&AspectValue> {
        self.field_values.get(path)
    }

    pub(super) fn digest_parts(&self) -> Vec<String> {
        std::iter::once(format!("entity:{}", self.entity_identity))
            .chain(self.aspect_values.iter().map(|(key, value)| {
                format!(
                    "aspect:{}={}",
                    key.as_str(),
                    prepare_aspect_value_identity_basis(value).as_str()
                )
            }))
            .chain(self.struct_aspect_values.iter().map(|(key, value)| {
                format!(
                    "struct-aspect:{}={}",
                    key.as_str(),
                    prepare_struct_aspect_value_identity_basis(value).as_str()
                )
            }))
            .chain(self.field_values.iter().map(|(path, value)| {
                format!(
                    "field:{}={}",
                    canonical_field_path(path),
                    prepare_aspect_value_identity_basis(value).as_str()
                )
            }))
            .collect()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryGraphReadRowConstructionDenial {
    EmptyEntityIdentity,
}

fn canonical_field_path(path: &CanonicalFieldPath) -> String {
    path.fields()
        .iter()
        .map(FieldKey::as_str)
        .collect::<Vec<_>>()
        .join(".")
}
