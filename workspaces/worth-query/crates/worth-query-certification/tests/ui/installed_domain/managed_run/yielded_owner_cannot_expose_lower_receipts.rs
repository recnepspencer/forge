use worth_query_host::facade::installed::domain_computation::{
    WorthQueryYieldedDirectRun, WorthQueryYieldedWorkflowRun,
};

fn direct(run: &WorthQueryYieldedDirectRun) {
    let _ = run.logical_run_identity();
    let _ = run.yielded_attempt_identity();
    let _ = run.operation_binding_identity();
    let _ = run.installed_operation_identity();
    let _ = run.semantic_basis_identity();
    let _ = run.installation_generation();
    let _ = run.bridge();
    let _ = run.bridge_request_identity();
    let _ = run.resource_attempt_evidence();
    let _ = run.resource_attempt_identity();
    let _ = run.provider_session_identity();
    let _ = run.relational_basis_identity();
    let _ = run.checkpoint();
    let _ = run.provider_work();
    let _ = run.run_counters();
    let _ = run.yield_counters();
    let _ = run.retained_capacity_reservation_count();
}

fn workflow(run: &WorthQueryYieldedWorkflowRun) {
    let _ = run.logical_run_identity();
    let _ = run.yielded_attempt_identity();
    let _ = run.operation_binding_identity();
    let _ = run.installed_operation_identity();
    let _ = run.semantic_basis_identity();
    let _ = run.installation_generation();
    let _ = run.bridge();
    let _ = run.bridge_request_identity();
    let _ = run.resource_attempt_evidence();
    let _ = run.resource_attempt_identity();
    let _ = run.provider_session_identity();
    let _ = run.relational_basis_identity();
    let _ = run.artifact_run_identity();
    let _ = run.checkpoint();
    let _ = run.provider_work();
    let _ = run.run_counters();
    let _ = run.yield_counters();
    let _ = run.retained_capacity_reservation_count();
    let _ = run.artifact_evidence();
}

fn main() {}
