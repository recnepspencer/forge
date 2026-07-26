use std::sync::Arc;

use super::WorthQueryConvergenceAdmissionCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryConvergenceAdmissionDenialKind {
    ForeignInstalledAuthorities,
    OperationEvidenceNotInstalled,
    AmbiguousWorkflowEvidence,
    ArtifactContractMismatch,
    NonIterativeContract,
    MissingCandidateSearch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryConvergenceAdmissionDenial {
    kind: WorthQueryConvergenceAdmissionDenialKind,
    detail: Arc<str>,
    counters: WorthQueryConvergenceAdmissionCounters,
}

impl WorthQueryConvergenceAdmissionDenial {
    pub(super) fn new(
        kind: WorthQueryConvergenceAdmissionDenialKind,
        detail: impl Into<Arc<str>>,
        counters: WorthQueryConvergenceAdmissionCounters,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
            counters,
        }
    }

    pub const fn kind(&self) -> WorthQueryConvergenceAdmissionDenialKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub const fn counters(&self) -> WorthQueryConvergenceAdmissionCounters {
        self.counters
    }
}
