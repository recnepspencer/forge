use worth_query_execution::facade::domain_computation::WorthQueryRunningWorkflowRun;

fn close_while_running(run: &WorthQueryRunningWorkflowRun) {
    run.artifacts().registry().close_cancelled();
}

fn main() {}
