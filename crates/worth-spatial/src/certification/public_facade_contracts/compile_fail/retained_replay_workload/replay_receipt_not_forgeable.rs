use worth_spatial::facade::retained_replay_workload::ReplayReceiptSet;

fn main() {
    let _ = ReplayReceiptSet {
        stage_receipt: unconstructible(),
        transformed_workload_identity: String::new(),
        retained_artifact_identity: String::new(),
        retained_artifact_capture_identity: String::new(),
        retained_basis_identity: String::new(),
        replay_checkpoint_identity: String::new(),
        replay_evidence_identity: String::new(),
        counters: unconstructible(),
    };
}

fn unconstructible<T>() -> T {
    panic!("compile-fail input is never executed")
}
