use forge_foundational::facade::AspectKey;

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

    pub(crate) fn new(
        source_aspect: impl Into<String>,
        source_field: impl Into<String>,
        binding_index: usize,
    ) -> Option<Self> {
        let source_aspect = source_aspect.into();
        let source_field = source_field.into();
        let source_field_key =
            AspectFieldKey::new(source_aspect.clone(), source_field.clone()).ok()?;
        let binding_aspect_key = AspectKey::new(format!("{source_aspect}.{source_field}"))?;
        Some(Self {
            source_field_key,
            binding_aspect_key,
            binding_index,
        })
    }
}
