use worth_query_execution::facade::domain_computation::{
    WorthQueryCompletedDirectGraphExecution, WorthQueryCompletedWorkflowGraphExecution,
    WorthQueryDirectRunTerminal, WorthQueryWorkflowRunTerminal,
};
use worth_query_host::facade::convergence_epoch::{
    WorthQueryPausedDirectConvergenceIteration, WorthQueryStartedDirectConvergenceIteration,
    WorthQueryStartedWorkflowConvergenceIteration,
};

fn split(started: WorthQueryStartedDirectConvergenceIteration) {
    let _ = started.into_parts();
}

fn cross_peer_completion(
    started: WorthQueryStartedDirectConvergenceIteration,
    completion: WorthQueryCompletedDirectGraphExecution,
) {
    let _ = started.admit_completion(completion);
}

fn cross_peer_terminal(
    started: WorthQueryStartedDirectConvergenceIteration,
    terminal: WorthQueryDirectRunTerminal,
) {
    let _ = started.admit_managed_terminal(terminal);
}

fn yield_before_safe_point(started: WorthQueryStartedDirectConvergenceIteration) {
    let _ = started.yield_iteration();
}

fn present_foreign_yield(paused: WorthQueryPausedDirectConvergenceIteration) {
    let _ = paused.admit_yield_outcome;
}

fn split_workflow(started: WorthQueryStartedWorkflowConvergenceIteration) {
    let _ = started.into_parts();
}

fn cross_peer_workflow_completion(
    started: WorthQueryStartedWorkflowConvergenceIteration,
    completion: WorthQueryCompletedWorkflowGraphExecution,
) {
    let _ = started.admit_completion(completion);
}

fn cross_peer_workflow_terminal(
    started: WorthQueryStartedWorkflowConvergenceIteration,
    terminal: WorthQueryWorkflowRunTerminal,
) {
    let _ = started.admit_managed_terminal(terminal);
}

fn workflow_yield_before_safe_point(started: WorthQueryStartedWorkflowConvergenceIteration) {
    let _ = started.yield_iteration();
}

fn main() {}
