use crate::facade::diagnostics::DiagnosticCode;
use crate::facade::publication::PublicationStage;
use crate::facade::storage::RecordLifecycleState;
use crate::facade::transactions::{
    CommitPatchBudgetSummary, CommitPhase, MutationIntent, RelationSpec, TransactionCommitError,
};
use crate::tests::support::*;

#[test]
fn failed_commit_carries_attempt_log() {
    let mut runtime = runtime_with_test_schema();
    let entity = create_entity(&mut runtime, "first");
    delete_entity(&mut runtime, entity);

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("stale-update").push(MutationIntent::Entity(
            EntityMutationIntent::UpdateFields(UpdateEntityFieldsIntent {
                entity_id: entity,
                fields: crate::tests::support::single_string_aspect_field_patch(
                    crate::tests::support::aspect_key("name"),
                    crate::tests::support::field_key("name"),
                    "stale",
                ),
            }),
        )),
    );
    let error = txn.commit().unwrap_err();

    assert!(!error.commit_log().events().is_empty());
    assert!(error
        .commit_log()
        .has_phase_started(CommitPhase::DraftPreparation));
    assert_eq!(
        error.commit_summary().phase_count,
        error.commit_log().summary().phase_count
    );
    assert!(error
        .commit_log()
        .has_rejection_code(DiagnosticCode::StaleHandle));
}

#[test]
fn patch_budget_failure_carries_artifact_phase_decision_trace() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .publication(PublicationConfig {
            coherent_publication_required: true,
            max_patch_records_per_commit: 0,
            max_published_snapshot_handles: 8,
        })
        .build();
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(batch_create("budget-fail"));
    let error = txn.commit().unwrap_err();

    assert!(matches!(
        error,
        TransactionCommitError::Publication { error: ref publication, .. }
            if publication.stage == PublicationStage::BundleAssembly
    ));
    assert_eq!(
        error.commit_log().patch_budget_summary_event(),
        Some(&CommitPatchBudgetSummary {
            patch_record_count: 1,
            max_patch_records_per_commit: 0,
        })
    );
    assert!(error.commit_log().has_rejection(
        CommitPhase::ArtifactAssembly,
        None,
        Some(PublicationStage::BundleAssembly)
    ));
}

#[test]
fn stale_entity_ids_are_rejected() {
    let mut runtime = runtime_with_test_schema();
    let entity = create_entity(&mut runtime, "first");
    delete_entity(&mut runtime, entity);
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("update").push(MutationIntent::Entity(
            EntityMutationIntent::UpdateFields(UpdateEntityFieldsIntent {
                entity_id: entity,
                fields: crate::tests::support::single_string_aspect_field_patch(
                    crate::tests::support::aspect_key("name"),
                    crate::tests::support::field_key("name"),
                    "stale",
                ),
            }),
        )),
    );
    let error = txn.commit().unwrap_err();

    assert!(matches!(
        error,
        TransactionCommitError::Conflict { error: ref conflict, .. } if conflict.code == DiagnosticCode::StaleHandle
    ));
}

#[test]
fn unknown_entity_kind_fails_explicitly() {
    let mut runtime = runtime_with_test_schema();
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("unknown-kind").push(MutationIntent::Create(CreateIntent::Entity(
            crate::transactions::data::EntitySpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(999),
                client_key: crate::symbols::data::ClientKey::raw("bad"),
                fields: crate::tests::support::single_string_aspect_field_patch(
                    crate::tests::support::aspect_key("name"),
                    crate::tests::support::field_key("name"),
                    "bad",
                ),
            },
        ))),
    );
    let error = txn.commit().unwrap_err();

    assert!(matches!(
        error,
        TransactionCommitError::Conflict { error: ref conflict, .. } if conflict.code == DiagnosticCode::InvariantViolation
    ));
}

#[test]
fn duplicate_relation_identity_is_rejected() {
    let mut runtime = runtime_with_test_schema();
    let source_outcome = create_entity_outcome(&mut runtime, "source");
    let target_outcome = create_entity_outcome(&mut runtime, "target");
    let source = changed_entities(&source_outcome)[0];
    let target = changed_entities(&target_outcome)[0];
    create_relation(&mut runtime, source, target, "r1");

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("duplicate").push(MutationIntent::Create(CreateIntent::Relation(
            RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: crate::symbols::data::ClientKey::raw("r2"),
                source: crate::transactions::data::EntityReference::Existing(source),
                target: crate::transactions::data::EntityReference::Existing(target),
                fields: crate::transactions::data::AspectFieldPatch::default(),
            },
        ))),
    );
    let error = txn.commit().unwrap_err();

    assert!(matches!(
        error,
        TransactionCommitError::Conflict { error: ref conflict, .. }
            if conflict.code == DiagnosticCode::DuplicateRelationIdentity
    ));
}

#[test]
fn savepoint_rollback_discards_inner_work_only() {
    let mut runtime = runtime_with_test_schema();
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(batch_create("outer"));
    let savepoint = txn.create_savepoint();
    txn.push_batch(batch_create("inner"));
    let rollback = txn.rollback_to_savepoint(savepoint).unwrap();
    let outcome = txn.commit().unwrap();
    let read = runtime
        .read_truth()
        .read_snapshot(&outcome.snapshot)
        .unwrap();
    let rollback_summary = rollback.summary();

    assert!(rollback_summary.has_discarded_entity_creation());
    assert!(rollback.has_effects());
    assert_eq!(rollback_summary.discarded_entity_creation_count, 1);
    assert_eq!(rollback_summary.restored_entity_count, 0);
    assert_eq!(read.entities().len(), 1);
}

#[test]
fn snapshot_audit_failure_discards_only_touched_overlay() {
    let mut runtime = runtime_with_test_schema_and_invariants(InvariantCatalog {
        registrations: vec![InvariantRegistration::snapshot_publication_blocking(
            InvariantRule::MaxSnapshotEntities(1),
        )],
        ..InvariantCatalog::default()
    });
    let baseline = create_entity_outcome(&mut runtime, "baseline");

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(batch_create("blocked"));
    let error = txn.commit().unwrap_err();
    let committed_read = runtime
        .read_truth()
        .read_snapshot(&baseline.snapshot)
        .unwrap();

    assert!(matches!(
        error,
        TransactionCommitError::Publication { error: ref publication, .. }
            if publication.stage == PublicationStage::InvariantCheck
    ));
    assert_eq!(committed_read.entities().len(), 1);
    assert!(committed_read
        .entities()
        .iter()
        .any(|record| read_entity_name(record) == Some("baseline".into())));
    assert_eq!(
        runtime.history().latest_commit().unwrap().commit_id,
        baseline.commit.commit_id
    );
}

#[test]
fn audit_retained_relations_remain_visible_after_endpoint_delete() {
    let schema = RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "test.entity".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            aspect_contract_declarations: KindAspectContractDeclarations::default(),
        })
        .and_then(|registry| {
            registry.register_relation_kind(RelationKindRegistration {
                kind_id: KindId(2),
                kind_name: "test.relation".to_string(),
                schema_id: SchemaId("test".to_string()),
                schema_version_id: SchemaVersionId(1),
                cross_context_policy: CrossContextPolicy::AllowExplicit,
                cascade_delete_policy: CascadeDeletePolicy::RetainDanglingForAudit,
                aspect_contract_declarations: KindAspectContractDeclarations::default(),
                relation_integrity: crate::schema::data::RelationIntegrityDeclarations::default(),
            })
        })
        .unwrap();
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(schema)
        .cascade_delete_policy(CascadeDeletePolicy::RetainDanglingForAudit)
        .build();
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    let relation_outcome = create_relation_outcome(&mut runtime, source, target, "r1");
    let relation = changed_relations(&relation_outcome)[0];
    let deleted = delete_entity(&mut runtime, source);
    let read = runtime
        .read_truth()
        .read_snapshot(&deleted.snapshot)
        .unwrap();
    let relation = read.get_relation(relation).unwrap();

    assert_eq!(
        relation.lifecycle,
        RecordLifecycleState::RetainedDanglingForAudit
    );
    assert_eq!(relation.source, source);
    assert_eq!(relation.target, target);
}

#[test]
fn merged_plan_is_stable_across_batch_order() {
    let mut runtime_a = runtime_with_test_schema();
    let mut txn_a = runtime_a.begin_transaction(TransactionOptions::default());
    txn_a.push_batch(batch_create("b"));
    txn_a.push_batch(batch_create("a"));
    let plan_a = txn_a.merged_plan().unwrap().clone();

    let mut runtime_b = runtime_with_test_schema();
    let mut txn_b = runtime_b.begin_transaction(TransactionOptions::default());
    txn_b.push_batch(batch_create("a"));
    txn_b.push_batch(batch_create("b"));
    let plan_b = txn_b.merged_plan().unwrap().clone();

    assert_eq!(plan_a, plan_b);
}
