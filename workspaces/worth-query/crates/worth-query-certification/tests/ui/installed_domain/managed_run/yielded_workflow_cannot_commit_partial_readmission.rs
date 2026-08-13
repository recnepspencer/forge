use worth_query_execution::facade::domain_computation::WorthQueryYieldedWorkflowRun;

fn commit(run: WorthQueryYieldedWorkflowRun) {
    run.commit();
}

fn main() {}
