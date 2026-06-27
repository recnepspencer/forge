use super::counters::EvidenceLookupIndexProductCounters;
use super::lifecycle_posture::EvidenceLookupIndexLifecyclePosture;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupIndexProductErrorKind {
    MissingSelectedStageLedgerRow,
    LedgerBasisExceedsSelectedScope,
    ReusedIndexBasisMismatch,
    PersistentCapabilitySupportRequired,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupIndexProductError {
    kind: EvidenceLookupIndexProductErrorKind,
    detail: String,
    counters: EvidenceLookupIndexProductCounters,
    required_lifecycle_posture: Option<EvidenceLookupIndexLifecyclePosture>,
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
}
