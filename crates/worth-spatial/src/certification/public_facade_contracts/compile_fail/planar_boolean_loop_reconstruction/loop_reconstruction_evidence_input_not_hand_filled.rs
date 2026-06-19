use worth_spatial::facade::planar_boolean_loop_reconstruction::{
    PlanarBooleanDegenerateLoopOutcomeSet, PlanarBooleanLoopDecisionLog,
    PlanarBooleanLoopIslandPartition, PlanarBooleanLoopReconstructionEvidenceInput,
    PlanarBooleanLoopReconstructionLedgerReceipt, PlanarBooleanLoopRoleOutcomeSet,
    PlanarBooleanReconstructedLoopBoundary, PlanarBooleanSourceLoopSplitAttribution,
};
use worth_spatial::facade::retained_replay_workload::ReplayReceiptSet;

fn bogus<T>() -> T {
    panic!("compile-fail fixture")
}

fn main() {
    let _ = PlanarBooleanLoopReconstructionEvidenceInput {
        reconstructed_boundary: bogus::<&PlanarBooleanReconstructedLoopBoundary>(),
        island_partition: bogus::<&PlanarBooleanLoopIslandPartition>(),
        split_attribution: bogus::<&PlanarBooleanSourceLoopSplitAttribution>(),
        role_outcomes: bogus::<&PlanarBooleanLoopRoleOutcomeSet>(),
        degenerate_outcomes: bogus::<&PlanarBooleanDegenerateLoopOutcomeSet>(),
        decision_log: bogus::<&PlanarBooleanLoopDecisionLog>(),
        ledger_receipt: bogus::<&PlanarBooleanLoopReconstructionLedgerReceipt>(),
        replay_receipts: bogus::<&ReplayReceiptSet>(),
    };
}
