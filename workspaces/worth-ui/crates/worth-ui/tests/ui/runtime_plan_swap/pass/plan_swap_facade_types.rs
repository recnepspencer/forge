use worth_ui::facade::{
    UiCommittedAllocationActivationCounters, WorthUiPlanSwapReceipt,
    WorthUiPriorValidPlanObservation,
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

fn observe_prior(prior: WorthUiPriorValidPlanObservation) {
    let _ = prior.artifact_digest();
    let _ = prior.active_plan_digest();
    let _ = prior.snapshot_digest();
    let _ = prior.lifecycle();
    let _ = prior.status();
    let _ = prior.frame_epoch();
}

fn observe_counters(counters: UiCommittedAllocationActivationCounters) {
    let _ = counters.ledger_predecessor_checks();
    let _ = counters.readiness_checks();
    let _ = counters.graph_predecessor_checks();
    let _ = counters.scroll_binding_checks();
    let _ = counters.frame_replacement_checks();
    let _ = counters.frame_boundary_checks();
    let _ = counters.active_successor_builds();
    let _ = counters.denial_count();
    let _ = counters.live_mutation_count();
}

fn main() {}
