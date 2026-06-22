use super::counters::PlanarBooleanLoopReconstructionRequestCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanLoopReconstructionRequestDenialKind {
    MissingLoopSplitConsumption,
    MissingSplitLedgerReceipt,
    MissingSplitRequest,
    MissingWorkloadStageIndex,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopReconstructionRequestDenial {
    kind: PlanarBooleanLoopReconstructionRequestDenialKind,
    rejected_identity: String,
    counters: PlanarBooleanLoopReconstructionRequestCounters,
    human_reason: &'static str,
}

impl PlanarBooleanLoopReconstructionRequestDenial {
    pub(crate) fn new(
        kind: PlanarBooleanLoopReconstructionRequestDenialKind,
        rejected_identity: impl Into<String>,
        counters: PlanarBooleanLoopReconstructionRequestCounters,
        human_reason: &'static str,
    ) -> Self {
        Self {
            kind,
            rejected_identity: rejected_identity.into(),
            counters,
            human_reason,
        }
    }

    pub fn kind(&self) -> PlanarBooleanLoopReconstructionRequestDenialKind {
        self.kind
    }

    pub fn rejected_identity(&self) -> &str {
        &self.rejected_identity
    }

    pub fn counters(&self) -> PlanarBooleanLoopReconstructionRequestCounters {
        self.counters
    }

    pub fn human_reason(&self) -> &'static str {
        self.human_reason
    }
}
