use crate::facade::durability::{
    DurabilityMode, DurableStore, DurableStoreLayout, RecoveryCompatibilityCheck, RecoveryCursor,
    RecoveryFailureClass, RecoveryIntegrityReport, RecoveryPlan,
};
use crate::facade::history::BranchId;
use crate::facade::identity::KindId;
use crate::facade::indexes::{
    DerivedIndexBuildRequest, DerivedIndexDefinition, DerivedIndexId, DerivedIndexKind,
};
use crate::facade::lineage::LineageEventKind;
use crate::facade::replay::CanonicalCommitEnvelope;
use crate::facade::runtime::RelationalRuntimeApi;
use crate::facade::schema::{
    EntityKindRegistration, KindAspectDeclarations, RelationalSchemaRegistry, SchemaId,
    SchemaVersionId,
};
use crate::facade::transactions::{TransactionCommitError, TransactionOptions};
use crate::tests::support::*;

// CONTRACT: durability
// LANES: success, failure, recovery

#[test]
fn durability_contract_recovery_rebuilds_branch_heads_and_latest_commit() {
    let mut runtime = persisted_runtime_with_test_schema();
    let main = create_entity_outcome(&mut runtime, "main-a");
    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let feature =
        create_entity_outcome_on_branch(&mut runtime, "feature-a", BranchId("feature".to_string()));
    let _checkpoint = runtime.durability_authority().checkpoint().unwrap();
    let plan = runtime.durability_access().recovery_plan();
    let mut recovered = persisted_runtime_with_test_schema();
    let outcome = recovered.durability_authority().recover(plan).unwrap();

    assert_eq!(outcome.recovered_commits, 2);
    assert_eq!(outcome.latest_commit, Some(feature.commit.clone()));
    assert_eq!(
        recovered
            .history_access()
            .branch_head(&BranchId("feature".to_string())),
        Some(&feature.commit)
    );
    assert_eq!(
        recovered
            .history_access()
            .branch_head(&BranchId("main".to_string())),
        Some(&main.commit)
    );
}

#[test]
fn durability_contract_recovery_preserves_aspect_bearing_patch_truth_and_history() {
    let mut runtime =
        persisted_runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    let created = create_entity_outcome(&mut runtime, "before");
    let entity = changed_entities(&created)[0];
    let updated = update_entity(&mut runtime, entity, "after");
    let expected_history =
        runtime
            .history_access()
            .entity_aspect_history(&BranchId("main".to_string()), entity, None);
    let expected_digest = runtime
        .history_access()
        .entity_aspect_history_with_trace(&BranchId("main".to_string()), entity, None)
        .aspect_history_digest();
    let expected_envelope = runtime
        .replay_access()
        .canonical_commit_envelope(updated.commit.commit_id)
        .cloned()
        .unwrap();
    let plan = runtime.durability_access().recovery_plan();
    let mut recovered =
        persisted_runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    let outcome = recovered.durability_authority().recover(plan).unwrap();

    let recovered_history = recovered.history_access().entity_aspect_history(
        &BranchId("main".to_string()),
        entity,
        None,
    );
    let recovered_digest = recovered
        .history_access()
        .entity_aspect_history_with_trace(&BranchId("main".to_string()), entity, None)
        .aspect_history_digest();
    let recovered_replay = recovered.replay_access();
    let recovered_envelope = recovered_replay
        .canonical_commit_envelope(updated.commit.commit_id)
        .unwrap();

    assert_eq!(outcome.latest_commit, Some(updated.commit.clone()));
    assert_eq!(expected_history, recovered_history);
    assert_eq!(expected_digest, recovered_digest);
    assert_eq!(
        expected_envelope.patch.records,
        recovered_envelope.patch.records
    );
    assert_eq!(
        recovered_envelope.patch.records[0].aspects,
        CanonicalAspectSet::new([aspect_key("lifecycle"), aspect_key("name")])
    );
    assert!(!recovered_envelope.patch.records[0].contains_degraded_precision);
}

#[test]
fn durability_contract_recovery_preserves_relation_aspect_history_for_retained_audit_relations() {
    let mut runtime =
        persisted_runtime_with_declared_aspect_schema(CascadeDeletePolicy::RetainDanglingForAudit);
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    let relation_outcome = create_relation_outcome(&mut runtime, source, target, "r-audit");
    let relation = changed_relations(&relation_outcome)[0];
    let deleted = delete_entity(&mut runtime, source);
    let expected_history = runtime.history_access().relation_aspect_history(
        &BranchId("main".to_string()),
        relation,
        None,
    );
    let expected_digest = runtime
        .history_access()
        .relation_aspect_history_with_trace(&BranchId("main".to_string()), relation, None)
        .aspect_history_digest();
    let plan = runtime.durability_access().recovery_plan();
    let mut recovered =
        persisted_runtime_with_declared_aspect_schema(CascadeDeletePolicy::RetainDanglingForAudit);
    let outcome = recovered.durability_authority().recover(plan).unwrap();

    let recovered_history = recovered.history_access().relation_aspect_history(
        &BranchId("main".to_string()),
        relation,
        None,
    );
    let recovered_digest = recovered
        .history_access()
        .relation_aspect_history_with_trace(&BranchId("main".to_string()), relation, None)
        .aspect_history_digest();

    assert_eq!(outcome.latest_commit, Some(deleted.commit.clone()));
    assert_eq!(expected_history, recovered_history);
    assert_eq!(expected_digest, recovered_digest);
    assert_eq!(recovered_history.len(), 2);
    assert_direct_history_origin_invariants(&recovered_history, RecordRef::Relation(relation));
    assert_eq!(
        recovered_history[0].origin.changed_aspects,
        CanonicalAspectSet::new([
            aspect_key("label"),
            aspect_key("lifecycle"),
            aspect_key("source"),
            aspect_key("target"),
        ])
    );
    assert_eq!(
        recovered_history[1].origin.changed_aspects,
        CanonicalAspectSet::new([aspect_key("lifecycle")])
    );
}

#[test]
fn durability_contract_failure_schema_mismatch_is_explicit() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "main-a");
    let plan = runtime.durability_access().recovery_plan();
    let mismatched_registry = RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(3),
            kind_name: "other.entity".to_string(),
            schema_id: SchemaId("other".to_string()),
            schema_version_id: SchemaVersionId(2),
            aspect_declarations: KindAspectDeclarations::default(),
        })
        .unwrap();
    let mut recovered = RelationalRuntimeApi::builder()
        .schema_registry(mismatched_registry)
        .build();
    let error = recovered.durability_authority().recover(plan).unwrap_err();

    assert_eq!(error.class, RecoveryFailureClass::SchemaMismatch);
}

#[test]
fn durability_contract_failure_aspect_plan_mismatch_is_explicit() {
    let mut runtime =
        persisted_runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    create_entity_outcome(&mut runtime, "main-a");
    let plan = runtime.durability_access().recovery_plan();
    let expected_registry =
        declared_aspect_schema_registry(CascadeDeletePolicy::CascadeDeleteRelations);
    let mismatched_registry = AspectSchemaFixture {
        entity_aspects: vec![
            entity_payload_aspect("display_name", "name"),
            lifecycle_aspect(),
        ],
        relation_aspects: vec![
            relation_payload_aspect("label", "label"),
            lifecycle_aspect(),
            relation_source_aspect(),
            relation_target_aspect(),
        ],
        ..AspectSchemaFixture::with_default_declared_aspects(
            CascadeDeletePolicy::CascadeDeleteRelations,
        )
    }
    .build_registry();
    let expected_revision = expected_registry
        .entity_aspect_declaration_trace(KindId(1))
        .unwrap()
        .plan_revision;
    let mismatched_revision = mismatched_registry
        .entity_aspect_declaration_trace(KindId(1))
        .unwrap()
        .plan_revision;
    let mut recovered = RelationalRuntimeApi::builder()
        .schema_registry(mismatched_registry)
        .build();
    let error = recovered.durability_authority().recover(plan).unwrap_err();

    assert_ne!(expected_revision, mismatched_revision);
    assert_eq!(error.class, RecoveryFailureClass::SchemaMismatch);
}

#[test]
fn durability_contract_failure_missing_parent_chain_is_explicit() {
    let mut runtime = runtime_with_test_schema();
    let parent = create_entity_outcome(&mut runtime, "main-a");
    let child = create_entity_outcome(&mut runtime, "main-b");
    let child_envelope = runtime
        .replay_access()
        .canonical_commit_envelope(child.commit.commit_id)
        .cloned()
        .unwrap();
    let corrupt_plan = RecoveryPlan {
        config: runtime.config().clone(),
        store: runtime
            .config()
            .durability
            .policy
            .store_layout
            .clone()
            .map(|layout| DurableStore {
                layout,
                segments: Vec::new(),
                checkpoints: Vec::new(),
            }),
        checkpoint_manifest: None,
        checkpoint: None,
        tail_log: vec![CanonicalCommitEnvelope {
            commit: child_envelope.commit.clone(),
            ..child_envelope
        }],
        cursor: RecoveryCursor {
            checkpoint_id: None,
            segment_ids: Vec::new(),
        },
        integrity_report: RecoveryIntegrityReport {
            selected_checkpoint_id: None,
            skipped_corrupt_checkpoints: Vec::new(),
            verified_segment_ids: Vec::new(),
            corrupt_segment_id: None,
        },
        compatibility: RecoveryCompatibilityCheck {
            schema_match: true,
            profile_match: true,
            runtime_name_match: true,
        },
    };
    let mut recovered = runtime_with_test_schema();
    let error = recovered
        .durability_authority()
        .recover(corrupt_plan)
        .unwrap_err();

    assert_eq!(parent.commit.commit_id.0, 1);
    assert_eq!(error.class, RecoveryFailureClass::MissingParentChain);
}

#[test]
fn durability_contract_recovery_preserves_merge_parent_order() {
    let mut runtime = persisted_runtime_with_test_schema();
    let main = create_entity_outcome(&mut runtime, "main");
    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let feature =
        create_entity_outcome_on_branch(&mut runtime, "feature", BranchId("feature".to_string()));
    let merge = merge_commit_from_branches(
        &mut runtime,
        BranchId("main".to_string()),
        vec![BranchId("feature".to_string())],
    );
    let plan = runtime.durability_access().recovery_plan();
    let mut recovered = persisted_runtime_with_test_schema();
    recovered.durability_authority().recover(plan).unwrap();
    let replay = recovered.replay_access();
    let recovered_merge = replay
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
    let _checkpoint = runtime.durability_authority().checkpoint().unwrap();
    let later = create_entity_outcome(&mut runtime, "main-b");
    let plan = runtime.durability_access().recovery_plan();
    let mut recovered = persisted_runtime_with_test_schema();
    let outcome = recovered.durability_authority().recover(plan).unwrap();

    assert_eq!(outcome.recovered_commits, 2);
    assert_eq!(outcome.latest_commit, Some(later.commit.clone()));
    assert_eq!(
        recovered
            .history_access()
            .branch_head(&BranchId("main".to_string())),
        Some(&later.commit)
    );
    assert_eq!(
        recovered
            .replay_access()
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
    let index = runtime.index_authority().register(DerivedIndexDefinition {
        index_id: DerivedIndexId(0),
        name: "entity-name".to_string(),
        kind: DerivedIndexKind::EntityPayloadField {
            field: "name".to_string(),
        },
        branch_scoped: false,
    });
    let build = runtime
        .index_authority()
        .build_for_commit(DerivedIndexBuildRequest {
            source_commit_id: commit.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            index_ids: vec![index.index_id],
        });
    runtime.durability_authority().checkpoint().unwrap();
    let plan = runtime.durability_access().recovery_plan();
    let mut recovered = persisted_runtime_with_test_schema();
    recovered.durability_authority().recover(plan).unwrap();

    let index_access = recovered.index_access();
    let generation = index_access
        .latest_generation(index.index_id, &BranchId("main".to_string()))
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
        .lineage_access()
        .for_record(changed_entities(&first)[0])
        .unwrap()
        .lineage_id;
    let second_lineage = runtime
        .lineage_access()
        .for_record(changed_entities(&second)[0])
        .unwrap()
        .lineage_id;
    let candidate = runtime.lineage_authority().record_correspondence_candidate(
        BranchId("main".to_string()),
        vec![first_lineage],
        vec![second_lineage],
        "recover-me",
    );
    runtime
        .lineage_authority()
        .promote_correspondence(candidate.candidate_id, second.commit.clone())
        .unwrap();
    runtime.durability_authority().checkpoint().unwrap();
    let plan = runtime.durability_access().recovery_plan();
    let mut recovered = persisted_runtime_with_test_schema();
    recovered.durability_authority().recover(plan).unwrap();
    let graph = recovered
        .lineage_access()
        .graph(&BranchId("main".to_string()));

    assert_eq!(graph.nodes.len(), 2);
    assert!(graph
        .events
        .iter()
        .any(|event| event.kind == LineageEventKind::Correspond));
    assert!(graph
        .correspondence_candidates
        .iter()
        .any(|entry| entry.candidate_id == candidate.candidate_id));
}

#[test]
fn durability_contract_corrupt_latest_checkpoint_falls_back_to_prior_valid_checkpoint() {
    let mut runtime = persisted_runtime_with_test_schema();
    let first = create_entity_outcome(&mut runtime, "first");
    runtime.durability_authority().checkpoint().unwrap();
    let second = create_entity_outcome(&mut runtime, "second");
    runtime.durability_authority().checkpoint().unwrap();
    let store = runtime.durability_access().recovery_plan().store.unwrap();
    let latest_checkpoint = store.checkpoints.last().unwrap();
    std::fs::write(&latest_checkpoint.path, b"{not-json").unwrap();

    let plan = runtime.durability_access().recovery_plan();
    let mut recovered = persisted_runtime_with_test_schema();
    let outcome = recovered.durability_authority().recover(plan).unwrap();

    assert_eq!(outcome.latest_commit, Some(second.commit.clone()));
    assert!(!outcome
        .integrity_report
        .skipped_corrupt_checkpoints
        .is_empty());
    assert_eq!(
        recovered
            .history_access()
            .branch_head(&BranchId("main".to_string())),
        Some(&second.commit)
    );
    assert!(recovered
        .replay_access()
        .canonical_commit_envelope(first.commit.commit_id)
        .is_some());
}

#[test]
fn durability_contract_compaction_only_removes_segments_covered_by_checkpoint() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "a");
    create_entity_outcome(&mut runtime, "b");
    runtime.durability_authority().checkpoint().unwrap();
    create_entity_outcome(&mut runtime, "c");
    let before = runtime.durability_access().recovery_plan().store.unwrap();

    let compaction = runtime.durability_authority().compact_store().unwrap();
    let after = runtime.durability_access().recovery_plan().store.unwrap();

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
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let _deleted = delete_entity(&mut runtime, source_entity);
    runtime.durability_authority().checkpoint().unwrap();
    let plan = runtime.durability_access().recovery_plan();
    let mut recovered = persisted_runtime_with_test_schema();
    recovered.durability_authority().recover(plan).unwrap();

    let retention = recovered.retention_authority().inspect_plan();
    assert_eq!(retention.active_snapshot_count, 0);
    assert!(retention.branch_pinned_entities >= 1);
    assert!(retention.branch_pinned_relations >= 1);
    assert_eq!(retention.reclaimable_entities, 0);
    assert_eq!(retention.reclaimable_relations, 0);
}

#[test]
fn durability_contract_recovery_preserves_inspection_truth_bundle() {
    let mut runtime = persisted_runtime_with_test_schema();
    let created = create_entity_outcome(&mut runtime, "source");
    let entity = changed_entities(&created)[0];
    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let _main_update = update_entity(&mut runtime, entity, "main");
    let _feature_update = {
        let mut txn = runtime.begin_transaction(TransactionOptions {
            target_branch: Some(BranchId("feature".to_string())),
            ..TransactionOptions::default()
        });
        txn.push_batch(WorkerIntentBatch::new("feature-update").push(MutationIntent::Entity(
            EntityMutationIntent::Update(UpdateEntityIntent {
                entity_id: entity,
                payload: RecordPayload::StructuredJson(json!({"name":"feature"})),
            }),
        )));
        txn.commit().unwrap()
    };
    runtime.durability_authority().checkpoint().unwrap();
    let expected = capture_inspection_truth_bundle(
        &runtime,
        &BranchId("feature".to_string()),
        entity,
        created.version_id,
    );

    let plan = runtime.durability_access().recovery_plan();
    let mut recovered = persisted_runtime_with_test_schema();
    recovered.durability_authority().recover(plan).unwrap();
    let actual = capture_inspection_truth_bundle(
        &recovered,
        &BranchId("feature".to_string()),
        entity,
        created.version_id,
    );

    assert_eq!(expected, actual);
}

#[test]
fn durability_contract_live_branch_pin_counts_match_branch_head_membership() {
    let mut runtime = persisted_runtime_with_test_schema();
    let created = create_entity_outcome(&mut runtime, "source");
    let entity = changed_entities(&created)[0];
    let inspection = runtime.inspection_access();
    assert_eq!(
        inspection
            .inspect_record_retention(RecordRef::Entity(entity))
            .expect("entity retention after create")
            .pins
            .branch_pins,
        1
    );

    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let inspection = runtime.inspection_access();
    assert_eq!(
        inspection
            .inspect_record_retention(RecordRef::Entity(entity))
            .expect("entity retention after branch create")
            .pins
            .branch_pins,
        2
    );

    update_entity(&mut runtime, entity, "main");
    let inspection = runtime.inspection_access();
    assert_eq!(
        inspection
            .inspect_record_retention(RecordRef::Entity(entity))
            .expect("entity retention after main update")
            .pins
            .branch_pins,
        2
    );

    update_entity_on_branch(
        &mut runtime,
        entity,
        "feature",
        BranchId("feature".to_string()),
    );
    let inspection = runtime.inspection_access();
    assert_eq!(
        inspection
            .inspect_record_retention(RecordRef::Entity(entity))
            .expect("entity retention after feature update")
            .pins
            .branch_pins,
        2
    );
}

#[test]
fn durability_contract_persisted_commit_fails_closed_when_store_path_is_not_directory() {
    let root_path = unique_test_store_path("forge-relational-bad-store");
    std::fs::write(&root_path, b"not-a-directory").unwrap();
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .durability_mode(DurabilityMode::PersistedSegmentedLocalFs)
        .durable_store_layout(DurableStoreLayout {
            root_path: root_path.clone(),
            segment_commit_capacity: 2,
        })
        .build();

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(batch_create("fail-closed"));
    let error = txn.commit().unwrap_err();

    assert!(matches!(error, TransactionCommitError::Publication { .. }));
    assert!(runtime.history_access().latest_commit().is_none());
}
