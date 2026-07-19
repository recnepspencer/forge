use super::{CanonicalizationWarning, NormalizationEvent};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CompatibilityEvidence {
    Compatible,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityFreezeEvidence {
    pub query_digest: String,
    pub result_shape_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalizationReport {
    warnings: Vec<CanonicalizationWarning>,
    events: Vec<NormalizationEvent>,
    compatibility: CompatibilityEvidence,
    normalized_projection_entries: usize,
    normalized_traversal_entries: usize,
    normalized_result_fields: usize,
    identity_freeze: IdentityFreezeEvidence,
}

impl CanonicalizationReport {
    pub fn new(
        warnings: Vec<CanonicalizationWarning>,
        events: Vec<NormalizationEvent>,
        compatibility: CompatibilityEvidence,
        normalized_projection_entries: usize,
        normalized_traversal_entries: usize,
        normalized_result_fields: usize,
        identity_freeze: IdentityFreezeEvidence,
    ) -> Self {
        Self {
            warnings,
            events,
            compatibility,
            normalized_projection_entries,
            normalized_traversal_entries,
            normalized_result_fields,
            identity_freeze,
        }
    }

    pub fn warnings(&self) -> &[CanonicalizationWarning] {
        &self.warnings
    }

    pub fn events(&self) -> &[NormalizationEvent] {
        &self.events
    }

    pub fn compatibility(&self) -> &CompatibilityEvidence {
        &self.compatibility
    }

    pub fn normalized_projection_entries(&self) -> usize {
        self.normalized_projection_entries
    }

    pub fn normalized_traversal_entries(&self) -> usize {
        self.normalized_traversal_entries
    }

    pub fn normalized_result_fields(&self) -> usize {
        self.normalized_result_fields
    }

    pub fn identity_freeze(&self) -> &IdentityFreezeEvidence {
        &self.identity_freeze
    }
}
