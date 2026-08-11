#![deny(unused_must_use)]

use worth_query_host::facade::convergence_epoch::{
    WorthQueryDeniedDirectConvergenceYield, WorthQueryDeniedWorkflowConvergenceYield,
};

fn preserve_direct(
    denial: WorthQueryDeniedDirectConvergenceYield,
) -> WorthQueryDeniedDirectConvergenceYield {
    denial
}

fn preserve_workflow(
    denial: WorthQueryDeniedWorkflowConvergenceYield,
) -> WorthQueryDeniedWorkflowConvergenceYield {
    denial
}

fn discard(
    direct: WorthQueryDeniedDirectConvergenceYield,
    direct_retry: WorthQueryDeniedDirectConvergenceYield,
    workflow: WorthQueryDeniedWorkflowConvergenceYield,
    workflow_retry: WorthQueryDeniedWorkflowConvergenceYield,
) {
    preserve_direct(direct);
    direct_retry.retry();
    preserve_workflow(workflow);
    workflow_retry.retry();
}

fn main() {}
