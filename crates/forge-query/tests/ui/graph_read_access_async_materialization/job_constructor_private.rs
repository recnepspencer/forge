use forge_query::facade::runtime::{
    ForgeQueryGraphReadMaterializationCheckpoint, ForgeQueryGraphReadMaterializationJob,
    ForgeQueryGraphReadMaterializationJobState, ForgeQueryGraphReadMaterializationProgress,
    ForgeQueryGraphReadMaterializationRequest,
};

fn main() {
    let _ = ForgeQueryGraphReadMaterializationJob {
        digest: String::new(),
        request: forged_request(),
        snapshot_identity: String::new(),
        progress: forged_progress(),
        target_progress: forged_progress(),
        checkpoint: forged_checkpoint(),
        checkpoints: vec![],
        state: ForgeQueryGraphReadMaterializationJobState::Running,
    };
}

fn forged_request() -> ForgeQueryGraphReadMaterializationRequest {
    loop {}
}

fn forged_progress() -> ForgeQueryGraphReadMaterializationProgress {
    loop {}
}

fn forged_checkpoint() -> ForgeQueryGraphReadMaterializationCheckpoint {
    loop {}
}
