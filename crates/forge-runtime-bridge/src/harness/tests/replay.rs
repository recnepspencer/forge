use forge_harness::facade::{ExecutionProfile, ExecutionRequest, ReplayRequest, ScenarioPlan};
use forge_harness::runtime::{HarnessAdapter, ReplayHarnessAdapter};
use std::sync::Arc;

use crate::facade::{
    BridgeBulkWorkloadRequest, BridgeBulkWorkloadSegment, BridgeContinuityAuthorityBasis,
    BridgeHistoricalLineageAuthority, BridgeHistoricalLineageRequest, BridgeLineageContext,
    BridgeLineageSourceError, BridgeRouteRequest, ContinuityLineageSource, SubscriptionSliceKind,
    TruthDeltaSurfaceKind, TruthSnapshotIdentity, BRIDGE_CANONICAL_BULK_PLAN_RECORD_SCHEMA_V1,
};

use crate::harness::adapter::BridgeHarnessAdapter;
use crate::harness::fixtures::{BridgeHarnessFixture, InMemoryRelationalBridgeSource, RecordingSignalBridgeSink};
use super::support::{
    build_runtime, build_runtime_with_aspects, committed_patch, committed_patch_items,
    field_aspect_registration, field_aspect_registration_with_kind, field_slice_snapshot,
    registration, snapshot,
};

#[derive(Debug, Clone, Default)]
struct ReplaySingleSuccessorLineageSource;

impl ContinuityLineageSource for ReplaySingleSuccessorLineageSource {
    fn historical_lineage(
        &self,
        request: BridgeHistoricalLineageRequest,
    ) -> Result<BridgeHistoricalLineageAuthority, BridgeLineageSourceError> {
        BridgeHistoricalLineageAuthority::try_new(
            request.authority_basis().clone(),
            vec![Arc::from("lineage:replay-successor")],
            vec![Arc::from("entity:0:4:2")],
            vec![7],
        )
    }
}

#[derive(Debug, Clone, Default)]
struct ReplayDriftedSuccessorLineageSource;

impl ContinuityLineageSource for ReplayDriftedSuccessorLineageSource {
    fn historical_lineage(
        &self,
        request: BridgeHistoricalLineageRequest,
    ) -> Result<BridgeHistoricalLineageAuthority, BridgeLineageSourceError> {
        BridgeHistoricalLineageAuthority::try_new(
            request.authority_basis().clone(),
            vec![Arc::from("lineage:replay-successor")],
            vec![Arc::from("entity:0:9:2")],
            vec![7],
        )
    }
}

#[test]
fn bridge_replay_capture_exposes_last_route_record() {
    let adapter = BridgeHarnessAdapter;
    let fixture = ScenarioPlan::new(
        "bridge-replay",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"))
            .with_snapshot(snapshot("snapshot-a", "alice")),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let request = ExecutionRequest::target("deliver-commit-a", "commit-a".to_string());
    let profile = ExecutionProfile::development("development");

    let mut session = adapter.create_runtime().expect("bridge harness runtime");
    adapter
        .prepare_runtime(&mut session, &profile)
        .expect("bridge harness prepare");
    adapter
        .load_fixture(&mut session, &fixture)
        .expect("bridge harness load fixture");
    let run = adapter
        .execute(&mut session, &fixture, &request, &profile)
        .expect("bridge harness execute");
    let replay = adapter
        .capture_replay(
            &session,
            &fixture,
            &ReplayRequest {
                name: "replay".to_string(),
                source_run: run,
                request: request.clone(),
                profile: profile.clone(),
            },
        )
        .expect("bridge replay capture should succeed");

    assert_eq!(replay.summary["source_commit"], "commit-a");
    assert_eq!(replay.summary["source_snapshot"], "snapshot-a");
}

#[test]
fn bridge_replay_accepts_versioned_canonical_route_record() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let runtime = build_runtime(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let route = runtime
        .plan_committed_patch(BridgeRouteRequest::for_commit("commit-a"))
        .expect("bridge should plan route before canonical replay capture");
    runtime
        .deliver_invalidation(route)
        .expect("bridge should deliver route before canonical replay capture");
    let canonical_record = runtime
        .diagnostics()
        .last_canonical_route_record()
        .expect("bridge should expose a versioned canonical route record");

    let replay = runtime
        .replay_canonical_record(&canonical_record)
        .expect("bridge should replay a supported canonical route record");

    assert_eq!(replay.source_commit().as_str(), "commit-a");
    assert_eq!(
        canonical_record.schema_version(),
        crate::facade::BRIDGE_CANONICAL_ROUTE_RECORD_SCHEMA_V3
    );
}

#[test]
fn bridge_replay_preserves_canonical_route_outcome_for_delivered_patch() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(field_slice_snapshot("snapshot-a", "alice"));
    let runtime = build_runtime_with_aspects(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
        vec![field_aspect_registration()],
    );

    let result = runtime
        .deliver_invalidation(
            runtime
                .plan_committed_patch(BridgeRouteRequest::for_commit("commit-a"))
                .expect("route should plan before replay parity certification"),
        )
        .expect("route should deliver before replay parity certification");
    let canonical_record = runtime
        .diagnostics()
        .last_canonical_route_record()
        .expect("bridge should retain a canonical route record for replay parity certification");

    let replay = runtime
        .replay_canonical_record(&canonical_record)
        .expect("bridge replay should preserve the canonical route outcome");

    assert_eq!(
        replay.route_identity(),
        result.result_summary().route_identity()
    );
    assert_eq!(
        replay.invalidation_identity(),
        result.result_summary().invalidation_identity()
    );
    assert_eq!(
        replay.subscription_slice_identity(),
        result.result_summary().subscription_slice_identity()
    );
    assert_eq!(replay.source_commit(), result.result_summary().source_commit());
    assert_eq!(replay.source_patch(), result.result_summary().source_patch());
    assert_eq!(
        replay.source_snapshot(),
        result.result_summary().snapshot_identity()
    );
}

#[test]
fn bridge_replay_rejects_subscription_slice_drift() {
    let original_source = InMemoryRelationalBridgeSource::default();
    original_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    original_source.insert_snapshot(field_slice_snapshot("snapshot-a", "alice"));
    let original_runtime = build_runtime_with_aspects(
        original_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
        vec![field_aspect_registration()],
    );

    let route = original_runtime
        .plan_committed_patch(BridgeRouteRequest::for_commit("commit-a"))
        .expect("original route should plan before replay certification");
    original_runtime
        .deliver_invalidation(route)
        .expect("original route should deliver before replay certification");
    let canonical_record = original_runtime
        .diagnostics()
        .last_canonical_route_record()
        .expect("original runtime should expose a canonical route record");

    let restarted_source = InMemoryRelationalBridgeSource::default();
    restarted_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    restarted_source.insert_snapshot(field_slice_snapshot("snapshot-a", "alice"));
    let restarted_runtime = build_runtime_with_aspects(
        restarted_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
        vec![field_aspect_registration_with_kind(
            "profile-name-region",
            TruthDeltaSurfaceKind::EntityField,
            SubscriptionSliceKind::SignalRegion,
        )],
    );

    let error = restarted_runtime
        .replay_canonical_record(&canonical_record)
        .expect_err("replay should reject subscription slice identity drift");

    assert_eq!(
        error.kind(),
        crate::facade::BridgeReplayErrorKind::SubscriptionSliceMismatch
    );
    assert!(error.to_string().contains("subscription slices"));
}

#[test]
fn bridge_replay_rejects_incompatible_canonical_route_record_version() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let runtime = build_runtime(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let route = runtime
        .plan_committed_patch(BridgeRouteRequest::for_commit("commit-a"))
        .expect("bridge should plan route before canonical replay failure test");
    runtime
        .deliver_invalidation(route)
        .expect("bridge should deliver route before canonical replay failure test");
    let canonical_record = runtime
        .diagnostics()
        .last_canonical_route_record()
        .expect("bridge should expose a canonical route record")
        .with_schema_version_for_test("forge-runtime-bridge.route-record.v999");

    let error = runtime
        .replay_canonical_record(&canonical_record)
        .expect_err("bridge should reject unsupported canonical route record versions");

    assert_eq!(
        error.kind(),
        crate::facade::BridgeReplayErrorKind::CanonicalArtifactCompatibilityFailure
    );
    assert!(error.to_string().contains("not supported"));
}

#[test]
fn bridge_replay_detects_route_drift_after_restart_shaped_truth_change() {
    let adapter = BridgeHarnessAdapter;
    let fixture = ScenarioPlan::new(
        "bridge-replay-restart-drift",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"))
            .with_snapshot(snapshot("snapshot-a", "alice")),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let request = ExecutionRequest::target("deliver-commit-a", "commit-a".to_string());
    let profile = ExecutionProfile::development("development");

    let mut original = adapter.create_runtime().expect("bridge harness runtime");
    adapter
        .prepare_runtime(&mut original, &profile)
        .expect("bridge harness prepare");
    adapter
        .load_fixture(&mut original, &fixture)
        .expect("bridge harness load fixture");
    adapter
        .execute(&mut original, &fixture, &request, &profile)
        .expect("bridge harness execute");
    let original_record = original
        .runtime
        .as_ref()
        .expect("bridge runtime")
        .diagnostics()
        .last_canonical_route_record()
        .expect("original canonical bridge route record");

    let drifted_fixture = ScenarioPlan::new(
        "bridge-replay-restart-drift-rehydrated",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch_items(
                "commit-a",
                "patch-a",
                "snapshot-a",
                vec![
                    crate::facade::BridgeCommittedPatchItem::new("user", "profile", "avatar"),
                    crate::facade::BridgeCommittedPatchItem::new("user", "profile", "name"),
                ],
            ))
            .with_snapshot(snapshot("snapshot-a", "alice")),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let mut restarted = adapter.create_runtime().expect("bridge harness runtime");
    adapter
        .prepare_runtime(&mut restarted, &profile)
        .expect("bridge harness prepare");
    adapter
        .load_fixture(&mut restarted, &drifted_fixture)
        .expect("bridge harness load fixture");
    let error = restarted
        .runtime
        .as_ref()
        .expect("bridge runtime")
        .replay_canonical_record(&original_record)
        .expect_err("bridge replay should reject route drift after restart");

    assert!(!error.to_string().is_empty());
    let failure_record = restarted
        .runtime
        .as_ref()
        .expect("bridge runtime")
        .diagnostics()
        .last_failure_record()
        .expect("bridge replay failure record");
    assert_eq!(failure_record.counters().route_replay_mismatch_count(), 1);
}

#[test]
fn bridge_continuity_replay_matches_original_canonical_artifact() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(field_slice_snapshot("snapshot-a", "alice"));
    let runtime = crate::facade::RuntimeBridgeBuilder::new()
        .with_relational_source(source)
        .with_signal_sink(RecordingSignalBridgeSink::default())
        .with_continuity_lineage_source(ReplaySingleSuccessorLineageSource)
        .register_mapping(registration())
        .register_aspect_mapping(field_aspect_registration())
        .build()
        .expect("runtime should build");

    let route = runtime
        .plan_committed_patch_with_mapping_context(
            BridgeRouteRequest::for_commit("commit-a"),
            crate::facade::BridgeMappingContext::default().with_lineage_context(
                BridgeLineageContext::new(BridgeContinuityAuthorityBasis::new(
                    crate::facade::TruthBranchIdentity::new("main"),
                    TruthSnapshotIdentity::new("snapshot-a"),
                )),
            ),
        )
        .expect("route should plan");
    let result = runtime
        .deliver_invalidation(route)
        .expect("delivery should succeed");
    let route_record = runtime
        .diagnostics()
        .route_record_for_route_identity(result.result_summary().route_identity().as_str())
        .expect("route record should be retained");
    let requests = runtime
        .plan_continuity_requests(&route_record)
        .expect("continuity requests should plan");
    let packet = runtime
        .plan_historical_lineage_packet(&requests)
        .expect("historical lineage packet should plan");
    let resolved = runtime
        .resolve_lineage_continuity(&packet)
        .expect("continuity should resolve");
    let artifact = runtime.lower_continuity_artifact(&resolved);
    let canonical = runtime.canonicalize_continuity_record(&route_record, &requests, &artifact);

    let replay = runtime
        .replay_canonical_continuity_record(&canonical)
        .expect("continuity replay should preserve canonical artifact");

    assert_eq!(replay.continuity_identity(), artifact.continuity_identity());
    assert_eq!(
        replay.remapped_subscription_slice_identity(),
        artifact.remapped_subscription_slice_identity()
    );
    assert_eq!(replay.remapped_slices(), artifact.remapped_slices());
}

#[test]
fn bridge_continuity_replay_rejects_artifact_drift() {
    let original_source = InMemoryRelationalBridgeSource::default();
    original_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    original_source.insert_snapshot(field_slice_snapshot("snapshot-a", "alice"));
    let original_runtime = crate::facade::RuntimeBridgeBuilder::new()
        .with_relational_source(original_source)
        .with_signal_sink(RecordingSignalBridgeSink::default())
        .with_continuity_lineage_source(ReplaySingleSuccessorLineageSource)
        .register_mapping(registration())
        .register_aspect_mapping(field_aspect_registration())
        .build()
        .expect("runtime should build");

    let route = original_runtime
        .plan_committed_patch_with_mapping_context(
            BridgeRouteRequest::for_commit("commit-a"),
            crate::facade::BridgeMappingContext::default().with_lineage_context(
                BridgeLineageContext::new(BridgeContinuityAuthorityBasis::new(
                    crate::facade::TruthBranchIdentity::new("main"),
                    TruthSnapshotIdentity::new("snapshot-a"),
                )),
            ),
        )
        .expect("route should plan");
    let result = original_runtime
        .deliver_invalidation(route)
        .expect("delivery should succeed");
    let route_record = original_runtime
        .diagnostics()
        .route_record_for_route_identity(result.result_summary().route_identity().as_str())
        .expect("route record should be retained");
    let requests = original_runtime
        .plan_continuity_requests(&route_record)
        .expect("continuity requests should plan");
    let packet = original_runtime
        .plan_historical_lineage_packet(&requests)
        .expect("historical lineage packet should plan");
    let resolved = original_runtime
        .resolve_lineage_continuity(&packet)
        .expect("continuity should resolve");
    let artifact = original_runtime.lower_continuity_artifact(&resolved);
    let canonical = original_runtime.canonicalize_continuity_record(&route_record, &requests, &artifact);

    let restarted_source = InMemoryRelationalBridgeSource::default();
    restarted_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    restarted_source.insert_snapshot(field_slice_snapshot("snapshot-a", "alice"));
    let restarted_runtime = crate::facade::RuntimeBridgeBuilder::new()
        .with_relational_source(restarted_source)
        .with_signal_sink(RecordingSignalBridgeSink::default())
        .with_continuity_lineage_source(ReplayDriftedSuccessorLineageSource)
        .register_mapping(registration())
        .register_aspect_mapping(field_aspect_registration())
        .build()
        .expect("runtime should build");

    let error = restarted_runtime
        .replay_canonical_continuity_record(&canonical)
        .expect_err("continuity replay should reject successor drift");

    assert_eq!(
        error.kind(),
        crate::facade::BridgeReplayErrorKind::ContinuityResolutionMismatch
    );
    assert!(!error.to_string().is_empty());
}

#[test]
fn bridge_continuity_replay_rejects_incompatible_canonical_record_version() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(field_slice_snapshot("snapshot-a", "alice"));
    let runtime = crate::facade::RuntimeBridgeBuilder::new()
        .with_relational_source(source)
        .with_signal_sink(RecordingSignalBridgeSink::default())
        .with_continuity_lineage_source(ReplaySingleSuccessorLineageSource)
        .register_mapping(registration())
        .register_aspect_mapping(field_aspect_registration())
        .build()
        .expect("runtime should build");

    let route = runtime
        .plan_committed_patch_with_mapping_context(
            BridgeRouteRequest::for_commit("commit-a"),
            crate::facade::BridgeMappingContext::default().with_lineage_context(
                BridgeLineageContext::new(BridgeContinuityAuthorityBasis::new(
                    crate::facade::TruthBranchIdentity::new("main"),
                    TruthSnapshotIdentity::new("snapshot-a"),
                )),
            ),
        )
        .expect("route should plan");
    let result = runtime
        .deliver_invalidation(route)
        .expect("delivery should succeed");
    let route_record = runtime
        .diagnostics()
        .route_record_for_route_identity(result.result_summary().route_identity().as_str())
        .expect("route record should be retained");
    let requests = runtime
        .plan_continuity_requests(&route_record)
        .expect("continuity requests should plan");
    let packet = runtime
        .plan_historical_lineage_packet(&requests)
        .expect("historical lineage packet should plan");
    let resolved = runtime
        .resolve_lineage_continuity(&packet)
        .expect("continuity should resolve");
    let artifact = runtime.lower_continuity_artifact(&resolved);
    let canonical = runtime
        .canonicalize_continuity_record(&route_record, &requests, &artifact)
        .with_schema_version_for_test("forge-runtime-bridge.continuity-record.v999");

    let error = runtime
        .replay_canonical_continuity_record(&canonical)
        .expect_err("continuity replay should reject unsupported canonical record versions");

    assert_eq!(
        error.kind(),
        crate::facade::BridgeReplayErrorKind::CanonicalArtifactCompatibilityFailure
    );
    assert!(error.to_string().contains("not supported"));
}

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
