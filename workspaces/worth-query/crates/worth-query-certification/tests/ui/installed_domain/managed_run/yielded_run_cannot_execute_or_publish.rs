use worth_query_execution::facade::domain_computation::{
    WorthQueryYieldedDirectRun, WorthQueryYieldedWorkflowRun,
};

fn advance_direct(run: WorthQueryYieldedDirectRun) {
    run.advance();
}

fn restore_direct(run: WorthQueryYieldedDirectRun) {
    run.restore();
}

fn publish_direct(run: WorthQueryYieldedDirectRun) {
    run.publish();
}

fn advance_workflow(run: WorthQueryYieldedWorkflowRun) {
    run.advance();
}

fn restore_workflow(run: WorthQueryYieldedWorkflowRun) {
    run.restore();
}

fn publish_workflow(run: WorthQueryYieldedWorkflowRun) {
    run.publish();
}

fn main() {}
