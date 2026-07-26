use worth_query_execution::facade::domain_computation::{
    WorthQueryPendingDirectGraphChunk, WorthQueryPendingWorkflowGraphChunk,
};

fn yield_direct(pending: WorthQueryPendingDirectGraphChunk) {
    let _ = pending.yield_run();
}

fn yield_workflow(pending: WorthQueryPendingWorkflowGraphChunk) {
    let _ = pending.yield_run();
}

fn main() {}
