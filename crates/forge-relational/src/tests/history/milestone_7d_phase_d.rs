use crate::facade::diagnostics::DiagnosticCode;
use crate::facade::history::BranchId;
use crate::facade::merge::{
    DeletionExecutionClass, LoweredMergeBlockedReason, LoweredRecordDenialKind,
    MergeExecutionError, MergeExecutionRequest, MergeIntent, MergeResolutionClass,
    RelationConflictPropagation, TopologyExecutionClass, TopologyRegionConflictReason,
};
use crate::facade::transactions::RecordRef;
use crate::tests::support::{
    capture_aspect_truth_bundle, certification_digest, checkpoint_and_recover_with,
    create_branch_from_main, create_entity, create_relation_in_partition_on_branch,
    delete_entity, delete_entity_on_branch, delete_relation_on_branch,
    persisted_runtime_with_test_schema, unique_test_store_path, update_entity,
};

#[test]
fn deleted_on_both_sides_merge_commit_has_replay_and_recovery_parity() {
    let mut runtime = persisted_runtime_with_test_schema();
    let entity = create_entity(&mut runtime, "shared");
    create_branch_from_main(&mut runtime, "feature");
    delete_entity(&mut runtime, entity);
    delete_entity_on_branch(&mut runtime, entity, BranchId("feature".to_string()));

    let prepared = runtime
        .prepare_merge_execution(MergeExecutionRequest {
            target_branch: BranchId("main".to_string()),
            source_branch: BranchId("feature".to_string()),
            merge_intent: MergeIntent::ReconcileIntoTarget,
        })
        .expect("prepared deleted-on-both-sides merge");
    let merge = runtime
        .execute_prepared_merge(prepared)
        .expect("executed deleted-on-both-sides merge");

    assert_eq!(merge.structural_summary.executed_record_count, 1);
    assert_eq!(merge.structural_summary.converged_deleted_on_both_sides_count, 1);
    assert_eq!(merge.structural_summary.emitted_mutation_intent_count, 0);

    let live_envelope = runtime
        .replay_access()
        .canonical_commit_envelope(merge.commit.commit.commit_id)
        .cloned()
        .expect("live merge envelope");
    let live_truth = capture_aspect_truth_bundle(&mut runtime, &[entity], &[], &[]);

    let replay = runtime
        .replay_authority()
        .replay_commit(crate::facade::replay::RelationalReplayRequest {
            commit_id: merge.commit.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: crate::facade::replay::ReplayExecutionMode::SerialDeterministic,
            verification_mode:
                crate::facade::replay::ReplayVerificationMode::AuditRecoveryVerification,
        });
    assert!(replay.failure.is_none(), "replay certification failure: {replay:?}");

    let (_recovery, mut recovered) =
        checkpoint_and_recover_with(&mut runtime, persisted_runtime_with_test_schema);
    let recovered_envelope = recovered
        .replay_access()
        .canonical_commit_envelope(merge.commit.commit.commit_id)
        .cloned()
        .expect("recovered merge envelope");
    let recovered_truth = capture_aspect_truth_bundle(&mut recovered, &[entity], &[], &[]);

    assert_eq!(live_envelope, recovered_envelope);
    assert_eq!(live_truth.visible_truth, recovered_truth.visible_truth);
    assert_eq!(
        live_truth.entity_history_digests,
        recovered_truth.entity_history_digests
    );
    assert_eq!(
        certification_digest(&live_envelope.diagnostics_summary),
        certification_digest(&recovered_envelope.diagnostics_summary)
    );

    let summary_entry = live_envelope
        .diagnostics_summary
        .entries
        .iter()
        .find(|entry| entry.code == DiagnosticCode::MergeExecutionPublished)
        .expect("merge execution summary entry");
    assert_eq!(
        summary_entry.fields["converged_deleted_on_both_sides_count"],
        serde_json::json!(1)
    );
    assert_eq!(
        summary_entry.fields["execution_digest"],
        serde_json::json!(merge.execution_summary.execution_digest)
    );
    assert_eq!(
        summary_entry.fields["diagnostics_digest"],
        serde_json::json!(merge.execution_summary.diagnostics_digest)
    );
}

#[test]
fn deleted_on_both_sides_prepared_merge_rejects_target_head_drift() {
    let mut runtime = persisted_runtime_with_test_schema();
    let entity = create_entity(&mut runtime, "shared");
    create_branch_from_main(&mut runtime, "feature");
    delete_entity(&mut runtime, entity);
    delete_entity_on_branch(&mut runtime, entity, BranchId("feature".to_string()));

    let prepared = runtime
        .prepare_merge_execution(MergeExecutionRequest {
            target_branch: BranchId("main".to_string()),
            source_branch: BranchId("feature".to_string()),
            merge_intent: MergeIntent::ReconcileIntoTarget,
        })
        .expect("prepared merge");
    create_entity(&mut runtime, "main-advance");

    match runtime.execute_prepared_merge(prepared) {
        Err(MergeExecutionError::StaleBranchHead { branch, .. }) => {
            assert_eq!(branch, BranchId("main".to_string()));
        }
        other => panic!("expected target stale-head rejection, got {other:?}"),
    }

    let diagnostics = runtime.publication_access().diagnostics();
    let failure_artifact = diagnostics
        .artifacts()
        .iter()
        .rev()
        .find(|artifact| {
            artifact.entries.iter().any(|entry| {
                entry.code == DiagnosticCode::DeterministicMergeViolation
                    || entry.code == DiagnosticCode::MissingMergeBase
            })
        })
        .expect("failure artifact");
    assert!(failure_artifact.entries.iter().any(|entry| {
        entry.code == DiagnosticCode::DeterministicMergeViolation
            && entry.fields["target_branch"] == serde_json::json!("main")
            && entry.fields["source_branch"] == serde_json::json!("feature")
    }));
}

#[test]
fn deleted_on_both_sides_prepared_merge_rejects_schema_semantic_drift() {
    let mut runtime = persisted_runtime_with_test_schema();
    let entity = create_entity(&mut runtime, "shared");
    create_branch_from_main(&mut runtime, "feature");
    delete_entity(&mut runtime, entity);
    delete_entity_on_branch(&mut runtime, entity, BranchId("feature".to_string()));

    let prepared = runtime
        .prepare_merge_execution(MergeExecutionRequest {
            target_branch: BranchId("main".to_string()),
            source_branch: BranchId("feature".to_string()),
            merge_intent: MergeIntent::ReconcileIntoTarget,
        })
        .expect("prepared merge");
    runtime.config.schema.registry = drifted_schema_registry();

    match runtime.execute_prepared_merge(prepared) {
        Err(MergeExecutionError::SchemaSemanticDrift { .. }) => {}
        other => panic!("expected schema semantic drift rejection, got {other:?}"),
    }
}

#[test]
fn non_executable_deletion_denial_is_stable_across_recovery() {
    let mut runtime = persisted_runtime_with_test_schema();
    let entity = create_entity(&mut runtime, "shared");
    create_branch_from_main(&mut runtime, "feature");
    update_entity(&mut runtime, entity, "main-modified");
    delete_entity_on_branch(&mut runtime, entity, BranchId("feature".to_string()));

    let live_artifact = runtime
        .merge_access()
        .inspect_planning_scope(
            MergeExecutionRequest {
                target_branch: BranchId("main".to_string()),
                source_branch: BranchId("feature".to_string()),
                merge_intent: MergeIntent::ReconcileIntoTarget,
            }
            .into(),
        )
        .expect("live planning artifact");
    let live_record = live_artifact
        .lowered_plan
        .records
        .iter()
        .find(|record| record.record == RecordRef::Entity(entity))
        .expect("live lowered record");
    let live_index = live_artifact
        .lowered_plan
        .records
        .iter()
        .position(|record| record.record == RecordRef::Entity(entity))
        .expect("live lowered index");

    let (_recovery, recovered) =
        checkpoint_and_recover_with(&mut runtime, persisted_runtime_with_test_schema);
    let recovered_artifact = recovered
        .merge_access()
        .inspect_planning_scope(
            MergeExecutionRequest {
                target_branch: BranchId("main".to_string()),
                source_branch: BranchId("feature".to_string()),
                merge_intent: MergeIntent::ReconcileIntoTarget,
            }
            .into(),
        )
        .expect("recovered planning artifact");
    let recovered_record = recovered_artifact
        .lowered_plan
        .records
        .iter()
        .find(|record| record.record == RecordRef::Entity(entity))
        .expect("recovered lowered record");
    let recovered_index = recovered_artifact
        .lowered_plan
        .records
        .iter()
        .position(|record| record.record == RecordRef::Entity(entity))
        .expect("recovered lowered index");

    assert_eq!(
        live_record.resolution_class,
        MergeResolutionClass::Deletion(DeletionExecutionClass::DeletedVsModified)
    );
    assert_eq!(
        live_record.blocked_reason,
        Some(LoweredMergeBlockedReason::DeletedVsModified)
    );
    assert_eq!(live_record.executable_class, None);
    assert_eq!(live_record, recovered_record);
    assert_eq!(
        live_artifact.digest_basis.lowered_plan.denial_bundle_kinds[live_index],
        Some(LoweredRecordDenialKind::BlockedDeletedVsModified)
    );
    assert_eq!(
        recovered_artifact.digest_basis.lowered_plan.denial_bundle_kinds[recovered_index],
        Some(LoweredRecordDenialKind::BlockedDeletedVsModified)
    );
    assert_eq!(
        certification_digest(&live_artifact.digest_basis.lowered_plan),
        certification_digest(&recovered_artifact.digest_basis.lowered_plan)
    );
}

#[test]
fn topology_endpoint_divergence_denial_is_stable_across_recovery() {
    let store_path = unique_test_store_path("forge-relational-7d-topology");
    let mut runtime = persisted_runtime_with_topology_identity_registry(store_path.clone());
    let source = create_entity(&mut runtime, "source");
    let target_a = create_entity(&mut runtime, "target-a");
    let target_b = create_entity(&mut runtime, "target-b");
    let relation = crate::tests::support::create_relation(
        &mut runtime,
        source,
        target_a,
        "shared-edge",
    );
    create_branch_from_main(&mut runtime, "feature");
    delete_relation_on_branch(&mut runtime, relation, BranchId("feature".to_string()));
    create_relation_in_partition_on_branch(
        &mut runtime,
        source,
        target_b,
        "shared-edge",
        "shared-edge",
        crate::facade::identity::PartitionId::main(),
        BranchId("feature".to_string()),
    );

    let live_artifact = runtime
        .merge_access()
        .inspect_planning_scope(
            MergeExecutionRequest {
                target_branch: BranchId("main".to_string()),
                source_branch: BranchId("feature".to_string()),
                merge_intent: MergeIntent::ReconcileIntoTarget,
            }
            .into(),
        )
        .expect("live planning artifact");
    let live_record = live_artifact
        .lowered_plan
        .records
        .iter()
        .find(|record| {
            matches!(
                record.resolution_class,
                MergeResolutionClass::Topology(
                    TopologyExecutionClass::RelationEndpointRewiredEscalated
                )
            )
        })
        .expect("live topology lowered record");
    let live_classification = live_artifact
        .conflict_classification
        .classifications
        .iter()
        .find(|classification| classification.record == live_record.record)
        .expect("live topology classification");
    let live_index = live_artifact
        .lowered_plan
        .records
        .iter()
        .position(|record| {
            matches!(
                record.resolution_class,
                MergeResolutionClass::Topology(
                    TopologyExecutionClass::RelationEndpointRewiredEscalated
                )
            )
        })
        .expect("live topology index");

    let (_recovery, recovered) = checkpoint_and_recover_with(&mut runtime, move || {
        persisted_runtime_with_topology_identity_registry(store_path.clone())
    });
    let recovered_artifact = recovered
        .merge_access()
        .inspect_planning_scope(
            MergeExecutionRequest {
                target_branch: BranchId("main".to_string()),
                source_branch: BranchId("feature".to_string()),
                merge_intent: MergeIntent::ReconcileIntoTarget,
            }
            .into(),
        )
        .expect("recovered planning artifact");
    let recovered_record = recovered_artifact
        .lowered_plan
        .records
        .iter()
        .find(|record| {
            matches!(
                record.resolution_class,
                MergeResolutionClass::Topology(
                    TopologyExecutionClass::RelationEndpointRewiredEscalated
                )
            )
        })
        .expect("recovered topology lowered record");
    let recovered_index = recovered_artifact
        .lowered_plan
        .records
        .iter()
        .position(|record| {
            matches!(
                record.resolution_class,
                MergeResolutionClass::Topology(
                    TopologyExecutionClass::RelationEndpointRewiredEscalated
                )
            )
        })
        .expect("recovered topology index");

    assert_eq!(
        live_record.blocked_reason,
        Some(LoweredMergeBlockedReason::RelationEndpointRewiredEscalated)
    );
    assert_eq!(
        live_classification
            .relation_evidence
            .as_ref()
            .expect("relation evidence")
            .propagation,
        RelationConflictPropagation::RelationLocalRewireCandidate
    );
    assert_eq!(
        live_classification
            .relation_evidence
            .as_ref()
            .expect("relation evidence")
            .topology_neighborhood_records
            .len(),
        2
    );
    assert_eq!(
        live_classification
            .relation_evidence
            .as_ref()
            .expect("relation evidence")
            .topology_neighborhood_rewired_records
            .len(),
        1
    );
    assert_eq!(
        live_classification
            .relation_evidence
            .as_ref()
            .expect("relation evidence")
            .topology_region_conflict_reason,
        None
    );
    assert_eq!(live_record.executable_class, None);
    assert_eq!(live_record, recovered_record);
    assert_eq!(
        live_artifact.digest_basis.lowered_plan.denial_bundle_kinds[live_index],
        Some(LoweredRecordDenialKind::BlockedRelationEndpointRewiredEscalated)
    );
    assert_eq!(
        recovered_artifact.digest_basis.lowered_plan.denial_bundle_kinds[recovered_index],
        Some(LoweredRecordDenialKind::BlockedRelationEndpointRewiredEscalated)
    );
    assert_eq!(
        certification_digest(&live_artifact.digest_basis.lowered_plan),
        certification_digest(&recovered_artifact.digest_basis.lowered_plan)
    );
}

#[test]
fn topology_region_conflict_detection_reports_bounded_neighborhood_counters() {
    let mut runtime = persisted_runtime_with_topology_identity_registry(unique_test_store_path(
        "forge-relational-7d-topology-region",
    ));
    let source = create_entity(&mut runtime, "source");
    let target_a = create_entity(&mut runtime, "target-a");
    let target_b = create_entity(&mut runtime, "target-b");
    let target_c = create_entity(&mut runtime, "target-c");
    let target_d = create_entity(&mut runtime, "target-d");
    let relation_a = crate::tests::support::create_relation(&mut runtime, source, target_a, "edge-a");
    let relation_b = crate::tests::support::create_relation(&mut runtime, source, target_b, "edge-b");
    create_branch_from_main(&mut runtime, "feature");
    delete_relation_on_branch(&mut runtime, relation_a, BranchId("feature".to_string()));
    delete_relation_on_branch(&mut runtime, relation_b, BranchId("feature".to_string()));
    create_relation_in_partition_on_branch(
        &mut runtime,
        source,
        target_c,
        "edge-a",
        "edge-a",
        crate::facade::identity::PartitionId::main(),
        BranchId("feature".to_string()),
    );
    create_relation_in_partition_on_branch(
        &mut runtime,
        source,
        target_d,
        "edge-b",
        "edge-b",
        crate::facade::identity::PartitionId::main(),
        BranchId("feature".to_string()),
    );

    runtime.performance_access().reset_counters();
    let artifact = runtime
        .merge_access()
        .inspect_planning_scope(
            MergeExecutionRequest {
                target_branch: BranchId("main".to_string()),
                source_branch: BranchId("feature".to_string()),
                merge_intent: MergeIntent::ReconcileIntoTarget,
            }
            .into(),
        )
        .expect("topology-region planning artifact");

    let topology_records = artifact
        .lowered_plan
        .records
        .iter()
        .filter(|record| {
            matches!(
                record.resolution_class,
                MergeResolutionClass::Topology(TopologyExecutionClass::TopologyRegionConflict)
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(topology_records.len(), 2);
    for record in &topology_records {
        assert_eq!(
            record.blocked_reason,
            Some(LoweredMergeBlockedReason::TopologyRegionConflict)
        );
    }

    let topology_record_refs = topology_records
        .iter()
        .map(|record| record.record.clone())
        .collect::<std::collections::BTreeSet<_>>();
    let topology_classifications = artifact
        .conflict_classification
        .classifications
        .iter()
        .filter(|classification| topology_record_refs.contains(&classification.record))
        .collect::<Vec<_>>();
    assert_eq!(topology_classifications.len(), 2);
    for classification in &topology_classifications {
        let evidence = classification
            .relation_evidence
            .as_ref()
            .expect("topology relation evidence");
        assert_eq!(
            evidence.propagation,
            RelationConflictPropagation::EscalatesToTopologyRegionConflict
        );
        assert_eq!(
            evidence.topology_region_conflict_reason,
            Some(TopologyRegionConflictReason::ConnectedRewireNeighborhood)
        );
        let neighborhood = evidence
            .topology_neighborhood_records
            .iter()
            .cloned()
            .collect::<std::collections::BTreeSet<_>>();
        let rewired = evidence
            .topology_neighborhood_rewired_records
            .iter()
            .cloned()
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(neighborhood.len(), 4);
        assert!(topology_record_refs.is_subset(&neighborhood));
        assert_eq!(rewired, topology_record_refs);
    }

    let counters = runtime.performance_access().counters();
    assert_eq!(counters.merge_topology_relation_candidates_scoped, 4);
    assert!(counters.merge_topology_endpoint_incidences_scoped >= 8);
    assert_eq!(counters.merge_topology_region_conflicts_detected, 1);
    assert_eq!(counters.merge_topology_region_records_escalated, 2);
}

#[test]
fn topology_region_conflict_denial_is_stable_across_recovery() {
    let store_path = unique_test_store_path("forge-relational-7d-topology-region-recovery");
    let mut runtime = persisted_runtime_with_topology_identity_registry(store_path.clone());
    let source = create_entity(&mut runtime, "source");
    let target_a = create_entity(&mut runtime, "target-a");
    let target_b = create_entity(&mut runtime, "target-b");
    let target_c = create_entity(&mut runtime, "target-c");
    let target_d = create_entity(&mut runtime, "target-d");
    let relation_a = crate::tests::support::create_relation(&mut runtime, source, target_a, "edge-a");
    let relation_b = crate::tests::support::create_relation(&mut runtime, source, target_b, "edge-b");
    create_branch_from_main(&mut runtime, "feature");
    delete_relation_on_branch(&mut runtime, relation_a, BranchId("feature".to_string()));
    delete_relation_on_branch(&mut runtime, relation_b, BranchId("feature".to_string()));
    create_relation_in_partition_on_branch(
        &mut runtime,
        source,
        target_c,
        "edge-a",
        "edge-a",
        crate::facade::identity::PartitionId::main(),
        BranchId("feature".to_string()),
    );
    create_relation_in_partition_on_branch(
        &mut runtime,
        source,
        target_d,
        "edge-b",
        "edge-b",
        crate::facade::identity::PartitionId::main(),
        BranchId("feature".to_string()),
    );

    let live_artifact = runtime
        .merge_access()
        .inspect_planning_scope(
            MergeExecutionRequest {
                target_branch: BranchId("main".to_string()),
                source_branch: BranchId("feature".to_string()),
                merge_intent: MergeIntent::ReconcileIntoTarget,
            }
            .into(),
        )
        .expect("live topology-region planning artifact");
    let live_topology_records = live_artifact
        .lowered_plan
        .records
        .iter()
        .filter(|record| {
            matches!(
                record.resolution_class,
                MergeResolutionClass::Topology(TopologyExecutionClass::TopologyRegionConflict)
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(live_topology_records.len(), 2);
    let live_record_refs = live_topology_records
        .iter()
        .map(|record| record.record.clone())
        .collect::<std::collections::BTreeSet<_>>();
    let live_classifications = live_artifact
        .conflict_classification
        .classifications
        .iter()
        .filter(|classification| live_record_refs.contains(&classification.record))
        .collect::<Vec<_>>();
    assert_eq!(live_classifications.len(), 2);
    for classification in &live_classifications {
        let evidence = classification
            .relation_evidence
            .as_ref()
            .expect("live topology-region evidence");
        assert_eq!(
            evidence.propagation,
            RelationConflictPropagation::EscalatesToTopologyRegionConflict
        );
        assert_eq!(
            evidence.topology_region_conflict_reason,
            Some(TopologyRegionConflictReason::ConnectedRewireNeighborhood)
        );
        assert_eq!(evidence.topology_neighborhood_records.len(), 4);
        assert_eq!(evidence.topology_neighborhood_rewired_records.len(), 2);
    }

    let (_recovery, recovered) = checkpoint_and_recover_with(&mut runtime, move || {
        persisted_runtime_with_topology_identity_registry(store_path.clone())
    });
    let recovered_artifact = recovered
        .merge_access()
        .inspect_planning_scope(
            MergeExecutionRequest {
                target_branch: BranchId("main".to_string()),
                source_branch: BranchId("feature".to_string()),
                merge_intent: MergeIntent::ReconcileIntoTarget,
            }
            .into(),
        )
        .expect("recovered topology-region planning artifact");

    assert_eq!(
        certification_digest(&live_artifact.digest_basis.conflict),
        certification_digest(&recovered_artifact.digest_basis.conflict)
    );
    assert_eq!(
        certification_digest(&live_artifact.digest_basis.lowered_plan),
        certification_digest(&recovered_artifact.digest_basis.lowered_plan)
    );
    assert_eq!(
        live_artifact.conflict_classification.classifications,
        recovered_artifact.conflict_classification.classifications
    );
    assert_eq!(
        live_artifact.lowered_plan.records,
        recovered_artifact.lowered_plan.records
    );
}

#[test]
fn disjoint_rewire_neighborhoods_do_not_escalate_to_topology_region_conflict() {
    let mut runtime = persisted_runtime_with_topology_identity_registry(unique_test_store_path(
        "forge-relational-7d-topology-disjoint-rewires",
    ));
    let source_left = create_entity(&mut runtime, "source-left");
    let target_left = create_entity(&mut runtime, "target-left");
    let target_left_rewired = create_entity(&mut runtime, "target-left-rewired");
    let source_right = create_entity(&mut runtime, "source-right");
    let target_right = create_entity(&mut runtime, "target-right");
    let target_right_rewired = create_entity(&mut runtime, "target-right-rewired");
    let relation_left =
        crate::tests::support::create_relation(&mut runtime, source_left, target_left, "edge-left");
    let relation_right =
        crate::tests::support::create_relation(&mut runtime, source_right, target_right, "edge-right");
    create_branch_from_main(&mut runtime, "feature");
    delete_relation_on_branch(&mut runtime, relation_left, BranchId("feature".to_string()));
    delete_relation_on_branch(&mut runtime, relation_right, BranchId("feature".to_string()));
    create_relation_in_partition_on_branch(
        &mut runtime,
        source_left,
        target_left_rewired,
        "edge-left",
        "edge-left",
        crate::facade::identity::PartitionId::main(),
        BranchId("feature".to_string()),
    );
    create_relation_in_partition_on_branch(
        &mut runtime,
        source_right,
        target_right_rewired,
        "edge-right",
        "edge-right",
        crate::facade::identity::PartitionId::main(),
        BranchId("feature".to_string()),
    );

    runtime.performance_access().reset_counters();
    let artifact = runtime
        .merge_access()
        .inspect_planning_scope(
            MergeExecutionRequest {
                target_branch: BranchId("main".to_string()),
                source_branch: BranchId("feature".to_string()),
                merge_intent: MergeIntent::ReconcileIntoTarget,
            }
            .into(),
        )
        .expect("disjoint rewires planning artifact");

    let local_topology_records = artifact
        .lowered_plan
        .records
        .iter()
        .filter(|record| {
            matches!(
                record.resolution_class,
                MergeResolutionClass::Topology(
                    TopologyExecutionClass::RelationEndpointRewiredEscalated
                )
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(local_topology_records.len(), 2);
    assert_eq!(
        artifact
            .lowered_plan
            .records
            .iter()
            .filter(|record| {
                matches!(
                    record.resolution_class,
                    MergeResolutionClass::Topology(TopologyExecutionClass::TopologyRegionConflict)
                )
            })
            .count(),
        0
    );

    for record in &local_topology_records {
        assert_eq!(
            record.blocked_reason,
            Some(LoweredMergeBlockedReason::RelationEndpointRewiredEscalated)
        );
        let classification = artifact
            .conflict_classification
            .classifications
            .iter()
            .find(|classification| classification.record == record.record)
            .expect("local topology classification");
        let evidence = classification
            .relation_evidence
            .as_ref()
            .expect("local topology evidence");
        assert_eq!(
            evidence.propagation,
            RelationConflictPropagation::RelationLocalRewireCandidate
        );
        assert_eq!(evidence.topology_region_conflict_reason, None);
        assert_eq!(evidence.topology_neighborhood_rewired_records.len(), 1);
        assert_eq!(evidence.topology_neighborhood_records.len(), 2);
    }

    let counters = runtime.performance_access().counters();
    assert_eq!(counters.merge_topology_region_conflicts_detected, 0);
    assert_eq!(counters.merge_topology_region_records_escalated, 0);
}

#[test]
fn unrelated_relation_additions_do_not_inflate_topology_region_detection_counters() {
    let mut runtime = persisted_runtime_with_topology_identity_registry(unique_test_store_path(
        "forge-relational-7d-topology-unrelated-breadth",
    ));
    let source = create_entity(&mut runtime, "source");
    let target_a = create_entity(&mut runtime, "target-a");
    let target_b = create_entity(&mut runtime, "target-b");
    let unrelated_source = create_entity(&mut runtime, "unrelated-source");
    let unrelated_target_a = create_entity(&mut runtime, "unrelated-target-a");
    let unrelated_target_b = create_entity(&mut runtime, "unrelated-target-b");
    let relation = crate::tests::support::create_relation(
        &mut runtime,
        source,
        target_a,
        "shared-edge",
    );
    create_branch_from_main(&mut runtime, "feature");
    delete_relation_on_branch(&mut runtime, relation, BranchId("feature".to_string()));
    create_relation_in_partition_on_branch(
        &mut runtime,
        source,
        target_b,
        "shared-edge",
        "shared-edge",
        crate::facade::identity::PartitionId::main(),
        BranchId("feature".to_string()),
    );
    create_relation_in_partition_on_branch(
        &mut runtime,
        unrelated_source,
        unrelated_target_a,
        "unrelated-a",
        "unrelated-a",
        crate::facade::identity::PartitionId::main(),
        BranchId("feature".to_string()),
    );
    create_relation_in_partition_on_branch(
        &mut runtime,
        unrelated_source,
        unrelated_target_b,
        "unrelated-b",
        "unrelated-b",
        crate::facade::identity::PartitionId::main(),
        BranchId("feature".to_string()),
    );

    runtime.performance_access().reset_counters();
    let artifact = runtime
        .merge_access()
        .inspect_planning_scope(
            MergeExecutionRequest {
                target_branch: BranchId("main".to_string()),
                source_branch: BranchId("feature".to_string()),
                merge_intent: MergeIntent::ReconcileIntoTarget,
            }
            .into(),
        )
        .expect("planning artifact");

    let local_topology_records = artifact
        .lowered_plan
        .records
        .iter()
        .filter(|record| {
            matches!(
                record.resolution_class,
                MergeResolutionClass::Topology(
                    TopologyExecutionClass::RelationEndpointRewiredEscalated
                )
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(local_topology_records.len(), 1);

    let counters = runtime.performance_access().counters();
    assert_eq!(counters.merge_topology_relation_candidates_scoped, 2);
    assert_eq!(counters.merge_topology_endpoint_incidences_scoped, 5);
    assert_eq!(counters.merge_topology_region_conflicts_detected, 0);
    assert_eq!(counters.merge_topology_region_records_escalated, 0);
}

fn drifted_schema_registry() -> crate::facade::schema::RelationalSchemaRegistry {
    crate::facade::schema::RelationalSchemaRegistry::new()
        .register_entity_kind(crate::facade::schema::EntityKindRegistration {
            kind_id: crate::facade::identity::KindId(1),
            kind_name: "test.entity".to_string(),
            schema_id: crate::facade::schema::SchemaId("test".to_string()),
            schema_version_id: crate::facade::schema::SchemaVersionId(2),
            aspect_declarations: crate::facade::schema::KindAspectDeclarations::new(vec![
                crate::tests::support::entity_payload_aspect("name", "name"),
                crate::tests::support::entity_payload_aspect("status", "status"),
            ]),
        })
        .and_then(|registry| {
            registry.register_relation_kind(crate::facade::schema::RelationKindRegistration {
                kind_id: crate::facade::identity::KindId(2),
                kind_name: "test.relation".to_string(),
                schema_id: crate::facade::schema::SchemaId("test".to_string()),
                schema_version_id: crate::facade::schema::SchemaVersionId(2),
                payload_class: crate::schema::data::RelationPayloadClass::PayloadBearingRelation,
                cross_context_policy: crate::tests::support::CrossContextPolicy::AllowExplicit,
                cascade_delete_policy:
                    crate::tests::support::CascadeDeletePolicy::CascadeDeleteRelations,
                aspect_declarations: crate::facade::schema::KindAspectDeclarations::default(),
                relation_integrity:
                    crate::facade::schema::RelationIntegrityDeclarations::default(),
            })
        })
        .expect("drifted schema registry")
}

fn topology_identity_registry() -> crate::facade::schema::RelationalSchemaRegistry {
    let label_key =
        crate::facade::schema::AspectKey(crate::symbols::data::InternedString::Raw(
            "label".to_string(),
        ));
    crate::facade::schema::RelationalSchemaRegistry::new()
        .register_entity_kind(crate::facade::schema::EntityKindRegistration {
            kind_id: crate::facade::identity::KindId(1),
            kind_name: "test.entity".to_string(),
            schema_id: crate::facade::schema::SchemaId("test".to_string()),
            schema_version_id: crate::facade::schema::SchemaVersionId(1),
            aspect_declarations: crate::facade::schema::KindAspectDeclarations::default(),
        })
        .and_then(|registry| {
            registry.register_relation_kind(crate::facade::schema::RelationKindRegistration {
                kind_id: crate::facade::identity::KindId(2),
                kind_name: "test.relation".to_string(),
                schema_id: crate::facade::schema::SchemaId("test".to_string()),
                schema_version_id: crate::facade::schema::SchemaVersionId(1),
                payload_class: crate::schema::data::RelationPayloadClass::PayloadBearingRelation,
                cross_context_policy: crate::tests::support::CrossContextPolicy::AllowExplicit,
                cascade_delete_policy:
                    crate::tests::support::CascadeDeletePolicy::CascadeDeleteRelations,
                aspect_declarations: crate::facade::schema::KindAspectDeclarations::new(vec![
                    crate::tests::support::relation_payload_aspect("label", "label"),
                    crate::tests::support::relation_source_aspect(),
                    crate::tests::support::relation_target_aspect(),
                ])
                .with_identity_declarations(vec![
                    crate::facade::merge::IdentityBasisDeclaration {
                        scope: crate::facade::merge::IdentityBasisScope::AspectKey(
                            label_key.clone(),
                        ),
                        basis: crate::facade::merge::IdentityBasisKind::DeclaredKeySet(
                            std::sync::Arc::from([label_key]),
                        ),
                    },
                ]),
                relation_integrity:
                    crate::facade::schema::RelationIntegrityDeclarations::default(),
            })
        })
        .expect("topology identity registry")
}

fn persisted_runtime_with_topology_identity_registry(
    root_path: std::path::PathBuf,
) -> crate::facade::runtime::RelationalRuntime {
    crate::facade::runtime::RelationalRuntimeApi::builder()
        .profile(crate::tests::support::RelationalRuntimeProfile::CertificationCore)
        .schema_registry(topology_identity_registry())
        .durability_mode(crate::tests::support::DurabilityMode::PersistedSegmentedLocalFs)
        .durable_store_layout(crate::tests::support::DurableStoreLayout {
            root_path,
            segment_commit_capacity: 2,
        })
        .build()
}
