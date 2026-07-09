use crate::runtime::tests::support::*;
use crate::runtime::tests::worker_runtime::fixtures::portable_counter_graph::{
    portable_counter_publication, set_counter,
};
use crate::runtime::worker_host::WorkerRuntimeShell;

fn worker_shell_with_counter_graph() -> WorkerRuntimeShell {
    let mut worker_shell = WorkerRuntimeShell::new(RuntimePolicySpec::default()).unwrap();
    worker_shell
        .publish_graph(portable_counter_publication())
        .unwrap();
    worker_shell
}

fn restore_feature_branch_after_main_churn() -> WorkerRuntimeShell {
    let mut worker_shell = worker_shell_with_counter_graph();
    let main_branch = worker_shell.branch_truth_envelope().unwrap();
    let feature_branch = worker_shell
        .create_branch("worker-replay-feature".to_owned())
        .unwrap();
    worker_shell.switch_branch(feature_branch.id.0).unwrap();
    worker_shell
        .apply_committed_transaction(set_counter(11.0))
        .unwrap();
    let feature_snapshot = worker_shell.branch_snapshot(feature_branch.id.0).unwrap();
    worker_shell.switch_branch(main_branch.branch_id).unwrap();
    worker_shell
        .apply_committed_transaction(set_counter(3.0))
        .unwrap();
    worker_shell
        .restore_branch_snapshot_with_capability_report(feature_branch.id.0, feature_snapshot)
        .unwrap();
    worker_shell
}

#[test]
fn worker_replay_restore_capability_certifies_same_runtime_exact_restore() {
    let mut worker_shell = restore_feature_branch_after_main_churn();

    let package = worker_shell
        .certify_worker_replay_restore_capability()
        .unwrap();

    assert_eq!(
        package.certification_family,
        "workerReplayRestoreCapabilityCertification"
    );
    assert_eq!(package.covered_suite_count, 1);
    assert_eq!(package.restore_outcome, "SameRuntimeExactRestore");
    assert_eq!(
        package.exact_restore_artifact,
        "sameRuntimeBranchSnapshotStore"
    );
    assert_eq!(package.incompatibility_artifact, "none");
    assert_eq!(package.fallback_count, 0);
    assert!(package.replay_frame_count > 0);
    assert_digest_shape(&package.worker_first_truth_digest);
    assert_digest_shape(&package.snapshot_digest);
    assert_digest_shape(&package.replay_restore_digest);
    assert_digest_shape(&package.capability_availability_digest);
    assert_digest_shape(&package.replay_import_compatibility_digest);
    assert_digest_shape(&package.placement_identity_digest);
    assert_digest_shape(&package.lowered_plan_identity_digest);
    assert_digest_shape(&package.branch_restore_digest);
    assert_digest_shape(&package.certification_digest);

    worker_shell.switch_branch(package.branch_id).unwrap();
    assert_eq!(
        worker_shell.read_value("doubleCounter").unwrap(),
        SignalValue::Number(22.0)
    );
}

#[test]
fn worker_replay_restore_capability_rejects_missing_restore_evidence() {
    let mut worker_shell = worker_shell_with_counter_graph();

    let error = worker_shell
        .certify_worker_replay_restore_capability()
        .unwrap_err();

    assert!(error.message.contains("replay/restore evidence"));
}

#[test]
fn worker_replay_restore_capability_rejects_cleared_evidence_after_mutation() {
    let mut worker_shell = restore_feature_branch_after_main_churn();
    worker_shell
        .apply_committed_transaction(set_counter(13.0))
        .unwrap();

    let error = worker_shell
        .certify_worker_replay_restore_capability()
        .unwrap_err();

    assert!(error.message.contains("replay/restore evidence"));
}

#[test]
fn worker_replay_restore_capability_rejects_capability_churn_after_restore() {
    let mut worker_shell = restore_feature_branch_after_main_churn();
    worker_shell
        .define_main_thread_hosted_callback_for_test(
            "lateHostedCallback".to_owned(),
            Box::new(|| {
                Ok(compute_callbacks::ComputeCallbackInvocationResult {
                    value: SignalValue::Number(5.0),
                    captured_read_ids: Vec::new(),
                    captured_host_capability_reads: Vec::new(),
                    runtime_read_breadth: 0,
                    return_serialization_breadth: 1,
                })
            }),
        )
        .unwrap();

    let error = worker_shell
        .certify_worker_replay_restore_capability()
        .unwrap_err();

    assert!(error.message.contains("current capability posture"));
}
