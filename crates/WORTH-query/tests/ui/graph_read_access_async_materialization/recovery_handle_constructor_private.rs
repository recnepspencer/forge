use worth_query::facade::runtime::WorthQueryGraphReadMaterializationRecoveryHandle;

fn main() {
    let _ = WorthQueryGraphReadMaterializationRecoveryHandle {
        digest: String::new(),
        job_digest: String::new(),
        request_digest: String::new(),
        last_checkpoint_digest: String::new(),
        progress_digest: String::new(),
        recovery_reason: String::new(),
    };
}
