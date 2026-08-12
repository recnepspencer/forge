use worth_query_host::facade::convergence_epoch::{
    WorthQueryConverged, WorthQueryDirectConvergenceCleanupFailure,
    WorthQueryDirectConvergenceCleanupReceipt, WorthQueryWorkflowConvergenceCleanupFailure,
    WorthQueryWorkflowConvergenceCleanupOutcome, WorthQueryWorkflowConvergenceCleanupPending,
    WorthQueryWorkflowConvergenceCleanupReceipt,
};
use worth_query_host::facade::installed::domain_computation::{
    WorthQueryDirectRunCleanupFailure, WorthQueryDirectRunCleanupReceipt,
    WorthQueryWorkflowRunCleanupFailure, WorthQueryWorkflowRunCleanupPending,
    WorthQueryWorkflowRunCleanupReceipt,
};

fn expose_direct_lower_objects(
    direct_receipt: &WorthQueryDirectConvergenceCleanupReceipt<WorthQueryConverged>,
    direct_failure: &WorthQueryDirectConvergenceCleanupFailure<WorthQueryConverged>,
) {
    let _: WorthQueryDirectRunCleanupReceipt = direct_receipt.managed_receipt().clone();
    let _: &WorthQueryDirectRunCleanupFailure = direct_failure.managed_failure();
    let _: &WorthQueryDirectRunCleanupReceipt = direct_receipt.run_cleanup_receipt();
    let _: &WorthQueryDirectRunCleanupFailure = direct_failure.run_cleanup_failure();
}

fn expose_workflow_lower_objects(
    workflow_receipt: &WorthQueryWorkflowConvergenceCleanupReceipt<WorthQueryConverged>,
    workflow_pending: &WorthQueryWorkflowConvergenceCleanupPending<WorthQueryConverged>,
    workflow_failure: &WorthQueryWorkflowConvergenceCleanupFailure<WorthQueryConverged>,
    workflow_outcome: &WorthQueryWorkflowConvergenceCleanupOutcome<WorthQueryConverged>,
) {
    let _: WorthQueryWorkflowRunCleanupReceipt = workflow_receipt.managed_receipt().clone();
    let _: &WorthQueryWorkflowRunCleanupPending = workflow_pending.managed_pending();
    let _: &WorthQueryWorkflowRunCleanupFailure = workflow_failure.managed_failure();

    let _: &WorthQueryWorkflowRunCleanupReceipt = workflow_receipt.run_cleanup_receipt();
    let _: &WorthQueryWorkflowRunCleanupPending = workflow_pending.run_cleanup_pending();
    let _: &WorthQueryWorkflowRunCleanupFailure = workflow_failure.run_cleanup_failure();
    let _ = workflow_outcome.disposition();
}

fn main() {}
