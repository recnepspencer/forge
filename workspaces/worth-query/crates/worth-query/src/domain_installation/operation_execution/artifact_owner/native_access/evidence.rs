use worth_foundational::facade::AspectKey;
use worth_query_installation::facade::WorthQueryArtifactNativeLayoutReference;

use super::WorthQueryArtifactNativeAccessCounters;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryArtifactNativeAccessEvidence {
    artifact_occurrence_identity: String,
    basis_identity: String,
    provider_session_identity: String,
    layout: WorthQueryArtifactNativeLayoutReference,
    requested_fields: Vec<AspectKey>,
    borrow_generation: u64,
    counters: WorthQueryArtifactNativeAccessCounters,
}

impl WorthQueryArtifactNativeAccessEvidence {
    pub(crate) fn new(
        artifact_occurrence_identity: String,
        basis_identity: String,
        provider_session_identity: String,
        layout: WorthQueryArtifactNativeLayoutReference,
        requested_fields: Vec<AspectKey>,
        borrow_generation: u64,
        counters: WorthQueryArtifactNativeAccessCounters,
    ) -> Self {
        Self {
            artifact_occurrence_identity,
            basis_identity,
            provider_session_identity,
            layout,
            requested_fields,
            borrow_generation,
            counters,
        }
    }

    pub fn artifact_occurrence_identity(&self) -> &str {
        &self.artifact_occurrence_identity
    }

    pub fn basis_identity(&self) -> &str {
        &self.basis_identity
    }

    pub fn provider_session_identity(&self) -> &str {
        &self.provider_session_identity
    }

    pub fn layout(&self) -> &WorthQueryArtifactNativeLayoutReference {
        &self.layout
    }

    pub fn requested_fields(&self) -> &[AspectKey] {
        &self.requested_fields
    }

    pub const fn borrow_generation(&self) -> u64 {
        self.borrow_generation
    }

    pub const fn counters(&self) -> WorthQueryArtifactNativeAccessCounters {
        self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryArtifactNativeAccessOutcome<T> {
    value: T,
    evidence: WorthQueryArtifactNativeAccessEvidence,
}

impl<T> WorthQueryArtifactNativeAccessOutcome<T> {
    pub(crate) fn new(value: T, evidence: WorthQueryArtifactNativeAccessEvidence) -> Self {
        Self { value, evidence }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn into_value(self) -> T {
        self.value
    }

    pub fn evidence(&self) -> &WorthQueryArtifactNativeAccessEvidence {
        &self.evidence
    }
}
