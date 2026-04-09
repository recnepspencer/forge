#[test]
fn bridge_resolved_lineage_continuity_lowers_single_successor_artifact() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(field_slice_snapshot("snapshot-a", "alice"));
    let sink = RecordingSignalBridgeSink::default();
    let runtime = crate::facade::RuntimeBridgeBuilder::new()
        .with_relational_source(source)
        .with_signal_sink(sink)
        .with_continuity_lineage_source(TestContinuityLineageSource)
        .register_mapping(registration())
        .register_aspect_mapping(field_aspect_registration())
        .build()
        .expect("runtime should build");

    let route = runtime
        .plan_committed_patch_with_mapping_context(
            BridgeRouteRequest::for_commit("commit-a"),
            BridgeMappingContext::default().with_lineage_context(BridgeLineageContext::new(
                BridgeContinuityAuthorityBasis::new(
                    crate::facade::TruthBranchIdentity::new("main"),
                    TruthSnapshotIdentity::new("snapshot-a"),
                ),
            )),
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

    assert_eq!(resolved.continuity_entries().len(), 1);
    assert_eq!(
        resolved.continuity_entries()[0].outcome_class(),
        crate::facade::BridgeContinuityOutcomeClass::ContinuesAsSingleSuccessor
    );
    assert_eq!(artifact.remapped_slices().len(), 1);
    assert_eq!(
        artifact.remapped_slices().slices()[0].entity_identity(),
        "entity:0:4:2"
    );
    assert_eq!(
        artifact.remapped_slices().slices()[0].aspect_label(),
        "profile"
    );
    assert_eq!(
        artifact.remapped_slices().slices()[0].surface_label(),
        "name"
    );
    assert_eq!(artifact.counters().continuity_request_count(), 1);
    assert_eq!(artifact.counters().continuity_prior_slice_count(), 1);
    assert_eq!(artifact.counters().continuity_single_successor_count(), 1);
}

#[test]
fn bridge_resolved_lineage_continuity_lowers_split_successor_artifact() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(field_slice_snapshot("snapshot-a", "alice"));
    let sink = RecordingSignalBridgeSink::default();
    let runtime = crate::facade::RuntimeBridgeBuilder::new()
        .with_relational_source(source)
        .with_signal_sink(sink)
        .with_continuity_lineage_source(TestSplitContinuityLineageSource)
        .register_mapping(registration())
        .register_aspect_mapping(field_aspect_registration())
        .build()
        .expect("runtime should build");

    let route = runtime
        .plan_committed_patch_with_mapping_context(
            BridgeRouteRequest::for_commit("commit-a"),
            BridgeMappingContext::default().with_lineage_context(BridgeLineageContext::new(
                BridgeContinuityAuthorityBasis::new(
                    crate::facade::TruthBranchIdentity::new("main"),
                    TruthSnapshotIdentity::new("snapshot-a"),
                ),
            )),
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

    assert_eq!(
        resolved.continuity_entries()[0].outcome_class(),
        crate::facade::BridgeContinuityOutcomeClass::ContinuesAsSplitSuccessors
    );
    assert_eq!(artifact.remapped_slices().len(), 2);
    assert_eq!(
        artifact
            .remapped_slices()
            .slices()
            .iter()
            .map(|slice| slice.entity_identity())
            .collect::<Vec<_>>(),
        vec!["entity:0:4:2", "entity:0:5:2"]
    );
}

#[test]
fn bridge_resolved_lineage_continuity_lowers_merge_like_successor_artifact() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(field_slice_snapshot("snapshot-a", "alice"));
    let sink = RecordingSignalBridgeSink::default();
    let runtime = crate::facade::RuntimeBridgeBuilder::new()
        .with_relational_source(source)
        .with_signal_sink(sink)
        .with_continuity_lineage_source(TestMergeLikeContinuityLineageSource)
        .register_mapping(registration())
        .register_aspect_mapping(field_aspect_registration())
        .build()
        .expect("runtime should build");

    let route = runtime
        .plan_committed_patch_with_mapping_context(
            BridgeRouteRequest::for_commit("commit-a"),
            BridgeMappingContext::default().with_lineage_context(BridgeLineageContext::new(
                BridgeContinuityAuthorityBasis::new(
                    crate::facade::TruthBranchIdentity::new("main"),
                    TruthSnapshotIdentity::new("snapshot-a"),
                ),
            )),
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

    assert_eq!(
        resolved.continuity_entries()[0].outcome_class(),
        crate::facade::BridgeContinuityOutcomeClass::ContinuesViaTruthLoweredCanonicalMergeSuccessor
    );
    assert_eq!(artifact.remapped_slices().len(), 1);
    assert_eq!(
        artifact.remapped_slices().slices()[0].entity_identity(),
        "entity:0:9:3"
    );
}

#[test]
fn bridge_resolved_lineage_continuity_rejects_ambiguous_successor_sets() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(field_slice_snapshot("snapshot-a", "alice"));
    let sink = RecordingSignalBridgeSink::default();
    let runtime = crate::facade::RuntimeBridgeBuilder::new()
        .with_relational_source(source)
        .with_signal_sink(sink)
        .with_continuity_lineage_source(TestAmbiguousContinuityLineageSource)
        .register_mapping(registration())
        .register_aspect_mapping(field_aspect_registration())
        .build()
        .expect("runtime should build");

    let route = runtime
        .plan_committed_patch_with_mapping_context(
            BridgeRouteRequest::for_commit("commit-a"),
            BridgeMappingContext::default().with_lineage_context(BridgeLineageContext::new(
                BridgeContinuityAuthorityBasis::new(
                    crate::facade::TruthBranchIdentity::new("main"),
                    TruthSnapshotIdentity::new("snapshot-a"),
                ),
            )),
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

    assert_eq!(
        resolved.continuity_entries()[0].outcome_class(),
        crate::facade::BridgeContinuityOutcomeClass::RejectedAmbiguousSuccessor
    );
    assert_eq!(artifact.remapped_slices().len(), 0);
}

#[test]
fn bridge_resolved_lineage_continuity_rejects_no_authoritative_successor() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(field_slice_snapshot("snapshot-a", "alice"));
    let sink = RecordingSignalBridgeSink::default();
    let runtime = crate::facade::RuntimeBridgeBuilder::new()
        .with_relational_source(source)
        .with_signal_sink(sink)
        .with_continuity_lineage_source(TestNoAuthoritativeSuccessorLineageSource)
        .register_mapping(registration())
        .register_aspect_mapping(field_aspect_registration())
        .build()
        .expect("runtime should build");

    let route = runtime
        .plan_committed_patch_with_mapping_context(
            BridgeRouteRequest::for_commit("commit-a"),
            BridgeMappingContext::default().with_lineage_context(BridgeLineageContext::new(
                BridgeContinuityAuthorityBasis::new(
                    crate::facade::TruthBranchIdentity::new("main"),
                    TruthSnapshotIdentity::new("snapshot-a"),
                ),
            )),
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

    assert_eq!(
        resolved.continuity_entries()[0].outcome_class(),
        crate::facade::BridgeContinuityOutcomeClass::RejectedNoAuthoritativeSuccessor
    );
    assert_eq!(artifact.remapped_slices().len(), 0);
}

#[test]
fn bridge_deliver_continuity_returns_delivered_result_and_canonical_record() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(field_slice_snapshot("snapshot-a", "alice"));
    let sink = RecordingSignalBridgeSink::default();
    let runtime = crate::facade::RuntimeBridgeBuilder::new()
        .with_relational_source(source)
        .with_signal_sink(sink)
        .with_continuity_lineage_source(TestContinuityLineageSource)
        .register_mapping(registration())
        .register_aspect_mapping(field_aspect_registration())
        .build()
        .expect("runtime should build");

    let route = runtime
        .plan_committed_patch_with_mapping_context(
            BridgeRouteRequest::for_commit("commit-a"),
            BridgeMappingContext::default().with_lineage_context(BridgeLineageContext::new(
                BridgeContinuityAuthorityBasis::new(
                    crate::facade::TruthBranchIdentity::new("main"),
                    TruthSnapshotIdentity::new("snapshot-a"),
                ),
            )),
        )
        .expect("route should plan");
    let result = runtime
        .deliver_invalidation(route)
        .expect("delivery should succeed");
    let route_record = runtime
        .diagnostics()
        .route_record_for_route_identity(result.result_summary().route_identity().as_str())
        .expect("route record should be retained");

    let delivered = runtime
        .deliver_continuity(&route_record)
        .expect("continuity should deliver as a single bridge-owned result");

    assert_eq!(
        delivered.canonical_record().continuity_artifact_identity(),
        delivered.continuity_identity()
    );
    assert_eq!(
        delivered
            .canonical_record()
            .remapped_subscription_slice_identity(),
        delivered.remapped_subscription_slice_identity()
    );
    assert_eq!(
        delivered.canonical_record().remapped_slices(),
        delivered.remapped_slices()
    );
}

#[test]
fn bridge_continuity_truth_is_invariant_across_diagnostics_tiers() {
    let standard_source = InMemoryRelationalBridgeSource::default();
    standard_source.insert_committed_patch(committed_patch(
        "commit-a",
        "patch-a",
        "snapshot-a",
        "name",
    ));
    standard_source.insert_snapshot(field_slice_snapshot("snapshot-a", "alice"));
    let forensic_source = standard_source.clone();

    let standard_runtime = crate::facade::RuntimeBridgeBuilder::new()
        .with_policy(BridgeRuntimePolicy::development())
        .with_relational_source(standard_source)
        .with_signal_sink(RecordingSignalBridgeSink::default())
        .with_continuity_lineage_source(TestContinuityLineageSource)
        .register_mapping(registration())
        .register_aspect_mapping(field_aspect_registration())
        .build()
        .expect("standard runtime should build");
    let forensic_runtime = crate::facade::RuntimeBridgeBuilder::new()
        .with_policy(BridgeRuntimePolicy::forensic())
        .with_relational_source(forensic_source)
        .with_signal_sink(RecordingSignalBridgeSink::default())
        .with_continuity_lineage_source(TestContinuityLineageSource)
        .register_mapping(registration())
        .register_aspect_mapping(field_aspect_registration())
        .build()
        .expect("forensic runtime should build");

    let standard_route = standard_runtime
        .plan_committed_patch_with_mapping_context(
            BridgeRouteRequest::for_commit("commit-a"),
            BridgeMappingContext::default().with_lineage_context(BridgeLineageContext::new(
                BridgeContinuityAuthorityBasis::new(
                    crate::facade::TruthBranchIdentity::new("main"),
                    TruthSnapshotIdentity::new("snapshot-a"),
                ),
            )),
        )
        .expect("standard route should plan");
    let forensic_route = forensic_runtime
        .plan_committed_patch_with_mapping_context(
            BridgeRouteRequest::for_commit("commit-a"),
            BridgeMappingContext::default().with_lineage_context(BridgeLineageContext::new(
                BridgeContinuityAuthorityBasis::new(
                    crate::facade::TruthBranchIdentity::new("main"),
                    TruthSnapshotIdentity::new("snapshot-a"),
                ),
            )),
        )
        .expect("forensic route should plan");

    let standard_result = standard_runtime
        .deliver_invalidation(standard_route)
        .expect("standard delivery should succeed");
    let forensic_result = forensic_runtime
        .deliver_invalidation(forensic_route)
        .expect("forensic delivery should succeed");

    let standard_route_record = standard_runtime
        .diagnostics()
        .route_record_for_route_identity(standard_result.result_summary().route_identity().as_str())
        .expect("standard route record should be retained");
    let forensic_route_record = forensic_runtime
        .diagnostics()
        .route_record_for_route_identity(forensic_result.result_summary().route_identity().as_str())
        .expect("forensic route record should be retained");

    let standard = standard_runtime
        .deliver_continuity(&standard_route_record)
        .expect("standard continuity should deliver");
    let forensic = forensic_runtime
        .deliver_continuity(&forensic_route_record)
        .expect("forensic continuity should deliver");

    assert_eq!(
        standard.continuity_identity(),
        forensic.continuity_identity()
    );
    assert_eq!(
        standard.canonical_record().continuity_resolution_digest(),
        forensic.canonical_record().continuity_resolution_digest()
    );
    assert_eq!(
        standard.remapped_subscription_slice_identity(),
        forensic.remapped_subscription_slice_identity()
    );
    assert_eq!(standard.remapped_slices(), forensic.remapped_slices());
}
use super::*;
