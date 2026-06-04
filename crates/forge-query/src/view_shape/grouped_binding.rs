use forge_foundational::facade::AspectKey;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryResultBindingProof {
    source_aspect: String,
    source_field: String,
    binding_aspect_key: AspectKey,
    binding_index: usize,
}

impl QueryResultBindingProof {
    pub fn source_aspect(&self) -> &str {
        &self.source_aspect
    }

    pub fn source_field(&self) -> &str {
        &self.source_field
    }

    pub fn binding_index(&self) -> usize {
        self.binding_index
    }

    pub fn field_key(&self) -> String {
        self.binding_aspect_key.as_str().to_string()
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
        let binding_aspect_key = AspectKey::new(format!("{source_aspect}.{source_field}"))?;
        Some(Self {
            source_aspect,
            source_field,
            binding_aspect_key,
            binding_index,
        })
    }
}
