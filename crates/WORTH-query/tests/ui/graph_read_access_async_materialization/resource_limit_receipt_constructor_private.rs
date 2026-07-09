use worth_query::facade::runtime::{
    WorthQueryGraphReadMaterializationJobState,
    WorthQueryGraphReadMaterializationResourceLimitReceipt,
};

fn main() {
    let _ = WorthQueryGraphReadMaterializationResourceLimitReceipt {
        digest: String::new(),
        job_digest: String::new(),
        request_digest: String::new(),
        progress_digest: String::new(),
        last_checkpoint_digest: String::new(),
        estimated_resident_bytes: 0,
        max_resident_bytes: 0,
        final_job_state: WorthQueryGraphReadMaterializationJobState::Indeterminate,
    };
}
