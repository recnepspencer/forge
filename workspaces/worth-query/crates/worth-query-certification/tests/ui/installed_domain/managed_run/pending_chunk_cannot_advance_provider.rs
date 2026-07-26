use worth_query_execution::facade::domain_computation::{
    WorthQueryPendingDirectGraphChunk, WorthQueryPendingWorkflowGraphChunk,
};

fn advance_direct(pending: WorthQueryPendingDirectGraphChunk) {
    let _ = pending.advance();
}

fn advance_workflow(pending: WorthQueryPendingWorkflowGraphChunk) {
    let _ = pending.advance();
}

fn main() {}
