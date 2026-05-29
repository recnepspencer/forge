use crate::facade::history::BranchId;
use crate::facade::indexes::{
    DerivedIndexBuildRequest, DerivedIndexDefinition, DerivedIndexId, DerivedIndexKind,
};
use crate::facade::query::{
    DeterministicQueryPlanKey, FallbackParityMode, PlannedQueryPacket, QueryExecutionShape,
    QueryFallbackContract, QueryLocalityClass, QueryOrderingContract, QueryScope,
    ReductionDiscipline,
};
use crate::tests::support::*;
use crate::validation::data::InvariantVerdict;

#[test]
fn complexity_budget_snapshot_visibility_state_avoids_record_materialization() {
    let mut runtime = runtime_with_test_schema();
    let _ = create_entity(&mut runtime, "first");
    let _ = create_entity(&mut runtime, "second");

    runtime.performance_access().reset_counters();
    let _snapshot = runtime.visibility_authority().snapshot();
    let counters = runtime.performance_access().counters();

    assert_eq!(counters.visible_entity_records_materialized, 0);
    assert_eq!(counters.visible_relation_records_materialized, 0);
}

#[test]
fn complexity_budget_snapshot_pin_maintenance_is_incremental() {
    let mut runtime = runtime_with_test_schema();
    for index in 0..6 {
        let _ = create_entity(&mut runtime, &format!("e{index}"));
    }
    let snapshot = runtime.visibility_authority().snapshot();
    let target = create_entity(&mut runtime, "target");

    runtime.performance_access().reset_counters();
    let _ = update_entity(&mut runtime, target, "updated");
    let after_commit = runtime.performance_access().counters();
    assert_eq!(after_commit.snapshot_pin_full_rebuilds, 0);

    runtime.performance_access().reset_counters();
    assert!(runtime.visibility_authority().release_snapshot(&snapshot));
    let after_release = runtime.performance_access().counters();
    assert_eq!(after_release.snapshot_pin_full_rebuilds, 0);
    assert!(after_release.snapshot_pin_adjustments > 0);
}

#[test]
fn complexity_budget_duplicate_active_snapshots_share_one_pin_lease_per_version() {
    let mut runtime = runtime_with_test_schema();
    for index in 0..6 {
        let _ = create_entity(&mut runtime, &format!("e{index}"));
    }

    runtime.performance_access().reset_counters();
    let first = runtime.visibility_authority().snapshot();
    let first_open = runtime.performance_access().counters();
    assert!(first_open.snapshot_pin_adjustments > 0);
    assert_eq!(first_open.visibility_cache_snapshot_promotions, 1);

    runtime.performance_access().reset_counters();
    let second = runtime.visibility_authority().snapshot();
    let second_open = runtime.performance_access().counters();
    assert_eq!(second_open.snapshot_pin_adjustments, 0);
    assert_eq!(second_open.visibility_cache_snapshot_promotions, 0);

    runtime.performance_access().reset_counters();
    assert!(runtime.visibility_authority().release_snapshot(&first));
    let first_release = runtime.performance_access().counters();
    assert_eq!(first_release.snapshot_pin_adjustments, 0);

    runtime.performance_access().reset_counters();
    assert!(runtime.visibility_authority().release_snapshot(&second));
    let second_release = runtime.performance_access().counters();
    assert!(second_release.snapshot_pin_adjustments > 0);
}

#[test]
fn complexity_budget_branch_creation_reuses_cached_visibility_state() {
    let mut runtime = runtime_with_test_schema();
    let left = create_entity(&mut runtime, "left");
    let right = create_entity(&mut runtime, "right");
    let _ = create_relation(&mut runtime, left, right, "r0");

    runtime.performance_access().reset_counters();
    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let counters = runtime.performance_access().counters();

    assert_eq!(counters.visibility_entity_slot_scans, 0);
    assert_eq!(counters.visibility_relation_slot_scans, 0);
}

#[test]
fn complexity_contract_visibility_scans_are_explicitly_measured() {
    let mut runtime = runtime_with_test_schema();
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    let relation_outcome = create_relation_outcome(&mut runtime, source, target, "r0");
    let snapshot = runtime.visibility_authority().snapshot();
    let historical_version = relation_outcome.version_id;
    let current_version = create_entity_outcome(&mut runtime, "later").version_id;

    runtime.performance_access().reset_counters();
    let _ = runtime.read_truth().read_snapshot(&snapshot).unwrap();
    let snapshot_counters = runtime.performance_access().counters();

    assert_eq!(snapshot_counters.visibility_entity_slot_scans, 0);
    assert_eq!(snapshot_counters.visibility_relation_slot_scans, 0);
    assert!(snapshot_counters.visible_entity_records_materialized >= 2);
    assert!(snapshot_counters.visible_relation_records_materialized >= 1);

    runtime.performance_access().reset_counters();
    let _ = runtime.read_truth().read_version(historical_version);
    let current_version_counters = runtime.performance_access().counters();

    assert_eq!(current_version_counters.visibility_entity_slot_scans, 0);
    assert_eq!(current_version_counters.visibility_relation_slot_scans, 0);
    assert!(current_version_counters.visible_entity_records_materialized >= 2);
    assert!(current_version_counters.visible_relation_records_materialized >= 1);

    runtime.performance_access().reset_counters();
    let _ = runtime.read_truth().read_version(current_version);
    let historical_version_counters = runtime.performance_access().counters();

    assert_eq!(historical_version_counters.visibility_entity_slot_scans, 0);
    assert_eq!(
        historical_version_counters.visibility_relation_slot_scans,
        0
    );
}

#[test]
fn complexity_contract_invariant_materialization_is_declared_and_measured() {
    let mut runtime = runtime_with_declared_aspect_schema_and_invariants(InvariantCatalog {
        registrations: vec![InvariantRegistration::commit_boundary_blocking(
            InvariantRule::unique_entity_aspect_field("name", "name")
                .expect("valid unique aspect field target"),
        )],
        ..InvariantCatalog::default()
    });
    let entity = create_entity(&mut runtime, "a");

    runtime.performance_access().reset_counters();
    let _ = update_entity(&mut runtime, entity, "a-2");
    let counters = runtime.performance_access().counters();

    assert!(counters.invariant_entity_slot_scans >= 1);
    assert_eq!(counters.invariant_entity_records_materialized, 0);
}

#[test]
fn complexity_budget_snapshot_entity_limit_uses_live_bitsets_for_current_version() {
    let mut runtime = runtime_with_test_schema_and_invariants(InvariantCatalog {
        registrations: vec![InvariantRegistration::snapshot_publication_blocking(
            InvariantRule::MaxSnapshotEntities(1),
        )],
        ..InvariantCatalog::default()
    });
    let _ = create_entity(&mut runtime, "visible");

    runtime.performance_access().reset_counters();
    let results = runtime
        .validation()
        .snapshot_publication_state()
        .into_results();
    let counters = runtime.performance_access().counters();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].class(), InvariantClass::SnapshotAudit);
    assert_eq!(results[0].verdict, InvariantVerdict::Pass);
    assert_eq!(counters.invariant_entity_slot_scans, 0);
    assert_eq!(counters.invariant_entity_records_materialized, 0);
}

#[test]
fn complexity_budget_live_history_trimming_is_touched_record_bounded() {
    let mut runtime = runtime_with_test_schema();
    let create_a = create_entity_outcome(&mut runtime, "a");
    let entity_a = changed_entities(&create_a)[0];
    let create_b = create_entity_outcome(&mut runtime, "b");
    let entity_b = changed_entities(&create_b)[0];
    assert!(runtime
        .visibility_authority()
        .release_snapshot(&create_a.snapshot));
    assert!(runtime
        .visibility_authority()
        .release_snapshot(&create_b.snapshot));

    runtime.performance_access().reset_counters();
    let update_a1 = update_entity(&mut runtime, entity_a, "a-1");
    assert!(runtime
        .visibility_authority()
        .release_snapshot(&update_a1.snapshot));
    let _ = update_entity(&mut runtime, entity_a, "a-2");
    let counters = runtime.performance_access().counters();

    assert_eq!(runtime.entity_history_len_for_test(entity_a), 1);
    assert_eq!(runtime.entity_history_len_for_test(entity_b), 1);
    assert!(counters.live_entity_history_entries_trimmed >= 1);
}

#[test]
fn complexity_budget_bidirectional_adjacency_avoids_relation_scans() {
    let mut runtime = runtime_with_test_schema();
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    let relation = create_relation(&mut runtime, source, target, "r0");
    let version_id = runtime.history().latest_commit().unwrap().version_id;

    runtime.performance_access().reset_counters();
    let outgoing = runtime
        .storage_access()
        .outgoing_relations_for_entity(source, version_id);
    let incoming = runtime
        .storage_access()
        .incoming_relations_for_entity(target, version_id);
    let counters = runtime.performance_access().counters();

    assert_eq!(outgoing, vec![relation]);
    assert_eq!(incoming, vec![relation]);
    assert_eq!(counters.visibility_relation_slot_scans, 0);
}

#[test]
fn complexity_budget_partition_scoped_historical_entity_scans_are_partition_bounded() {
    let mut runtime = runtime_with_test_schema();
    let _left_a = create_entity_in_partition(&mut runtime, "left-a", PartitionId(7));
    let _left_b = create_entity_in_partition(&mut runtime, "left-b", PartitionId(7));
    let historical_version = runtime.history().latest_commit().unwrap().version_id;
    let _right_a = create_entity_in_partition(&mut runtime, "right-a", PartitionId(11));
    let _right_b = create_entity_in_partition(&mut runtime, "right-b", PartitionId(11));

    runtime.performance_access().reset_counters();
    let records = runtime
        .read_truth()
        .project_version(historical_version)
        .entity_records_in(PartitionId(7), KindId(1));
    let counters = runtime.performance_access().counters();

    assert_eq!(records.len(), 2);
    assert_eq!(counters.visibility_entity_slot_scans, 2);
}

#[test]
fn complexity_budget_partition_scoped_historical_relation_scans_are_partition_bounded() {
    let mut runtime = runtime_with_test_schema();
    let left_source = create_entity_in_partition(&mut runtime, "left-source", PartitionId(7));
    let left_target = create_entity_in_partition(&mut runtime, "left-target", PartitionId(7));
    let right_source = create_entity_in_partition(&mut runtime, "right-source", PartitionId(11));
    let right_target = create_entity_in_partition(&mut runtime, "right-target", PartitionId(11));
    let _left_relation = create_relation_in_partition(
        &mut runtime,
        left_source,
        left_target,
        "left-r0",
        PartitionId(7),
    );
    let historical_version = runtime.history().latest_commit().unwrap().version_id;
    let _right_relation = create_relation_in_partition(
        &mut runtime,
        right_source,
        right_target,
        "right-r0",
        PartitionId(11),
    );

    runtime.performance_access().reset_counters();
    let records = runtime
        .read_truth()
        .project_version(historical_version)
        .relation_records_in(PartitionId(7), KindId(2));
    let counters = runtime.performance_access().counters();

    assert_eq!(records.len(), 1);
    assert_eq!(counters.visibility_relation_slot_scans, 1);
}

#[test]
fn complexity_budget_index_entity_field_equals_avoids_snapshot_materialization() {
    let mut runtime = runtime_with_test_schema();
    let alpha = create_entity_outcome(&mut runtime, "alpha");
    let index = runtime.index_authority().register(DerivedIndexDefinition {
        index_id: DerivedIndexId(0),
        name: "entity.name.lookup".to_string(),
        kind: DerivedIndexKind::EntityField {
            field: field_key("name"),
        },
        branch_scoped: false,
    });
    let build = runtime
        .index_authority()
        .build_for_commit(DerivedIndexBuildRequest {
            source_commit_id: alpha.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            index_ids: vec![index.index_id],
        });
    assert!(build.failed_indexes.is_empty());

    let snapshot = runtime.visibility_authority().snapshot();
    let context = runtime
        .read_truth()
        .query_plan_context(&snapshot)
        .expect("query plan context");
    let packet = PlannedQueryPacket {
        label: "entity-name-equals".to_string(),
        context_id: context,
        scope: QueryScope::EntityFieldEquals {
            field: field_key("name"),
            value: string_aspect_value("alpha"),
            partition_scope: None,
        },
        locality: QueryLocalityClass::CrossPartitionTraversal,
        ordering: QueryOrderingContract::CanonicalEntityIdOrder,
        fallback: QueryFallbackContract::IndexAdmissibleStorageEquivalent,
        execution_shape: QueryExecutionShape::BulkPacketized,
        reduction: ReductionDiscipline::DeterministicMerge,
        plan_key: DeterministicQueryPlanKey(2001),
        target_count_hint: 0,
    };

    runtime.performance_access().reset_counters();
    let _ = runtime
        .index_access()
        .execute_query_plan_with_fallback_parity(
            runtime
                .read_truth()
                .plan_query_packet(&snapshot, packet)
                .expect("query plan"),
            FallbackParityMode::ProductionAdmissibility,
        )
        .expect("query outcome");
    let counters = runtime.performance_access().counters();

    assert_eq!(counters.visible_entity_records_materialized, 0);
    assert_eq!(counters.query_index_attempt_count, 1);
    assert_eq!(counters.query_index_path_count, 1);
    assert_eq!(counters.query_index_parity_verification_count, 0);
    assert_eq!(counters.query_entity_records_emitted, 1);
}

#[test]
fn complexity_budget_index_relation_field_equals_avoids_snapshot_materialization() {
    let mut runtime = runtime_with_test_schema();
    let source = create_entity_outcome(&mut runtime, "source");
    let source_id = changed_entities(&source)[0];
    let target = create_entity_outcome(&mut runtime, "target");
    let target_id = changed_entities(&target)[0];
    let relation = create_relation_outcome(&mut runtime, source_id, target_id, "edge");
    let index = runtime.index_authority().register(DerivedIndexDefinition {
        index_id: DerivedIndexId(1),
        name: "relation.label.lookup".to_string(),
        kind: DerivedIndexKind::RelationField {
            field: field_key("label"),
        },
        branch_scoped: false,
    });
    let build = runtime
        .index_authority()
        .build_for_commit(DerivedIndexBuildRequest {
            source_commit_id: relation.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            index_ids: vec![index.index_id],
        });
    assert!(build.failed_indexes.is_empty());

    let snapshot = runtime.visibility_authority().snapshot();
    let context = runtime
        .read_truth()
        .query_plan_context(&snapshot)
        .expect("query plan context");
    let packet = PlannedQueryPacket {
        label: "relation-label-equals".to_string(),
        context_id: context,
        scope: QueryScope::RelationFieldEquals {
            field: field_key("label"),
            value: string_aspect_value("edge"),
            partition_scope: None,
        },
        locality: QueryLocalityClass::CrossPartitionTraversal,
        ordering: QueryOrderingContract::CanonicalRelationIdOrder,
        fallback: QueryFallbackContract::IndexAdmissibleStorageEquivalent,
        execution_shape: QueryExecutionShape::BulkPacketized,
        reduction: ReductionDiscipline::DeterministicMerge,
        plan_key: DeterministicQueryPlanKey(2002),
        target_count_hint: 0,
    };

    runtime.performance_access().reset_counters();
    let _ = runtime
        .index_access()
        .execute_query_plan_with_fallback_parity(
            runtime
                .read_truth()
                .plan_query_packet(&snapshot, packet)
                .expect("query plan"),
            FallbackParityMode::ProductionAdmissibility,
        )
        .expect("query outcome");
    let counters = runtime.performance_access().counters();

    assert_eq!(counters.visible_relation_records_materialized, 0);
    assert_eq!(counters.query_index_attempt_count, 1);
    assert_eq!(counters.query_index_path_count, 1);
    assert_eq!(counters.query_index_parity_verification_count, 0);
    assert_eq!(counters.query_relation_records_emitted, 1);
}

#[test]
fn complexity_budget_index_field_equals_reuses_warm_index_scratch_on_repeated_lookup() {
    let mut runtime = runtime_with_test_schema();
    let alpha = create_entity_outcome(&mut runtime, "alpha");
    let index = runtime.index_authority().register(DerivedIndexDefinition {
        index_id: DerivedIndexId(2),
        name: "entity.name.reuse".to_string(),
        kind: DerivedIndexKind::EntityField {
            field: field_key("name"),
        },
        branch_scoped: false,
    });
    let build = runtime
        .index_authority()
        .build_for_commit(DerivedIndexBuildRequest {
            source_commit_id: alpha.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            index_ids: vec![index.index_id],
        });
    assert!(build.failed_indexes.is_empty());

    let snapshot = runtime.visibility_authority().snapshot();
    let context = runtime
        .read_truth()
        .query_plan_context(&snapshot)
        .expect("query plan context");
    let packet = PlannedQueryPacket {
        label: "entity-name-reuse".to_string(),
        context_id: context,
        scope: QueryScope::EntityFieldEquals {
            field: field_key("name"),
            value: string_aspect_value("alpha"),
            partition_scope: None,
        },
        locality: QueryLocalityClass::CrossPartitionTraversal,
        ordering: QueryOrderingContract::CanonicalEntityIdOrder,
        fallback: QueryFallbackContract::IndexAdmissibleStorageEquivalent,
        execution_shape: QueryExecutionShape::BulkPacketized,
        reduction: ReductionDiscipline::DeterministicMerge,
        plan_key: DeterministicQueryPlanKey(2003),
        target_count_hint: 0,
    };

    runtime.performance_access().reset_counters();
    for _ in 0..2 {
        let _ = runtime
            .index_access()
            .execute_query_plan_with_fallback_parity(
                runtime
                    .read_truth()
                    .plan_query_packet(&snapshot, packet.clone())
                    .expect("query plan"),
                FallbackParityMode::ProductionAdmissibility,
            )
            .expect("query outcome");
    }
    let counters = runtime.performance_access().counters();

    assert_eq!(counters.query_index_attempt_count, 2);
    assert_eq!(counters.query_index_path_count, 2);
    assert!(counters.query_index_scratch_reuse_count > 0);
}

#[test]
fn complexity_budget_index_field_equals_reports_actual_result_width() {
    let mut runtime = runtime_with_test_schema();
    let _alpha_a = create_entity_outcome(&mut runtime, "alpha");
    let _alpha_b = create_entity_outcome(&mut runtime, "alpha");
    let index = runtime.index_authority().register(DerivedIndexDefinition {
        index_id: DerivedIndexId(3),
        name: "entity.name.width".to_string(),
        kind: DerivedIndexKind::EntityField {
            field: field_key("name"),
        },
        branch_scoped: false,
    });
    let latest_commit_id = runtime
        .history()
        .latest_commit()
        .expect("latest commit")
        .commit_id;
    let build = runtime
        .index_authority()
        .build_for_commit(DerivedIndexBuildRequest {
            source_commit_id: latest_commit_id,
            branch_id: BranchId("main".to_string()),
            index_ids: vec![index.index_id],
        });
    assert!(build.failed_indexes.is_empty());

    let snapshot = runtime.visibility_authority().snapshot();
    let context = runtime
        .read_truth()
        .query_plan_context(&snapshot)
        .expect("query plan context");
    let packet = PlannedQueryPacket {
        label: "entity-name-width".to_string(),
        context_id: context,
        scope: QueryScope::EntityFieldEquals {
            field: field_key("name"),
            value: string_aspect_value("alpha"),
            partition_scope: None,
        },
        locality: QueryLocalityClass::CrossPartitionTraversal,
        ordering: QueryOrderingContract::CanonicalEntityIdOrder,
        fallback: QueryFallbackContract::IndexAdmissibleStorageEquivalent,
        execution_shape: QueryExecutionShape::BulkPacketized,
        reduction: ReductionDiscipline::DeterministicMerge,
        plan_key: DeterministicQueryPlanKey(2004),
        target_count_hint: 0,
    };

    runtime.performance_access().reset_counters();
    let outcome = runtime
        .index_access()
        .execute_query_plan_with_fallback_parity(
            runtime
                .read_truth()
                .plan_query_packet(&snapshot, packet)
                .expect("query plan"),
            FallbackParityMode::ProductionAdmissibility,
        )
        .expect("query outcome");
    let counters = runtime.performance_access().counters();

    assert_eq!(outcome.execution.result.entities.len(), 2);
    assert_eq!(counters.query_packet_count, 1);
    assert_eq!(counters.query_packet_item_count, 2);
    assert_eq!(counters.query_packet_peak_width_total, 2);
}

#[test]
fn complexity_budget_query_packetization_reports_parallel_shape_for_cross_partition_reads() {
    let mut runtime = runtime_with_test_schema_execution_model(
        crate::facade::runtime::RelationalExecutionModel::StagedParallelPreparation,
    );
    let left_a = create_entity_in_partition(&mut runtime, "left-a", PartitionId(7));
    let left_b = create_entity_in_partition(&mut runtime, "left-b", PartitionId(7));
    let right = create_entity_in_partition(&mut runtime, "right", PartitionId(11));
    let snapshot = runtime.visibility_authority().snapshot();

    runtime.performance_access().reset_counters();
    let _ = runtime
        .read_truth()
        .execute_query_plan(planned_explicit_query(
            &runtime,
            &snapshot,
            "cross-partition",
            vec![
                crate::facade::transactions::RecordRef::Entity(left_a),
                crate::facade::transactions::RecordRef::Entity(left_b),
                crate::facade::transactions::RecordRef::Entity(right),
            ],
        ))
        .expect("query outcome");
    let counters = runtime.performance_access().counters();

    assert_eq!(counters.query_packet_count, 2);
    assert_eq!(counters.query_packet_item_count, 3);
    assert_eq!(counters.query_packet_peak_width_total, 2);
    assert_eq!(counters.query_scope_unit_count, 2);
    assert_eq!(counters.query_parallel_legal_count, 1);
    assert_eq!(counters.query_parallel_profitable_count, 1);
    assert_eq!(counters.query_staged_parallel_strategy_count, 1);
}

#[test]
fn complexity_budget_query_packetization_reports_serial_shape_for_narrow_reads() {
    let mut runtime = runtime_with_test_schema_execution_model(
        crate::facade::runtime::RelationalExecutionModel::StagedParallelPreparation,
    );
    let entity = create_entity(&mut runtime, "single");
    let snapshot = runtime.visibility_authority().snapshot();

    runtime.performance_access().reset_counters();
    let _ = runtime
        .read_truth()
        .execute_query_plan(planned_explicit_query(
            &runtime,
            &snapshot,
            "single-target",
            vec![crate::facade::transactions::RecordRef::Entity(entity)],
        ))
        .expect("query outcome");
    let counters = runtime.performance_access().counters();

    assert_eq!(counters.query_packet_count, 1);
    assert_eq!(counters.query_packet_item_count, 1);
    assert_eq!(counters.query_packet_peak_width_total, 1);
    assert_eq!(counters.query_scope_unit_count, 1);
    assert_eq!(counters.query_parallel_legal_count, 1);
    assert_eq!(counters.query_parallel_profitable_count, 0);
    assert_eq!(counters.query_serial_strategy_count, 1);
}
