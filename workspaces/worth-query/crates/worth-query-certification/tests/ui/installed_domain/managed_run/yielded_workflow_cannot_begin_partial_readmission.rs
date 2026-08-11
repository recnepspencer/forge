use worth_query_execution::facade::domain_computation::WorthQueryYieldedWorkflowRun;

fn begin(run: WorthQueryYieldedWorkflowRun) {
    run.begin_readmission();
}

fn main() {}
