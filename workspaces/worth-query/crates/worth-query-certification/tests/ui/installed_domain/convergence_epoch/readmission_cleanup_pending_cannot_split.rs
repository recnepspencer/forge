use worth_query_host::facade::convergence_epoch::{
    WorthQueryDirectConvergenceReadmissionCleanupPending,
    WorthQueryWorkflowConvergenceReadmissionCleanupPending,
};

fn split_direct(
    pending: WorthQueryDirectConvergenceReadmissionCleanupPending,
) -> WorthQueryDirectConvergenceReadmissionCleanupPending {
    let WorthQueryDirectConvergenceReadmissionCleanupPending { association } = pending;
    WorthQueryDirectConvergenceReadmissionCleanupPending { association }
}

fn split_workflow(
    pending: WorthQueryWorkflowConvergenceReadmissionCleanupPending,
) -> WorthQueryWorkflowConvergenceReadmissionCleanupPending {
    let WorthQueryWorkflowConvergenceReadmissionCleanupPending { association } = pending;
    WorthQueryWorkflowConvergenceReadmissionCleanupPending { association }
}

fn main() {}
