use worth_ui::facade::{
    WorthUiActivationGateCounters, WorthUiExecutionPlanDigest, WorthUiPendingActivation,
    WorthUiReadyActivation,
};

fn main() {
    let _ready = WorthUiReadyActivation {
        pending_activation: pending_activation(),
        candidate_execution_plan_digest: execution_plan_digest(),
        handle_allocation_basis_digest: 1,
        reconciliation_basis_digest: 2,
        query_rebind_basis_digest: 3,
        query_rebind_denied_count: 0,
        lane_parity_semantic_reference_digest: None,
        counters: WorthUiActivationGateCounters::default(),
    };
}

fn pending_activation() -> WorthUiPendingActivation {
    todo!()
}

fn execution_plan_digest() -> WorthUiExecutionPlanDigest {
    todo!()
}
