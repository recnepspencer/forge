use forge_foundational::facade::{AspectKey, FieldKey};

use crate::authoring::AspectFieldKey;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryResultBindingProof {
    source_field_key: AspectFieldKey,
    binding_aspect_key: AspectKey,
    binding_index: usize,
}

impl QueryResultBindingProof {
    pub fn source_field_key(&self) -> &AspectFieldKey {
        &self.source_field_key
    }

    pub fn binding_index(&self) -> usize {
        self.binding_index
    }

    pub fn native_binding_aspect_key(&self) -> &AspectKey {
        &self.binding_aspect_key
    }

    pub(crate) fn from_native_source_keys(
        source_aspect: &AspectKey,
        source_field: &FieldKey,
        binding_index: usize,
    ) -> Option<Self> {
        let source_field_key = AspectFieldKey::from_native_keys(source_aspect, source_field);
        let binding_aspect_key = AspectKey::new(format!(
            "{}.{}",
            source_aspect.as_str(),
            source_field.as_str()
        ))?;
        Some(Self {
            source_field_key,
            binding_aspect_key,
            binding_index,
        })
    }
}
