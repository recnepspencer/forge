use worth_query_host::facade::convergence_epoch::{
    WorthQueryDirectConvergenceReadmissionDenied, WorthQueryDirectConvergenceReadmissionOutcome,
    WorthQueryWorkflowConvergenceReadmissionDenied,
    WorthQueryWorkflowConvergenceReadmissionOutcome,
};
use worth_query_host::facade::runtime::WorthQueryExecutionRuntime;
use worth_runtime_bridge::facade::RuntimeBridge;

pub(super) fn resolve_direct(
    denial: WorthQueryDirectConvergenceReadmissionDenied,
    query: &WorthQueryExecutionRuntime,
    bridge: &RuntimeBridge,
) -> WorthQueryDirectConvergenceReadmissionOutcome {
    let _ = denial.readmission_evidence();
    denial.into_yielded().readmit_same_runtime(query, bridge)
}

pub(super) fn resolve_workflow(
    denial: WorthQueryWorkflowConvergenceReadmissionDenied,
    query: &WorthQueryExecutionRuntime,
    bridge: &RuntimeBridge,
) -> WorthQueryWorkflowConvergenceReadmissionOutcome {
    let _ = denial.readmission_evidence();
    denial.into_yielded().readmit_same_runtime(query, bridge)
}
