use crate::facade::snapshots::SnapshotHandle;
use crate::tests::support::*;
use std::sync::Arc;

#[test]
fn planned_query_execution_reduces_explicit_targets_into_canonical_entity_order() {
    let mut runtime = runtime_with_test_schema();
    let first = create_entity_outcome(&mut runtime, "first");
    let second = create_entity_outcome(&mut runtime, "second");
    let first_id = changed_entities(&first)[0];
    let second_id = changed_entities(&second)[0];

    let outcome = execute_explicit_query(
        &runtime,
        &second.snapshot,
        "reverse-order",
        vec![RecordRef::Entity(second_id), RecordRef::Entity(first_id)],
    );

    assert_eq!(
        outcome.result.ordering,
        QueryOrderingContract::CanonicalRecordRefOrder
    );
    assert_eq!(
        outcome
            .result
            .entities
            .iter()
            .map(|record| record.entity_id)
            .collect::<Vec<_>>(),
        vec![first_id, second_id]
    );
    assert_eq!(outcome.complexity.packet_count, 1);
    assert_eq!(outcome.complexity.fragment_count, 1);
    assert_eq!(outcome.complexity.target_count, 2);
    assert_eq!(outcome.complexity.unmasked_entity_records_emitted, 2);
    assert_eq!(outcome.complexity.unmasked_relation_records_emitted, 0);
}

#[test]
fn planned_query_execution_is_deterministic_for_identical_inputs() {
    let mut runtime = runtime_with_test_schema();
    let first = create_entity_outcome(&mut runtime, "first");
    let second = create_entity_outcome(&mut runtime, "second");
    let first_id = changed_entities(&first)[0];
    let second_id = changed_entities(&second)[0];
    let first_outcome = execute_explicit_query(
        &runtime,
        &second.snapshot,
        "stable-execution",
        vec![RecordRef::Entity(second_id), RecordRef::Entity(first_id)],
    );
    let second_outcome = execute_explicit_query(
        &runtime,
        &second.snapshot,
        "stable-execution",
        vec![RecordRef::Entity(second_id), RecordRef::Entity(first_id)],
    );

    assert_eq!(first_outcome.result, second_outcome.result);
    assert_eq!(first_outcome.complexity, second_outcome.complexity);
}

#[test]
fn planned_query_execution_uses_staged_parallel_packets_for_profitable_cross_partition_reads() {
    let mut runtime = runtime_with_test_schema_execution_model(
        crate::facade::runtime::RelationalExecutionModel::StagedParallelPreparation,
    );
    let targets = vec![
        RecordRef::Entity(create_entity_in_partition(
            &mut runtime,
            "a-1",
            PartitionId(7),
        )),
        RecordRef::Entity(create_entity_in_partition(
            &mut runtime,
            "b-1",
            PartitionId(11),
        )),
        RecordRef::Entity(create_entity_in_partition(
            &mut runtime,
            "a-2",
            PartitionId(7),
        )),
        RecordRef::Entity(create_entity_in_partition(
            &mut runtime,
            "b-2",
            PartitionId(11),
        )),
        RecordRef::Entity(create_entity_in_partition(
            &mut runtime,
            "a-3",
            PartitionId(7),
        )),
        RecordRef::Entity(create_entity_in_partition(
            &mut runtime,
            "b-3",
            PartitionId(11),
        )),
    ];
    let snapshot = runtime.visibility_authority().snapshot();
    runtime.performance_access().reset_counters();

    let outcome = execute_explicit_query(&runtime, &snapshot, "parallel-query", targets);
    let counters = runtime.performance_access().counters();

    assert_eq!(outcome.complexity.fragment_count, 2);
    assert_eq!(outcome.complexity.touched_partitions, 2);
    assert_eq!(counters.query_packet_count, 2);
    assert_eq!(counters.query_packet_item_count, 6);
    assert_eq!(counters.query_parallel_legal_count, 1);
    assert_eq!(counters.query_parallel_profitable_count, 1);
    assert_eq!(counters.query_staged_parallel_strategy_count, 1);
    assert_eq!(counters.query_serial_strategy_count, 0);
    assert_eq!(counters.query_unmasked_entity_records_emitted, 6);
    assert_eq!(counters.query_unmasked_relation_records_emitted, 0);
}

#[test]
fn planned_query_execution_supports_entity_kind_scans_through_reducer_path() {
    let mut runtime =
        runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    let left = create_entity_in_partition(&mut runtime, "left-a", PartitionId(7));
    let _other_left = create_entity_in_partition(&mut runtime, "left-b", PartitionId(7));
    let right = create_entity_in_partition(&mut runtime, "right-a", PartitionId(11));
    let snapshot = runtime.visibility_authority().snapshot();
    let context = runtime
        .read_truth()
        .query_plan_context(&snapshot)
        .expect("query plan context");
    let packet = PlannedQueryPacket {
        label: "entity-kind-scan".to_string(),
        context_id: context,
        scope: QueryScope::EntityKindScan {
            kind_id: KindId(1),
            partition_scope: Some(Arc::from([PartitionId(7), PartitionId(11)])),
        },
        locality: QueryLocalityClass::PartitionBounded {
            partitions: Arc::from([PartitionId(7), PartitionId(11)]),
        },
        ordering: QueryOrderingContract::CanonicalEntityIdOrder,
        access_contract: QueryAccessContract::AuthoritativeStorageOnly,
        execution_shape: QueryExecutionShape::BulkPacketized,
        reduction: ReductionDiscipline::DeterministicMerge,
        plan_key: DeterministicQueryPlanKey(501),
        target_count_hint: 0,
    };

    let plan = runtime
        .read_truth()
        .plan_query_packet(&snapshot, packet)
        .expect("planned query packet");
    let outcome = runtime
        .read_truth()
        .execute_query_plan(plan)
        .expect("query execution outcome");

    assert_eq!(outcome.complexity.packet_count, 2);
    assert_eq!(outcome.complexity.fragment_count, 2);
    assert_eq!(outcome.complexity.unmasked_entity_records_emitted, 3);
    assert_eq!(outcome.result.entities.len(), 3);
    assert_eq!(outcome.result.entities[0].entity_id, left);
    assert_eq!(outcome.result.entities[2].entity_id, right);
}

#[test]
fn planned_query_execution_reports_non_zero_packet_items_for_kind_scans() {
    let mut runtime =
        runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    let _left = create_entity_in_partition(&mut runtime, "left-a", PartitionId(7));
    let _right = create_entity_in_partition(&mut runtime, "right-a", PartitionId(11));
    let snapshot = runtime.visibility_authority().snapshot();
    let context = runtime
        .read_truth()
        .query_plan_context(&snapshot)
        .expect("query plan context");
    let packet = PlannedQueryPacket {
        label: "entity-kind-scan-accounting".to_string(),
        context_id: context,
        scope: QueryScope::EntityKindScan {
            kind_id: KindId(1),
            partition_scope: Some(Arc::from([PartitionId(7), PartitionId(11)])),
        },
        locality: QueryLocalityClass::PartitionBounded {
            partitions: Arc::from([PartitionId(7), PartitionId(11)]),
        },
        ordering: QueryOrderingContract::CanonicalEntityIdOrder,
        access_contract: QueryAccessContract::AuthoritativeStorageOnly,
        execution_shape: QueryExecutionShape::BulkPacketized,
        reduction: ReductionDiscipline::DeterministicMerge,
        plan_key: DeterministicQueryPlanKey(502),
        target_count_hint: 0,
    };

    runtime.performance_access().reset_counters();
    let plan = runtime
        .read_truth()
        .plan_query_packet(&snapshot, packet)
        .expect("planned query packet");
    let _ = runtime
        .read_truth()
        .execute_query_plan(plan)
        .expect("query execution outcome");
    let counters = runtime.performance_access().counters();

    assert_eq!(counters.query_packet_count, 2);
    assert_eq!(counters.query_packet_item_count, 2);
}

#[test]
fn planned_query_execution_supports_relation_kind_scans_through_reducer_path() {
    let mut runtime =
        runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    let left = create_entity_in_partition(&mut runtime, "left", PartitionId(7));
    let right = create_entity_in_partition(&mut runtime, "right", PartitionId(11));
    let third = create_entity_in_partition(&mut runtime, "third", PartitionId(11));
    let first_relation =
        create_relation_in_partition(&mut runtime, left, right, "r1", PartitionId(7));
    let second_relation =
        create_relation_in_partition(&mut runtime, right, third, "r2", PartitionId(11));
    let snapshot = runtime.visibility_authority().snapshot();
    let context = runtime
        .read_truth()
        .query_plan_context(&snapshot)
        .expect("query plan context");
    let packet = PlannedQueryPacket {
        label: "relation-kind-scan".to_string(),
        context_id: context,
        scope: QueryScope::RelationKindScan {
            kind_id: KindId(2),
            partition_scope: None,
        },
        locality: QueryLocalityClass::CrossPartitionTraversal,
        ordering: QueryOrderingContract::CanonicalRelationIdOrder,
        access_contract: QueryAccessContract::AuthoritativeStorageOnly,
        execution_shape: QueryExecutionShape::BulkPacketized,
        reduction: ReductionDiscipline::DeterministicMerge,
        plan_key: DeterministicQueryPlanKey(777),
        target_count_hint: 0,
    };

    let plan = runtime
        .read_truth()
        .plan_query_packet(&snapshot, packet)
        .expect("planned query packet");
    let outcome = runtime
        .read_truth()
        .execute_query_plan(plan)
        .expect("query execution outcome");

    assert_eq!(outcome.complexity.packet_count, 2);
    assert_eq!(outcome.complexity.unmasked_relation_records_emitted, 2);
    assert_eq!(
        outcome
            .result
            .relations
            .iter()
            .map(|record| record.relation_id)
            .collect::<Vec<_>>(),
        vec![first_relation, second_relation]
    );
}

#[test]
fn planned_query_execution_supports_aspect_filtered_entity_scans_through_reducer_path() {
    let mut runtime =
        runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    let left = create_entity_in_partition(&mut runtime, "left-a", PartitionId(7));
    let right = create_entity_in_partition(&mut runtime, "right-a", PartitionId(11));
    let snapshot = runtime.visibility_authority().snapshot();
    let context = runtime
        .read_truth()
        .query_plan_context(&snapshot)
        .expect("query plan context");
    let packet = PlannedQueryPacket {
        label: "aspect-filtered-entities".to_string(),
        context_id: context,
        scope: QueryScope::AspectFilteredEntities {
            kind_id: Some(KindId(1)),
            aspect_filter: ProjectionAspectFilter::whole_aspects(
                ProjectionAspectFilterMode::All,
                [aspect_key("name")],
            ),
            partition_scope: Some(Arc::from([PartitionId(7), PartitionId(11)])),
        },
        locality: QueryLocalityClass::PartitionBounded {
            partitions: Arc::from([PartitionId(7), PartitionId(11)]),
        },
        ordering: QueryOrderingContract::CanonicalEntityIdOrder,
        access_contract: QueryAccessContract::AuthoritativeStorageOnly,
        execution_shape: QueryExecutionShape::BulkPacketized,
        reduction: ReductionDiscipline::DeterministicMerge,
        plan_key: DeterministicQueryPlanKey(901),
        target_count_hint: 0,
    };

    let plan = runtime
        .read_truth()
        .plan_query_packet(&snapshot, packet)
        .expect("planned query packet");
    let outcome = runtime
        .read_truth()
        .execute_query_plan(plan)
        .expect("query execution outcome");

    assert_eq!(outcome.complexity.packet_count, 2);
    assert_eq!(
        outcome
            .result
            .entities
            .iter()
            .map(|record| record.entity_id)
            .collect::<Vec<_>>(),
        vec![left, right]
    );
}

#[test]
fn planned_query_execution_aspect_filter_requires_projected_authoritative_presence() {
    let mut fixture = AspectSchemaFixture::with_default_declared_aspects(
        CascadeDeletePolicy::CascadeDeleteRelations,
    );
    fixture.entity_aspects.push(entity_field_aspect(
        aspect_key("status"),
        field_key("status"),
    ));
    let mut runtime = fixture.build_runtime();
    create_entity_in_partition(&mut runtime, "declared-without-status", PartitionId(7));
    let snapshot = runtime.visibility_authority().snapshot();
    let context = runtime
        .read_truth()
        .query_plan_context(&snapshot)
        .expect("query plan context");
    let packet = PlannedQueryPacket {
        label: "actual-authoritative-aspect-filter".to_string(),
        context_id: context,
        scope: QueryScope::AspectFilteredEntities {
            kind_id: Some(KindId(1)),
            aspect_filter: ProjectionAspectFilter::whole_aspects(
                ProjectionAspectFilterMode::All,
                [aspect_key("status")],
            ),
            partition_scope: Some(Arc::from([PartitionId(7)])),
        },
        locality: QueryLocalityClass::PartitionBounded {
            partitions: Arc::from([PartitionId(7)]),
        },
        ordering: QueryOrderingContract::CanonicalEntityIdOrder,
        access_contract: QueryAccessContract::AuthoritativeStorageOnly,
        execution_shape: QueryExecutionShape::BulkPacketized,
        reduction: ReductionDiscipline::DeterministicMerge,
        plan_key: DeterministicQueryPlanKey(903),
        target_count_hint: 0,
    };

    let plan = runtime
        .read_truth()
        .plan_query_packet(&snapshot, packet)
        .expect("planned query packet");
    let outcome = runtime
        .read_truth()
        .execute_query_plan(plan)
        .expect("query execution outcome");

    assert_eq!(outcome.result.entities.len(), 0);
    assert_eq!(outcome.complexity.unmasked_entity_records_emitted, 0);
}

#[test]
fn planned_query_execution_aspect_filter_supports_field_mask_presence() {
    let mut runtime = AspectSchemaFixture {
        entity_aspects: vec![
            entity_field_aspect(aspect_key("name"), field_key("name")),
            entity_summary_struct_aspect(aspect_key("summary"), field_key("summary")),
        ],
        ..AspectSchemaFixture::default()
    }
    .build_runtime();
    let matched = create_entity_with_summary_title(&mut runtime, "matched", "visible-title");
    create_entity_in_partition(&mut runtime, "missing-summary", PartitionId(7));
    let snapshot = runtime.visibility_authority().snapshot();
    let context = runtime
        .read_truth()
        .query_plan_context(&snapshot)
        .expect("query plan context");
    let packet = PlannedQueryPacket {
        label: "field-mask-aspect-filter".to_string(),
        context_id: context,
        scope: QueryScope::AspectFilteredEntities {
            kind_id: Some(KindId(1)),
            aspect_filter: ProjectionAspectFilter::new(
                ProjectionAspectFilterMode::All,
                ProjectionAspectScope::fields(aspect_key("summary"), [field_key("title")]),
            ),
            partition_scope: Some(Arc::from([PartitionId::main(), PartitionId(7)])),
        },
        locality: QueryLocalityClass::PartitionBounded {
            partitions: Arc::from([PartitionId::main(), PartitionId(7)]),
        },
        ordering: QueryOrderingContract::CanonicalEntityIdOrder,
        access_contract: QueryAccessContract::AuthoritativeStorageOnly,
        execution_shape: QueryExecutionShape::BulkPacketized,
        reduction: ReductionDiscipline::DeterministicMerge,
        plan_key: DeterministicQueryPlanKey(905),
        target_count_hint: 0,
    };

    let plan = runtime
        .read_truth()
        .plan_query_packet(&snapshot, packet)
        .expect("planned query packet");
    let outcome = runtime
        .read_truth()
        .execute_query_plan(plan)
        .expect("query execution outcome");

    assert_eq!(
        outcome
            .result
            .entities
            .iter()
            .map(|record| record.entity_id)
            .collect::<Vec<_>>(),
        vec![matched]
    );
}

#[test]
fn planned_query_execution_supports_aspect_filtered_relation_scans_through_reducer_path() {
    let mut runtime =
        runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    let left = create_entity_in_partition(&mut runtime, "left", PartitionId(7));
    let right = create_entity_in_partition(&mut runtime, "right", PartitionId(11));
    let third = create_entity_in_partition(&mut runtime, "third", PartitionId(11));
    let first_relation =
        create_relation_in_partition(&mut runtime, left, right, "r1", PartitionId(7));
    let second_relation =
        create_relation_in_partition(&mut runtime, right, third, "r2", PartitionId(11));
    let snapshot = runtime.visibility_authority().snapshot();
    let context = runtime
        .read_truth()
        .query_plan_context(&snapshot)
        .expect("query plan context");
    let packet = PlannedQueryPacket {
        label: "aspect-filtered-relations".to_string(),
        context_id: context,
        scope: QueryScope::AspectFilteredRelations {
            kind_id: Some(KindId(2)),
            aspect_filter: ProjectionAspectFilter::whole_aspects(
                ProjectionAspectFilterMode::All,
                [aspect_key("label")],
            ),
            partition_scope: Some(Arc::from([PartitionId(7), PartitionId(11)])),
        },
        locality: QueryLocalityClass::PartitionBounded {
            partitions: Arc::from([PartitionId(7), PartitionId(11)]),
        },
        ordering: QueryOrderingContract::CanonicalRelationIdOrder,
        access_contract: QueryAccessContract::AuthoritativeStorageOnly,
        execution_shape: QueryExecutionShape::BulkPacketized,
        reduction: ReductionDiscipline::DeterministicMerge,
        plan_key: DeterministicQueryPlanKey(904),
        target_count_hint: 0,
    };

    let plan = runtime
        .read_truth()
        .plan_query_packet(&snapshot, packet)
        .expect("planned query packet");
    let outcome = runtime
        .read_truth()
        .execute_query_plan(plan)
        .expect("query execution outcome");

    assert_eq!(outcome.complexity.packet_count, 2);
    assert_eq!(
        outcome
            .result
            .relations
            .iter()
            .map(|record| record.relation_id)
            .collect::<Vec<_>>(),
        vec![first_relation, second_relation]
    );
}

fn create_entity_with_summary_title(
    runtime: &mut RelationalRuntime,
    client_key: &str,
    summary_title: &str,
) -> crate::facade::identity::EntityId {
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new(format!("summary-{client_key}")).push(MutationIntent::Create(
            CreateIntent::Entity(crate::transactions::data::EntitySpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(1),
                client_key: crate::symbols::data::ClientKey::raw(client_key),
                fields: AspectFieldPatch::new(std::collections::BTreeMap::from([
                    (
                        crate::transactions::data::planned_single_field_locator(
                            aspect_key("name"),
                            field_key("name"),
                        ),
                        string_aspect_value(client_key),
                    ),
                    (
                        crate::transactions::data::planned_single_field_locator(
                            aspect_key("summary"),
                            field_key("title"),
                        ),
                        string_aspect_value(summary_title),
                    ),
                ])),
            }),
        )),
    );
    changed_entities(&txn.commit().unwrap())[0]
}

#[test]
fn planned_query_execution_supports_outgoing_neighborhood_with_canonical_traversal_order() {
    let mut runtime =
        runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    let a = create_entity_in_partition(&mut runtime, "a", PartitionId(7));
    let b = create_entity_in_partition(&mut runtime, "b", PartitionId(11));
    let c = create_entity_in_partition(&mut runtime, "c", PartitionId(13));
    let first_relation = create_relation_in_partition(&mut runtime, a, b, "ab", PartitionId(7));
    let second_relation = create_relation_in_partition(&mut runtime, a, c, "ac", PartitionId(13));
    let snapshot = runtime.visibility_authority().snapshot();
    let context = runtime
        .read_truth()
        .query_plan_context(&snapshot)
        .expect("query plan context");
    let packet = PlannedQueryPacket {
        label: "outgoing-neighborhood".to_string(),
        context_id: context,
        scope: QueryScope::OutgoingNeighborhood {
            seeds: Arc::from([a]),
            relation_kind_scope: Some(Arc::from([KindId(2)])),
        },
        locality: QueryLocalityClass::CrossPartitionTraversal,
        ordering: QueryOrderingContract::CanonicalTraversalOrder,
        access_contract: QueryAccessContract::AuthoritativeStorageOnly,
        execution_shape: QueryExecutionShape::BulkPacketized,
        reduction: ReductionDiscipline::DeterministicMerge,
        plan_key: DeterministicQueryPlanKey(902),
        target_count_hint: 1,
    };

    let plan = runtime
        .read_truth()
        .plan_query_packet(&snapshot, packet)
        .expect("planned query packet");
    let outcome = runtime
        .read_truth()
        .execute_query_plan(plan)
        .expect("query execution outcome");

    assert_eq!(outcome.complexity.packet_count, 1);
    assert_eq!(
        outcome
            .result
            .entities
            .iter()
            .map(|record| record.entity_id)
            .collect::<Vec<_>>(),
        vec![a, b, c]
    );
    assert_eq!(
        outcome
            .result
            .relations
            .iter()
            .map(|record| record.relation_id)
            .collect::<Vec<_>>(),
        vec![first_relation, second_relation]
    );
}

#[test]
fn planned_query_execution_supports_incoming_neighborhood_with_canonical_traversal_order() {
    let mut runtime =
        runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    let a = create_entity_in_partition(&mut runtime, "a", PartitionId(7));
    let b = create_entity_in_partition(&mut runtime, "b", PartitionId(11));
    let c = create_entity_in_partition(&mut runtime, "c", PartitionId(13));
    let first_relation = create_relation_in_partition(&mut runtime, a, c, "ac", PartitionId(7));
    let second_relation = create_relation_in_partition(&mut runtime, b, c, "bc", PartitionId(11));
    let snapshot = runtime.visibility_authority().snapshot();
    let context = runtime
        .read_truth()
        .query_plan_context(&snapshot)
        .expect("query plan context");
    let packet = PlannedQueryPacket {
        label: "incoming-neighborhood".to_string(),
        context_id: context,
        scope: QueryScope::IncomingNeighborhood {
            seeds: Arc::from([c]),
            relation_kind_scope: Some(Arc::from([KindId(2)])),
        },
        locality: QueryLocalityClass::CrossPartitionTraversal,
        ordering: QueryOrderingContract::CanonicalTraversalOrder,
        access_contract: QueryAccessContract::AuthoritativeStorageOnly,
        execution_shape: QueryExecutionShape::BulkPacketized,
        reduction: ReductionDiscipline::DeterministicMerge,
        plan_key: DeterministicQueryPlanKey(905),
        target_count_hint: 1,
    };

    let plan = runtime
        .read_truth()
        .plan_query_packet(&snapshot, packet)
        .expect("planned query packet");
    let outcome = runtime
        .read_truth()
        .execute_query_plan(plan)
        .expect("query execution outcome");

    assert_eq!(
        outcome
            .result
            .entities
            .iter()
            .map(|record| record.entity_id)
            .collect::<Vec<_>>(),
        vec![c, a, b]
    );
    assert_eq!(
        outcome
            .result
            .relations
            .iter()
            .map(|record| record.relation_id)
            .collect::<Vec<_>>(),
        vec![first_relation, second_relation]
    );
}

#[test]
fn planned_query_execution_supports_connectivity_traversal_with_depth_bound() {
    let mut runtime =
        runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    let a = create_entity_in_partition(&mut runtime, "a", PartitionId(7));
    let b = create_entity_in_partition(&mut runtime, "b", PartitionId(11));
    let c = create_entity_in_partition(&mut runtime, "c", PartitionId(13));
    let first_relation = create_relation_in_partition(&mut runtime, a, b, "ab", PartitionId(7));
    let _second_relation = create_relation_in_partition(&mut runtime, b, c, "bc", PartitionId(11));
    let snapshot = runtime.visibility_authority().snapshot();
    let context = runtime
        .read_truth()
        .query_plan_context(&snapshot)
        .expect("query plan context");
    let packet = PlannedQueryPacket {
        label: "connectivity-traversal".to_string(),
        context_id: context,
        scope: QueryScope::ConnectivityTraversal {
            seeds: Arc::from([a]),
            relation_kind_scope: Some(Arc::from([KindId(2)])),
            max_depth: Some(1),
        },
        locality: QueryLocalityClass::CrossPartitionTraversal,
        ordering: QueryOrderingContract::CanonicalTraversalOrder,
        access_contract: QueryAccessContract::AuthoritativeStorageOnly,
        execution_shape: QueryExecutionShape::BulkPacketized,
        reduction: ReductionDiscipline::DeterministicMerge,
        plan_key: DeterministicQueryPlanKey(903),
        target_count_hint: 1,
    };

    let plan = runtime
        .read_truth()
        .plan_query_packet(&snapshot, packet)
        .expect("planned query packet");
    let outcome = runtime
        .read_truth()
        .execute_query_plan(plan)
        .expect("query execution outcome");

    assert_eq!(
        outcome
            .result
            .entities
            .iter()
            .map(|record| record.entity_id)
            .collect::<Vec<_>>(),
        vec![a, b]
    );
    assert_eq!(
        outcome
            .result
            .relations
            .iter()
            .map(|record| record.relation_id)
            .collect::<Vec<_>>(),
        vec![first_relation]
    );
    assert_eq!(outcome.complexity.unmasked_entity_records_emitted, 2);
    assert_eq!(outcome.complexity.unmasked_relation_records_emitted, 1);
}

#[test]
fn planned_query_execution_normalizes_traversal_seed_order_deterministically() {
    let mut runtime =
        runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    let a = create_entity_in_partition(&mut runtime, "a", PartitionId(7));
    let b = create_entity_in_partition(&mut runtime, "b", PartitionId(11));
    let c = create_entity_in_partition(&mut runtime, "c", PartitionId(13));
    let _first_relation = create_relation_in_partition(&mut runtime, a, c, "ac", PartitionId(7));
    let _second_relation = create_relation_in_partition(&mut runtime, b, c, "bc", PartitionId(11));
    let snapshot = runtime.visibility_authority().snapshot();
    let context = runtime
        .read_truth()
        .query_plan_context(&snapshot)
        .expect("query plan context");

    let descending_packet = PlannedQueryPacket {
        label: "incoming-neighborhood-desc".to_string(),
        context_id: context.clone(),
        scope: QueryScope::IncomingNeighborhood {
            seeds: Arc::from([c, a, c]),
            relation_kind_scope: Some(Arc::from([KindId(2)])),
        },
        locality: QueryLocalityClass::CrossPartitionTraversal,
        ordering: QueryOrderingContract::CanonicalTraversalOrder,
        access_contract: QueryAccessContract::AuthoritativeStorageOnly,
        execution_shape: QueryExecutionShape::BulkPacketized,
        reduction: ReductionDiscipline::DeterministicMerge,
        plan_key: DeterministicQueryPlanKey(906),
        target_count_hint: 3,
    };
    let ascending_packet = PlannedQueryPacket {
        label: "incoming-neighborhood-asc".to_string(),
        context_id: context,
        scope: QueryScope::IncomingNeighborhood {
            seeds: Arc::from([a, c]),
            relation_kind_scope: Some(Arc::from([KindId(2)])),
        },
        locality: QueryLocalityClass::CrossPartitionTraversal,
        ordering: QueryOrderingContract::CanonicalTraversalOrder,
        access_contract: QueryAccessContract::AuthoritativeStorageOnly,
        execution_shape: QueryExecutionShape::BulkPacketized,
        reduction: ReductionDiscipline::DeterministicMerge,
        plan_key: DeterministicQueryPlanKey(907),
        target_count_hint: 2,
    };

    let descending = runtime
        .read_truth()
        .execute_query_plan(
            runtime
                .read_truth()
                .plan_query_packet(&snapshot, descending_packet)
                .expect("descending plan"),
        )
        .expect("descending outcome");
    let ascending = runtime
        .read_truth()
        .execute_query_plan(
            runtime
                .read_truth()
                .plan_query_packet(&snapshot, ascending_packet)
                .expect("ascending plan"),
        )
        .expect("ascending outcome");

    assert_eq!(descending.result.entities, ascending.result.entities);
    assert_eq!(descending.result.relations, ascending.result.relations);
    assert_eq!(
        descending.result.reduction_digest,
        ascending.result.reduction_digest
    );
}

#[test]
fn planned_query_execution_parallelizes_profitable_multi_seed_traversal_packets() {
    let mut runtime = RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::CertificationCore)
        .schema_registry(declared_aspect_schema_registry(
            CascadeDeletePolicy::CascadeDeleteRelations,
        ))
        .execution_model(
            crate::facade::runtime::RelationalExecutionModel::StagedParallelPreparation,
        )
        .build();
    let seeds = vec![
        create_entity_in_partition(&mut runtime, "s0", PartitionId(7)),
        create_entity_in_partition(&mut runtime, "s1", PartitionId(11)),
        create_entity_in_partition(&mut runtime, "s2", PartitionId(13)),
        create_entity_in_partition(&mut runtime, "s3", PartitionId(17)),
        create_entity_in_partition(&mut runtime, "s4", PartitionId(19)),
    ];
    let neighbors = vec![
        create_entity_in_partition(&mut runtime, "n0", PartitionId(23)),
        create_entity_in_partition(&mut runtime, "n1", PartitionId(29)),
        create_entity_in_partition(&mut runtime, "n2", PartitionId(31)),
        create_entity_in_partition(&mut runtime, "n3", PartitionId(37)),
        create_entity_in_partition(&mut runtime, "n4", PartitionId(41)),
    ];
    let relations = seeds
        .iter()
        .zip(neighbors.iter())
        .enumerate()
        .map(|(index, (seed, neighbor))| {
            create_relation_in_partition(
                &mut runtime,
                *seed,
                *neighbor,
                &format!("edge-{index}"),
                PartitionId(43 + index as u32),
            )
        })
        .collect::<Vec<_>>();
    let snapshot = runtime.visibility_authority().snapshot();
    let context = runtime
        .read_truth()
        .query_plan_context(&snapshot)
        .expect("query plan context");
    let packet = PlannedQueryPacket {
        label: "parallel-traversal".to_string(),
        context_id: context,
        scope: QueryScope::OutgoingNeighborhood {
            seeds: Arc::from(seeds.clone()),
            relation_kind_scope: Some(Arc::from([KindId(2)])),
        },
        locality: QueryLocalityClass::CrossPartitionTraversal,
        ordering: QueryOrderingContract::CanonicalTraversalOrder,
        access_contract: QueryAccessContract::AuthoritativeStorageOnly,
        execution_shape: QueryExecutionShape::BulkPacketized,
        reduction: ReductionDiscipline::DeterministicMerge,
        plan_key: DeterministicQueryPlanKey(1001),
        target_count_hint: seeds.len(),
    };

    runtime.performance_access().reset_counters();
    let plan = runtime
        .read_truth()
        .plan_query_packet(&snapshot, packet)
        .expect("planned query packet");
    let outcome = runtime
        .read_truth()
        .execute_query_plan(plan)
        .expect("query execution outcome");
    let counters = runtime.performance_access().counters();
    let expected_packet_count = 2;

    assert_eq!(outcome.complexity.packet_count, expected_packet_count);
    assert_eq!(outcome.complexity.fragment_count, expected_packet_count);
    assert_eq!(counters.query_packet_count, expected_packet_count);
    assert_eq!(counters.query_packet_item_count, seeds.len());
    assert_eq!(counters.query_packet_peak_width_total, 4);
    assert_eq!(counters.query_parallel_legal_count, 1);
    assert_eq!(counters.query_parallel_profitable_count, 1);
    assert_eq!(counters.query_staged_parallel_strategy_count, 1);
    assert_eq!(counters.query_serial_strategy_count, 0);
    assert_eq!(
        outcome
            .result
            .entities
            .iter()
            .map(|record| record.entity_id)
            .collect::<Vec<_>>(),
        seeds
            .iter()
            .copied()
            .chain(neighbors.iter().copied())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        outcome
            .result
            .relations
            .iter()
            .map(|record| record.relation_id)
            .collect::<Vec<_>>(),
        relations
    );
}

#[test]
fn planned_query_execution_parallelized_traversal_matches_serial_reference() {
    fn build_runtime(
        execution_model: crate::facade::runtime::RelationalExecutionModel,
    ) -> RelationalRuntime {
        RelationalRuntimeApi::builder()
            .profile(RelationalRuntimeProfile::CertificationCore)
            .schema_registry(declared_aspect_schema_registry(
                CascadeDeletePolicy::CascadeDeleteRelations,
            ))
            .execution_model(execution_model)
            .build()
    }

    fn build_fixture(runtime: &mut RelationalRuntime) -> (SnapshotHandle, PlannedQueryPacket) {
        let seeds = vec![
            create_entity_in_partition(runtime, "s0", PartitionId(7)),
            create_entity_in_partition(runtime, "s1", PartitionId(11)),
            create_entity_in_partition(runtime, "s2", PartitionId(13)),
            create_entity_in_partition(runtime, "s3", PartitionId(17)),
            create_entity_in_partition(runtime, "s4", PartitionId(19)),
        ];
        let neighbors = vec![
            create_entity_in_partition(runtime, "n0", PartitionId(23)),
            create_entity_in_partition(runtime, "n1", PartitionId(29)),
            create_entity_in_partition(runtime, "n2", PartitionId(31)),
            create_entity_in_partition(runtime, "n3", PartitionId(37)),
            create_entity_in_partition(runtime, "n4", PartitionId(41)),
        ];
        for (index, (seed, neighbor)) in seeds.iter().zip(neighbors.iter()).enumerate() {
            create_relation_in_partition(
                runtime,
                *seed,
                *neighbor,
                &format!("edge-{index}"),
                PartitionId(43 + index as u32),
            );
        }
        let snapshot = runtime.visibility_authority().snapshot();
        let context = runtime
            .read_truth()
            .query_plan_context(&snapshot)
            .expect("query plan context");
        let packet = PlannedQueryPacket {
            label: "parallel-traversal-parity".to_string(),
            context_id: context,
            scope: QueryScope::OutgoingNeighborhood {
                seeds: Arc::from(seeds),
                relation_kind_scope: Some(Arc::from([KindId(2)])),
            },
            locality: QueryLocalityClass::CrossPartitionTraversal,
            ordering: QueryOrderingContract::CanonicalTraversalOrder,
            access_contract: QueryAccessContract::AuthoritativeStorageOnly,
            execution_shape: QueryExecutionShape::BulkPacketized,
            reduction: ReductionDiscipline::DeterministicMerge,
            plan_key: DeterministicQueryPlanKey(1002),
            target_count_hint: 5,
        };
        (snapshot, packet)
    }

    let mut serial_runtime =
        build_runtime(crate::facade::runtime::RelationalExecutionModel::SerialAuthority);
    let (serial_snapshot, serial_packet) = build_fixture(&mut serial_runtime);
    let serial = serial_runtime
        .read_truth()
        .execute_query_plan(
            serial_runtime
                .read_truth()
                .plan_query_packet(&serial_snapshot, serial_packet)
                .expect("serial query plan"),
        )
        .expect("serial execution");

    let mut staged_runtime =
        build_runtime(crate::facade::runtime::RelationalExecutionModel::StagedParallelPreparation);
    let (staged_snapshot, staged_packet) = build_fixture(&mut staged_runtime);
    let staged = staged_runtime
        .read_truth()
        .execute_query_plan(
            staged_runtime
                .read_truth()
                .plan_query_packet(&staged_snapshot, staged_packet)
                .expect("staged query plan"),
        )
        .expect("staged execution");

    assert_eq!(serial.result, staged.result);
    assert_eq!(
        serial.complexity.target_count,
        staged.complexity.target_count
    );
    assert_eq!(
        serial
            .result
            .entities
            .iter()
            .map(|record| read_entity_name(record).unwrap().to_string())
            .collect::<Vec<_>>(),
        staged
            .result
            .entities
            .iter()
            .map(|record| read_entity_name(record).unwrap().to_string())
            .collect::<Vec<_>>()
    );
}

#[test]
fn planned_query_execution_reports_workload_derived_scratch_reuse_consistently_across_execution_models(
) {
    fn build_runtime(
        execution_model: crate::facade::runtime::RelationalExecutionModel,
    ) -> RelationalRuntime {
        RelationalRuntimeApi::builder()
            .profile(RelationalRuntimeProfile::CertificationCore)
            .schema_registry(declared_aspect_schema_registry(
                CascadeDeletePolicy::CascadeDeleteRelations,
            ))
            .execution_model(execution_model)
            .build()
    }

    fn build_fixture(runtime: &mut RelationalRuntime) -> (SnapshotHandle, PlannedQueryPacket) {
        let seeds = vec![
            create_entity_in_partition(runtime, "s0", PartitionId(7)),
            create_entity_in_partition(runtime, "s1", PartitionId(11)),
            create_entity_in_partition(runtime, "s2", PartitionId(13)),
            create_entity_in_partition(runtime, "s3", PartitionId(17)),
        ];
        let neighbors = vec![
            create_entity_in_partition(runtime, "n0", PartitionId(19)),
            create_entity_in_partition(runtime, "n1", PartitionId(23)),
            create_entity_in_partition(runtime, "n2", PartitionId(29)),
            create_entity_in_partition(runtime, "n3", PartitionId(31)),
        ];
        for (index, (seed, neighbor)) in seeds.iter().zip(neighbors.iter()).enumerate() {
            create_relation_in_partition(
                runtime,
                *seed,
                *neighbor,
                &format!("edge-{index}"),
                PartitionId(41 + index as u32),
            );
        }
        let snapshot = runtime.visibility_authority().snapshot();
        let context = runtime
            .read_truth()
            .query_plan_context(&snapshot)
            .expect("query plan context");
        (
            snapshot,
            PlannedQueryPacket {
                label: "scratch-reuse-parity".to_string(),
                context_id: context,
                scope: QueryScope::OutgoingNeighborhood {
                    seeds: Arc::from(seeds),
                    relation_kind_scope: Some(Arc::from([KindId(2)])),
                },
                locality: QueryLocalityClass::CrossPartitionTraversal,
                ordering: QueryOrderingContract::CanonicalTraversalOrder,
                access_contract: QueryAccessContract::AuthoritativeStorageOnly,
                execution_shape: QueryExecutionShape::BulkPacketized,
                reduction: ReductionDiscipline::DeterministicMerge,
                plan_key: DeterministicQueryPlanKey(1017),
                target_count_hint: 4,
            },
        )
    }

    let mut serial_runtime =
        build_runtime(crate::facade::runtime::RelationalExecutionModel::SerialAuthority);
    let (serial_snapshot, serial_packet) = build_fixture(&mut serial_runtime);
    serial_runtime.performance_access().reset_counters();
    let serial = serial_runtime
        .read_truth()
        .execute_query_plan(
            serial_runtime
                .read_truth()
                .plan_query_packet(&serial_snapshot, serial_packet)
                .expect("serial query plan"),
        )
        .expect("serial execution");
    let serial_counters = serial_runtime.performance_access().counters();

    let mut staged_runtime =
        build_runtime(crate::facade::runtime::RelationalExecutionModel::StagedParallelPreparation);
    let (staged_snapshot, staged_packet) = build_fixture(&mut staged_runtime);
    staged_runtime.performance_access().reset_counters();
    let staged = staged_runtime
        .read_truth()
        .execute_query_plan(
            staged_runtime
                .read_truth()
                .plan_query_packet(&staged_snapshot, staged_packet)
                .expect("staged query plan"),
        )
        .expect("staged execution");
    let staged_counters = staged_runtime.performance_access().counters();

    assert_eq!(serial.result, staged.result);
    assert_eq!(
        serial_counters.query_fragment_scratch_reuse_count,
        staged_counters.query_fragment_scratch_reuse_count
    );
}

#[test]
fn planned_query_execution_explicit_targets_do_not_claim_fragment_scratch_reuse() {
    let mut runtime = runtime_with_test_schema_execution_model(
        crate::facade::runtime::RelationalExecutionModel::SerialAuthority,
    );
    let left = create_entity_in_partition(&mut runtime, "left", PartitionId(7));
    let right = create_entity_in_partition(&mut runtime, "right", PartitionId(11));
    let snapshot = runtime.visibility_authority().snapshot();

    runtime.performance_access().reset_counters();
    let outcome = execute_explicit_query(
        &runtime,
        &snapshot,
        "explicit-targets",
        vec![
            crate::facade::transactions::RecordRef::Entity(left),
            crate::facade::transactions::RecordRef::Entity(right),
        ],
    );
    let counters = runtime.performance_access().counters();

    assert_eq!(outcome.complexity.packet_count, 2);
    assert_eq!(counters.query_fragment_scratch_reuse_count, 0);
}

#[test]
fn planned_query_execution_parallelized_overlapping_seed_traversal_dedupes_and_matches_serial() {
    fn build_runtime(
        execution_model: crate::facade::runtime::RelationalExecutionModel,
    ) -> RelationalRuntime {
        RelationalRuntimeApi::builder()
            .profile(RelationalRuntimeProfile::CertificationCore)
            .schema_registry(declared_aspect_schema_registry(
                CascadeDeletePolicy::CascadeDeleteRelations,
            ))
            .execution_model(execution_model)
            .build()
    }

    fn build_fixture(runtime: &mut RelationalRuntime) -> (SnapshotHandle, PlannedQueryPacket) {
        let seed_a = create_entity_in_partition(runtime, "seed-a", PartitionId(7));
        let seed_b = create_entity_in_partition(runtime, "seed-b", PartitionId(11));
        let shared = create_entity_in_partition(runtime, "shared", PartitionId(13));
        let tail = create_entity_in_partition(runtime, "tail", PartitionId(17));
        create_relation_in_partition(runtime, seed_a, shared, "a-shared", PartitionId(23));
        create_relation_in_partition(runtime, seed_b, shared, "b-shared", PartitionId(29));
        create_relation_in_partition(runtime, shared, tail, "shared-tail", PartitionId(31));
        let snapshot = runtime.visibility_authority().snapshot();
        let context = runtime
            .read_truth()
            .query_plan_context(&snapshot)
            .expect("query plan context");
        (
            snapshot,
            PlannedQueryPacket {
                label: "overlap-traversal".to_string(),
                context_id: context,
                scope: QueryScope::ConnectivityTraversal {
                    seeds: Arc::from([seed_a, seed_b]),
                    relation_kind_scope: Some(Arc::from([KindId(2)])),
                    max_depth: Some(2),
                },
                locality: QueryLocalityClass::CrossPartitionTraversal,
                ordering: QueryOrderingContract::CanonicalTraversalOrder,
                access_contract: QueryAccessContract::AuthoritativeStorageOnly,
                execution_shape: QueryExecutionShape::BulkPacketized,
                reduction: ReductionDiscipline::DeterministicMerge,
                plan_key: DeterministicQueryPlanKey(1003),
                target_count_hint: 2,
            },
        )
    }

    let mut serial_runtime =
        build_runtime(crate::facade::runtime::RelationalExecutionModel::SerialAuthority);
    let (serial_snapshot, serial_packet) = build_fixture(&mut serial_runtime);
    let serial = serial_runtime
        .read_truth()
        .execute_query_plan(
            serial_runtime
                .read_truth()
                .plan_query_packet(&serial_snapshot, serial_packet)
                .expect("serial plan"),
        )
        .expect("serial outcome");

    let mut staged_runtime =
        build_runtime(crate::facade::runtime::RelationalExecutionModel::StagedParallelPreparation);
    let (staged_snapshot, staged_packet) = build_fixture(&mut staged_runtime);
    let staged = staged_runtime
        .read_truth()
        .execute_query_plan(
            staged_runtime
                .read_truth()
                .plan_query_packet(&staged_snapshot, staged_packet)
                .expect("staged plan"),
        )
        .expect("staged outcome");

    assert_eq!(serial.result, staged.result);
    assert_eq!(
        staged
            .result
            .entities
            .iter()
            .map(|record| record.entity_id)
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        staged.result.entities.len()
    );
    assert_eq!(
        staged
            .result
            .relations
            .iter()
            .map(|record| record.relation_id)
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        staged.result.relations.len()
    );
}
