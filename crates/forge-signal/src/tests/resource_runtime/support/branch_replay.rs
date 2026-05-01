use super::*;

pub(in crate::tests::resource_runtime) fn resource_branch_replay_workload(
    retained_denial_request_id: ResourceRequestId,
) -> ResourceBranchReplayWorkloadOutcome {
    let mut graph = SignalGraph::new();
    let lifecycle_node = graph.node().build();
    let cancel_node = graph.node().build();
    let timeout_node = graph.node().build();
    let malformed_node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(lifecycle_node))
        .expect("lifecycle declaration should lower");
    runtime
        .declare_resource_node(resource_declaration(cancel_node))
        .expect("cancel declaration should lower");
    runtime
        .declare_resource_node(timeout_resource_declaration(timeout_node, 3))
        .expect("timeout declaration should lower");
    runtime
        .declare_resource_node(resource_declaration(malformed_node))
        .expect("malformed declaration should lower");

    let main = runtime.observe().current_branch();
    let feature = runtime
        .create_branch("resource-branch-feature")
        .expect("feature branch should create");
    let sibling = runtime
        .create_branch("resource-branch-sibling")
        .expect("sibling branch should create");

    runtime
        .switch_branch(feature.clone())
        .expect("feature branch should activate");
    let (
        feature_snapshot,
        feature_head_before_restore,
        feature_before_restore,
        feature_replay_history_before_restore,
        feature_after_snapshot_drift,
    ) = exercise_resource_async_hostile_suffix_on_active_branch(
        &mut runtime,
        lifecycle_node,
        cancel_node,
        timeout_node,
        malformed_node,
        retained_denial_request_id,
    );

    runtime
        .switch_branch(sibling.clone())
        .expect("sibling branch should activate");
    let (
        sibling_snapshot,
        sibling_head_before_restore,
        sibling_before_restore,
        sibling_replay_history_before_restore,
        sibling_after_snapshot_drift,
    ) = exercise_resource_async_hostile_suffix_on_active_branch(
        &mut runtime,
        lifecycle_node,
        cancel_node,
        timeout_node,
        malformed_node,
        retained_denial_request_id,
    );

    runtime
        .switch_branch(main.clone())
        .expect("main branch should reactivate before inactive restores");
    runtime
        .restore_branch_snapshot(feature.clone(), &feature_snapshot)
        .expect("inactive feature branch restore should succeed");

    runtime
        .switch_branch(sibling.clone())
        .expect("sibling branch should still be independently accessible before its restore");
    let sibling_still_drifted = runtime.reconstruct_resource_replay_summary();
    assert_eq!(
        sibling_still_drifted.replay_digest(),
        sibling_after_snapshot_drift.replay_digest(),
        "restoring feature must not mutate sibling branch-local replay truth"
    );

    runtime
        .switch_branch(feature.clone())
        .expect("feature branch should activate after restore");
    let feature_head_after_restore = runtime.observe().branch_head_snapshot_id(feature.id);
    let feature_after_restore = runtime.reconstruct_resource_replay_summary();
    let feature_replay_history_after_restore = runtime.observe().replay_for_branch(feature.id);
    let feature_diagnostics_after_restore =
        runtime.resource_diagnostics_summary_with_unbounded_cold_reconstruction();
    runtime
        .restore_snapshot(&feature_snapshot)
        .expect("active feature restore should publish resource evidence");
    let feature_restore_report = runtime
        .latest_resource_branch_restore_report()
        .expect("active feature restore should publish resource evidence");
    let feature_after_reported_restore = runtime.reconstruct_resource_replay_summary();
    assert_eq!(
        feature_after_reported_restore.replay_digest(),
        feature_after_restore.replay_digest(),
        "report-emitting active restore must preserve feature replay truth"
    );

    runtime
        .restore_branch_snapshot(sibling.clone(), &sibling_snapshot)
        .expect("inactive sibling branch restore should succeed");

    let feature_still_restored = runtime.reconstruct_resource_replay_summary();
    assert_eq!(
        feature_still_restored.replay_digest(),
        feature_after_restore.replay_digest(),
        "restoring sibling must not perturb already-restored feature truth"
    );

    runtime
        .switch_branch(sibling.clone())
        .expect("sibling branch should activate after restore");
    let sibling_head_after_restore = runtime.observe().branch_head_snapshot_id(sibling.id);
    let sibling_after_restore = runtime.reconstruct_resource_replay_summary();
    let sibling_replay_history_after_restore = runtime.observe().replay_for_branch(sibling.id);
    let sibling_diagnostics_after_restore =
        runtime.resource_diagnostics_summary_with_unbounded_cold_reconstruction();
    runtime
        .restore_snapshot(&sibling_snapshot)
        .expect("active sibling restore should publish resource evidence");
    let sibling_restore_report = runtime
        .latest_resource_branch_restore_report()
        .expect("active sibling restore should publish resource evidence");
    let sibling_after_reported_restore = runtime.reconstruct_resource_replay_summary();
    assert_eq!(
        sibling_after_reported_restore.replay_digest(),
        sibling_after_restore.replay_digest(),
        "report-emitting active restore must preserve sibling replay truth"
    );

    ResourceBranchReplayWorkloadOutcome {
        feature: ResourceBranchReplayWorkloadBranchState {
            branch_id: feature.id,
            head_snapshot_before_restore: feature_head_before_restore,
            head_snapshot_after_restore: feature_head_after_restore,
            replay_before_restore: feature_before_restore,
            replay_after_snapshot_drift: feature_after_snapshot_drift,
            replay_after_restore: feature_after_restore,
            replay_history_before_restore: feature_replay_history_before_restore,
            replay_history_after_restore: feature_replay_history_after_restore,
            diagnostics_after_restore: feature_diagnostics_after_restore,
            restore_report: feature_restore_report,
        },
        sibling: ResourceBranchReplayWorkloadBranchState {
            branch_id: sibling.id,
            head_snapshot_before_restore: sibling_head_before_restore,
            head_snapshot_after_restore: sibling_head_after_restore,
            replay_before_restore: sibling_before_restore,
            replay_after_snapshot_drift: sibling_after_snapshot_drift,
            replay_after_restore: sibling_after_restore,
            replay_history_before_restore: sibling_replay_history_before_restore,
            replay_history_after_restore: sibling_replay_history_after_restore,
            diagnostics_after_restore: sibling_diagnostics_after_restore,
            restore_report: sibling_restore_report,
        },
    }
}
