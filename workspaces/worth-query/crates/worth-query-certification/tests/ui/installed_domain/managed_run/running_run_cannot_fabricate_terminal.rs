use worth_query_execution::facade::domain_computation::{
    WorthQueryRunningDirectRun, WorthQueryRunningWorkflowRun,
};

fn fabricate_direct(run: WorthQueryRunningDirectRun) {
    let _ = run.cancelled();
}

fn fabricate_workflow(run: WorthQueryRunningWorkflowRun) {
    let _ = run.failed();
}

fn main() {}
