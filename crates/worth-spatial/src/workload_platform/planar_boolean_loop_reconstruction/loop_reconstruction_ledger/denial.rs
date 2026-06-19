use super::counters::PlanarBooleanLoopReconstructionLedgerCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanLoopReconstructionLedgerDenialKind {
    RequestIdentityMismatch,
    SplitLedgerLineageMismatch,
    MissingTrackedLoop,
    MissingRoleOutcome,
    MissingDegenerateOutcome,
    MissingDecisionTrace,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopReconstructionLedgerDenial {
    kind: PlanarBooleanLoopReconstructionLedgerDenialKind,
    detail_identity: String,
    counters: PlanarBooleanLoopReconstructionLedgerCounters,
    human_reason: String,
}

impl PlanarBooleanLoopReconstructionLedgerDenial {
    pub(crate) fn new(
        kind: PlanarBooleanLoopReconstructionLedgerDenialKind,
        detail_identity: impl Into<String>,
        counters: PlanarBooleanLoopReconstructionLedgerCounters,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail_identity: detail_identity.into(),
            counters,
            human_reason: human_reason.into(),
        }
    }

    pub fn kind(&self) -> PlanarBooleanLoopReconstructionLedgerDenialKind {
        self.kind
    }

    pub fn detail_identity(&self) -> &str {
        &self.detail_identity
    }

    pub fn counters(&self) -> PlanarBooleanLoopReconstructionLedgerCounters {
        self.counters
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}
