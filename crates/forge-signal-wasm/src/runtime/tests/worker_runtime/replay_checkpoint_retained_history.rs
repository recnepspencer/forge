use crate::runtime::tests::support::*;
use crate::runtime::tests::worker_runtime::fixtures::portable_counter_graph::{
    set_counter, worker_shell_with_counter_graph,
};
use crate::runtime::worker_host::WorkerRuntimeShell;

fn worker_shell_with_checkpoint_and_retained_history() -> (
    WorkerRuntimeShell,
    u64,
    forge_signal::facade::history::RuntimeSnapshot,
) {
    let mut worker_shell = worker_shell_with_counter_graph();
    let feature_branch = worker_shell
        .create_branch("worker-checkpoint-feature".to_owned())
        .unwrap();
    worker_shell.switch_branch(feature_branch.id.0).unwrap();
    worker_shell
        .apply_committed_transaction(set_counter(5.0))
        .unwrap();
    let checkpoint = worker_shell.branch_snapshot(feature_branch.id.0).unwrap();
    worker_shell
        .apply_committed_transaction(set_counter(8.0))
        .unwrap();
    worker_shell
        .apply_committed_transaction(set_counter(13.0))
        .unwrap();

    (worker_shell, feature_branch.id.0, checkpoint)
}

#[test]
fn worker_replay_checkpoint_retained_history_certifies_checkpoint_plus_suffix() {
    let (mut worker_shell, branch_id, checkpoint) =
        worker_shell_with_checkpoint_and_retained_history();
    let report = worker_shell
        .record_worker_replay_checkpoint_retained_history(branch_id, checkpoint)
        .unwrap();

    let package = worker_shell
        .certify_worker_replay_checkpoint_retained_history()
        .unwrap();

    assert_eq!(report.envelope_family, "replayCheckpointRetainedHistory");
    assert_eq!(
        package.certification_family,
        "workerReplayCheckpointRetainedHistoryCertification"
    );
    assert_eq!(package.covered_suite_count, 1);
    assert_eq!(
        package.checkpoint_artifact,
        "workerBranchCheckpointSnapshot"
    );
    assert_eq!(
        package.retained_history_artifact,
        "checkpointPlusRetainedReplayHistory"
    );
    assert_eq!(
        package.exact_restore_artifact,
        "checkpointPlusRetainedReplayHistory"
    );
    assert_eq!(package.incompatibility_artifact, "none");
    assert_eq!(package.fallback_count, 0);
    assert!(package.checkpoint_replay_cursor > 0);
    assert!(package.retained_replay_frame_count > 0);
    assert!(package.full_replay_frame_count >= package.retained_replay_frame_count);
    assert_digest_shape(&package.worker_first_truth_digest);
    assert_digest_shape(&package.checkpoint_digest);
    assert_digest_shape(&package.full_replay_digest);
    assert_digest_shape(&package.retained_history_digest);
    assert_digest_shape(&package.replay_restore_digest);
    assert_digest_shape(&package.capability_availability_digest);
    assert_digest_shape(&package.replay_import_compatibility_digest);
    assert_digest_shape(&package.placement_identity_digest);
    assert_digest_shape(&package.lowered_plan_identity_digest);
    assert_digest_shape(&package.certification_digest);
    assert_eq!(
        worker_shell.read_value("doubleCounter").unwrap(),
        SignalValue::Number(26.0)
    );
}

#[test]
fn worker_replay_checkpoint_retained_history_rejects_missing_evidence() {
    let mut worker_shell = worker_shell_with_counter_graph();

    let error = worker_shell
        .certify_worker_replay_checkpoint_retained_history()
        .unwrap_err();

    assert!(error
        .message
        .contains("checkpoint retained-history evidence"));
}

#[test]
fn worker_replay_checkpoint_retained_history_rejects_checkpoint_without_suffix() {
    let mut worker_shell = worker_shell_with_counter_graph();
    let feature_branch = worker_shell
        .create_branch("worker-empty-suffix-feature".to_owned())
        .unwrap();
    worker_shell.switch_branch(feature_branch.id.0).unwrap();
    worker_shell
        .apply_committed_transaction(set_counter(5.0))
        .unwrap();
    let checkpoint = worker_shell.branch_snapshot(feature_branch.id.0).unwrap();

    let error = worker_shell
        .record_worker_replay_checkpoint_retained_history(feature_branch.id.0, checkpoint)
        .unwrap_err();

    assert!(error.message.contains("retained history after checkpoint"));
}

#[test]
fn worker_replay_checkpoint_retained_history_rejects_cross_branch_checkpoint() {
    let mut worker_shell = worker_shell_with_counter_graph();
    let main_branch = worker_shell.branch_truth_envelope().unwrap();
    let main_checkpoint = worker_shell.branch_snapshot(main_branch.branch_id).unwrap();
    let feature_branch = worker_shell
        .create_branch("worker-cross-branch-feature".to_owned())
        .unwrap();
    worker_shell.switch_branch(feature_branch.id.0).unwrap();
    worker_shell
        .apply_committed_transaction(set_counter(9.0))
        .unwrap();

    let error = worker_shell
        .record_worker_replay_checkpoint_retained_history(feature_branch.id.0, main_checkpoint)
        .unwrap_err();

    assert!(error
        .message
        .contains("checkpoint from the certified branch"));
}

#[test]
fn worker_replay_checkpoint_retained_history_rejects_cleared_evidence_after_mutation() {
    let (mut worker_shell, branch_id, checkpoint) =
        worker_shell_with_checkpoint_and_retained_history();
    worker_shell
        .record_worker_replay_checkpoint_retained_history(branch_id, checkpoint)
        .unwrap();
    worker_shell
        .apply_committed_transaction(set_counter(21.0))
        .unwrap();

    let error = worker_shell
        .certify_worker_replay_checkpoint_retained_history()
        .unwrap_err();

    assert!(error
        .message
        .contains("checkpoint retained-history evidence"));
}

#[test]
fn worker_replay_checkpoint_retained_history_rejects_capability_churn() {
    let (mut worker_shell, branch_id, checkpoint) =
        worker_shell_with_checkpoint_and_retained_history();
    worker_shell
        .record_worker_replay_checkpoint_retained_history(branch_id, checkpoint)
        .unwrap();
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
        .certify_worker_replay_checkpoint_retained_history()
        .unwrap_err();

    assert!(error.message.contains("current capability posture"));
}

#[test]
fn worker_replay_checkpoint_retained_history_rejects_branch_topology_churn() {
    let (mut worker_shell, branch_id, checkpoint) =
        worker_shell_with_checkpoint_and_retained_history();
    worker_shell
        .record_worker_replay_checkpoint_retained_history(branch_id, checkpoint)
        .unwrap();
    worker_shell
        .create_branch("late-topology-churn".to_owned())
        .unwrap();

    let error = worker_shell
        .certify_worker_replay_checkpoint_retained_history()
        .unwrap_err();

    assert!(error
        .message
        .contains("checkpoint retained-history evidence"));
}

#[test]
fn worker_replay_checkpoint_retained_history_rejects_snapshot_churn() {
    let (mut worker_shell, branch_id, checkpoint) =
        worker_shell_with_checkpoint_and_retained_history();
    worker_shell
        .record_worker_replay_checkpoint_retained_history(branch_id, checkpoint)
        .unwrap();
    worker_shell.branch_snapshot(branch_id).unwrap();

    let error = worker_shell
        .certify_worker_replay_checkpoint_retained_history()
        .unwrap_err();

    assert!(error
        .message
        .contains("checkpoint retained-history evidence"));
}
