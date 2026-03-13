use crate::facade::history::BranchId;
use crate::facade::indexes::{
    DerivedIndexBuildRequest, DerivedIndexDefinition, DerivedIndexId, DerivedIndexKind,
};
use crate::facade::query::QueryWorkPacket;
use crate::facade::runtime::RelationalExecutionModel;
use crate::facade::transactions::RecordRef;
use crate::tests::support::*;

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
    let packet = QueryWorkPacket::bulk(
        "entities",
        vec![RecordRef::Entity(changed_entities(&main_outcome)[0])],
    );
    let fallback = runtime
        .index_access()
        .read_with_storage_fallback(&main_outcome.snapshot, &packet)
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
    assert_eq!(fallback.result.entities.len(), 1);
    assert_eq!(fallback.used_index_generation, None);
}

#[test]
fn derived_index_contract_unscoped_generation_can_be_selected_across_branches() {
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
    let packet = QueryWorkPacket::bulk(
        "entities",
        vec![RecordRef::Entity(changed_entities(&main_outcome)[0])],
    );
    let fallback = runtime
        .index_access()
        .read_with_storage_fallback(&snapshot, &packet)
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
        fallback.used_index_generation,
        build
            .generations
            .first()
            .map(|generation| generation.generation_id)
    );
}

#[test]
fn derived_index_contract_failure_unknown_index_keeps_truth_reads_correct() {
    let mut runtime = runtime_with_test_schema();
    let outcome = create_entity_outcome(&mut runtime, "main-a");
    let snapshot = runtime.visibility_authority().snapshot();
    let packet = QueryWorkPacket::bulk(
        "entities",
        vec![RecordRef::Entity(changed_entities(&outcome)[0])],
    );
    let storage_only = runtime
        .visibility_reads()
        .execute_read_packet(&snapshot, &packet)
        .unwrap();
    let fallback_before = runtime
        .index_access()
        .read_with_storage_fallback(&snapshot, &packet)
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
        .read_with_storage_fallback(&snapshot, &packet)
        .unwrap();

    assert_eq!(build.failed_indexes, vec![DerivedIndexId(999)]);
    assert_eq!(fallback_before.used_index_generation, None);
    assert_eq!(fallback_before.result, storage_only);
    assert_eq!(fallback_after.result, storage_only);
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
