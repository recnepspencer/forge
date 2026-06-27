use worth_ui::facade::{
    WorthUiAtomicPlanSwapCounters, WorthUiPlanSwapDenialReason, WorthUiPlanSwapReceipt,
    WorthUiPlanSwapRollback, WorthUiPriorValidPlanObservation,
};

fn observe_receipt(receipt: WorthUiPlanSwapReceipt) {
    let _ = receipt.previous_active_artifact_digest();
    let _ = receipt.previous_active_plan_digest();
    let _ = receipt.previous_active_snapshot_digest();
    let _ = receipt.next_active_artifact_digest();
    let _ = receipt.next_active_plan_digest();
    let _ = receipt.next_active_snapshot_digest();
    let _ = receipt.activation_gate_receipt();
    let _ = receipt.prior_valid_plan();
    let _ = receipt.readiness_frame_epoch();
    let _ = receipt.boundary_frame_epoch();
    let _ = receipt.query_rebind_basis_digest();
    let _ = receipt.query_rebind_entry_count();
    let _ = receipt.query_rebind_denied_count();
    let _ = receipt.reconciliation_basis_digest();
    let _ = receipt.reconciliation_receipt_count();
    let _ = receipt.lane_parity_semantic_reference_digest();
    let _ = receipt.counters();
}

fn observe_rollback(rollback: WorthUiPlanSwapRollback) {
    let _ = rollback.reason();
    let _ = rollback.prior_valid_plan();
    let _ = rollback.restored_active_artifact_digest();
    let _ = rollback.restored_active_plan_digest();
    let _ = rollback.attempted_next_artifact_digest();
    let _ = rollback.attempted_next_plan_digest();
    let _ = rollback.counters();
}

fn observe_prior(prior: WorthUiPriorValidPlanObservation) {
    let _ = prior.artifact_digest();
    let _ = prior.active_plan_digest();
    let _ = prior.snapshot_digest();
    let _ = prior.lifecycle();
    let _ = prior.status();
    let _ = prior.frame_epoch();
}

fn observe_counters(counters: WorthUiAtomicPlanSwapCounters) {
    let _ = counters.prior_valid_capture_count();
    let _ = counters.activation_gate_count();
    let _ = counters.next_active_state_build_count();
    let _ = counters.active_state_mutation_count();
    let _ = counters.rollback_restore_count();
    let _ = counters.source_reparse_count();
    let _ = counters.registry_rebuild_count();
    let _ = counters.semantic_replanning_count();
    let _ = counters.query_replanning_count();
    let _ = counters.handle_allocation_count();
    let _ = counters.denial_count();
}

fn observe_reason(reason: WorthUiPlanSwapDenialReason) {
    let _ = reason;
}

fn main() {}
