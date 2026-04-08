use super::*;

#[test]
fn replayed_bulk_plan_matches_original_canonical_artifact() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-b", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    source.insert_snapshot(snapshot("snapshot-b", "bob"));
    let runtime = build_runtime(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let planned = runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-b")),
        ]))
        .expect("bulk workload should plan before canonical bulk replay");
    let canonical = runtime.canonicalize_bulk_workload_plan(&planned);

    let replayed = runtime
        .replay_canonical_bulk_plan_record(&canonical)
        .expect("bulk canonical replay should preserve the canonical plan");

    assert_eq!(
        canonical.schema_version(),
        BRIDGE_CANONICAL_BULK_PLAN_RECORD_SCHEMA_V1
    );
    assert_eq!(replayed.workload_identity(), planned.workload_identity());
    assert_eq!(
        replayed.canonical_planning_identity(),
        planned.canonical_planning_identity()
    );
    assert_eq!(replayed.packet_set().digest(), planned.packet_set().digest());
    assert_eq!(
        replayed.execution_plan().digest(),
        planned.execution_plan().digest()
    );
}

#[test]
fn bulk_replay_rejects_incompatible_canonical_plan_record_version() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let runtime = build_runtime(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let planned = runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![BridgeBulkWorkloadSegment::new(
            BridgeRouteRequest::for_commit("commit-a"),
        )]))
        .expect("bulk workload should plan before canonical bulk replay failure test");
    let canonical = runtime
        .canonicalize_bulk_workload_plan(&planned)
        .with_schema_version_for_test("forge-runtime-bridge.bulk-plan-record.v999");

    let error = runtime
        .replay_canonical_bulk_plan_record(&canonical)
        .expect_err("bulk replay should reject unsupported canonical bulk plan record versions");

    assert_eq!(
        error.kind(),
        crate::facade::BridgeReplayErrorKind::CanonicalArtifactCompatibilityFailure
    );
    assert!(error.to_string().contains("bulk plan record schema"));
}

#[test]
fn bulk_replay_rejects_drift_after_restart_shaped_truth_change() {
    let original_source = InMemoryRelationalBridgeSource::default();
    original_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    original_source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-b", "name"));
    original_source.insert_snapshot(snapshot("snapshot-a", "alice"));
    original_source.insert_snapshot(snapshot("snapshot-b", "bob"));
    let original_runtime = build_runtime(
        original_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let planned = original_runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-b")),
        ]))
        .expect("original bulk workload should plan");
    let canonical = original_runtime.canonicalize_bulk_workload_plan(&planned);

    let restarted_source = InMemoryRelationalBridgeSource::default();
    restarted_source.insert_committed_patch(committed_patch_items(
        "commit-a",
        "patch-a",
        "snapshot-a",
        vec![
            crate::facade::BridgeCommittedPatchItem::new("user", "profile", "avatar"),
            crate::facade::BridgeCommittedPatchItem::new("user", "profile", "name"),
        ],
    ));
    restarted_source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-b", "name"));
    restarted_source.insert_snapshot(snapshot("snapshot-a", "alice"));
    restarted_source.insert_snapshot(snapshot("snapshot-b", "bob"));
    let restarted_runtime = build_runtime(
        restarted_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let error = restarted_runtime
        .replay_canonical_bulk_plan_record(&canonical)
        .expect_err("bulk replay should reject drift after restart-shaped truth change");

    assert_eq!(
        error.kind(),
        crate::facade::BridgeReplayErrorKind::BulkPlanReplayMismatch
    );
    assert!(error.to_string().contains("bulk replay"));
}
