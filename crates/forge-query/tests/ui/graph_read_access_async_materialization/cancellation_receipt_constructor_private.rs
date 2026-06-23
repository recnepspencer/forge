use forge_query::facade::runtime::{
    ForgeQueryGraphReadMaterializationCancellationReceipt,
    ForgeQueryGraphReadMaterializationJobState,
};

fn main() {
    let _ = ForgeQueryGraphReadMaterializationCancellationReceipt {
        digest: String::new(),
        job_digest: String::new(),
        request_digest: String::new(),
        progress_digest: String::new(),
        last_checkpoint_digest: String::new(),
        released_frontier_pages: 0,
        released_allocated_bytes: 0,
        cancellation_poll_count: 0,
        cancellation_scope: String::new(),
        final_job_state: ForgeQueryGraphReadMaterializationJobState::Cancelled,
    };
}
