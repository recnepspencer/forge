use crate::tests::support::*;

#[test]
fn query_planning_context_binds_snapshot_runtime_and_schema_identity() {
    let mut runtime = runtime_with_test_schema();
    let committed = create_entity_outcome(&mut runtime, "first");

    let context = runtime
        .visibility_reads()
        .query_plan_context(&committed.snapshot)
        .expect("query plan context");

    assert_eq!(context.runtime_instance_id, runtime.runtime_instance_id());
    assert_eq!(context.snapshot_id, committed.snapshot.snapshot_id);
    assert_eq!(context.version_id, committed.snapshot.version_id);
    assert_eq!(context.schema_version, committed.envelope().schema_version);
    assert_eq!(
        context.descriptor_semantics_version,
        committed.envelope().descriptor_semantics_version
    );
    assert_eq!(
        context.evidence_basis,
        QueryPlanEvidenceBasis::CanonicalCommitEnvelope {
            commit_id: committed.commit.commit_id,
        }
    );
}

#[test]
fn legacy_query_packet_planning_marks_single_target_packets_serial_preferred() {
    let mut runtime = runtime_with_test_schema();
    let created = create_entity_outcome(&mut runtime, "single");
    let entity = changed_entities(&created)[0];
    let packet = QueryWorkPacket::bulk("single-target", vec![RecordRef::Entity(entity)]);

    let plan = runtime
        .visibility_reads()
        .plan_legacy_query_packet(&created.snapshot, packet)
        .expect("planned legacy packet");

    assert_eq!(plan.legality, QueryParallelLegality::LegalReadOnlySnapshot);
    assert_eq!(
        plan.profitability,
        QueryParallelProfitability::SerialPreferred {
            reason: QuerySerialReason::TinyPacket,
        }
    );
}

#[test]
fn legacy_query_packet_planning_marks_single_chunk_packets_serial_preferred() {
    let mut runtime = runtime_with_test_schema();
    let first = create_entity_outcome(&mut runtime, "first");
    let second = create_entity_outcome(&mut runtime, "second");
    let targets = vec![
        RecordRef::Entity(changed_entities(&first)[0]),
        RecordRef::Entity(changed_entities(&second)[0]),
    ];

    let plan = runtime
        .visibility_reads()
        .plan_legacy_query_packet(&second.snapshot, QueryWorkPacket::bulk("pair", targets))
        .expect("planned packet");

    assert_eq!(plan.legality, QueryParallelLegality::LegalReadOnlySnapshot);
    assert_eq!(
        plan.profitability,
        QueryParallelProfitability::SerialPreferred {
            reason: QuerySerialReason::SingleChunkSurface,
        }
    );
}

#[test]
fn traversal_query_packets_require_serial_reduction_admission() {
    let mut runtime = runtime_with_test_schema();
    let created = create_entity_outcome(&mut runtime, "seed");
    let seed = changed_entities(&created)[0];
    let context = runtime
        .visibility_reads()
        .query_plan_context(&created.snapshot)
        .expect("query plan context");

    let packet = PlannedQueryPacket {
        label: "connectivity".to_string(),
        context_id: context,
        scope: QueryScope::ConnectivityTraversal {
            seeds: std::sync::Arc::from([seed]),
            relation_kind_scope: None,
            max_depth: Some(2),
        },
        locality: QueryLocalityClass::CrossPartitionTraversal,
        ordering: QueryOrderingContract::CanonicalTraversalOrder,
        fallback: QueryFallbackContract::StorageOnly,
        execution_shape: QueryExecutionShape::BulkPacketized,
        reduction: ReductionDiscipline::DeterministicMerge,
        plan_key: DeterministicQueryPlanKey(17),
        target_count_hint: 1,
    };

    let plan = runtime
        .visibility_reads()
        .plan_query_packet(&created.snapshot, packet)
        .expect("snapshot pinned plan");

    assert_eq!(
        plan.legality,
        QueryParallelLegality::RequiresSerialReduction
    );
    assert_eq!(
        plan.profitability,
        QueryParallelProfitability::SerialPreferred {
            reason: QuerySerialReason::TinyPacket,
        }
    );
}

#[test]
fn query_planning_rejects_packets_with_forged_context() {
    let mut runtime = runtime_with_test_schema();
    let created = create_entity_outcome(&mut runtime, "forged");
    let entity = changed_entities(&created)[0];
    let mut context = runtime
        .visibility_reads()
        .query_plan_context(&created.snapshot)
        .expect("query plan context");
    context.version_id = crate::facade::identity::VersionId(context.version_id.0 + 1);

    let packet = PlannedQueryPacket {
        label: "forged-context".to_string(),
        context_id: context,
        scope: QueryScope::ExplicitTargets {
            targets: std::sync::Arc::from([RecordRef::Entity(entity)]),
        },
        locality: QueryLocalityClass::CrossPartitionTraversal,
        ordering: QueryOrderingContract::CanonicalRecordRefOrder,
        fallback: QueryFallbackContract::StorageOnly,
        execution_shape: QueryExecutionShape::BulkPacketized,
        reduction: ReductionDiscipline::DeterministicMerge,
        plan_key: DeterministicQueryPlanKey(99),
        target_count_hint: 1,
    };

    assert!(
        runtime
            .visibility_reads()
            .plan_query_packet(&created.snapshot, packet)
            .is_none()
    );
}

#[test]
fn legacy_packet_plan_key_is_deterministic_for_identical_inputs() {
    let mut runtime = runtime_with_test_schema();
    let created = create_entity_outcome(&mut runtime, "plan-key");
    let entity = changed_entities(&created)[0];

    let first = runtime
        .visibility_reads()
        .plan_legacy_query_packet(
            &created.snapshot,
            QueryWorkPacket::bulk("stable", vec![RecordRef::Entity(entity)]),
        )
        .expect("first plan");
    let second = runtime
        .visibility_reads()
        .plan_legacy_query_packet(
            &created.snapshot,
            QueryWorkPacket::bulk("stable", vec![RecordRef::Entity(entity)]),
        )
        .expect("second plan");

    assert_ne!(first.packet.plan_key, DeterministicQueryPlanKey(0));
    assert_eq!(first.packet.plan_key, second.packet.plan_key);
}

#[test]
fn empty_runtime_query_planning_uses_explicit_genesis_basis() {
    let mut runtime = runtime_with_test_schema();
    let snapshot = runtime.visibility_authority().snapshot();

    let context = runtime
        .visibility_reads()
        .query_plan_context(&snapshot)
        .expect("genesis query plan context");

    assert_eq!(context.version_id, crate::facade::identity::VersionId(0));
    assert_eq!(
        context.evidence_basis,
        QueryPlanEvidenceBasis::GenesisRuntimeBootstrap
    );
}
