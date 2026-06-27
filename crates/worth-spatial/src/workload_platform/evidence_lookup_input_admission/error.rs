use super::counters::EvidenceLookupInputAdmissionCounters;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupInputAdmissionError {
    kind: EvidenceLookupInputAdmissionErrorKind,
    detail: String,
    counters: EvidenceLookupInputAdmissionCounters,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupInputAdmissionErrorKind {
    MissingStageReceiptIdentity,
    StageReceiptAuthorityMismatch,
    SpatialTouchStageMismatch,
    NoFamilyForStageReceiptIdentity,
    MissingTopologySeed,
    MissingRequiredTopologyReceipt,
    MissingQueryImportEvidence,
}

impl EvidenceLookupInputAdmissionError {
    pub(crate) fn new(
        kind: EvidenceLookupInputAdmissionErrorKind,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
            counters: EvidenceLookupInputAdmissionCounters::default(),
        }
    }

    pub(crate) fn with_counters(mut self, counters: EvidenceLookupInputAdmissionCounters) -> Self {
        self.counters = counters;
        self
    }

    pub const fn kind(&self) -> EvidenceLookupInputAdmissionErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub const fn counters(&self) -> &EvidenceLookupInputAdmissionCounters {
        &self.counters
    }
}
