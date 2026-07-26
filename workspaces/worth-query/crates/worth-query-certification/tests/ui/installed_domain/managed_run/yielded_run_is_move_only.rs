use worth_query_execution::facade::domain_computation::{
    WorthQueryYieldedDirectRun, WorthQueryYieldedWorkflowRun,
};

fn clone_direct(run: &WorthQueryYieldedDirectRun) {
    let _: WorthQueryYieldedDirectRun = Clone::clone(run);
}

fn clone_workflow(run: &WorthQueryYieldedWorkflowRun) {
    let _: WorthQueryYieldedWorkflowRun = Clone::clone(run);
}

fn main() {}
