use crate::tests::support::*;
use crate::history::data::{AspectFilter, AspectFilterMode, RequestedAspectSet};
use std::sync::Arc;

#[test]
fn planned_query_execution_reduces_explicit_targets_into_canonical_entity_order() {
    let mut runtime = runtime_with_test_schema();
    let first = create_entity_outcome(&mut runtime, "first");
    let second = create_entity_outcome(&mut runtime, "second");
    let first_id = changed_entities(&first)[0];
    let second_id = changed_entities(&second)[0];

    let plan = runtime
        .visibility_reads()
        .plan_legacy_query_packet(
            &second.snapshot,
            QueryWorkPacket::bulk(
                "reverse-order",
                vec![RecordRef::Entity(second_id), RecordRef::Entity(first_id)],
            ),
        )
        .expect("query plan");
    let outcome = runtime
        .visibility_reads()
        .execute_query_plan(plan)
        .expect("query execution outcome");

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
    assert_eq!(outcome.complexity.entity_records_emitted, 2);
    assert_eq!(outcome.complexity.relation_records_emitted, 0);
}

#[test]
fn planned_query_execution_is_deterministic_for_identical_inputs() {
    let mut runtime = runtime_with_test_schema();
    let first = create_entity_outcome(&mut runtime, "first");
    let second = create_entity_outcome(&mut runtime, "second");
    let first_id = changed_entities(&first)[0];
    let second_id = changed_entities(&second)[0];
    let packet = QueryWorkPacket::bulk(
        "stable-execution",
        vec![RecordRef::Entity(second_id), RecordRef::Entity(first_id)],
    );

    let first_outcome = runtime
        .visibility_reads()
        .execute_query_plan(
            runtime
                .visibility_reads()
                .plan_legacy_query_packet(&second.snapshot, packet.clone())
                .expect("first query plan"),
        )
        .expect("first execution");
    let second_outcome = runtime
        .visibility_reads()
        .execute_query_plan(
            runtime
                .visibility_reads()
                .plan_legacy_query_packet(&second.snapshot, packet)
                .expect("second query plan"),
        )
        .expect("second execution");

    assert_eq!(first_outcome.result, second_outcome.result);
    assert_eq!(first_outcome.complexity, second_outcome.complexity);
}

#[test]
fn planned_query_execution_uses_staged_parallel_packets_for_profitable_cross_partition_reads() {
    let mut runtime = runtime_with_test_schema_execution_model(
        crate::facade::runtime::RelationalExecutionModel::StagedParallelPreparation,
    );
    let targets = vec![
        RecordRef::Entity(create_entity_in_partition(&mut runtime, "a-1", PartitionId(7))),
        RecordRef::Entity(create_entity_in_partition(&mut runtime, "b-1", PartitionId(11))),
        RecordRef::Entity(create_entity_in_partition(&mut runtime, "a-2", PartitionId(7))),
        RecordRef::Entity(create_entity_in_partition(&mut runtime, "b-2", PartitionId(11))),
        RecordRef::Entity(create_entity_in_partition(&mut runtime, "a-3", PartitionId(7))),
        RecordRef::Entity(create_entity_in_partition(&mut runtime, "b-3", PartitionId(11))),
    ];
    let snapshot = runtime.visibility_authority().snapshot();
    runtime.performance_access().reset_counters();

    let plan = runtime
        .visibility_reads()
        .plan_legacy_query_packet(&snapshot, QueryWorkPacket::bulk("parallel-query", targets))
        .expect("query plan");
    let outcome = runtime
        .visibility_reads()
        .execute_query_plan(plan)
        .expect("query execution outcome");
    let counters = runtime.performance_access().counters();

    assert_eq!(outcome.complexity.fragment_count, 2);
    assert_eq!(outcome.complexity.touched_partitions, 2);
    assert_eq!(counters.query_packet_count, 2);
    assert_eq!(counters.query_packet_item_count, 6);
    assert_eq!(counters.query_parallel_legal_count, 1);
    assert_eq!(counters.query_parallel_profitable_count, 1);
    assert_eq!(counters.query_staged_parallel_strategy_count, 1);
    assert_eq!(counters.query_serial_strategy_count, 0);
    assert_eq!(counters.query_entity_records_emitted, 6);
    assert_eq!(counters.query_relation_records_emitted, 0);
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
        .visibility_reads()
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
        fallback: QueryFallbackContract::StorageOnly,
        execution_shape: QueryExecutionShape::BulkPacketized,
        reduction: ReductionDiscipline::DeterministicMerge,
        plan_key: DeterministicQueryPlanKey(501),
        target_count_hint: 0,
    };

    let plan = runtime
        .visibility_reads()
        .plan_query_packet(&snapshot, packet)
        .expect("planned query packet");
    let outcome = runtime
        .visibility_reads()
        .execute_query_plan(plan)
        .expect("query execution outcome");

    assert_eq!(outcome.complexity.packet_count, 2);
    assert_eq!(outcome.complexity.fragment_count, 2);
    assert_eq!(outcome.complexity.entity_records_emitted, 3);
    assert_eq!(outcome.result.entities.len(), 3);
    assert_eq!(outcome.result.entities[0].entity_id, left);
    assert_eq!(outcome.result.entities[2].entity_id, right);
}

#[test]
fn planned_query_execution_supports_relation_kind_scans_through_reducer_path() {
    let mut runtime =
        runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    let left = create_entity_in_partition(&mut runtime, "left", PartitionId(7));
    let right = create_entity_in_partition(&mut runtime, "right", PartitionId(11));
    let third = create_entity_in_partition(&mut runtime, "third", PartitionId(11));
    let first_relation = create_relation_in_partition(&mut runtime, left, right, "r1", PartitionId(7));
    let second_relation =
        create_relation_in_partition(&mut runtime, right, third, "r2", PartitionId(11));
    let snapshot = runtime.visibility_authority().snapshot();
    let context = runtime
        .visibility_reads()
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
        fallback: QueryFallbackContract::StorageOnly,
        execution_shape: QueryExecutionShape::BulkPacketized,
        reduction: ReductionDiscipline::DeterministicMerge,
        plan_key: DeterministicQueryPlanKey(777),
        target_count_hint: 0,
    };

    let plan = runtime
        .visibility_reads()
        .plan_query_packet(&snapshot, packet)
        .expect("planned query packet");
    let outcome = runtime
        .visibility_reads()
        .execute_query_plan(plan)
        .expect("query execution outcome");

    assert_eq!(outcome.complexity.packet_count, 2);
    assert_eq!(outcome.complexity.relation_records_emitted, 2);
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
        .visibility_reads()
        .query_plan_context(&snapshot)
        .expect("query plan context");
    let packet = PlannedQueryPacket {
        label: "aspect-filtered-entities".to_string(),
        context_id: context,
        scope: QueryScope::AspectFilteredEntities {
            kind_id: Some(KindId(1)),
            aspect_filter: AspectFilter {
                mode: AspectFilterMode::All,
                aspects: RequestedAspectSet::new([aspect_key("name"), aspect_key("lifecycle")]),
            },
            partition_scope: Some(Arc::from([PartitionId(7), PartitionId(11)])),
        },
        locality: QueryLocalityClass::PartitionBounded {
            partitions: Arc::from([PartitionId(7), PartitionId(11)]),
        },
        ordering: QueryOrderingContract::CanonicalEntityIdOrder,
        fallback: QueryFallbackContract::StorageOnly,
        execution_shape: QueryExecutionShape::BulkPacketized,
        reduction: ReductionDiscipline::DeterministicMerge,
        plan_key: DeterministicQueryPlanKey(901),
        target_count_hint: 0,
    };

    let plan = runtime
        .visibility_reads()
        .plan_query_packet(&snapshot, packet)
        .expect("planned query packet");
    let outcome = runtime
        .visibility_reads()
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
fn planned_query_execution_supports_aspect_filtered_relation_scans_through_reducer_path() {
    let mut runtime =
        runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    let left = create_entity_in_partition(&mut runtime, "left", PartitionId(7));
    let right = create_entity_in_partition(&mut runtime, "right", PartitionId(11));
    let third = create_entity_in_partition(&mut runtime, "third", PartitionId(11));
    let first_relation = create_relation_in_partition(&mut runtime, left, right, "r1", PartitionId(7));
    let second_relation =
        create_relation_in_partition(&mut runtime, right, third, "r2", PartitionId(11));
    let snapshot = runtime.visibility_authority().snapshot();
    let context = runtime
        .visibility_reads()
        .query_plan_context(&snapshot)
        .expect("query plan context");
    let packet = PlannedQueryPacket {
        label: "aspect-filtered-relations".to_string(),
        context_id: context,
        scope: QueryScope::AspectFilteredRelations {
            kind_id: Some(KindId(2)),
            aspect_filter: AspectFilter {
                mode: AspectFilterMode::All,
                aspects: RequestedAspectSet::new([
                    aspect_key("label"),
                    aspect_key("lifecycle"),
                    aspect_key("source"),
                    aspect_key("target"),
                ]),
            },
            partition_scope: Some(Arc::from([PartitionId(7), PartitionId(11)])),
        },
        locality: QueryLocalityClass::PartitionBounded {
            partitions: Arc::from([PartitionId(7), PartitionId(11)]),
        },
        ordering: QueryOrderingContract::CanonicalRelationIdOrder,
        fallback: QueryFallbackContract::StorageOnly,
        execution_shape: QueryExecutionShape::BulkPacketized,
        reduction: ReductionDiscipline::DeterministicMerge,
        plan_key: DeterministicQueryPlanKey(904),
        target_count_hint: 0,
    };

    let plan = runtime
        .visibility_reads()
        .plan_query_packet(&snapshot, packet)
        .expect("planned query packet");
    let outcome = runtime
        .visibility_reads()
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
        .visibility_reads()
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
        fallback: QueryFallbackContract::StorageOnly,
        execution_shape: QueryExecutionShape::BulkPacketized,
        reduction: ReductionDiscipline::DeterministicMerge,
        plan_key: DeterministicQueryPlanKey(902),
        target_count_hint: 1,
    };

    let plan = runtime
        .visibility_reads()
        .plan_query_packet(&snapshot, packet)
        .expect("planned query packet");
    let outcome = runtime
        .visibility_reads()
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
        .visibility_reads()
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
        fallback: QueryFallbackContract::StorageOnly,
        execution_shape: QueryExecutionShape::BulkPacketized,
        reduction: ReductionDiscipline::DeterministicMerge,
        plan_key: DeterministicQueryPlanKey(905),
        target_count_hint: 1,
    };

    let plan = runtime
        .visibility_reads()
        .plan_query_packet(&snapshot, packet)
        .expect("planned query packet");
    let outcome = runtime
        .visibility_reads()
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
        .visibility_reads()
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
        fallback: QueryFallbackContract::StorageOnly,
        execution_shape: QueryExecutionShape::BulkPacketized,
        reduction: ReductionDiscipline::DeterministicMerge,
        plan_key: DeterministicQueryPlanKey(903),
        target_count_hint: 1,
    };

    let plan = runtime
        .visibility_reads()
        .plan_query_packet(&snapshot, packet)
        .expect("planned query packet");
    let outcome = runtime
        .visibility_reads()
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
    assert_eq!(outcome.complexity.entity_records_emitted, 2);
    assert_eq!(outcome.complexity.relation_records_emitted, 1);
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
        .visibility_reads()
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
        fallback: QueryFallbackContract::StorageOnly,
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
        fallback: QueryFallbackContract::StorageOnly,
        execution_shape: QueryExecutionShape::BulkPacketized,
        reduction: ReductionDiscipline::DeterministicMerge,
        plan_key: DeterministicQueryPlanKey(907),
        target_count_hint: 2,
    };

    let descending = runtime
        .visibility_reads()
        .execute_query_plan(
            runtime
                .visibility_reads()
                .plan_query_packet(&snapshot, descending_packet)
                .expect("descending plan"),
        )
        .expect("descending outcome");
    let ascending = runtime
        .visibility_reads()
        .execute_query_plan(
            runtime
                .visibility_reads()
                .plan_query_packet(&snapshot, ascending_packet)
                .expect("ascending plan"),
        )
        .expect("ascending outcome");

    assert_eq!(descending.result.entities, ascending.result.entities);
    assert_eq!(descending.result.relations, ascending.result.relations);
    assert_eq!(descending.result.reduction_digest, ascending.result.reduction_digest);
}
