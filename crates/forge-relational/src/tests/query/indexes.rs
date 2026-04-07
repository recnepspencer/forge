use crate::facade::history::BranchId;
use crate::facade::indexes::{
    DerivedIndexBuildRequest, DerivedIndexDefinition, DerivedIndexId, DerivedIndexKind,
};
use crate::facade::query::{
    FallbackParityMode, IndexQueryRejectionClass, QueryAccessPath, QueryFallbackContract,
};
use crate::facade::runtime::RelationalExecutionModel;
use crate::facade::transactions::RecordRef;
use crate::tests::support::*;
use std::sync::Arc;

fn sampled_plan_key_for(
    generation_id: crate::facade::indexes::DerivedIndexGenerationId,
    version_id: crate::facade::identity::VersionId,
    should_sample: bool,
) -> crate::facade::query::DeterministicQueryPlanKey {
    for key in 1u128..512 {
        let sample_key = key ^ ((generation_id.0 as u128) << 64) ^ (version_id.0 as u128);
        let sampled = sample_key % 8 == 0;
        if sampled == should_sample {
            return crate::facade::query::DeterministicQueryPlanKey(key);
        }
    }
    panic!("unable to derive deterministic sampled plan key");
}

// CONTRACT: derived_index
// LANES: success, fallback, determinism

#[test]
fn derived_index_contract_success_branch_scoped_build_keeps_storage_fallback() {
    let mut runtime = runtime_with_test_schema();
    let main_outcome = create_entity_outcome(&mut runtime, "main-a");
    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let feature_outcome =
        create_entity_outcome_on_branch(&mut runtime, "feature-a", BranchId("feature".to_string()));
    let index = runtime.index_authority().register(DerivedIndexDefinition {
        index_id: DerivedIndexId(0),
        name: "entity.name".to_string(),
        kind: DerivedIndexKind::EntityPayloadField {
            field: "name".to_string(),
        },
        branch_scoped: true,
    });
    let feature_build = runtime
        .index_authority()
        .build_for_commit(DerivedIndexBuildRequest {
            source_commit_id: feature_outcome.commit.commit_id,
            branch_id: BranchId("feature".to_string()),
            index_ids: vec![index.index_id],
        });
    let context = runtime
        .read_truth()
        .query_plan_context(&main_outcome.snapshot)
        .expect("query plan context");
    let mut planned = crate::facade::query::PlannedQueryPacket::explicit_targets(
        "entities",
        context,
        vec![RecordRef::Entity(changed_entities(&main_outcome)[0])],
    );
    planned.fallback = QueryFallbackContract::IndexAdmissibleStorageEquivalent;
    let plan = runtime
        .read_truth()
        .plan_query_packet(&main_outcome.snapshot, planned)
        .expect("query plan");
    let fallback = runtime
        .index_access()
        .execute_query_plan_with_fallback_parity(plan, FallbackParityMode::ProductionAdmissibility)
        .unwrap();

    assert!(feature_build.failed_indexes.is_empty());
    assert_eq!(
        runtime
            .index_access()
            .latest_generation(index.index_id, &BranchId("feature".to_string()))
            .unwrap()
            .source_branch_id,
        BranchId("feature".to_string())
    );
    assert_eq!(fallback.execution.result.entities.len(), 1);
    assert_eq!(
        fallback.access_path,
        QueryAccessPath::DerivedIndexRejectedStorageFallback {
            rejection: IndexQueryRejectionClass::MissingGeneration,
        }
    );
}

#[test]
fn derived_index_contract_unscoped_generation_is_rejected_for_unsupported_scope() {
    let mut runtime = runtime_with_test_schema();
    let main_outcome = create_entity_outcome(&mut runtime, "main-a");
    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let feature_outcome =
        create_entity_outcome_on_branch(&mut runtime, "feature-a", BranchId("feature".to_string()));
    let index = runtime.index_authority().register(DerivedIndexDefinition {
        index_id: DerivedIndexId(0),
        name: "entity.name.global".to_string(),
        kind: DerivedIndexKind::EntityPayloadField {
            field: "name".to_string(),
        },
        branch_scoped: false,
    });
    let build = runtime
        .index_authority()
        .build_for_commit(DerivedIndexBuildRequest {
            source_commit_id: feature_outcome.commit.commit_id,
            branch_id: BranchId("feature".to_string()),
            index_ids: vec![index.index_id],
        });
    let snapshot = runtime.visibility_authority().snapshot();
    let context = runtime
        .read_truth()
        .query_plan_context(&snapshot)
        .expect("query plan context");
    let mut planned = crate::facade::query::PlannedQueryPacket::explicit_targets(
        "entities",
        context,
        vec![RecordRef::Entity(changed_entities(&main_outcome)[0])],
    );
    planned.fallback = QueryFallbackContract::IndexAdmissibleStorageEquivalent;
    let plan = runtime
        .read_truth()
        .plan_query_packet(&snapshot, planned)
        .expect("query plan");
    let fallback = runtime
        .index_access()
        .execute_query_plan_with_fallback_parity(plan, FallbackParityMode::ProductionAdmissibility)
        .unwrap();

    assert!(build.failed_indexes.is_empty());
    assert_eq!(
        runtime
            .index_access()
            .latest_generation(index.index_id, &BranchId("main".to_string()))
            .unwrap()
            .source_branch_id,
        BranchId("feature".to_string())
    );
    assert_eq!(
        fallback.access_path,
        QueryAccessPath::DerivedIndexRejectedStorageFallback {
            rejection: IndexQueryRejectionClass::UnsupportedScope,
        }
    );
}

#[test]
fn derived_index_contract_failure_unknown_index_keeps_truth_reads_correct() {
    let mut runtime = runtime_with_test_schema();
    let outcome = create_entity_outcome(&mut runtime, "main-a");
    let snapshot = runtime.visibility_authority().snapshot();
    let storage_only = execute_explicit_query(
        &runtime,
        &snapshot,
        "entities",
        vec![RecordRef::Entity(changed_entities(&outcome)[0])],
    )
    .result;
    let fallback_before = runtime
        .index_access()
        .execute_query_plan_with_fallback_parity(
            planned_explicit_query(
                &runtime,
                &snapshot,
                "entities",
                vec![RecordRef::Entity(changed_entities(&outcome)[0])],
            ),
            FallbackParityMode::ProductionAdmissibility,
        )
        .unwrap();
    let build = runtime
        .index_authority()
        .build_for_commit(DerivedIndexBuildRequest {
            source_commit_id: outcome.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            index_ids: vec![DerivedIndexId(999)],
        });
    let fallback_after = runtime
        .index_access()
        .execute_query_plan_with_fallback_parity(
            planned_explicit_query(
                &runtime,
                &snapshot,
                "entities",
                vec![RecordRef::Entity(changed_entities(&outcome)[0])],
            ),
            FallbackParityMode::ProductionAdmissibility,
        )
        .unwrap();

    assert_eq!(build.failed_indexes, vec![DerivedIndexId(999)]);
    assert_eq!(
        fallback_before.access_path,
        QueryAccessPath::AuthoritativeStorage
    );
    assert_eq!(fallback_before.execution.result, storage_only);
    assert_eq!(fallback_after.execution.result, storage_only);
}

#[test]
fn derived_index_contract_certification_mode_emits_stable_parity_digest() {
    let mut runtime = runtime_with_test_schema();
    let outcome = create_entity_outcome(&mut runtime, "main-a");
    let index = runtime.index_authority().register(DerivedIndexDefinition {
        index_id: DerivedIndexId(0),
        name: "entity.name.global".to_string(),
        kind: DerivedIndexKind::EntityPayloadField {
            field: "name".to_string(),
        },
        branch_scoped: false,
    });
    let build = runtime
        .index_authority()
        .build_for_commit(DerivedIndexBuildRequest {
            source_commit_id: outcome.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            index_ids: vec![index.index_id],
        });
    assert!(build.failed_indexes.is_empty());

    let snapshot = runtime.visibility_authority().snapshot();
    let context = runtime
        .read_truth()
        .query_plan_context(&snapshot)
        .expect("query plan context");
    let mut planned = crate::facade::query::PlannedQueryPacket::explicit_targets(
        "entities",
        context,
        vec![RecordRef::Entity(changed_entities(&outcome)[0])],
    );
    planned.fallback = QueryFallbackContract::IndexAdmissibleStorageEquivalent;
    let plan = runtime
        .read_truth()
        .plan_query_packet(&snapshot, planned)
        .expect("query plan");

    let first = runtime
        .index_access()
        .execute_query_plan_with_fallback_parity(
            plan.clone(),
            FallbackParityMode::CertificationParity,
        )
        .expect("first parity outcome");
    let second = runtime
        .index_access()
        .execute_query_plan_with_fallback_parity(plan, FallbackParityMode::CertificationParity)
        .expect("second parity outcome");

    assert_eq!(first.access_path, second.access_path);
    assert_eq!(first.parity_basis_digest, second.parity_basis_digest);
}

#[test]
fn derived_index_contract_entity_field_equals_executes_through_real_index_path_with_storage_parity()
{
    let mut runtime = runtime_with_test_schema();
    let alpha = create_entity_outcome(&mut runtime, "alpha");
    let _beta = create_entity_outcome(&mut runtime, "beta");
    let index = runtime.index_authority().register(DerivedIndexDefinition {
        index_id: DerivedIndexId(0),
        name: "entity.name.lookup".to_string(),
        kind: DerivedIndexKind::EntityPayloadField {
            field: "name".to_string(),
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
    let packet = crate::facade::query::PlannedQueryPacket {
        label: "entity-name-equals".to_string(),
        context_id: context,
        scope: crate::facade::query::QueryScope::EntityPayloadFieldEquals {
            field: "name".to_string(),
            value: "alpha".to_string(),
            partition_scope: None,
        },
        locality: crate::facade::query::QueryLocalityClass::CrossPartitionTraversal,
        ordering: crate::facade::query::QueryOrderingContract::CanonicalEntityIdOrder,
        fallback: QueryFallbackContract::IndexAdmissibleStorageEquivalent,
        execution_shape: crate::facade::query::QueryExecutionShape::BulkPacketized,
        reduction: crate::facade::query::ReductionDiscipline::DeterministicMerge,
        plan_key: crate::facade::query::DeterministicQueryPlanKey(1001),
        target_count_hint: 0,
    };
    let plan = runtime
        .read_truth()
        .plan_query_packet(&snapshot, packet)
        .expect("query plan");
    let storage = runtime
        .read_truth()
        .execute_query_plan(plan.clone())
        .expect("storage outcome");
    let indexed = runtime
        .index_access()
        .execute_query_plan_with_fallback_parity(plan, FallbackParityMode::CertificationParity)
        .expect("indexed outcome");

    assert_eq!(
        indexed.access_path,
        QueryAccessPath::DerivedIndexGeneration {
            generation_id: build.generations[0].generation_id,
        }
    );
    assert_eq!(indexed.execution.result, storage.result);
    assert_eq!(indexed.execution.result.entities.len(), 1);
    assert_eq!(
        indexed.execution.result.entities[0].entity_id,
        changed_entities(&alpha)[0]
    );
}

#[test]
fn derived_index_contract_relation_field_equals_executes_through_real_index_path_with_storage_parity(
) {
    let mut runtime = runtime_with_test_schema();
    let source = create_entity_outcome(&mut runtime, "source");
    let source_id = changed_entities(&source)[0];
    let target = create_entity_outcome(&mut runtime, "target");
    let target_id = changed_entities(&target)[0];
    let relation = create_relation_outcome(&mut runtime, source_id, target_id, "edge");
    let index = runtime.index_authority().register(DerivedIndexDefinition {
        index_id: DerivedIndexId(1),
        name: "relation.label.lookup".to_string(),
        kind: DerivedIndexKind::RelationPayloadField {
            field: "label".to_string(),
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
    let packet = crate::facade::query::PlannedQueryPacket {
        label: "relation-label-equals".to_string(),
        context_id: context,
        scope: crate::facade::query::QueryScope::RelationPayloadFieldEquals {
            field: "label".to_string(),
            value: "edge".to_string(),
            partition_scope: None,
        },
        locality: crate::facade::query::QueryLocalityClass::CrossPartitionTraversal,
        ordering: crate::facade::query::QueryOrderingContract::CanonicalRelationIdOrder,
        fallback: QueryFallbackContract::IndexAdmissibleStorageEquivalent,
        execution_shape: crate::facade::query::QueryExecutionShape::BulkPacketized,
        reduction: crate::facade::query::ReductionDiscipline::DeterministicMerge,
        plan_key: crate::facade::query::DeterministicQueryPlanKey(1011),
        target_count_hint: 0,
    };
    let plan = runtime
        .read_truth()
        .plan_query_packet(&snapshot, packet)
        .expect("query plan");
    let storage = runtime
        .read_truth()
        .execute_query_plan(plan.clone())
        .expect("storage outcome");
    let indexed = runtime
        .index_access()
        .execute_query_plan_with_fallback_parity(plan, FallbackParityMode::CertificationParity)
        .expect("indexed outcome");

    assert_eq!(
        indexed.access_path,
        QueryAccessPath::DerivedIndexGeneration {
            generation_id: build.generations[0].generation_id,
        }
    );
    assert_eq!(indexed.execution.result, storage.result);
    assert_eq!(indexed.execution.result.relations.len(), 1);
    assert_eq!(
        indexed.execution.result.relations[0].relation_id,
        changed_relations(&relation)[0]
    );
}

#[test]
fn derived_index_contract_relation_field_equals_reports_corrupt_generation() {
    let mut runtime = runtime_with_test_schema();
    let source = create_entity_outcome(&mut runtime, "source");
    let target = create_entity_outcome(&mut runtime, "target");
    let relation = create_relation_outcome(
        &mut runtime,
        changed_entities(&source)[0],
        changed_entities(&target)[0],
        "edge",
    );
    let index = runtime.index_authority().register(DerivedIndexDefinition {
        index_id: DerivedIndexId(1),
        name: "relation.label.lookup".to_string(),
        kind: DerivedIndexKind::RelationPayloadField {
            field: "label".to_string(),
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
    runtime
        .indexes
        .generations
        .get_mut(&index.index_id)
        .expect("relation index generations")
        .last_mut()
        .expect("built relation generation")
        .status = crate::facade::indexes::DerivedIndexPublicationStatus::BuildFailed;

    let snapshot = runtime.visibility_authority().snapshot();
    let context = runtime
        .read_truth()
        .query_plan_context(&snapshot)
        .expect("query plan context");
    let packet = crate::facade::query::PlannedQueryPacket {
        label: "relation-label-equals".to_string(),
        context_id: context,
        scope: crate::facade::query::QueryScope::RelationPayloadFieldEquals {
            field: "label".to_string(),
            value: "edge".to_string(),
            partition_scope: None,
        },
        locality: crate::facade::query::QueryLocalityClass::CrossPartitionTraversal,
        ordering: crate::facade::query::QueryOrderingContract::CanonicalRelationIdOrder,
        fallback: QueryFallbackContract::IndexAdmissibleStorageEquivalent,
        execution_shape: crate::facade::query::QueryExecutionShape::BulkPacketized,
        reduction: crate::facade::query::ReductionDiscipline::DeterministicMerge,
        plan_key: crate::facade::query::DeterministicQueryPlanKey(1012),
        target_count_hint: 0,
    };
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

    assert_eq!(
        outcome.access_path,
        QueryAccessPath::DerivedIndexRejectedStorageFallback {
            rejection: IndexQueryRejectionClass::CorruptPayload,
        }
    );
}

#[test]
fn derived_index_contract_relation_field_equals_persisted_recovery_preserves_parity() {
    let mut runtime = persisted_runtime_with_test_schema();
    let source = create_entity_outcome(&mut runtime, "source");
    let target = create_entity_outcome(&mut runtime, "target");
    let relation = create_relation_outcome(
        &mut runtime,
        changed_entities(&source)[0],
        changed_entities(&target)[0],
        "edge",
    );
    let index = runtime.index_authority().register(DerivedIndexDefinition {
        index_id: DerivedIndexId(1),
        name: "relation.label.lookup".to_string(),
        kind: DerivedIndexKind::RelationPayloadField {
            field: "label".to_string(),
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
    let packet = crate::facade::query::PlannedQueryPacket {
        label: "relation-label-equals".to_string(),
        context_id: context,
        scope: crate::facade::query::QueryScope::RelationPayloadFieldEquals {
            field: "label".to_string(),
            value: "edge".to_string(),
            partition_scope: None,
        },
        locality: crate::facade::query::QueryLocalityClass::CrossPartitionTraversal,
        ordering: crate::facade::query::QueryOrderingContract::CanonicalRelationIdOrder,
        fallback: QueryFallbackContract::IndexAdmissibleStorageEquivalent,
        execution_shape: crate::facade::query::QueryExecutionShape::BulkPacketized,
        reduction: crate::facade::query::ReductionDiscipline::DeterministicMerge,
        plan_key: crate::facade::query::DeterministicQueryPlanKey(1013),
        target_count_hint: 0,
    };
    let original = runtime
        .index_access()
        .execute_query_plan_with_fallback_parity(
            runtime
                .read_truth()
                .plan_query_packet(&snapshot, packet.clone())
                .expect("original plan"),
            FallbackParityMode::CertificationParity,
        )
        .expect("original outcome");

    let (_recovery, mut recovered) =
        checkpoint_and_recover_with(&mut runtime, persisted_runtime_with_test_schema);
    let recovered_snapshot = recovered.visibility_authority().snapshot();
    let recovered_context = recovered
        .read_truth()
        .query_plan_context(&recovered_snapshot)
        .expect("recovered context");
    let mut recovered_packet = packet;
    recovered_packet.context_id = recovered_context;
    let recovered_outcome = recovered
        .index_access()
        .execute_query_plan_with_fallback_parity(
            recovered
                .read_truth()
                .plan_query_packet(&recovered_snapshot, recovered_packet)
                .expect("recovered plan"),
            FallbackParityMode::CertificationParity,
        )
        .expect("recovered outcome");

    assert_eq!(original.access_path, recovered_outcome.access_path);
    assert_eq!(
        original.execution.result,
        recovered_outcome.execution.result
    );
    assert_eq!(
        original.execution.result.reduction_digest,
        recovered_outcome.execution.result.reduction_digest
    );
}

#[test]
fn derived_index_contract_sampled_parity_is_bounded_and_deterministic() {
    let mut runtime = runtime_with_test_schema();
    let alpha = create_entity_outcome(&mut runtime, "alpha");
    let index = runtime.index_authority().register(DerivedIndexDefinition {
        index_id: DerivedIndexId(9),
        name: "entity.name.sampled".to_string(),
        kind: DerivedIndexKind::EntityPayloadField {
            field: "name".to_string(),
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
    let generation_id = build.generations[0].generation_id;
    let sampled_key = sampled_plan_key_for(generation_id, snapshot.version_id, true);
    let unsampled_key = sampled_plan_key_for(generation_id, snapshot.version_id, false);

    let sampled_packet = crate::facade::query::PlannedQueryPacket {
        label: "entity-name-sampled".to_string(),
        context_id: context.clone(),
        scope: crate::facade::query::QueryScope::EntityPayloadFieldEquals {
            field: "name".to_string(),
            value: "alpha".to_string(),
            partition_scope: None,
        },
        locality: crate::facade::query::QueryLocalityClass::CrossPartitionTraversal,
        ordering: crate::facade::query::QueryOrderingContract::CanonicalEntityIdOrder,
        fallback: QueryFallbackContract::IndexAdmissibleStorageEquivalent,
        execution_shape: crate::facade::query::QueryExecutionShape::BulkPacketized,
        reduction: crate::facade::query::ReductionDiscipline::DeterministicMerge,
        plan_key: sampled_key,
        target_count_hint: 0,
    };
    let unsampled_packet = crate::facade::query::PlannedQueryPacket {
        plan_key: unsampled_key,
        label: "entity-name-unsampled".to_string(),
        ..sampled_packet.clone()
    };

    runtime.performance_access().reset_counters();
    let sampled = runtime
        .index_access()
        .execute_query_plan_with_fallback_parity(
            runtime
                .read_truth()
                .plan_query_packet(&snapshot, sampled_packet.clone())
                .expect("sampled plan"),
            FallbackParityMode::SampledParity,
        )
        .expect("sampled outcome");
    let sampled_repeat = runtime
        .index_access()
        .execute_query_plan_with_fallback_parity(
            runtime
                .read_truth()
                .plan_query_packet(&snapshot, sampled_packet)
                .expect("sampled plan repeat"),
            FallbackParityMode::SampledParity,
        )
        .expect("sampled repeat outcome");
    let unsampled = runtime
        .index_access()
        .execute_query_plan_with_fallback_parity(
            runtime
                .read_truth()
                .plan_query_packet(&snapshot, unsampled_packet)
                .expect("unsampled plan"),
            FallbackParityMode::SampledParity,
        )
        .expect("unsampled outcome");
    let counters = runtime.performance_access().counters();

    assert_eq!(sampled.access_path, sampled_repeat.access_path);
    assert_eq!(
        sampled.parity_basis_digest,
        sampled_repeat.parity_basis_digest
    );
    assert_eq!(
        sampled.access_path,
        QueryAccessPath::DerivedIndexGeneration { generation_id }
    );
    assert_eq!(
        unsampled.access_path,
        QueryAccessPath::DerivedIndexGeneration { generation_id }
    );
    assert_eq!(counters.query_index_attempt_count, 3);
    assert_eq!(counters.query_index_path_count, 3);
    assert_eq!(counters.query_index_parity_verification_count, 2);
}

#[test]
fn derived_index_contract_runtime_drop_releases_index_scratch_hints() {
    let baseline_hint_count = crate::indexes::logic::index_query_scratch_hint_count();
    {
        let mut runtime = runtime_with_test_schema();
        let runtime_id = runtime.runtime_instance_id();
        let _alpha_a = create_entity_outcome(&mut runtime, "alpha");
        let _alpha_b = create_entity_outcome(&mut runtime, "alpha");
        let index = runtime.index_authority().register(DerivedIndexDefinition {
            index_id: DerivedIndexId(77),
            name: "entity.name.drop-release".to_string(),
            kind: DerivedIndexKind::EntityPayloadField {
                field: "name".to_string(),
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
        let packet = crate::facade::query::PlannedQueryPacket {
            label: "entity-name-drop-release".to_string(),
            context_id: context,
            scope: crate::facade::query::QueryScope::EntityPayloadFieldEquals {
                field: "name".to_string(),
                value: "alpha".to_string(),
                partition_scope: None,
            },
            locality: crate::facade::query::QueryLocalityClass::CrossPartitionTraversal,
            ordering: crate::facade::query::QueryOrderingContract::CanonicalEntityIdOrder,
            fallback: QueryFallbackContract::IndexAdmissibleStorageEquivalent,
            execution_shape: crate::facade::query::QueryExecutionShape::BulkPacketized,
            reduction: crate::facade::query::ReductionDiscipline::DeterministicMerge,
            plan_key: crate::facade::query::DeterministicQueryPlanKey(2077),
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
        assert!(counters.query_index_scratch_reuse_count > 0);
        assert!(crate::indexes::logic::index_query_scratch_hint_exists(runtime_id));
        assert!(crate::indexes::logic::index_query_scratch_hint_count() >= baseline_hint_count);
    }

    assert_eq!(
        crate::indexes::logic::index_query_scratch_hint_count(),
        baseline_hint_count
    );
}

#[test]
fn derived_index_contract_entity_field_any_of_executes_through_real_index_path_with_storage_parity()
{
    let mut runtime = runtime_with_test_schema();
    let alpha = create_entity_outcome(&mut runtime, "alpha");
    let beta = create_entity_outcome(&mut runtime, "beta");
    let _gamma = create_entity_outcome(&mut runtime, "gamma");
    let index = runtime.index_authority().register(DerivedIndexDefinition {
        index_id: DerivedIndexId(10),
        name: "entity.name.any-of".to_string(),
        kind: DerivedIndexKind::EntityPayloadField {
            field: "name".to_string(),
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
    let packet = crate::facade::query::PlannedQueryPacket {
        label: "entity-name-any-of".to_string(),
        context_id: context,
        scope: crate::facade::query::QueryScope::EntityPayloadFieldAnyOf {
            field: "name".to_string(),
            values: Arc::from(["beta".to_string(), "alpha".to_string()]),
            partition_scope: None,
        },
        locality: crate::facade::query::QueryLocalityClass::CrossPartitionTraversal,
        ordering: crate::facade::query::QueryOrderingContract::CanonicalEntityIdOrder,
        fallback: QueryFallbackContract::IndexAdmissibleStorageEquivalent,
        execution_shape: crate::facade::query::QueryExecutionShape::BulkPacketized,
        reduction: crate::facade::query::ReductionDiscipline::DeterministicMerge,
        plan_key: crate::facade::query::DeterministicQueryPlanKey(2010),
        target_count_hint: 2,
    };
    let plan = runtime
        .read_truth()
        .plan_query_packet(&snapshot, packet)
        .expect("query plan");
    let storage = runtime
        .read_truth()
        .execute_query_plan(plan.clone())
        .expect("storage outcome");
    let indexed = runtime
        .index_access()
        .execute_query_plan_with_fallback_parity(plan, FallbackParityMode::CertificationParity)
        .expect("indexed outcome");

    assert_eq!(
        indexed.access_path,
        QueryAccessPath::DerivedIndexGeneration {
            generation_id: build.generations[0].generation_id,
        }
    );
    assert_eq!(indexed.execution.result, storage.result);
    assert_eq!(
        indexed
            .execution
            .result
            .entities
            .iter()
            .map(|record| record.entity_id)
            .collect::<Vec<_>>(),
        vec![changed_entities(&alpha)[0], changed_entities(&beta)[0]]
    );
}

#[test]
fn derived_index_contract_relation_field_any_of_executes_through_real_index_path_with_storage_parity(
) {
    let mut runtime = runtime_with_test_schema();
    let source = create_entity_outcome(&mut runtime, "source");
    let target = create_entity_outcome(&mut runtime, "target");
    let third = create_entity_outcome(&mut runtime, "third");
    let edge = create_relation_outcome(
        &mut runtime,
        changed_entities(&source)[0],
        changed_entities(&target)[0],
        "edge",
    );
    let arc = create_relation_outcome(
        &mut runtime,
        changed_entities(&target)[0],
        changed_entities(&third)[0],
        "arc",
    );
    let _other = create_relation_outcome(
        &mut runtime,
        changed_entities(&third)[0],
        changed_entities(&source)[0],
        "other",
    );
    let index = runtime.index_authority().register(DerivedIndexDefinition {
        index_id: DerivedIndexId(11),
        name: "relation.label.any-of".to_string(),
        kind: DerivedIndexKind::RelationPayloadField {
            field: "label".to_string(),
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
    let packet = crate::facade::query::PlannedQueryPacket {
        label: "relation-label-any-of".to_string(),
        context_id: context,
        scope: crate::facade::query::QueryScope::RelationPayloadFieldAnyOf {
            field: "label".to_string(),
            values: Arc::from(["arc".to_string(), "edge".to_string()]),
            partition_scope: None,
        },
        locality: crate::facade::query::QueryLocalityClass::CrossPartitionTraversal,
        ordering: crate::facade::query::QueryOrderingContract::CanonicalRelationIdOrder,
        fallback: QueryFallbackContract::IndexAdmissibleStorageEquivalent,
        execution_shape: crate::facade::query::QueryExecutionShape::BulkPacketized,
        reduction: crate::facade::query::ReductionDiscipline::DeterministicMerge,
        plan_key: crate::facade::query::DeterministicQueryPlanKey(2011),
        target_count_hint: 2,
    };
    let plan = runtime
        .read_truth()
        .plan_query_packet(&snapshot, packet)
        .expect("query plan");
    let storage = runtime
        .read_truth()
        .execute_query_plan(plan.clone())
        .expect("storage outcome");
    let indexed = runtime
        .index_access()
        .execute_query_plan_with_fallback_parity(plan, FallbackParityMode::CertificationParity)
        .expect("indexed outcome");

    assert_eq!(
        indexed.access_path,
        QueryAccessPath::DerivedIndexGeneration {
            generation_id: build.generations[0].generation_id,
        }
    );
    assert_eq!(indexed.execution.result, storage.result);
    assert_eq!(
        indexed
            .execution
            .result
            .relations
            .iter()
            .map(|record| record.relation_id)
            .collect::<Vec<_>>(),
        vec![changed_relations(&edge)[0], changed_relations(&arc)[0]]
    );
}

#[test]
fn derived_index_contract_relation_field_equals_branch_scoped_generation_reports_incompatible_branch(
) {
    let mut runtime = runtime_with_test_schema();
    let main_source = create_entity_outcome(&mut runtime, "main-source");
    let main_target = create_entity_outcome(&mut runtime, "main-target");
    let main_relation = create_relation_outcome(
        &mut runtime,
        changed_entities(&main_source)[0],
        changed_entities(&main_target)[0],
        "edge",
    );
    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let feature_source = create_entity_outcome_on_branch(
        &mut runtime,
        "feature-source",
        BranchId("feature".to_string()),
    );
    let feature_target = create_entity_outcome_on_branch(
        &mut runtime,
        "feature-target",
        BranchId("feature".to_string()),
    );
    let mut feature_txn = runtime.begin_transaction(TransactionOptions {
        target_branch: Some(BranchId("feature".to_string())),
        ..TransactionOptions::default()
    });
    feature_txn.push_batch(WorkerIntentBatch::new("feature-relation").push(
        MutationIntent::Create(CreateIntent::Relation(
            crate::transactions::data::RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: InternedString::Raw("edge".to_string()),
                source: changed_entities(&feature_source)[0],
                target: changed_entities(&feature_target)[0],
                payload: Some(RecordPayload::StructuredJson(
                    serde_json::json!({"label":"edge"}),
                )),
            },
        )),
    ));
    let feature_relation = feature_txn.commit().expect("feature relation");
    let index = runtime.index_authority().register(DerivedIndexDefinition {
        index_id: DerivedIndexId(1),
        name: "relation.label.branch".to_string(),
        kind: DerivedIndexKind::RelationPayloadField {
            field: "label".to_string(),
        },
        branch_scoped: true,
    });
    let build = runtime
        .index_authority()
        .build_for_commit(DerivedIndexBuildRequest {
            source_commit_id: feature_relation.commit.commit_id,
            branch_id: BranchId("feature".to_string()),
            index_ids: vec![index.index_id],
        });
    assert!(build.failed_indexes.is_empty());

    let context = runtime
        .read_truth()
        .query_plan_context(&main_relation.snapshot)
        .expect("query plan context");
    let packet = crate::facade::query::PlannedQueryPacket {
        label: "relation-branch-mismatch".to_string(),
        context_id: context,
        scope: crate::facade::query::QueryScope::RelationPayloadFieldEquals {
            field: "label".to_string(),
            value: "edge".to_string(),
            partition_scope: None,
        },
        locality: crate::facade::query::QueryLocalityClass::CrossPartitionTraversal,
        ordering: crate::facade::query::QueryOrderingContract::CanonicalRelationIdOrder,
        fallback: QueryFallbackContract::IndexAdmissibleStorageEquivalent,
        execution_shape: crate::facade::query::QueryExecutionShape::BulkPacketized,
        reduction: crate::facade::query::ReductionDiscipline::DeterministicMerge,
        plan_key: crate::facade::query::DeterministicQueryPlanKey(1014),
        target_count_hint: 0,
    };
    let outcome = runtime
        .index_access()
        .execute_query_plan_with_fallback_parity(
            runtime
                .read_truth()
                .plan_query_packet(&main_relation.snapshot, packet)
                .expect("query plan"),
            FallbackParityMode::ProductionAdmissibility,
        )
        .expect("query outcome");

    assert_eq!(
        outcome.access_path,
        QueryAccessPath::DerivedIndexRejectedStorageFallback {
            rejection: IndexQueryRejectionClass::IncompatibleBranch,
        }
    );
}

#[test]
fn derived_index_contract_relation_field_equals_partition_scope_keeps_bounded_parity() {
    let mut runtime = runtime_with_test_schema();
    let left_source = create_entity_in_partition(&mut runtime, "left-source", PartitionId(7));
    let left_target = create_entity_in_partition(&mut runtime, "left-target", PartitionId(7));
    let right_source = create_entity_in_partition(&mut runtime, "right-source", PartitionId(11));
    let right_target = create_entity_in_partition(&mut runtime, "right-target", PartitionId(11));
    let left_relation = create_relation_in_partition(
        &mut runtime,
        left_source,
        left_target,
        "edge",
        PartitionId(7),
    );
    let _right_relation = create_relation_in_partition(
        &mut runtime,
        right_source,
        right_target,
        "edge",
        PartitionId(11),
    );
    let commit_id = runtime
        .history()
        .latest_commit()
        .expect("latest commit")
        .commit_id;
    let index = runtime.index_authority().register(DerivedIndexDefinition {
        index_id: DerivedIndexId(1),
        name: "relation.label.partitioned".to_string(),
        kind: DerivedIndexKind::RelationPayloadField {
            field: "label".to_string(),
        },
        branch_scoped: false,
    });
    let build = runtime
        .index_authority()
        .build_for_commit(DerivedIndexBuildRequest {
            source_commit_id: commit_id,
            branch_id: BranchId("main".to_string()),
            index_ids: vec![index.index_id],
        });
    assert!(build.failed_indexes.is_empty());

    let snapshot = runtime.visibility_authority().snapshot();
    let context = runtime
        .read_truth()
        .query_plan_context(&snapshot)
        .expect("query plan context");
    let packet = crate::facade::query::PlannedQueryPacket {
        label: "relation-label-left-only".to_string(),
        context_id: context,
        scope: crate::facade::query::QueryScope::RelationPayloadFieldEquals {
            field: "label".to_string(),
            value: "edge".to_string(),
            partition_scope: Some(std::sync::Arc::from([PartitionId(7)])),
        },
        locality: crate::facade::query::QueryLocalityClass::PartitionBounded {
            partitions: std::sync::Arc::from([PartitionId(7)]),
        },
        ordering: crate::facade::query::QueryOrderingContract::CanonicalRelationIdOrder,
        fallback: QueryFallbackContract::IndexAdmissibleStorageEquivalent,
        execution_shape: crate::facade::query::QueryExecutionShape::BulkPacketized,
        reduction: crate::facade::query::ReductionDiscipline::DeterministicMerge,
        plan_key: crate::facade::query::DeterministicQueryPlanKey(1015),
        target_count_hint: 0,
    };
    let plan = runtime
        .read_truth()
        .plan_query_packet(&snapshot, packet)
        .expect("query plan");
    let storage = runtime
        .read_truth()
        .execute_query_plan(plan.clone())
        .expect("storage outcome");
    let indexed = runtime
        .index_access()
        .execute_query_plan_with_fallback_parity(plan, FallbackParityMode::CertificationParity)
        .expect("indexed outcome");

    assert_eq!(indexed.execution.result, storage.result);
    assert_eq!(indexed.execution.result.relations.len(), 1);
    assert_eq!(
        indexed.execution.result.relations[0].relation_id,
        left_relation
    );
    assert_eq!(
        indexed.access_path,
        QueryAccessPath::DerivedIndexGeneration {
            generation_id: build.generations[0].generation_id,
        }
    );
}

#[test]
fn derived_index_contract_branch_scoped_generation_reports_incompatible_branch() {
    let mut runtime = runtime_with_test_schema();
    let main_outcome = create_entity_outcome(&mut runtime, "main-a");
    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let feature_outcome =
        create_entity_outcome_on_branch(&mut runtime, "feature-a", BranchId("feature".to_string()));
    let index = runtime.index_authority().register(DerivedIndexDefinition {
        index_id: DerivedIndexId(0),
        name: "entity.name.branch".to_string(),
        kind: DerivedIndexKind::EntityPayloadField {
            field: "name".to_string(),
        },
        branch_scoped: true,
    });
    let build = runtime
        .index_authority()
        .build_for_commit(DerivedIndexBuildRequest {
            source_commit_id: feature_outcome.commit.commit_id,
            branch_id: BranchId("feature".to_string()),
            index_ids: vec![index.index_id],
        });
    assert!(build.failed_indexes.is_empty());

    let context = runtime
        .read_truth()
        .query_plan_context(&main_outcome.snapshot)
        .expect("query plan context");
    let packet = crate::facade::query::PlannedQueryPacket {
        label: "branch-mismatch".to_string(),
        context_id: context,
        scope: crate::facade::query::QueryScope::EntityPayloadFieldEquals {
            field: "name".to_string(),
            value: "main-a".to_string(),
            partition_scope: None,
        },
        locality: crate::facade::query::QueryLocalityClass::CrossPartitionTraversal,
        ordering: crate::facade::query::QueryOrderingContract::CanonicalEntityIdOrder,
        fallback: QueryFallbackContract::IndexAdmissibleStorageEquivalent,
        execution_shape: crate::facade::query::QueryExecutionShape::BulkPacketized,
        reduction: crate::facade::query::ReductionDiscipline::DeterministicMerge,
        plan_key: crate::facade::query::DeterministicQueryPlanKey(1002),
        target_count_hint: 0,
    };
    let plan = runtime
        .read_truth()
        .plan_query_packet(&main_outcome.snapshot, packet)
        .expect("query plan");
    let outcome = runtime
        .index_access()
        .execute_query_plan_with_fallback_parity(plan, FallbackParityMode::ProductionAdmissibility)
        .expect("fallback outcome");

    assert_eq!(
        outcome.access_path,
        QueryAccessPath::DerivedIndexRejectedStorageFallback {
            rejection: IndexQueryRejectionClass::IncompatibleBranch,
        }
    );
}

#[test]
fn derived_index_contract_index_counters_track_attempts_paths_and_rejections() {
    let mut runtime = runtime_with_test_schema();
    let alpha = create_entity_outcome(&mut runtime, "alpha");
    let index = runtime.index_authority().register(DerivedIndexDefinition {
        index_id: DerivedIndexId(0),
        name: "entity.name.lookup".to_string(),
        kind: DerivedIndexKind::EntityPayloadField {
            field: "name".to_string(),
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
    runtime.performance_access().reset_counters();

    let snapshot = runtime.visibility_authority().snapshot();
    let context = runtime
        .read_truth()
        .query_plan_context(&snapshot)
        .expect("query plan context");
    let success_packet = crate::facade::query::PlannedQueryPacket {
        label: "entity-name-equals".to_string(),
        context_id: context.clone(),
        scope: crate::facade::query::QueryScope::EntityPayloadFieldEquals {
            field: "name".to_string(),
            value: "alpha".to_string(),
            partition_scope: None,
        },
        locality: crate::facade::query::QueryLocalityClass::CrossPartitionTraversal,
        ordering: crate::facade::query::QueryOrderingContract::CanonicalEntityIdOrder,
        fallback: QueryFallbackContract::IndexAdmissibleStorageEquivalent,
        execution_shape: crate::facade::query::QueryExecutionShape::BulkPacketized,
        reduction: crate::facade::query::ReductionDiscipline::DeterministicMerge,
        plan_key: crate::facade::query::DeterministicQueryPlanKey(1003),
        target_count_hint: 0,
    };
    runtime
        .index_access()
        .execute_query_plan_with_fallback_parity(
            runtime
                .read_truth()
                .plan_query_packet(&snapshot, success_packet)
                .expect("success plan"),
            FallbackParityMode::CertificationParity,
        )
        .expect("success outcome");

    let rejection_packet = crate::facade::query::PlannedQueryPacket {
        label: "entity-name-equals-rejected".to_string(),
        context_id: context,
        scope: crate::facade::query::QueryScope::EntityKindScan {
            kind_id: KindId(1),
            partition_scope: None,
        },
        locality: crate::facade::query::QueryLocalityClass::CrossPartitionTraversal,
        ordering: crate::facade::query::QueryOrderingContract::CanonicalEntityIdOrder,
        fallback: QueryFallbackContract::IndexAdmissibleStorageEquivalent,
        execution_shape: crate::facade::query::QueryExecutionShape::BulkPacketized,
        reduction: crate::facade::query::ReductionDiscipline::DeterministicMerge,
        plan_key: crate::facade::query::DeterministicQueryPlanKey(1004),
        target_count_hint: 0,
    };
    runtime
        .index_access()
        .execute_query_plan_with_fallback_parity(
            runtime
                .read_truth()
                .plan_query_packet(&snapshot, rejection_packet)
                .expect("rejection plan"),
            FallbackParityMode::ProductionAdmissibility,
        )
        .expect("rejection outcome");

    let counters = runtime.performance_access().counters();
    assert_eq!(counters.query_index_attempt_count, 2);
    assert_eq!(counters.query_index_path_count, 1);
    assert_eq!(counters.query_index_rejection_count, 1);
    assert_eq!(counters.query_index_parity_verification_count, 1);
}

#[test]
fn derived_index_contract_prefers_older_compatible_generation_over_newer_incompatible_one() {
    let mut runtime = runtime_with_test_schema();
    let main_alpha = create_entity_outcome(&mut runtime, "alpha");
    let index = runtime.index_authority().register(DerivedIndexDefinition {
        index_id: DerivedIndexId(0),
        name: "entity.name.lookup".to_string(),
        kind: DerivedIndexKind::EntityPayloadField {
            field: "name".to_string(),
        },
        branch_scoped: true,
    });
    let main_build = runtime
        .index_authority()
        .build_for_commit(DerivedIndexBuildRequest {
            source_commit_id: main_alpha.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            index_ids: vec![index.index_id],
        });
    assert!(main_build.failed_indexes.is_empty());

    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let feature_alpha =
        create_entity_outcome_on_branch(&mut runtime, "alpha", BranchId("feature".to_string()));
    let feature_build = runtime
        .index_authority()
        .build_for_commit(DerivedIndexBuildRequest {
            source_commit_id: feature_alpha.commit.commit_id,
            branch_id: BranchId("feature".to_string()),
            index_ids: vec![index.index_id],
        });
    assert!(feature_build.failed_indexes.is_empty());
    assert!(feature_build.generations[0].generation_id > main_build.generations[0].generation_id);

    let snapshot = main_alpha.snapshot.clone();
    let context = runtime
        .read_truth()
        .query_plan_context(&snapshot)
        .expect("query plan context");
    let packet = crate::facade::query::PlannedQueryPacket {
        label: "entity-name-equals-main".to_string(),
        context_id: context,
        scope: crate::facade::query::QueryScope::EntityPayloadFieldEquals {
            field: "name".to_string(),
            value: "alpha".to_string(),
            partition_scope: None,
        },
        locality: crate::facade::query::QueryLocalityClass::CrossPartitionTraversal,
        ordering: crate::facade::query::QueryOrderingContract::CanonicalEntityIdOrder,
        fallback: QueryFallbackContract::IndexAdmissibleStorageEquivalent,
        execution_shape: crate::facade::query::QueryExecutionShape::BulkPacketized,
        reduction: crate::facade::query::ReductionDiscipline::DeterministicMerge,
        plan_key: crate::facade::query::DeterministicQueryPlanKey(1005),
        target_count_hint: 0,
    };
    let plan = runtime
        .read_truth()
        .plan_query_packet(&snapshot, packet)
        .expect("query plan");
    let outcome = runtime
        .index_access()
        .execute_query_plan_with_fallback_parity(plan, FallbackParityMode::ProductionAdmissibility)
        .expect("query outcome");

    assert_eq!(
        outcome.access_path,
        QueryAccessPath::DerivedIndexGeneration {
            generation_id: main_build.generations[0].generation_id,
        }
    );
}

#[test]
fn derived_index_contract_matching_definition_without_generation_reports_missing_generation() {
    let mut runtime = runtime_with_test_schema();
    let outcome = create_entity_outcome(&mut runtime, "alpha");
    let _index = runtime.index_authority().register(DerivedIndexDefinition {
        index_id: DerivedIndexId(0),
        name: "entity.name.lookup".to_string(),
        kind: DerivedIndexKind::EntityPayloadField {
            field: "name".to_string(),
        },
        branch_scoped: false,
    });

    let context = runtime
        .read_truth()
        .query_plan_context(&outcome.snapshot)
        .expect("query plan context");
    let packet = crate::facade::query::PlannedQueryPacket {
        label: "entity-name-equals".to_string(),
        context_id: context,
        scope: crate::facade::query::QueryScope::EntityPayloadFieldEquals {
            field: "name".to_string(),
            value: "alpha".to_string(),
            partition_scope: None,
        },
        locality: crate::facade::query::QueryLocalityClass::CrossPartitionTraversal,
        ordering: crate::facade::query::QueryOrderingContract::CanonicalEntityIdOrder,
        fallback: QueryFallbackContract::IndexAdmissibleStorageEquivalent,
        execution_shape: crate::facade::query::QueryExecutionShape::BulkPacketized,
        reduction: crate::facade::query::ReductionDiscipline::DeterministicMerge,
        plan_key: crate::facade::query::DeterministicQueryPlanKey(1006),
        target_count_hint: 0,
    };
    let plan = runtime
        .read_truth()
        .plan_query_packet(&outcome.snapshot, packet)
        .expect("query plan");
    let result = runtime
        .index_access()
        .execute_query_plan_with_fallback_parity(plan, FallbackParityMode::ProductionAdmissibility)
        .expect("query result");

    assert_eq!(
        result.access_path,
        QueryAccessPath::DerivedIndexRejectedStorageFallback {
            rejection: IndexQueryRejectionClass::MissingGeneration,
        }
    );
}

#[test]
fn derived_index_contract_explicit_corrupt_generation_reports_corrupt_payload() {
    let mut runtime = runtime_with_test_schema();
    let alpha = create_entity_outcome(&mut runtime, "alpha");
    let index = runtime.index_authority().register(DerivedIndexDefinition {
        index_id: DerivedIndexId(0),
        name: "entity.name.lookup".to_string(),
        kind: DerivedIndexKind::EntityPayloadField {
            field: "name".to_string(),
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
    runtime
        .indexes
        .generations
        .get_mut(&index.index_id)
        .expect("index generations")
        .last_mut()
        .expect("built generation")
        .status = crate::facade::indexes::DerivedIndexPublicationStatus::BuildFailed;

    let context = runtime
        .read_truth()
        .query_plan_context(&alpha.snapshot)
        .expect("query plan context");
    let packet = crate::facade::query::PlannedQueryPacket {
        label: "entity-name-equals".to_string(),
        context_id: context,
        scope: crate::facade::query::QueryScope::EntityPayloadFieldEquals {
            field: "name".to_string(),
            value: "alpha".to_string(),
            partition_scope: None,
        },
        locality: crate::facade::query::QueryLocalityClass::CrossPartitionTraversal,
        ordering: crate::facade::query::QueryOrderingContract::CanonicalEntityIdOrder,
        fallback: QueryFallbackContract::IndexAdmissibleStorageEquivalent,
        execution_shape: crate::facade::query::QueryExecutionShape::BulkPacketized,
        reduction: crate::facade::query::ReductionDiscipline::DeterministicMerge,
        plan_key: crate::facade::query::DeterministicQueryPlanKey(1007),
        target_count_hint: 0,
    };
    let outcome = runtime
        .index_access()
        .execute_query_plan_with_fallback_parity(
            runtime
                .read_truth()
                .plan_query_packet(&alpha.snapshot, packet)
                .expect("query plan"),
            FallbackParityMode::ProductionAdmissibility,
        )
        .expect("query outcome");

    assert_eq!(
        outcome.access_path,
        QueryAccessPath::DerivedIndexRejectedStorageFallback {
            rejection: IndexQueryRejectionClass::CorruptPayload,
        }
    );
}

#[test]
fn derived_index_contract_persisted_recovery_preserves_entity_field_equals_parity() {
    let mut runtime = persisted_runtime_with_test_schema();
    let alpha = create_entity_outcome(&mut runtime, "alpha");
    let index = runtime.index_authority().register(DerivedIndexDefinition {
        index_id: DerivedIndexId(0),
        name: "entity.name.lookup".to_string(),
        kind: DerivedIndexKind::EntityPayloadField {
            field: "name".to_string(),
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

    let snapshot = alpha.snapshot.clone();
    let context = runtime
        .read_truth()
        .query_plan_context(&snapshot)
        .expect("query plan context");
    let packet = crate::facade::query::PlannedQueryPacket {
        label: "entity-name-equals".to_string(),
        context_id: context,
        scope: crate::facade::query::QueryScope::EntityPayloadFieldEquals {
            field: "name".to_string(),
            value: "alpha".to_string(),
            partition_scope: None,
        },
        locality: crate::facade::query::QueryLocalityClass::CrossPartitionTraversal,
        ordering: crate::facade::query::QueryOrderingContract::CanonicalEntityIdOrder,
        fallback: QueryFallbackContract::IndexAdmissibleStorageEquivalent,
        execution_shape: crate::facade::query::QueryExecutionShape::BulkPacketized,
        reduction: crate::facade::query::ReductionDiscipline::DeterministicMerge,
        plan_key: crate::facade::query::DeterministicQueryPlanKey(1008),
        target_count_hint: 0,
    };
    let original = runtime
        .index_access()
        .execute_query_plan_with_fallback_parity(
            runtime
                .read_truth()
                .plan_query_packet(&snapshot, packet.clone())
                .expect("original plan"),
            FallbackParityMode::CertificationParity,
        )
        .expect("original outcome");

    let (_recovery, mut recovered) =
        checkpoint_and_recover_with(&mut runtime, persisted_runtime_with_test_schema);
    let recovered_snapshot = recovered.visibility_authority().snapshot();
    let recovered_context = recovered
        .read_truth()
        .query_plan_context(&recovered_snapshot)
        .expect("recovered context");
    let mut recovered_packet = packet;
    recovered_packet.context_id = recovered_context;
    let recovered_outcome = recovered
        .index_access()
        .execute_query_plan_with_fallback_parity(
            recovered
                .read_truth()
                .plan_query_packet(&recovered_snapshot, recovered_packet)
                .expect("recovered plan"),
            FallbackParityMode::CertificationParity,
        )
        .expect("recovered outcome");

    assert_eq!(original.access_path, recovered_outcome.access_path);
    assert_eq!(
        original.execution.result,
        recovered_outcome.execution.result
    );
    assert_eq!(
        original.execution.result.reduction_digest,
        recovered_outcome.execution.result.reduction_digest
    );
}

#[test]
fn derived_index_contract_entity_field_equals_is_stable_across_execution_models() {
    fn build_runtime(
        execution_model: RelationalExecutionModel,
    ) -> (
        crate::facade::runtime::RelationalRuntime,
        crate::facade::snapshots::SnapshotHandle,
    ) {
        let mut runtime = runtime_with_test_schema_execution_model(execution_model);
        let alpha = create_entity_outcome(&mut runtime, "alpha");
        let index = runtime.index_authority().register(DerivedIndexDefinition {
            index_id: DerivedIndexId(0),
            name: "entity.name.lookup".to_string(),
            kind: DerivedIndexKind::EntityPayloadField {
                field: "name".to_string(),
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
        (runtime, alpha.snapshot.clone())
    }

    let (serial_runtime, serial_snapshot) =
        build_runtime(RelationalExecutionModel::SerialAuthority);
    let (staged_runtime, staged_snapshot) =
        build_runtime(RelationalExecutionModel::StagedParallelPreparation);

    let serial_context = serial_runtime
        .read_truth()
        .query_plan_context(&serial_snapshot)
        .expect("serial context");
    let staged_context = staged_runtime
        .read_truth()
        .query_plan_context(&staged_snapshot)
        .expect("staged context");

    let serial_outcome = serial_runtime
        .index_access()
        .execute_query_plan_with_fallback_parity(
            serial_runtime
                .read_truth()
                .plan_query_packet(
                    &serial_snapshot,
                    crate::facade::query::PlannedQueryPacket {
                        label: "entity-name-equals".to_string(),
                        context_id: serial_context,
                        scope: crate::facade::query::QueryScope::EntityPayloadFieldEquals {
                            field: "name".to_string(),
                            value: "alpha".to_string(),
                            partition_scope: None,
                        },
                        locality: crate::facade::query::QueryLocalityClass::CrossPartitionTraversal,
                        ordering:
                            crate::facade::query::QueryOrderingContract::CanonicalEntityIdOrder,
                        fallback: QueryFallbackContract::IndexAdmissibleStorageEquivalent,
                        execution_shape: crate::facade::query::QueryExecutionShape::BulkPacketized,
                        reduction: crate::facade::query::ReductionDiscipline::DeterministicMerge,
                        plan_key: crate::facade::query::DeterministicQueryPlanKey(1009),
                        target_count_hint: 0,
                    },
                )
                .expect("serial plan"),
            FallbackParityMode::CertificationParity,
        )
        .expect("serial outcome");

    let staged_outcome = staged_runtime
        .index_access()
        .execute_query_plan_with_fallback_parity(
            staged_runtime
                .read_truth()
                .plan_query_packet(
                    &staged_snapshot,
                    crate::facade::query::PlannedQueryPacket {
                        label: "entity-name-equals".to_string(),
                        context_id: staged_context,
                        scope: crate::facade::query::QueryScope::EntityPayloadFieldEquals {
                            field: "name".to_string(),
                            value: "alpha".to_string(),
                            partition_scope: None,
                        },
                        locality: crate::facade::query::QueryLocalityClass::CrossPartitionTraversal,
                        ordering:
                            crate::facade::query::QueryOrderingContract::CanonicalEntityIdOrder,
                        fallback: QueryFallbackContract::IndexAdmissibleStorageEquivalent,
                        execution_shape: crate::facade::query::QueryExecutionShape::BulkPacketized,
                        reduction: crate::facade::query::ReductionDiscipline::DeterministicMerge,
                        plan_key: crate::facade::query::DeterministicQueryPlanKey(1010),
                        target_count_hint: 0,
                    },
                )
                .expect("staged plan"),
            FallbackParityMode::CertificationParity,
        )
        .expect("staged outcome");

    assert_eq!(serial_outcome.access_path, staged_outcome.access_path);
    assert_eq!(
        serial_outcome.execution.result,
        staged_outcome.execution.result
    );
    assert_eq!(
        serial_outcome.execution.result.reduction_digest,
        staged_outcome.execution.result.reduction_digest
    );
}

#[test]
fn derived_index_contract_staged_parallel_generation_matches_serial_reference() {
    fn build_runtime(
        execution_model: RelationalExecutionModel,
    ) -> (
        crate::facade::runtime::RelationalRuntime,
        crate::facade::history::CommitId,
        Vec<crate::facade::indexes::DerivedIndexId>,
    ) {
        let mut runtime = runtime_with_test_schema_execution_model(execution_model);
        let commit = create_entity_outcome(&mut runtime, "main-a");
        let name_index = runtime.index_authority().register(DerivedIndexDefinition {
            index_id: DerivedIndexId(0),
            name: "entity.name".to_string(),
            kind: DerivedIndexKind::EntityPayloadField {
                field: "name".to_string(),
            },
            branch_scoped: false,
        });
        let missing_index = runtime.index_authority().register(DerivedIndexDefinition {
            index_id: DerivedIndexId(0),
            name: "entity.missing".to_string(),
            kind: DerivedIndexKind::EntityPayloadField {
                field: "missing".to_string(),
            },
            branch_scoped: false,
        });

        (
            runtime,
            commit.commit.commit_id,
            vec![name_index.index_id, missing_index.index_id],
        )
    }

    let (mut serial_runtime, serial_commit_id, index_ids) =
        build_runtime(RelationalExecutionModel::SerialAuthority);
    let (mut staged_runtime, staged_commit_id, staged_index_ids) =
        build_runtime(RelationalExecutionModel::StagedParallelPreparation);

    serial_runtime.performance_access().reset_counters();
    staged_runtime.performance_access().reset_counters();

    let serial = serial_runtime
        .index_authority()
        .build_for_commit(DerivedIndexBuildRequest {
            source_commit_id: serial_commit_id,
            branch_id: BranchId("main".to_string()),
            index_ids,
        });
    let staged = staged_runtime
        .index_authority()
        .build_for_commit(DerivedIndexBuildRequest {
            source_commit_id: staged_commit_id,
            branch_id: BranchId("main".to_string()),
            index_ids: staged_index_ids,
        });

    let staged_counters = staged_runtime.performance_access().counters();

    assert_eq!(staged, serial);
    assert_eq!(staged_counters.preparation_packet_count, 2);
    assert_eq!(staged_counters.preparation_parallel_legal_count, 1);
    assert_eq!(staged_counters.preparation_parallel_profitable_count, 1);
    assert_eq!(
        staged_counters.preparation_staged_parallel_strategy_count,
        1
    );
    assert_eq!(staged_counters.preparation_serial_strategy_count, 0);
}
