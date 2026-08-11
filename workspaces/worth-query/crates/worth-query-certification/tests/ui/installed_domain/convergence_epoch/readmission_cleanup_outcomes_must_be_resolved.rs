#![deny(unused_must_use)]

use worth_query_host::facade::convergence_epoch::{
    WorthQueryDirectConvergenceReadmissionCleanupPending,
    WorthQueryDirectConvergenceReadmissionCleanupRequired,
    WorthQueryWorkflowConvergenceReadmissionCleanupPending,
    WorthQueryWorkflowConvergenceReadmissionCleanupRequired,
};

fn ignore_direct_finish(cleanup: WorthQueryDirectConvergenceReadmissionCleanupRequired) {
    cleanup.finish();
}

fn ignore_direct_retry(pending: WorthQueryDirectConvergenceReadmissionCleanupPending) {
    pending.retry();
}

fn ignore_workflow_finish(cleanup: WorthQueryWorkflowConvergenceReadmissionCleanupRequired) {
    cleanup.finish();
}

fn ignore_workflow_retry(pending: WorthQueryWorkflowConvergenceReadmissionCleanupPending) {
    pending.retry();
}

fn main() {}
