#[test]
fn bridge_bulk_reduction_collapses_duplicate_publications_deterministically() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch(
        crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
        crate::truth_identity_fixtures::truth_patch_fixture("patch-a"),
        crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
        worth_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid harness field key"),
    ));
    source.insert_snapshot(snapshot(
        crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
        "alice",
    ));
    let runtime = build_runtime(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let planned = runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit(
                crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
            )),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit(
                crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
            )),
        ]))
        .expect("duplicate bulk workload should plan");
    let reduced_artifact = planned.execution_plan().reduced_artifact();

    assert_eq!(reduced_artifact.reduction_input_count(), 2);
    assert_eq!(reduced_artifact.reduction_output_count(), 2);
    assert_eq!(reduced_artifact.counters().bulk_reduction_input_count(), 2);
    assert_eq!(reduced_artifact.reduced_truth_views().len(), 1);
    assert_eq!(reduced_artifact.reduced_publications().len(), 1);
    assert_eq!(
        reduced_artifact.reduced_publications()[0]
            .reduced_route_identities()
            .len(),
        2
    );
    assert_eq!(
        reduced_artifact.reduced_publications()[0].reduced_route_identities(),
        planned
            .packet_set()
            .routing_packets()
            .iter()
            .map(|packet| packet.route_identity().clone())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        reduced_artifact.reduced_publications()[0].invalidation_target_count(),
        2
    );
}

#[test]
fn bridge_bulk_reduction_artifact_is_stable_across_input_order() {
    let left_source = InMemoryRelationalBridgeSource::default();
    left_source.insert_committed_patch(committed_patch(
        crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
        crate::truth_identity_fixtures::truth_patch_fixture("patch-a"),
        crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
        worth_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid harness field key"),
    ));
    left_source.insert_committed_patch(committed_patch(
        crate::truth_identity_fixtures::truth_commit_fixture("commit-b"),
        crate::truth_identity_fixtures::truth_patch_fixture("patch-b"),
        crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-b"),
        worth_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid harness field key"),
    ));
    left_source.insert_snapshot(snapshot(
        crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
        "alice",
    ));
    left_source.insert_snapshot(snapshot(
        crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-b"),
        "bob",
    ));
    let left_runtime = build_runtime(
        left_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let right_source = InMemoryRelationalBridgeSource::default();
    right_source.insert_committed_patch(committed_patch(
        crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
        crate::truth_identity_fixtures::truth_patch_fixture("patch-a"),
        crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
        worth_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid harness field key"),
    ));
    right_source.insert_committed_patch(committed_patch(
        crate::truth_identity_fixtures::truth_commit_fixture("commit-b"),
        crate::truth_identity_fixtures::truth_patch_fixture("patch-b"),
        crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-b"),
        worth_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid harness field key"),
    ));
    right_source.insert_snapshot(snapshot(
        crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
        "alice",
    ));
    right_source.insert_snapshot(snapshot(
        crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-b"),
        "bob",
    ));
    let right_runtime = build_runtime(
        right_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let left = left_runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit(
                crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
            )),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit(
                crate::truth_identity_fixtures::truth_commit_fixture("commit-b"),
            )),
        ]))
        .expect("left workload should plan");
    let right = right_runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit(
                crate::truth_identity_fixtures::truth_commit_fixture("commit-b"),
            )),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit(
                crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
            )),
        ]))
        .expect("right workload should plan");

    assert_eq!(
        left.execution_plan().reduced_artifact().digest(),
        right.execution_plan().reduced_artifact().digest()
    );
    assert_eq!(
        left.execution_plan()
            .reduced_artifact()
            .reduced_publications()[0]
            .publication_identity(),
        right
            .execution_plan()
            .reduced_artifact()
            .reduced_publications()[0]
            .publication_identity()
    );
    assert_eq!(left.packet_set().digest(), right.packet_set().digest());
}

#[test]
fn bridge_bulk_packet_set_is_stable_across_input_order() {
    let left_source = InMemoryRelationalBridgeSource::default();
    left_source.insert_committed_patch(committed_patch(
        crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
        crate::truth_identity_fixtures::truth_patch_fixture("patch-a"),
        crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
        worth_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid harness field key"),
    ));
    left_source.insert_committed_patch(committed_patch(
        crate::truth_identity_fixtures::truth_commit_fixture("commit-b"),
        crate::truth_identity_fixtures::truth_patch_fixture("patch-b"),
        crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-b"),
        worth_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid harness field key"),
    ));
    left_source.insert_snapshot(snapshot(
        crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
        "alice",
    ));
    left_source.insert_snapshot(snapshot(
        crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-b"),
        "bob",
    ));
    let left_runtime = build_runtime(
        left_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let right_source = InMemoryRelationalBridgeSource::default();
    right_source.insert_committed_patch(committed_patch(
        crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
        crate::truth_identity_fixtures::truth_patch_fixture("patch-a"),
        crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
        worth_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid harness field key"),
    ));
    right_source.insert_committed_patch(committed_patch(
        crate::truth_identity_fixtures::truth_commit_fixture("commit-b"),
        crate::truth_identity_fixtures::truth_patch_fixture("patch-b"),
        crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-b"),
        worth_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid harness field key"),
    ));
    right_source.insert_snapshot(snapshot(
        crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
        "alice",
    ));
    right_source.insert_snapshot(snapshot(
        crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-b"),
        "bob",
    ));
    let right_runtime = build_runtime(
        right_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let left = left_runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit(
                crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
            )),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit(
                crate::truth_identity_fixtures::truth_commit_fixture("commit-b"),
            )),
        ]))
        .expect("left workload should plan");
    let right = right_runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit(
                crate::truth_identity_fixtures::truth_commit_fixture("commit-b"),
            )),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit(
                crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
            )),
        ]))
        .expect("right workload should plan");

    assert_eq!(left.packet_set().digest(), right.packet_set().digest());
    assert_eq!(
        left.packet_set()
            .routing_packets()
            .iter()
            .map(|packet| packet.packet_identity())
            .collect::<Vec<_>>(),
        right
            .packet_set()
            .routing_packets()
            .iter()
            .map(|packet| packet.packet_identity())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        left.packet_set()
            .reduction_packets()
            .iter()
            .map(|packet| packet.reduced_target_identity())
            .collect::<Vec<_>>(),
        right
            .packet_set()
            .reduction_packets()
            .iter()
            .map(|packet| packet.reduced_target_identity())
            .collect::<Vec<_>>()
    );
}

#[test]
fn bridge_bulk_packet_reduction_collapses_duplicate_slice_targets() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch(
        crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
        crate::truth_identity_fixtures::truth_patch_fixture("patch-a"),
        crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
        worth_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid harness field key"),
    ));
    source.insert_snapshot(snapshot(
        crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
        "alice",
    ));
    let runtime = build_runtime(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let planned = runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit(
                crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
            )),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit(
                crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
            )),
        ]))
        .expect("duplicate workload should plan");

    assert_eq!(planned.packet_set().routing_packets().len(), 2);
    assert_eq!(planned.packet_set().truth_view_packets().len(), 1);
    assert_eq!(planned.packet_set().reduction_packets().len(), 1);
    assert_eq!(
        planned.packet_set().reduction_packets()[0].reduction_family(),
        crate::facade::BridgeInvalidationReductionFamily::Publication
    );
    assert_eq!(
        planned.packet_set().reduction_packets()[0].reduced_subscription_slice_identity(),
        planned.packet_set().routing_packets()[0].subscription_slice_identity()
    );
    assert_eq!(planned.packet_set().counters().bulk_packet_count(), 4);
    assert_eq!(
        planned.execution_plan().parallel_admission().reason(),
        BridgeParallelAdmissionReason::SharedTruthViewMaterializationTarget
    );
    assert_eq!(
        planned
            .execution_plan()
            .locality_footprint()
            .publication_scope_count(),
        1
    );
}

#[test]
fn bridge_bulk_packet_set_emits_widening_packets_for_widening_admitted_slices() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch(
        crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
        crate::truth_identity_fixtures::truth_patch_fixture("patch-a"),
        crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
        worth_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid harness field key"),
    ));
    source.insert_snapshot(snapshot(
        crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
        "alice",
    ));
    let runtime = build_runtime(
        source,
        RecordingSignalBridgeSink::default(),
        vec![surface_widening_registration()],
    );

    let planned = runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit(
                crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
            )),
        ]))
        .expect("widening workload should plan");

    assert_eq!(planned.packet_set().routing_packets().len(), 1);
    assert_eq!(planned.packet_set().widening_packets().len(), 1);
    assert_eq!(
        planned.packet_set().widening_packets()[0].originating_route_identity(),
        planned.packet_set().routing_packets()[0].route_identity()
    );
    assert_eq!(
        planned.packet_set().widening_packets()[0].widening_class(),
        crate::facade::BridgeMappingWideningClass::Surface
    );
    assert!(planned.packet_set().widening_packets()[0]
        .bounded_scope_identity()
        .starts_with("truth-delta-surface:sha256:"));
    assert!(!planned.packet_set().widening_packets()[0]
        .bounded_scope_identity()
        .contains("committed-patch-target"));
    assert_eq!(planned.packet_set().truth_view_packets().len(), 1);
    assert_eq!(planned.packet_set().counters().bulk_packet_count(), 4);
    assert_eq!(
        planned
            .execution_plan()
            .reduced_artifact()
            .reduced_widenings()
            .len(),
        1
    );
    assert_eq!(
        planned.packet_set().widening_packets()[0].bounded_truth_delta_surface_identity(),
        planned
            .execution_plan()
            .reduced_artifact()
            .reduced_widenings()[0]
            .bounded_truth_delta_surface_identity()
    );
    assert_eq!(
        planned
            .execution_plan()
            .reduced_artifact()
            .reduced_widenings()[0]
            .reduced_route_identities(),
        &[planned.packet_set().routing_packets()[0]
            .route_identity()
            .clone()]
    );
    assert_eq!(
        planned
            .execution_plan()
            .reduced_artifact()
            .reduction_output_count(),
        3
    );
}

use super::*;
