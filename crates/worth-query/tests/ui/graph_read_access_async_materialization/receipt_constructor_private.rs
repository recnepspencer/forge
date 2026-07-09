use worth_query::facade::runtime::WorthQueryGraphReadMaterializationReceipt;

fn main() {
    let _ = WorthQueryGraphReadMaterializationReceipt {
        digest: String::new(),
        job_digest: String::new(),
        request_digest: String::new(),
        admission_digest: String::new(),
        materialization_digest: String::new(),
        final_progress_digest: String::new(),
        final_checkpoint_digest: String::new(),
        emitted_rows: 0,
        touched_edges: 0,
        max_resident_bytes_observed: 0,
        checkpoint_count: 0,
    };
}
