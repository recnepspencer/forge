use crate::facade::{
    BranchId, CanonicalCommitEnvelope, DerivedIndexBuildRequest, DerivedIndexDefinition,
    DerivedIndexKind, DurableCommitEnvelope, EntityKindRegistration, KindId, RecoveryFailureClass,
    RecoveryIntegrityReport, RecoveryPlan, RelationalRuntimeApi, RelationalSchemaRegistry,
    SchemaId, SchemaVersionId,
};
use crate::tests::support::*;

// CONTRACT: durability
// LANES: success, failure, recovery

#[test]
fn durability_contract_recovery_rebuilds_branch_heads_and_latest_commit() {
    let mut runtime = persisted_runtime_with_test_schema();
    let main = create_entity_outcome(&mut runtime, "main-a");
    runtime
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let feature = create_entity_outcome_on_branch(
        &mut runtime,
        "feature-a",
        BranchId("feature".to_string()),
    );
    let _checkpoint = runtime.checkpoint().unwrap();
    let plan = runtime.recovery_plan();
    let mut recovered = persisted_runtime_with_test_schema();
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
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "main-a");
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
    let mut runtime = runtime_with_test_schema();
    let parent = create_entity_outcome(&mut runtime, "main-a");
    let child = create_entity_outcome(&mut runtime, "main-b");
    let child_envelope = runtime
        .canonical_commit_envelope(child.commit.commit_id)
        .cloned()
        .unwrap();
    let corrupt_plan = RecoveryPlan {
        config: runtime.config().clone(),
        store: runtime.config().durable_store_layout.clone().map(|layout| {
            crate::facade::DurableStore {
                layout,
                segments: Vec::new(),
                checkpoints: Vec::new(),
            }
        }),
        checkpoint_manifest: None,
        checkpoint: None,
        tail_log: vec![DurableCommitEnvelope {
            envelope: CanonicalCommitEnvelope {
                commit: child_envelope.commit.clone(),
                ..child_envelope
            },
        }],
        cursor: crate::facade::RecoveryCursor {
            checkpoint_id: None,
            segment_ids: Vec::new(),
        },
        integrity_report: RecoveryIntegrityReport {
            selected_checkpoint_id: None,
            skipped_corrupt_checkpoints: Vec::new(),
            verified_segment_ids: Vec::new(),
            corrupt_segment_id: None,
        },
        compatibility: crate::facade::RecoveryCompatibilityCheck {
            schema_match: true,
            profile_match: true,
            runtime_name_match: true,
        },
    };
    let mut recovered = runtime_with_test_schema();
    let error = recovered.recover(corrupt_plan).unwrap_err();

    assert_eq!(parent.commit.commit_id.0, 1);
    assert_eq!(error.class, RecoveryFailureClass::MissingParentChain);
}

#[test]
fn durability_contract_recovery_preserves_merge_parent_order() {
    let mut runtime = persisted_runtime_with_test_schema();
    let main = create_entity_outcome(&mut runtime, "main");
    runtime
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let feature = create_entity_outcome_on_branch(
        &mut runtime,
        "feature",
        BranchId("feature".to_string()),
    );
    let merge = merge_commit_from_branches(
        &mut runtime,
        BranchId("main".to_string()),
        vec![BranchId("feature".to_string())],
    );
    let plan = runtime.recovery_plan();
    let mut recovered = persisted_runtime_with_test_schema();
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
    let mut runtime = persisted_runtime_with_test_schema();
    let main = create_entity_outcome(&mut runtime, "main-a");
    let _checkpoint = runtime.checkpoint().unwrap();
    let later = create_entity_outcome(&mut runtime, "main-b");
    let plan = runtime.recovery_plan();
    let mut recovered = persisted_runtime_with_test_schema();
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
    let mut runtime = persisted_runtime_with_test_schema();
    let commit = create_entity_outcome(&mut runtime, "indexed");
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
    let mut recovered = persisted_runtime_with_test_schema();
    recovered.recover(plan).unwrap();

    let generation = recovered
        .latest_index_generation(index.index_id, &BranchId("main".to_string()))
        .unwrap();
    assert_eq!(generation.generation_id, build.generations[0].generation_id);
    assert_eq!(generation.source_commit_id, commit.commit.commit_id);
}

#[test]
fn durability_contract_checkpoint_recovers_lineage_metadata() {
    let mut runtime = persisted_runtime_with_test_schema();
    let first = create_entity_outcome(&mut runtime, "first");
    let second = create_entity_outcome(&mut runtime, "second");
    let first_lineage = runtime
        .lineage_for_record(changed_entities(&first)[0])
        .unwrap()
        .lineage_id;
    let second_lineage = runtime
        .lineage_for_record(changed_entities(&second)[0])
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
    let mut recovered = persisted_runtime_with_test_schema();
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

#[test]
fn durability_contract_corrupt_latest_checkpoint_falls_back_to_prior_valid_checkpoint() {
    let mut runtime = persisted_runtime_with_test_schema();
    let first = create_entity_outcome(&mut runtime, "first");
    runtime.checkpoint().unwrap();
    let second = create_entity_outcome(&mut runtime, "second");
    runtime.checkpoint().unwrap();
    let store = runtime.recovery_plan().store.unwrap();
    let latest_checkpoint = store.checkpoints.last().unwrap();
    std::fs::write(&latest_checkpoint.path, b"{not-json").unwrap();

    let plan = runtime.recovery_plan();
    let mut recovered = persisted_runtime_with_test_schema();
    let outcome = recovered.recover(plan).unwrap();

    assert_eq!(outcome.latest_commit, Some(second.commit.clone()));
    assert!(!outcome
        .integrity_report
        .skipped_corrupt_checkpoints
        .is_empty());
    assert_eq!(
        recovered.branch_head(&BranchId("main".to_string())),
        Some(&second.commit)
    );
    assert!(recovered
        .canonical_commit_envelope(first.commit.commit_id)
        .is_some());
}

#[test]
fn durability_contract_compaction_only_removes_segments_covered_by_checkpoint() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "a");
    create_entity_outcome(&mut runtime, "b");
    runtime.checkpoint().unwrap();
    create_entity_outcome(&mut runtime, "c");
    let before = runtime.recovery_plan().store.unwrap();

    let compaction = runtime.compact_store().unwrap();
    let after = runtime.recovery_plan().store.unwrap();

    assert!(!before.segments.is_empty());
    assert!(after.segments.len() <= before.segments.len());
    assert_eq!(after.segments.len(), compaction.retained_segments.len());
}

#[test]
fn durability_contract_recovery_rebuilds_branch_pinned_retention_from_branch_heads() {
    let mut runtime = persisted_runtime_with_test_schema();
    let source = create_entity_outcome(&mut runtime, "source");
    let source_entity = changed_entities(&source)[0];
    let target = create_entity_outcome(&mut runtime, "target");
    let target_entity = changed_entities(&target)[0];
    let _relation = create_relation_outcome(&mut runtime, source_entity, target_entity, "r1");
    runtime
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let _deleted = delete_entity(&mut runtime, source_entity);
    runtime.checkpoint().unwrap();
    let plan = runtime.recovery_plan();
    let mut recovered = persisted_runtime_with_test_schema();
    recovered.recover(plan).unwrap();

    let retention = recovered.inspect_retention_plan();
    assert_eq!(retention.active_snapshot_count, 0);
    assert!(retention.branch_pinned_entities >= 1);
    assert!(retention.branch_pinned_relations >= 1);
    assert_eq!(retention.reclaimable_entities, 0);
    assert_eq!(retention.reclaimable_relations, 0);
}

#[test]
fn durability_contract_persisted_commit_fails_closed_when_store_path_is_not_directory() {
    let root_path = unique_test_store_path("forge-relational-bad-store");
    std::fs::write(&root_path, b"not-a-directory").unwrap();
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .durability_mode(crate::facade::DurabilityMode::PersistedSegmentedLocalFs)
        .durable_store_layout(crate::facade::DurableStoreLayout {
            root_path: root_path.clone(),
            segment_commit_capacity: 2,
        })
        .build();

    let mut txn = runtime.begin_transaction(crate::facade::TransactionOptions::default());
    txn.push_batch(batch_create("fail-closed"));
    let error = txn.commit().unwrap_err();

    assert!(matches!(
        error,
        crate::facade::TransactionCommitError::Publication(_)
    ));
    assert!(runtime.latest_commit().is_none());
}
