use worth_query_host::facade::convergence_epoch::{
    WorthQueryYieldedDirectConvergenceIteration, WorthQueryYieldedWorkflowConvergenceIteration,
};

fn direct_lower_observations(yielded: &WorthQueryYieldedDirectConvergenceIteration) {
    let _ = yielded.checkpoint();
    let _ = yielded.resource_attempt_evidence();
    let _ = yielded.bridge();
}

fn workflow_lower_observations(yielded: &WorthQueryYieldedWorkflowConvergenceIteration) {
    let _ = yielded.checkpoint();
    let _ = yielded.resource_attempt_evidence();
    let _ = yielded.bridge();
}

fn main() {}
