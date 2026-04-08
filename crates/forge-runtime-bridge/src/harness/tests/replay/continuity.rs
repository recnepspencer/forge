use super::*;

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

