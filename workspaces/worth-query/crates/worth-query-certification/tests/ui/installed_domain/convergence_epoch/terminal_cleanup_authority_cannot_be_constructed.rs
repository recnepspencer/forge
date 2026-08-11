use std::marker::PhantomData;

use worth_query_host::facade::convergence_epoch::{
    WorthQueryConverged, WorthQueryDirectConvergenceCleanupFailure,
    WorthQueryDirectConvergenceCleanupReceipt, WorthQueryWorkflowConvergenceCleanupFailure,
    WorthQueryWorkflowConvergenceCleanupPending, WorthQueryWorkflowConvergenceCleanupReceipt,
};

type DirectReceipt = WorthQueryDirectConvergenceCleanupReceipt<WorthQueryConverged>;
type DirectFailure = WorthQueryDirectConvergenceCleanupFailure<WorthQueryConverged>;
type WorkflowReceipt = WorthQueryWorkflowConvergenceCleanupReceipt<WorthQueryConverged>;
type WorkflowPending = WorthQueryWorkflowConvergenceCleanupPending<WorthQueryConverged>;
type WorkflowFailure = WorthQueryWorkflowConvergenceCleanupFailure<WorthQueryConverged>;

fn recompose_direct_cleanup_authority(
    direct_receipt: DirectReceipt,
    direct_failure: DirectFailure,
) {
    let _ = DirectReceipt {
        terminal_state: PhantomData,
        ..direct_receipt
    };
    let _ = DirectFailure {
        terminal_state: PhantomData,
        ..direct_failure
    };
}

fn recompose_workflow_cleanup_authority(
    workflow_receipt: WorkflowReceipt,
    workflow_pending: WorkflowPending,
    workflow_failure: WorkflowFailure,
) {
    let _ = WorkflowReceipt {
        terminal_state: PhantomData,
        ..workflow_receipt
    };
    let _ = WorkflowPending {
        terminal_state: PhantomData,
        ..workflow_pending
    };
    let _ = WorkflowFailure {
        terminal_state: PhantomData,
        ..workflow_failure
    };
}

fn main() {}
