use worth_ui::facade::{
    WorthUiActivationGateCounters,
    WorthUiActivationGateReceipt,
    runtime::WorthUiRuntimeFrameEpoch,
};

fn main() {
    let _receipt = WorthUiActivationGateReceipt {
        active_artifact_digest: 1,
        active_plan_digest: 2,
        active_snapshot_digest: 3,
        candidate_artifact_digest: 4,
        candidate_execution_plan_digest: 5,
        handle_allocation_basis_digest: 6,
        reconciliation_basis_digest: 7,
        reconciliation_receipt_count: 8,
        query_rebind_basis_digest: 9,
        query_rebind_entry_count: 10,
        query_rebind_denied_count: 11,
        lane_parity_semantic_reference_digest: Some(12),
        readiness_frame_epoch: WorthUiRuntimeFrameEpoch::initial(),
        boundary_frame_epoch: WorthUiRuntimeFrameEpoch::initial(),
        counters: WorthUiActivationGateCounters::default(),
    };
}
