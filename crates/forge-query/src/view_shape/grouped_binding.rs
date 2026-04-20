#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryResultBindingProof {
    source_aspect: String,
    source_field: String,
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
        format!("{}.{}", self.source_aspect, self.source_field)
    }

    pub(crate) fn new(
        source_aspect: impl Into<String>,
        source_field: impl Into<String>,
        binding_index: usize,
    ) -> Self {
        Self {
            source_aspect: source_aspect.into(),
            source_field: source_field.into(),
            binding_index,
        }
    }
}
