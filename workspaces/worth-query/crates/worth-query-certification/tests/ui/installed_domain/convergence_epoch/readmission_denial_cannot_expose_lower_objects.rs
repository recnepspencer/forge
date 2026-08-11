use worth_query_host::facade::convergence_epoch::{
    WorthQueryDirectConvergenceReadmissionDenied,
    WorthQueryWorkflowConvergenceReadmissionDenied,
};
use worth_query_host::facade::installed::domain_computation::{
    WorthQueryDirectReadmissionDenied, WorthQueryWorkflowReadmissionDenied,
};

fn expose(
    direct: &WorthQueryDirectConvergenceReadmissionDenied,
    workflow: &WorthQueryWorkflowConvergenceReadmissionDenied,
) {
    let _: &WorthQueryDirectReadmissionDenied = direct.managed_denial();
    let _: &WorthQueryWorkflowReadmissionDenied = workflow.managed_denial();
    let _: &WorthQueryDirectReadmissionDenied = direct.run_denial();
    let _: &WorthQueryWorkflowReadmissionDenied = workflow.run_denial();
}

fn main() {}
