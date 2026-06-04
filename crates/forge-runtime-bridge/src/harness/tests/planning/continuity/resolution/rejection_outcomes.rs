use super::*;

#[test]
fn bridge_resolved_lineage_continuity_rejects_ambiguous_successor_sets() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch(
        crate::facade::TruthCommitIdentity::new("commit-a"),
        crate::facade::TruthPatchIdentity::new("patch-a"),
        TruthSnapshotIdentity::new("snapshot-a"),
        forge_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid harness field key"),
    ));
    source.insert_snapshot(field_slice_snapshot(
        TruthSnapshotIdentity::new("snapshot-a"),
        "alice",
    ));
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
            BridgeRouteRequest::for_commit(crate::facade::TruthCommitIdentity::new("commit-a")),
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
        .route_record_for_route_identity(result.result_summary().route_identity())
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
    source.insert_committed_patch(committed_patch(
        crate::facade::TruthCommitIdentity::new("commit-a"),
        crate::facade::TruthPatchIdentity::new("patch-a"),
        TruthSnapshotIdentity::new("snapshot-a"),
        forge_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid harness field key"),
    ));
    source.insert_snapshot(field_slice_snapshot(
        TruthSnapshotIdentity::new("snapshot-a"),
        "alice",
    ));
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
            BridgeRouteRequest::for_commit(crate::facade::TruthCommitIdentity::new("commit-a")),
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
        .route_record_for_route_identity(result.result_summary().route_identity())
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
