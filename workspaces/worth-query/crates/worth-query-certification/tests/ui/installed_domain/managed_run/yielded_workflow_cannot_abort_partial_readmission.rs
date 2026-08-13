use worth_query_execution::facade::domain_computation::WorthQueryYieldedWorkflowRun;

fn abort(run: WorthQueryYieldedWorkflowRun) {
    run.abort();
}

fn main() {}
