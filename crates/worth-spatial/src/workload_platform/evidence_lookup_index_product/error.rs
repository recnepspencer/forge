use super::counters::EvidenceLookupIndexProductCounters;
use super::lifecycle_posture::EvidenceLookupIndexLifecyclePosture;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupIndexProductErrorKind {
    MissingSelectedStageLedgerRow,
    LedgerBasisExceedsSelectedScope,
    SpatialAdmissionDenied,
    ReusedIndexBasisMismatch,
    PersistentCapabilitySupportRequired,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupIndexProductError {
    kind: EvidenceLookupIndexProductErrorKind,
    detail: String,
    counters: EvidenceLookupIndexProductCounters,
    required_lifecycle_posture: Option<EvidenceLookupIndexLifecyclePosture>,
    rebuild_denial_identity_digest: Option<String>,
}

impl EvidenceLookupIndexProductError {
    pub(crate) fn new(
        kind: EvidenceLookupIndexProductErrorKind,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
            counters: EvidenceLookupIndexProductCounters::default(),
            required_lifecycle_posture: None,
            rebuild_denial_identity_digest: None,
        }
    }

    pub(crate) fn with_counters(mut self, counters: EvidenceLookupIndexProductCounters) -> Self {
        self.counters = counters;
        self
    }

    pub(crate) fn with_required_lifecycle_posture(
        mut self,
        posture: EvidenceLookupIndexLifecyclePosture,
    ) -> Self {
        self.required_lifecycle_posture = Some(posture);
        self
    }

    pub(crate) fn with_rebuild_denial_identity_digest(mut self, digest: impl Into<String>) -> Self {
        self.rebuild_denial_identity_digest = Some(digest.into());
        self
    }

    pub const fn kind(&self) -> EvidenceLookupIndexProductErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub const fn counters(&self) -> &EvidenceLookupIndexProductCounters {
        &self.counters
    }

    pub const fn required_lifecycle_posture(&self) -> Option<EvidenceLookupIndexLifecyclePosture> {
        self.required_lifecycle_posture
    }

    pub fn rebuild_denial_identity_digest(&self) -> Option<&str> {
        self.rebuild_denial_identity_digest.as_deref()
    }
}
