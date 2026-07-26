use worth_query_execution::facade::domain_computation::{
    WorthQueryActiveDirectGraphExecution, WorthQueryActiveWorkflowGraphExecution,
};

fn yield_active_direct(active: WorthQueryActiveDirectGraphExecution) {
    let _ = active.yield_run();
}

fn yield_active_workflow(active: WorthQueryActiveWorkflowGraphExecution) {
    let _ = active.yield_run();
}

fn main() {}
