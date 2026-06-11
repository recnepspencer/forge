use worth_spatial::facade::retained_replay_workload::RetainedArtifactCaptureReceipt;

fn main() {
    let _ = RetainedArtifactCaptureReceipt {
        capture_identity: String::new(),
        retained_artifact_identity: String::new(),
        retained_basis_identity: String::new(),
        replay_checkpoint_identity: String::new(),
        retained_artifact_rows: 0,
    };
}
