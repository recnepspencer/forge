#[test]
fn bridge_bulk_packet_set_tracks_continuity_remap_packets() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-a", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let runtime = build_runtime(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );
    let lineage_context = BridgeLineageContext::new(BridgeContinuityAuthorityBasis::new(
        crate::facade::TruthBranchIdentity::new("main"),
        TruthSnapshotIdentity::new("snapshot-a"),
    ));

    let planned = runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a"))
                .with_mapping_context(
                    BridgeMappingContext::default().with_lineage_context(lineage_context.clone()),
                ),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-b"))
                .with_mapping_context(
                    BridgeMappingContext::default().with_lineage_context(lineage_context),
                ),
        ]))
        .expect("continuity-bearing bulk workload should plan");

    assert_eq!(planned.packet_set().continuity_packets().len(), 2);
    assert_eq!(
        planned
            .packet_set()
            .continuity_packets()
            .iter()
            .map(|packet| packet.snapshot_identity())
            .collect::<Vec<_>>(),
        vec!["snapshot-a", "snapshot-a"]
    );
    assert_eq!(
        planned
            .packet_set()
            .continuity_packets()
            .iter()
            .map(|packet| packet.prior_slice_count())
            .sum::<usize>(),
        2
    );
}

#[test]
fn bridge_bulk_reduction_artifact_carries_continuity_remaps() {
    let left_source = InMemoryRelationalBridgeSource::default();
    left_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    left_source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let left_runtime = build_runtime(
        left_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let right_source = InMemoryRelationalBridgeSource::default();
    right_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    right_source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let right_runtime = build_runtime(
        right_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );
    let lineage_context = BridgeLineageContext::new(BridgeContinuityAuthorityBasis::new(
        crate::facade::TruthBranchIdentity::new("main"),
        TruthSnapshotIdentity::new("snapshot-a"),
    ));
    let request = BridgeBulkWorkloadRequest::new(vec![BridgeBulkWorkloadSegment::new(
        BridgeRouteRequest::for_commit("commit-a"),
    )
    .with_mapping_context(BridgeMappingContext::default().with_lineage_context(lineage_context))]);

    let left = left_runtime
        .plan_bulk_workload(request.clone())
        .expect("left continuity-bearing workload should plan");
    let right = right_runtime
        .plan_bulk_workload(request)
        .expect("right continuity-bearing workload should plan");

    assert_eq!(
        left.execution_plan()
            .reduced_artifact()
            .reduced_continuity_remaps()
            .len(),
        1
    );
    assert_eq!(
        left.execution_plan()
            .reduced_artifact()
            .reduced_continuity_remaps(),
        right
            .execution_plan()
            .reduced_artifact()
            .reduced_continuity_remaps()
    );
    assert_eq!(
        left.execution_plan()
            .reduced_artifact()
            .reduced_continuity_remaps()[0]
            .snapshot_identity(),
        "snapshot-a"
    );
    assert_eq!(
        left.execution_plan()
            .reduced_artifact()
            .reduced_continuity_remaps()[0]
            .prior_slice_count(),
        1
    );
}

use super::*;
