use forge_query::facade::runtime::ForgeQueryGraphReadMaterializationRecoveryHandle;

fn main() {
    let _ = ForgeQueryGraphReadMaterializationRecoveryHandle {
        digest: String::new(),
        job_digest: String::new(),
        request_digest: String::new(),
        last_checkpoint_digest: String::new(),
        progress_digest: String::new(),
        recovery_reason: String::new(),
    };
}
