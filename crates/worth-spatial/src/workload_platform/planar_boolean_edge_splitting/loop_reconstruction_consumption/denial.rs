use super::counters::PlanarBooleanLoopReconstructionSplitConsumptionCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanLoopReconstructionSplitConsumptionDenialKind {
    MissingDownstreamSplitConsumption,
    MissingSplitLedgerReceipt,
    MissingSplitLedgerDownstreamIdentity,
    MissingSplitRequest,
    MissingWorkloadStageIndex,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopReconstructionSplitConsumptionDenial {
    kind: PlanarBooleanLoopReconstructionSplitConsumptionDenialKind,
    rejected_identity: String,
    counters: PlanarBooleanLoopReconstructionSplitConsumptionCounters,
    human_reason: &'static str,
}

impl PlanarBooleanLoopReconstructionSplitConsumptionDenial {
    pub(crate) fn new(
        kind: PlanarBooleanLoopReconstructionSplitConsumptionDenialKind,
        rejected_identity: impl Into<String>,
        counters: PlanarBooleanLoopReconstructionSplitConsumptionCounters,
        human_reason: &'static str,
    ) -> Self {
        Self {
            kind,
            rejected_identity: rejected_identity.into(),
            counters,
            human_reason,
        }
    }

    pub fn kind(&self) -> PlanarBooleanLoopReconstructionSplitConsumptionDenialKind {
        self.kind
    }

    pub fn rejected_identity(&self) -> &str {
        &self.rejected_identity
    }

    pub fn counters(&self) -> PlanarBooleanLoopReconstructionSplitConsumptionCounters {
        self.counters
    }

    pub fn human_reason(&self) -> &'static str {
        self.human_reason
    }
}
