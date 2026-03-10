use crate::facade::{
    BranchId, CanonicalCommitEnvelope, DerivedIndexBuildRequest, DerivedIndexDefinition,
    DerivedIndexKind, DurableCommitEnvelope, EntityKindRegistration, KindId, RecoveryFailureClass,
    RelationalRuntimeApi, RelationalSchemaRegistry, SchemaId, SchemaVersionId,
};

// CONTRACT: durability
// LANES: success, failure, recovery

#[test]
fn durability_contract_recovery_rebuilds_branch_heads_and_latest_commit() {
    let mut runtime = super::runtime_with_test_schema();
    let main = super::create_entity_outcome(&mut runtime, "main-a");
    runtime
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let feature = super::create_entity_outcome_on_branch(
        &mut runtime,
        "feature-a",
        BranchId("feature".to_string()),
    );
    let _checkpoint = runtime.checkpoint().unwrap();
    let plan = runtime.recovery_plan();
    let mut recovered = super::runtime_with_test_schema();
    let outcome = recovered.recover(plan).unwrap();

    assert_eq!(outcome.recovered_commits, 2);
    assert_eq!(outcome.latest_commit, Some(feature.commit.clone()));
    assert_eq!(
        recovered.branch_head(&BranchId("feature".to_string())),
        Some(&feature.commit)
    );
    assert_eq!(
        recovered.branch_head(&BranchId("main".to_string())),
        Some(&main.commit)
    );
}

#[test]
fn durability_contract_failure_schema_mismatch_is_explicit() {
    let mut runtime = super::runtime_with_test_schema();
    super::create_entity_outcome(&mut runtime, "main-a");
    let plan = runtime.recovery_plan();
    let mismatched_registry = RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(3),
            kind_name: "other.entity".to_string(),
            schema_id: SchemaId("other".to_string()),
            schema_version_id: SchemaVersionId(2),
        })
        .unwrap();
    let mut recovered = RelationalRuntimeApi::builder()
        .schema_registry(mismatched_registry)
        .build();
    let error = recovered.recover(plan).unwrap_err();

    assert_eq!(error.class, RecoveryFailureClass::SchemaMismatch);
}

#[test]
fn durability_contract_failure_missing_parent_chain_is_explicit() {
    let mut runtime = super::runtime_with_test_schema();
    let parent = super::create_entity_outcome(&mut runtime, "main-a");
    let child = super::create_entity_outcome(&mut runtime, "main-b");
    let child_envelope = runtime
        .canonical_commit_envelope(child.commit.commit_id)
        .cloned()
        .unwrap();
    let corrupt_plan = crate::facade::RecoveryPlan {
        config: runtime.config().clone(),
        checkpoint: None,
        tail_log: vec![DurableCommitEnvelope {
            envelope: CanonicalCommitEnvelope {
                commit: child_envelope.commit.clone(),
                ..child_envelope
            },
        }],
    };
    let mut recovered = super::runtime_with_test_schema();
    let error = recovered.recover(corrupt_plan).unwrap_err();

    assert_eq!(parent.commit.commit_id.0, 1);
    assert_eq!(error.class, RecoveryFailureClass::MissingParentChain);
}

#[test]
fn durability_contract_recovery_preserves_merge_parent_order() {
    let mut runtime = super::runtime_with_test_schema();
    let main = super::create_entity_outcome(&mut runtime, "main");
    runtime
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let feature = super::create_entity_outcome_on_branch(
        &mut runtime,
        "feature",
        BranchId("feature".to_string()),
    );
    let merge = super::merge_commit_from_branches(
        &mut runtime,
        BranchId("main".to_string()),
        vec![BranchId("feature".to_string())],
    );
    let plan = runtime.recovery_plan();
    let mut recovered = super::runtime_with_test_schema();
    recovered.recover(plan).unwrap();
    let recovered_merge = recovered
        .canonical_commit_envelope(merge.commit.commit_id)
        .unwrap();

    assert_eq!(
        recovered_merge.commit.parents,
        vec![main.commit.commit_id, feature.commit.commit_id]
    );
    assert_eq!(
        recovered_merge.merge_parent_branches,
        vec![BranchId("feature".to_string())]
    );
    assert_eq!(
        recovered_merge.merge_base_commits,
        vec![main.commit.commit_id]
    );
}

#[test]
fn durability_contract_checkpoint_tail_recovery_preserves_post_checkpoint_commits() {
    let mut runtime = super::runtime_with_test_schema();
    let main = super::create_entity_outcome(&mut runtime, "main-a");
    let _checkpoint = runtime.checkpoint().unwrap();
    let later = super::create_entity_outcome(&mut runtime, "main-b");
    let plan = runtime.recovery_plan();
    let mut recovered = super::runtime_with_test_schema();
    let outcome = recovered.recover(plan).unwrap();

    assert_eq!(outcome.recovered_commits, 2);
    assert_eq!(outcome.latest_commit, Some(later.commit.clone()));
    assert_eq!(
        recovered.branch_head(&BranchId("main".to_string())),
        Some(&later.commit)
    );
    assert_eq!(
        recovered
            .canonical_commit_envelope(main.commit.commit_id)
            .unwrap()
            .commit
            .commit_id,
        main.commit.commit_id
    );
}

#[test]
fn durability_contract_checkpoint_recovers_index_metadata() {
    let mut runtime = super::runtime_with_test_schema();
    let commit = super::create_entity_outcome(&mut runtime, "indexed");
    let index = runtime.register_index(DerivedIndexDefinition {
        index_id: crate::facade::DerivedIndexId(0),
        name: "entity-name".to_string(),
        kind: DerivedIndexKind::EntityPayloadField {
            field: "name".to_string(),
        },
        branch_scoped: false,
    });
    let build = runtime.build_indexes_for_commit(DerivedIndexBuildRequest {
        source_commit_id: commit.commit.commit_id,
        branch_id: BranchId("main".to_string()),
        index_ids: vec![index.index_id],
    });
    runtime.checkpoint().unwrap();
    let plan = runtime.recovery_plan();
    let mut recovered = super::runtime_with_test_schema();
    recovered.recover(plan).unwrap();

    let generation = recovered
        .latest_index_generation(index.index_id, &BranchId("main".to_string()))
        .unwrap();
    assert_eq!(generation.generation_id, build.generations[0].generation_id);
    assert_eq!(generation.source_commit_id, commit.commit.commit_id);
}

#[test]
fn durability_contract_checkpoint_recovers_lineage_metadata() {
    let mut runtime = super::runtime_with_test_schema();
    let first = super::create_entity_outcome(&mut runtime, "first");
    let second = super::create_entity_outcome(&mut runtime, "second");
    let first_lineage = runtime
        .lineage_for_record(super::changed_entities(&first)[0])
        .unwrap()
        .lineage_id;
    let second_lineage = runtime
        .lineage_for_record(super::changed_entities(&second)[0])
        .unwrap()
        .lineage_id;
    let candidate = runtime.record_correspondence_candidate(
        BranchId("main".to_string()),
        vec![first_lineage],
        vec![second_lineage],
        "recover-me",
    );
    runtime
        .promote_correspondence(candidate.candidate_id, second.commit.clone())
        .unwrap();
    runtime.checkpoint().unwrap();
    let plan = runtime.recovery_plan();
    let mut recovered = super::runtime_with_test_schema();
    recovered.recover(plan).unwrap();
    let graph = recovered.lineage_graph(&BranchId("main".to_string()));

    assert_eq!(graph.nodes.len(), 2);
    assert!(graph
        .events
        .iter()
        .any(|event| event.kind == crate::facade::LineageEventKind::Correspond));
    assert!(graph
        .correspondence_candidates
        .iter()
        .any(|entry| entry.candidate_id == candidate.candidate_id));
}
