use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanDownstreamSplitConsumption, PlanarBooleanDownstreamSplitConsumptionDenial,
    PlanarBooleanDownstreamSplitConsumptionInput, PlanarBooleanEdgeSplitReplayParityReceipt,
    PlanarBooleanSplitChainValidationReceipt, PlanarBooleanSplitDecisionLogReceipt,
    PlanarBooleanSplitEdgeChainLedgerReceipt, PlanarBooleanSplitPersistentNamingReceipt,
};

use super::{WorkloadCompositionError, WorthWorkload};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompletedBooleanSplitHandoff {
    completed_workload: WorthWorkload,
    split_ledger_receipt: PlanarBooleanSplitEdgeChainLedgerReceipt,
}

impl CompletedBooleanSplitHandoff {
    pub(crate) fn new(
        completed_workload: WorthWorkload,
        split_ledger_receipt: PlanarBooleanSplitEdgeChainLedgerReceipt,
    ) -> Self {
        Self {
            completed_workload,
            split_ledger_receipt,
        }
    }

    pub fn completed_workload(&self) -> &WorthWorkload {
        &self.completed_workload
    }

    pub fn split_ledger_receipt(&self) -> &PlanarBooleanSplitEdgeChainLedgerReceipt {
        &self.split_ledger_receipt
    }

    pub fn workload_stage_index_identity(&self) -> &str {
        self.completed_workload
            .evidence_ledger()
            .stage_index()
            .index_identity()
    }

    pub fn require_boolean_split(&self) -> Result<(), WorkloadCompositionError> {
        self.completed_workload
            .require_boolean_split(&self.split_ledger_receipt)
    }

    pub fn admit_downstream_split_consumption(
        &self,
        decision_log_receipt: &PlanarBooleanSplitDecisionLogReceipt,
        validation_receipt: &PlanarBooleanSplitChainValidationReceipt,
        persistent_naming_receipt: &PlanarBooleanSplitPersistentNamingReceipt,
        replay_parity_receipt: &PlanarBooleanEdgeSplitReplayParityReceipt,
    ) -> Result<
        PlanarBooleanDownstreamSplitConsumption,
        PlanarBooleanDownstreamSplitConsumptionDenial,
    > {
        PlanarBooleanDownstreamSplitConsumption::admit(
            PlanarBooleanDownstreamSplitConsumptionInput::from_split_ledger_receipt(
                &self.split_ledger_receipt,
                decision_log_receipt,
                validation_receipt,
                persistent_naming_receipt,
                replay_parity_receipt,
                self.completed_workload.evidence_ledger().stage_index(),
            ),
        )
    }
}
