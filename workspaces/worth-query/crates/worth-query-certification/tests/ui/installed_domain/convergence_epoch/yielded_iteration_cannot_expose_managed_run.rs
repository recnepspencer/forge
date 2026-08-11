use worth_query_host::facade::convergence_epoch::{
    WorthQueryYieldedDirectConvergenceIteration, WorthQueryYieldedWorkflowConvergenceIteration,
};
use worth_query_host::facade::installed::domain_computation::{
    WorthQueryYieldedDirectRun, WorthQueryYieldedWorkflowRun,
};

fn direct_whole_run(
    yielded: &WorthQueryYieldedDirectConvergenceIteration,
) -> &WorthQueryYieldedDirectRun {
    yielded.yielded_run()
}

fn workflow_whole_run(
    yielded: &WorthQueryYieldedWorkflowConvergenceIteration,
) -> &WorthQueryYieldedWorkflowRun {
    yielded.yielded_run()
}

fn main() {}
