use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanDegenerateLoopOutcomeSet, PlanarBooleanLoopDecisionLog,
    PlanarBooleanLoopIslandPartition, PlanarBooleanLoopReconstructionLedgerReceipt,
    PlanarBooleanLoopRoleOutcomeSet, PlanarBooleanReconstructedLoopBoundary,
    PlanarBooleanSourceLoopSplitAttribution,
};
use crate::workload_platform::retained_replay_workload::ReplayReceiptSet;

#[derive(Clone, Copy, Debug)]
pub struct PlanarBooleanLoopReconstructionEvidenceInput<'a> {
    reconstructed_boundary: &'a PlanarBooleanReconstructedLoopBoundary,
    island_partition: &'a PlanarBooleanLoopIslandPartition,
    split_attribution: &'a PlanarBooleanSourceLoopSplitAttribution,
    role_outcomes: &'a PlanarBooleanLoopRoleOutcomeSet,
    degenerate_outcomes: &'a PlanarBooleanDegenerateLoopOutcomeSet,
    decision_log: &'a PlanarBooleanLoopDecisionLog,
    ledger_receipt: &'a PlanarBooleanLoopReconstructionLedgerReceipt,
    replay_receipts: &'a ReplayReceiptSet,
}

impl<'a> PlanarBooleanLoopReconstructionEvidenceInput<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn from_phase_sixteen_products(
        reconstructed_boundary: &'a PlanarBooleanReconstructedLoopBoundary,
        island_partition: &'a PlanarBooleanLoopIslandPartition,
        split_attribution: &'a PlanarBooleanSourceLoopSplitAttribution,
        role_outcomes: &'a PlanarBooleanLoopRoleOutcomeSet,
        degenerate_outcomes: &'a PlanarBooleanDegenerateLoopOutcomeSet,
        decision_log: &'a PlanarBooleanLoopDecisionLog,
        ledger_receipt: &'a PlanarBooleanLoopReconstructionLedgerReceipt,
        replay_receipts: &'a ReplayReceiptSet,
    ) -> Self {
        Self {
            reconstructed_boundary,
            island_partition,
            split_attribution,
            role_outcomes,
            degenerate_outcomes,
            decision_log,
            ledger_receipt,
            replay_receipts,
        }
    }

    pub(crate) fn reconstructed_boundary(self) -> &'a PlanarBooleanReconstructedLoopBoundary {
        self.reconstructed_boundary
    }

    pub(crate) fn island_partition(self) -> &'a PlanarBooleanLoopIslandPartition {
        self.island_partition
    }

    pub(crate) fn split_attribution(self) -> &'a PlanarBooleanSourceLoopSplitAttribution {
        self.split_attribution
    }

    pub(crate) fn role_outcomes(self) -> &'a PlanarBooleanLoopRoleOutcomeSet {
        self.role_outcomes
    }

    pub(crate) fn degenerate_outcomes(self) -> &'a PlanarBooleanDegenerateLoopOutcomeSet {
        self.degenerate_outcomes
    }

    pub(crate) fn decision_log(self) -> &'a PlanarBooleanLoopDecisionLog {
        self.decision_log
    }

    pub(crate) fn ledger_receipt(self) -> &'a PlanarBooleanLoopReconstructionLedgerReceipt {
        self.ledger_receipt
    }

    pub(crate) fn replay_receipts(self) -> &'a ReplayReceiptSet {
        self.replay_receipts
    }
}
