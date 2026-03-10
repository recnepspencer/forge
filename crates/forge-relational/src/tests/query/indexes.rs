use crate::facade::{
    BranchId, DerivedIndexBuildRequest, DerivedIndexDefinition, DerivedIndexId, DerivedIndexKind,
    QueryWorkPacket, ReadTarget,
};
use crate::tests::support::*;

// CONTRACT: derived_index
// LANES: success, fallback, determinism

#[test]
fn derived_index_contract_success_branch_scoped_build_keeps_storage_fallback() {
    let mut runtime = runtime_with_test_schema();
    let main_outcome = create_entity_outcome(&mut runtime, "main-a");
    runtime
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let feature_outcome = create_entity_outcome_on_branch(
        &mut runtime,
        "feature-a",
        BranchId("feature".to_string()),
    );
    let index = runtime.register_index(DerivedIndexDefinition {
        index_id: crate::facade::DerivedIndexId(0),
        name: "entity.name".to_string(),
        kind: DerivedIndexKind::EntityPayloadField {
            field: "name".to_string(),
        },
        branch_scoped: true,
    });
    let feature_build = runtime.build_indexes_for_commit(DerivedIndexBuildRequest {
        source_commit_id: feature_outcome.commit.commit_id,
        branch_id: BranchId("feature".to_string()),
        index_ids: vec![index.index_id],
    });
    let packet = QueryWorkPacket::bulk(
        "entities",
        vec![ReadTarget::Entity(
            changed_entities(&main_outcome)[0],
        )],
    );
    let fallback = runtime
        .read_with_storage_fallback(&main_outcome.snapshot, &packet)
        .unwrap();

    assert!(feature_build.failed_indexes.is_empty());
    assert_eq!(
        runtime
            .latest_index_generation(index.index_id, &BranchId("feature".to_string()))
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
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let feature_outcome = create_entity_outcome_on_branch(
        &mut runtime,
        "feature-a",
        BranchId("feature".to_string()),
    );
    let index = runtime.register_index(DerivedIndexDefinition {
        index_id: crate::facade::DerivedIndexId(0),
        name: "entity.name.global".to_string(),
        kind: DerivedIndexKind::EntityPayloadField {
            field: "name".to_string(),
        },
        branch_scoped: false,
    });
    let build = runtime.build_indexes_for_commit(DerivedIndexBuildRequest {
        source_commit_id: feature_outcome.commit.commit_id,
        branch_id: BranchId("feature".to_string()),
        index_ids: vec![index.index_id],
    });
    let snapshot = runtime.snapshot();
    let packet = QueryWorkPacket::bulk(
        "entities",
        vec![ReadTarget::Entity(
            changed_entities(&main_outcome)[0],
        )],
    );
    let fallback = runtime
        .read_with_storage_fallback(&snapshot, &packet)
        .unwrap();

    assert!(build.failed_indexes.is_empty());
    assert_eq!(
        runtime
            .latest_index_generation(index.index_id, &BranchId("main".to_string()))
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
    let snapshot = runtime.snapshot();
    let packet = QueryWorkPacket::bulk(
        "entities",
        vec![ReadTarget::Entity(changed_entities(&outcome)[0])],
    );
    let storage_only = runtime.execute_read_packet(&snapshot, &packet).unwrap();
    let fallback_before = runtime
        .read_with_storage_fallback(&snapshot, &packet)
        .unwrap();
    let build = runtime.build_indexes_for_commit(DerivedIndexBuildRequest {
        source_commit_id: outcome.commit.commit_id,
        branch_id: BranchId("main".to_string()),
        index_ids: vec![DerivedIndexId(999)],
    });
    let fallback_after = runtime
        .read_with_storage_fallback(&snapshot, &packet)
        .unwrap();

    assert_eq!(build.failed_indexes, vec![DerivedIndexId(999)]);
    assert_eq!(fallback_before.used_index_generation, None);
    assert_eq!(fallback_before.result, storage_only);
    assert_eq!(fallback_after.result, storage_only);
}
