use worth_query::facade::runtime::{WorthQueryGraphReadMaterializationCheckpoint, WorthQueryGraphReadMaterializationJob, WorthQueryGraphReadMaterializationJobState, WorthQueryGraphReadMaterializationProgress, WorthQueryGraphReadMaterializationRequest};

fn main() {
    let _ = WorthQueryGraphReadMaterializationJob {
        digest: String::new(),
        request: worthd_request(),
        snapshot_identity: String::new(),
        progress: worthd_progress(),
        target_progress: worthd_progress(),
        checkpoint: worthd_checkpoint(),
        checkpoints: vec![],
        state: WorthQueryGraphReadMaterializationJobState::Running,
    };
}

fn worthd_request() -> WorthQueryGraphReadMaterializationRequest {
    loop {}
}

fn worthd_progress() -> WorthQueryGraphReadMaterializationProgress {
    loop {}
}

fn worthd_checkpoint() -> WorthQueryGraphReadMaterializationCheckpoint {
    loop {}
}
