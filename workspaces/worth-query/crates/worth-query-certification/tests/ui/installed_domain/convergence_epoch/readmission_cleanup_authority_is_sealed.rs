use worth_query_host::facade::convergence_epoch::{
    WorthQueryDirectConvergenceReadmissionCleanupPending,
    WorthQueryDirectConvergenceReadmissionCleanupRequired,
    WorthQueryWorkflowConvergenceReadmissionCleanupPending,
    WorthQueryWorkflowConvergenceReadmissionCleanupRequired,
};
use worth_query_host::facade::installed::domain_computation::{
    WorthQueryDirectReadmissionCleanupRequired, WorthQueryWorkflowReadmissionCleanupRequired,
};

fn lower_direct_cleanup_cannot_escape(
    cleanup: &WorthQueryDirectConvergenceReadmissionCleanupRequired,
) {
    let _: &WorthQueryDirectReadmissionCleanupRequired = cleanup.managed_cleanup();
}

fn lower_workflow_cleanup_cannot_escape(
    cleanup: &WorthQueryWorkflowConvergenceReadmissionCleanupRequired,
) {
    let _: &WorthQueryWorkflowReadmissionCleanupRequired = cleanup.managed_cleanup();
}

fn workflow_pending_cannot_enter_direct_lane(
    pending: WorthQueryWorkflowConvergenceReadmissionCleanupPending,
) -> WorthQueryDirectConvergenceReadmissionCleanupPending {
    pending
}

fn main() {}
