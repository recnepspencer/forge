use crate::facade::config::{
    ConfigValueSource, MvccConfig, RetentionBackend, SnapshotReleasePolicy,
};
use crate::facade::diagnostics::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticsProfile,
};
use crate::facade::durability::{DurabilityError, RecoveryFailureClass};
use crate::facade::errors::{RelationalError, RelationalSubsystem};
use crate::facade::history::BranchCreateError;
use crate::facade::identity::{EntityId, EntityStorageId, RelationId, RelationStorageId};
use crate::facade::publication::{PublicationError, PublicationStage};
use crate::facade::replay::{ReplayError, ReplayFailureClass};
use crate::facade::runtime::{InvariantExecutionPoint, RelationalExecutionModel};
use crate::facade::schema::SchemaRegistryError;
use crate::facade::storage::RecordLifecycleState;
use crate::facade::transactions::{
    AspectTraceEvidence, AuthorityMode, CommitPatchBudgetSummary, CommitPhase, CommitTopology,
    CommitTraceEvent, EntitySpec, MutationIntent, RelationSpec, TransactionCommitError,
};
use crate::publication::patch::data::{
    PublishedAuthoritativePatchOperation, PublishedAuthoritativePatchValue,
};
use crate::tests::support::*;
use forge_foundational::facade::{
    AspectKey, AspectValue, ContractValidatedAspectValueView, FieldKey,
};

mod relation_integrity;
mod relation_updates;

#[test]
fn runtime_defaults_to_serialized_authority() {
    let runtime = runtime_with_test_schema();

    assert_eq!(
        runtime.config().execution.execution_model,
        RelationalExecutionModel::SerialAuthority
    );
    assert_eq!(
        runtime.config().execution.commit_authority.authority.mode,
        AuthorityMode::SerializedCommit
    );
}

#[test]
fn harness_defaults_require_determinism_and_parity() {
    let expectations = crate::facade::harness::default_harness_expectations();
    assert!(expectations.serial_parallel_parity_required);
}

#[test]
fn tagged_record_ids_preserve_storage_identity() {
    let entity_id = EntityId::new(PartitionId(7), 11, 3);
    let relation_id = RelationId::new(PartitionId(9), 13, 4);

    let entity_storage: EntityStorageId = entity_id.storage_id();
    let relation_storage: RelationStorageId = relation_id.storage_id();

    assert_eq!(entity_storage.partition_id, PartitionId(7));
    assert_eq!(entity_storage.local_slot.0, 11);
    assert_eq!(relation_storage.partition_id, PartitionId(9));
    assert_eq!(relation_storage.local_slot.0, 13);
    assert_ne!(entity_id.partition_id, relation_id.partition_id);
}

#[test]
fn relational_error_wraps_authority_failures_with_context() {
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
    let transaction_error = txn.commit().unwrap_err();
    let wrapped: RelationalError = transaction_error.into();
    assert!(matches!(wrapped, RelationalError::Transaction(_)));
    assert_eq!(
        wrapped.context().subsystem,
        RelationalSubsystem::Transaction
    );

    let wrapped: RelationalError = SchemaRegistryError::unknown_entity_kind(KindId(999)).into();
    assert!(matches!(wrapped, RelationalError::Schema(_)));
    assert_eq!(wrapped.context().subsystem, RelationalSubsystem::Schema);

    let wrapped: RelationalError = BranchCreateError::branch_already_exists().into();
    assert!(matches!(wrapped, RelationalError::History(_)));
    assert_eq!(wrapped.context().subsystem, RelationalSubsystem::History);

    let wrapped: RelationalError =
        PublicationError::new(PublicationStage::Visibility, "publication failed").into();
    assert!(matches!(wrapped, RelationalError::Publication(_)));

    let wrapped: RelationalError =
        DurabilityError::new(RecoveryFailureClass::DurableIoFailure, "durability failed").into();
    assert!(matches!(wrapped, RelationalError::Durability(_)));

    let wrapped: RelationalError =
        ReplayError::new(ReplayFailureClass::SchemaMismatch, "replay failed").into();
    assert!(matches!(wrapped, RelationalError::Replay(_)));
}

#[test]
fn transaction_intent_is_the_shared_mutation_intent_type() {
    let create = MutationIntent::Create(CreateIntent::Entity(
        crate::transactions::data::EntitySpec {
            partition_id: PartitionId::main(),
            kind_id: KindId(1),
            client_key: crate::symbols::data::ClientKey::raw("alias"),
            fields: crate::tests::support::single_string_aspect_field_patch(
                crate::tests::support::aspect_key("name"),
                crate::tests::support::field_key("name"),
                "alias",
            ),
        },
    ));
    let transaction_intent: MutationIntent = create.clone();

    assert_eq!(transaction_intent, create);
}

#[test]
fn update_entity_fields_rejects_undeclared_aspect_targets() {
    let mut runtime =
        runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    let entity = create_entity(&mut runtime, "field-guard");

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("update-fields-undeclared").push(MutationIntent::Entity(
            crate::transactions::data::EntityMutationIntent::UpdateFields(
                crate::transactions::data::UpdateEntityFieldsIntent {
                    entity_id: entity,
                    fields: crate::transactions::data::AspectFieldPatch::from(
                        std::collections::BTreeMap::from([(
                            crate::transactions::data::planned_single_field_locator(
                                forge_foundational::facade::AspectKey::new("undeclared")
                                    .expect("valid test aspect key"),
                                forge_foundational::facade::FieldKey::new("undeclared").unwrap(),
                            ),
                            forge_foundational::facade::AspectValue::String(
                                forge_foundational::facade::InternedString::Raw("nope".to_string()),
                            ),
                        )]),
                    ),
                },
            ),
        )),
    );

    let error = txn.commit().unwrap_err();
    match error {
        crate::transactions::data::TransactionCommitError::Conflict { error, .. } => {
            match error.class {
                crate::transactions::data::ConflictClass::EntityFieldAspectPatchDenied {
                    denial:
                        crate::transactions::data::EntityFieldAspectPatchDenial::UndeclaredEntityAspectTarget {
                            ref field_locator,
                            ..
                        },
                    ..
                } => assert_eq!(field_locator.aspect().aspect_key().as_str(), "undeclared"),
                other => panic!("expected typed entity field aspect patch denial, got {other:?}"),
            }
            assert!(error.detail.contains("targets undeclared aspect"));
        }
        other => panic!("expected conflict error, got {other:?}"),
    }
}

#[test]
fn update_entity_fields_state_conflict_is_typed_not_json() {
    let entity_id = EntityId::new(PartitionId::main(), 7, 0);
    let conflict = crate::transactions::data::CommitConflict::new(
        crate::transactions::data::ConflictClass::EntityFieldUpdateStateInconsistency {
            entity_id,
            missing:
                crate::transactions::data::EntityFieldUpdateMissingState::AuthoritativeAspectState,
        },
    );

    match conflict.class {
        crate::transactions::data::ConflictClass::EntityFieldUpdateStateInconsistency {
            entity_id: actual_entity_id,
            missing,
        } => {
            assert_eq!(actual_entity_id, entity_id);
            assert_eq!(
                missing,
                crate::transactions::data::EntityFieldUpdateMissingState::AuthoritativeAspectState
            );
        }
        other => panic!("expected typed entity field update state conflict, got {other:?}"),
    }
    assert!(conflict
        .detail
        .contains("retained authoritative aspect state after stale-target validation"));
}

#[test]
fn update_entity_fields_canonical_delta_uses_authoritative_patch_evidence() {
    let mut runtime =
        runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    let entity = create_entity(&mut runtime, "before");

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("field-patch").push(MutationIntent::Entity(
            EntityMutationIntent::UpdateFields(
                crate::transactions::data::UpdateEntityFieldsIntent {
                    entity_id: entity,
                    fields: crate::transactions::data::AspectFieldPatch::from_locator(
                        crate::transactions::data::planned_single_field_locator(
                            forge_foundational::facade::AspectKey::new("name")
                                .expect("valid test aspect key"),
                            FieldKey::new("name").expect("valid test field key"),
                        ),
                        forge_foundational::facade::AspectValue::String("after".into()),
                    ),
                },
            ),
        )),
    );
    let outcome = txn.commit().unwrap();
    let patch_record = &outcome.patch()[0];
    let current_read = runtime
        .read_truth()
        .read_snapshot(&outcome.snapshot)
        .unwrap();

    let trace = outcome
        .aspect_evaluation_traces()
        .iter()
        .find(|trace| trace.target == RecordRef::Entity(entity))
        .expect("entity field patch trace");
    let row = trace
        .binding_rows
        .iter()
        .find(|row| row.aspect_key == AspectKey::new("name").unwrap())
        .expect("name aspect row");

    assert!(row.changed);
    assert_eq!(
        trace.changed_aspects,
        ordered_aspect_keys([AspectKey::new("name").unwrap()])
    );
    assert_eq!(
        patch_record.authoritative_changed_aspects(),
        ordered_aspect_keys([AspectKey::new("name").unwrap()])
    );
    assert!(matches!(
        patch_record.authoritative_patch.full_grammar_operations(),
        [PublishedAuthoritativePatchOperation::WholeAspectSet {
            aspect_key,
            value: PublishedAuthoritativePatchValue::Scalar(value),
        }] if aspect_key == &AspectKey::new("name").unwrap()
            && value == &AspectValue::String("after".into())
    ));
    let authoritative_name = current_read
        .get_entity(entity)
        .unwrap()
        .authoritative_aspect_state
        .as_ref()
        .and_then(|state| state.get(&AspectKey::new("name").unwrap()))
        .expect("name aspect state");
    assert!(matches!(
        authoritative_name.view(),
        ContractValidatedAspectValueView::Scalar(value)
            if value == &AspectValue::String("after".into())
    ));
    let AspectTraceEvidence::AuthoritativePatch { patch, .. } = &row.evidence else {
        panic!("expected authoritative patch trace evidence");
    };
    assert_eq!(
        patch.scalar_set_for(&AspectKey::new("name").unwrap()),
        Some(&AspectValue::String("after".into()))
    );
}

#[test]
fn update_entity_fields_applies_struct_contract_field_patch() {
    let mut runtime = AspectSchemaFixture {
        entity_aspects: vec![
            entity_field_aspect(
                crate::tests::support::aspect_key("name"),
                crate::tests::support::field_key("name"),
            ),
            entity_summary_struct_aspect(
                crate::tests::support::aspect_key("summary"),
                crate::tests::support::field_key("summary"),
            ),
        ],
        ..AspectSchemaFixture::default()
    }
    .build_runtime();
    let entity = create_entity_with_summary_fields(
        &mut runtime,
        "struct-patch",
        "before",
        "open",
        true,
        false,
    );

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("summary-field-patch").push(MutationIntent::Entity(
            EntityMutationIntent::UpdateFields(
                crate::transactions::data::UpdateEntityFieldsIntent {
                    entity_id: entity,
                    fields: crate::transactions::data::AspectFieldPatch::from_locator(
                        crate::transactions::data::planned_single_field_locator(
                            forge_foundational::facade::AspectKey::new("summary")
                                .expect("valid test aspect key"),
                            FieldKey::new("title").expect("valid test field key"),
                        ),
                        forge_foundational::facade::AspectValue::String("after".into()),
                    ),
                },
            ),
        )),
    );
    let outcome = txn.commit().unwrap();
    let patch_record = &outcome.patch()[0];
    let current_read = runtime
        .read_truth()
        .read_snapshot(&outcome.snapshot)
        .unwrap();
    let record = current_read.get_entity(entity).unwrap();

    let authoritative_summary = record
        .authoritative_aspect_state
        .as_ref()
        .and_then(|state| state.get(&AspectKey::new("summary").unwrap()))
        .expect("summary aspect state");
    let ContractValidatedAspectValueView::Struct(authoritative_summary) =
        authoritative_summary.view()
    else {
        panic!("summary aspect state must remain struct shaped");
    };
    assert_eq!(
        authoritative_summary.get(&FieldKey::new("title").unwrap()),
        Some(&AspectValue::String("after".into()))
    );
    assert_eq!(
        authoritative_summary.get(&FieldKey::new("status").unwrap()),
        Some(&AspectValue::String("open".into()))
    );
    assert_eq!(
        patch_record.authoritative_changed_aspects(),
        ordered_aspect_keys([AspectKey::new("summary").unwrap()])
    );
    assert!(matches!(
        patch_record.authoritative_patch.full_grammar_operations(),
        [PublishedAuthoritativePatchOperation::FieldLevelPatch {
            aspect_key,
            field_sets,
            field_clears,
        }] if aspect_key == &AspectKey::new("summary").unwrap()
            && field_sets.len() == 1
            && field_sets[0].field == FieldKey::new("title").unwrap()
            && field_sets[0].value == AspectValue::String("after".into())
            && field_clears.is_empty()
    ));

    let trace = outcome
        .aspect_evaluation_traces()
        .iter()
        .find(|trace| trace.target == RecordRef::Entity(entity))
        .expect("summary field patch trace");
    let row = trace
        .binding_rows
        .iter()
        .find(|row| row.aspect_key == AspectKey::new("summary").unwrap())
        .expect("summary aspect row");

    assert!(row.changed);
    let AspectTraceEvidence::AuthoritativePatch { patch, .. } = &row.evidence else {
        panic!("expected authoritative patch trace evidence");
    };
    let summary = AspectKey::new("summary").unwrap();
    let field_sets = patch.field_sets_for(&summary).collect::<Vec<_>>();
    let field_clears = patch.field_clears_for(&summary).collect::<Vec<_>>();
    assert_eq!(
        field_sets,
        vec![
            &crate::publication::patch::data::PublishedAuthoritativeFieldSet {
                field: FieldKey::new("title").expect("valid test field key"),
                value: AspectValue::String("after".into()),
            }
        ]
    );
    assert!(field_clears.is_empty());
}

#[test]
fn update_entity_fields_rejects_explicit_aspect_field_path_mismatch() {
    let mut runtime = AspectSchemaFixture {
        entity_aspects: vec![
            entity_field_aspect(
                crate::tests::support::aspect_key("title.scalar"),
                crate::tests::support::field_key("title"),
            ),
            entity_summary_struct_aspect(
                crate::tests::support::aspect_key("summary"),
                crate::tests::support::field_key("summary"),
            ),
        ],
        ..AspectSchemaFixture::default()
    }
    .build_runtime();
    let entity = create_entity_with_summary_fields(
        &mut runtime,
        "ambiguous-title",
        "before",
        "open",
        false,
        true,
    );

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("mismatched-aspect-field").push(MutationIntent::Entity(
            EntityMutationIntent::UpdateFields(
                crate::transactions::data::UpdateEntityFieldsIntent {
                    entity_id: entity,
                    fields: crate::transactions::data::AspectFieldPatch::from_locator(
                        crate::transactions::data::planned_single_field_locator(
                            forge_foundational::facade::AspectKey::new("title.scalar")
                                .expect("valid test aspect key"),
                            FieldKey::new("status").expect("valid test field key"),
                        ),
                        forge_foundational::facade::AspectValue::String("after".into()),
                    ),
                },
            ),
        )),
    );

    let error = txn.commit().unwrap_err();
    match error {
        TransactionCommitError::Conflict { error, .. } => match error.class {
            crate::transactions::data::ConflictClass::EntityFieldAspectPatchDenied {
                denial:
                    crate::transactions::data::EntityFieldAspectPatchDenial::EntityAspectFieldPathMismatch {
                        field_locator,
                        ..
                    },
                ..
            } => assert_eq!(field_locator.aspect().aspect_key().as_str(), "title.scalar"),
            other => panic!("expected entity aspect field path mismatch denial, got {other:?}"),
        },
        other => panic!("expected conflict error, got {other:?}"),
    }
}

#[test]
fn update_entity_fields_validation_denial_carries_aspect_field_path() {
    let mut runtime =
        runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    let entity = create_entity(&mut runtime, "type-denial");

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("field-patch-type-denial").push(MutationIntent::Entity(
            EntityMutationIntent::UpdateFields(
                crate::transactions::data::UpdateEntityFieldsIntent {
                    entity_id: entity,
                    fields: crate::transactions::data::AspectFieldPatch::from_locator(
                        crate::transactions::data::planned_single_field_locator(
                            forge_foundational::facade::AspectKey::new("name")
                                .expect("valid test aspect key"),
                            FieldKey::new("name").expect("valid test field key"),
                        ),
                        forge_foundational::facade::AspectValue::UInt64(7),
                    ),
                },
            ),
        )),
    );

    let error = txn.commit().unwrap_err();
    match error {
        TransactionCommitError::Conflict { error, .. } => match error.class {
            crate::transactions::data::ConflictClass::EntityFieldAspectPatchDenied {
                denial:
                    crate::transactions::data::EntityFieldAspectPatchDenial::ContractValidationDenied {
                        field_locator,
                        ..
                    },
                ..
            } => {
                assert_eq!(field_locator.aspect().aspect_key().as_str(), "name");
                assert_eq!(
                    field_locator.field_path(),
                    &forge_foundational::facade::CanonicalFieldPath::single(
                        FieldKey::new("name").expect("valid test field key")
                    )
                );
            }
            other => panic!("expected contract validation denial, got {other:?}"),
        },
        other => panic!("expected conflict error, got {other:?}"),
    }
}

fn create_entity_with_summary_fields(
    runtime: &mut RelationalRuntime,
    client_key: &str,
    summary_title: &str,
    summary_status: &str,
    include_name: bool,
    include_scalar_title: bool,
) -> EntityId {
    let mut fields = std::collections::BTreeMap::from([
        (
            crate::transactions::data::planned_single_field_locator(
                AspectKey::new("summary").expect("valid summary aspect key"),
                FieldKey::new("title").expect("valid title field key"),
            ),
            AspectValue::String(summary_title.into()),
        ),
        (
            crate::transactions::data::planned_single_field_locator(
                AspectKey::new("summary").expect("valid summary aspect key"),
                FieldKey::new("status").expect("valid status field key"),
            ),
            AspectValue::String(summary_status.into()),
        ),
    ]);
    if include_name {
        fields.insert(
            crate::transactions::data::planned_single_field_locator(
                AspectKey::new("name").expect("valid name aspect key"),
                FieldKey::new("name").expect("valid name field key"),
            ),
            AspectValue::String(client_key.into()),
        );
    }
    if include_scalar_title {
        fields.insert(
            crate::transactions::data::planned_single_field_locator(
                AspectKey::new("title.scalar").expect("valid scalar title aspect key"),
                FieldKey::new("title").expect("valid title field key"),
            ),
            AspectValue::String("scalar-title".into()),
        );
    }
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(WorkerIntentBatch::new(format!("batch-{client_key}")).push(
        MutationIntent::Create(CreateIntent::Entity(EntitySpec {
            partition_id: PartitionId::main(),
            kind_id: KindId(1),
            client_key: crate::symbols::data::ClientKey::raw(client_key),
            fields: crate::transactions::data::AspectFieldPatch::new(fields),
        })),
    ));
    changed_entities(&txn.commit().unwrap())[0]
}

#[test]
fn commit_log_records_structural_summary_and_phase_progress() {
    let mut runtime = runtime_with_test_schema();
    let outcome = create_entity_outcome(&mut runtime, "logged");
    let structural_summary = outcome.structural_summary();
    let history_summary = outcome.history_summary().unwrap();
    let change_summary = outcome.change_summary().unwrap();
    let aspect_summary = outcome.aspect_summary().unwrap();
    let patch_budget_summary = outcome.patch_budget_summary().unwrap();
    let publication_summary = outcome.publication_summary().unwrap();
    let commit_summary = outcome.commit_summary();

    assert_eq!(
        structural_summary.commit_topology,
        CommitTopology::FlatEntityBatch
    );
    assert!(!structural_summary.invariant_groups.is_empty());
    assert!(!structural_summary.touched_partitions.is_empty());
    assert!(commit_summary.invariant_result_count >= 1);
    assert_eq!(
        commit_summary.structural_summary.as_ref(),
        Some(structural_summary)
    );
    assert_eq!(
        publication_summary.patch_position,
        Some(outcome.patch_position())
    );
    assert_eq!(
        publication_summary.final_snapshot_id,
        Some(outcome.final_snapshot_id())
    );
    assert_eq!(outcome.outcome.history_summary(), Some(history_summary));
    assert_eq!(outcome.outcome.change_summary(), Some(change_summary));
    assert_eq!(outcome.outcome.aspect_summary(), Some(aspect_summary));
    assert_eq!(
        outcome.outcome.patch_budget_summary(),
        Some(patch_budget_summary)
    );
    assert_eq!(
        outcome.outcome.publication_summary(),
        Some(publication_summary)
    );
    assert_eq!(history_summary.parent_count, outcome.commit.parents.len());
    assert_eq!(history_summary.target_branch, outcome.commit.branch_id.0);
    assert!(change_summary.changed_record_count >= 1);
    assert!(change_summary.adjacency_delta_count <= change_summary.changed_record_count);
    assert!(patch_budget_summary.patch_record_count >= 1);
    assert_eq!(aspect_summary.changed_entity_aspect_count, 2);
    assert_eq!(aspect_summary.changed_relation_aspect_count, 0);
    assert_eq!(
        outcome.commit_log().structural_summary_event(),
        Some(structural_summary)
    );
    assert!(outcome
        .commit_log()
        .has_phase_started(CommitPhase::DraftPreparation));
    assert!(outcome
        .commit_log()
        .has_phase_completed(CommitPhase::Publication));
    assert_eq!(
        outcome.commit_log().history_summary_event(),
        Some(history_summary)
    );
    assert!(outcome.commit_log().events().iter().any(|event| matches!(
        event,
        CommitTraceEvent::InvariantEvaluated {
            execution_point: InvariantExecutionPoint::CommitBoundary,
            ..
        }
    )));
    assert!(outcome.commit_log().events().iter().any(|event| matches!(
        event,
        CommitTraceEvent::InvariantEvaluated {
            execution_point: InvariantExecutionPoint::MutationSensitive,
            ..
        }
    )));
    assert_eq!(
        outcome.commit_log().change_summary_event(),
        Some(change_summary)
    );
    assert_eq!(
        outcome.commit_log().patch_budget_summary_event(),
        Some(patch_budget_summary)
    );
    assert_eq!(
        outcome.commit_log().aspect_summary_event(),
        Some(aspect_summary)
    );
    assert!(outcome
        .commit_log()
        .events()
        .iter()
        .any(|event| matches!(event, CommitTraceEvent::DurableAppendPrepared { .. })));
    assert!(outcome.commit_log().has_commit_published());
    assert_eq!(
        outcome.commit_log().publication_summary_event(),
        Some(publication_summary)
    );
}

#[test]
fn commit_returns_envelope_with_patch_diagnostics_invariants_and_complexity() {
    let mut runtime = runtime_with_test_schema();
    let result = create_entity_outcome(&mut runtime, "enveloped");
    let validation_summary = result.validation_summary();
    let structural_summary = result.structural_summary();
    let change_summary = result.change_summary().unwrap();
    let aspect_summary = result.aspect_summary().unwrap();
    let history_summary = result.history_summary().unwrap();
    let patch_budget_summary = result.patch_budget_summary().unwrap();
    let publication_summary = result.publication_summary().unwrap();

    assert!(!result.patch().is_empty());
    assert!(!result.envelope().patch.records.is_empty());
    assert!(!result.diagnostics().is_empty());
    assert!(!structural_summary.invariant_groups.is_empty());
    assert!(!structural_summary.touched_partitions.is_empty());
    assert!(!result.invariant_executions().is_empty());
    assert!(result.invariant_executions().iter().any(|execution| {
        execution.metadata().execution_point() == InvariantExecutionPoint::CommitBoundary
    }));
    assert!(result.invariant_executions().iter().any(|execution| {
        execution.metadata().execution_point() == InvariantExecutionPoint::MutationSensitive
    }));
    assert!(result
        .invariant_executions()
        .iter()
        .all(|execution| execution.summary().result_count() >= execution.results().len()));
    assert_eq!(
        validation_summary.execution_count,
        result.invariant_executions().len()
    );
    assert!(validation_summary.executed_count >= 1);
    assert!(validation_summary.plan_backed_execution_count >= 1);
    assert!(validation_summary.commit_boundary_seen);
    assert!(validation_summary.mutation_sensitive_seen);
    assert!(validation_summary.committed_observation_count >= 1);
    assert!(validation_summary.speculative_observation_count >= 1);
    assert!(!validation_summary.consumed_groups.is_empty());
    assert!(!validation_summary.applicable_groups.is_empty());
    assert_eq!(
        validation_summary.result_count,
        result
            .invariant_executions()
            .iter()
            .map(|execution| execution.summary().result_count())
            .sum::<usize>()
    );
    assert_eq!(history_summary.parent_count, result.commit.parents.len());
    assert_eq!(
        change_summary.changed_record_count,
        result.changed_records.len()
    );
    assert_eq!(aspect_summary.changed_entity_aspect_count, 2);
    assert_eq!(aspect_summary.changed_relation_aspect_count, 0);
    assert_eq!(
        publication_summary.final_snapshot_id,
        Some(result.final_snapshot_id())
    );
    assert_eq!(
        publication_summary.patch_position,
        Some(result.patch_position())
    );
    assert_eq!(publication_summary.patch_record_count, result.patch().len());
    assert_eq!(
        patch_budget_summary.patch_record_count,
        result.patch().len()
    );
    assert!(result.complexity_delta().partitions_touched_by_commit >= 1);
    assert_eq!(
        result.outcome.commit.commit_id,
        result.envelope().commit.commit_id
    );
    assert_eq!(result.patch_position(), result.envelope().patch.position);
    assert_eq!(result.final_snapshot_id(), result.snapshot.snapshot_id);
    assert_eq!(result.merge_parent_count(), 0);
}

#[test]
fn visibility_aspect_versions_follow_canonical_delta_truth_and_ignore_undeclared_fields() {
    let mut runtime =
        runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    let created = create_entity_outcome(&mut runtime, "alpha");
    let entity = changed_entities(&created)[0];
    let updated = update_entity(&mut runtime, entity, "beta");
    let versions = runtime.read_truth().entity_aspect_versions(entity).unwrap();

    assert_eq!(
        versions
            .iter()
            .map(|(aspect, _)| aspect.clone())
            .collect::<Vec<_>>(),
        vec![
            AspectKey::new("lifecycle").unwrap(),
            AspectKey::new("name").unwrap(),
        ]
    );
    assert_eq!(
        versions,
        vec![
            (AspectKey::new("lifecycle").unwrap(), created.version_id.0),
            (AspectKey::new("name").unwrap(), updated.version_id.0),
        ]
    );

    let relation = create_relation(&mut runtime, entity, entity, "edge");
    let relation_versions = runtime
        .read_truth()
        .relation_aspect_versions(relation)
        .unwrap();
    assert_eq!(
        relation_versions
            .iter()
            .map(|(aspect, _)| aspect.clone())
            .collect::<Vec<_>>(),
        vec![
            AspectKey::new("label").unwrap(),
            AspectKey::new("lifecycle").unwrap(),
            AspectKey::new("source").unwrap(),
            AspectKey::new("target").unwrap(),
        ]
    );
}

#[test]
fn visibility_aspect_versions_reject_stale_generation_ids() {
    let mut runtime =
        runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    let entity = create_entity(&mut runtime, "before");
    let stale = EntityId::new(
        entity.partition_id,
        entity.local_slot.0,
        entity.generation.0 + 1,
    );

    assert!(runtime.read_truth().entity_aspect_versions(stale).is_none());
    assert!(runtime
        .read_truth()
        .entity_aspect_versions(entity)
        .is_some());
}

#[test]
fn commit_publication_exposes_aspect_evaluation_and_emission_traces() {
    let mut runtime =
        runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    let result = create_entity_outcome(&mut runtime, "traced");
    let evaluation_traces = result.aspect_evaluation_traces();
    let emission_traces = result.aspect_emission_traces();
    let patch_vs_truth = assert_patch_truth_invariants(&result);

    assert_eq!(evaluation_traces.len(), 1);
    assert_eq!(emission_traces.len(), 1);
    assert_eq!(
        evaluation_traces[0].target,
        RecordRef::Entity(changed_entities(&result)[0])
    );
    assert_eq!(evaluation_traces[0].kind_id, KindId(1));
    assert_eq!(
        evaluation_traces[0].structural_change,
        RecordStructuralChange::Created
    );
    assert_eq!(
        evaluation_traces[0].changed_aspects,
        ordered_aspect_keys([
            AspectKey::new("lifecycle").unwrap(),
            AspectKey::new("name").unwrap(),
        ])
    );
    assert_eq!(evaluation_traces[0].binding_rows.len(), 2);
    assert_eq!(emission_traces[0].target, evaluation_traces[0].target);
    assert_eq!(emission_traces[0].patch_position, result.patch_position());
    assert_eq!(emission_traces[0].patch_record_index, 0);
    assert_eq!(
        emission_traces[0].changed_aspects,
        evaluation_traces[0].changed_aspects
    );
    assert!(patch_vs_truth.exact_match);
    assert_eq!(patch_vs_truth.records_checked, 1);
    assert_eq!(result.aspect_tag_accuracy_report().records_checked, 1);
    assert_eq!(
        result.aspect_tag_accuracy_report().correctly_tagged_records,
        1
    );
}

#[test]
fn detailed_trace_profile_emits_commit_side_aspect_trace_diagnostics() {
    let diagnostics = RelationalDiagnosticsProfile {
        detailed_traces_enabled: true,
        ..RelationalDiagnosticsProfile::default()
    };
    let mut runtime = RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::CertificationCore)
        .schema_registry(declared_aspect_schema_registry(
            CascadeDeletePolicy::CascadeDeleteRelations,
        ))
        .diagnostics(diagnostics)
        .build();
    let result = create_entity_outcome(&mut runtime, "diagnostic-traced");

    assert!(result.diagnostics().iter().any(|artifact| {
        artifact.scope == DiagnosticsScope::Transaction
            && artifact.kind == DiagnosticsArtifactKind::DetailedTrace
            && artifact
                .entries
                .iter()
                .any(|entry| entry.code == DiagnosticCode::AspectEvaluationTraced)
    }));
    assert!(result.diagnostics().iter().any(|artifact| {
        artifact.scope == DiagnosticsScope::PatchPublication
            && artifact.kind == DiagnosticsArtifactKind::DetailedTrace
            && artifact
                .entries
                .iter()
                .any(|entry| entry.code == DiagnosticCode::AspectEmissionTraced)
    }));
}

#[test]
fn aspect_evaluation_trace_retains_unchanged_bindings_for_auditability() {
    let fixture = AspectSchemaFixture {
        entity_aspects: vec![
            entity_field_aspect(
                crate::tests::support::aspect_key("name"),
                crate::tests::support::field_key("name"),
            ),
            entity_field_aspect(
                crate::tests::support::aspect_key("status"),
                crate::tests::support::field_key("status"),
            ),
            lifecycle_aspect(),
        ],
        relation_aspects: vec![relation_source_aspect(), relation_target_aspect()],
        ..AspectSchemaFixture::default()
    };
    let mut runtime = fixture.build_runtime();
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("create").push(MutationIntent::Create(CreateIntent::Entity(
            crate::transactions::data::EntitySpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(1),
                client_key: crate::symbols::data::ClientKey::raw("row"),
                fields: crate::tests::support::string_aspect_field_patch([
                    (
                        crate::tests::support::aspect_key("name"),
                        crate::tests::support::field_key("name"),
                        "before",
                    ),
                    (
                        crate::tests::support::aspect_key("status"),
                        crate::tests::support::field_key("status"),
                        "stable",
                    ),
                ]),
            },
        ))),
    );
    let created = txn.commit().unwrap();
    let entity = changed_entities(&created)[0];

    let mut update_txn = runtime.begin_transaction(TransactionOptions::default());
    update_txn.push_batch(
        WorkerIntentBatch::new("update-name-only").push(MutationIntent::Entity(
            EntityMutationIntent::UpdateFields(UpdateEntityFieldsIntent {
                entity_id: entity,
                fields: crate::tests::support::string_aspect_field_patch([
                    (
                        crate::tests::support::aspect_key("name"),
                        crate::tests::support::field_key("name"),
                        "after",
                    ),
                    (
                        crate::tests::support::aspect_key("status"),
                        crate::tests::support::field_key("status"),
                        "stable",
                    ),
                ]),
            }),
        )),
    );
    let result = update_txn.commit().unwrap();
    let trace = &result.aspect_evaluation_traces()[0];
    let status_key = AspectKey::new("status").unwrap();
    let status_row = trace
        .binding_rows
        .iter()
        .find(|row| row.aspect_key == status_key)
        .expect("status aspect row");

    assert_eq!(trace.binding_rows.len(), 3);
    assert!(!status_row.changed);
    assert!(!trace
        .changed_aspects
        .iter()
        .any(|aspect| aspect == &status_key));
}

#[test]
fn staged_parallel_commit_records_preparation_strategy_and_packet_counters() {
    let mut runtime = runtime_with_test_schema_execution_model(
        RelationalExecutionModel::StagedParallelPreparation,
    );
    let result = create_entity_outcome(&mut runtime, "staged");

    assert!(result.complexity_delta().preparation_packet_count >= 1);
    assert!(result.complexity_delta().preparation_parallel_legal_count >= 1);
    assert!(
        result
            .complexity_delta()
            .preparation_parallel_profitable_count
            >= 1
    );
    assert!(
        result
            .complexity_delta()
            .preparation_staged_parallel_strategy_count
            >= 1
    );
    assert_eq!(
        result.complexity_delta().preparation_reducer_conflict_count,
        0
    );

    let staged_execution = result
        .invariant_executions()
        .iter()
        .find(|execution| {
            execution.metadata().execution_model()
                == RelationalExecutionModel::StagedParallelPreparation
        })
        .expect("staged preparation execution");
    assert_eq!(
        staged_execution.metadata().execution_model(),
        RelationalExecutionModel::StagedParallelPreparation
    );
    assert_eq!(
        staged_execution
            .metadata()
            .preparation_strategy()
            .map(|strategy| strategy.selected_mode),
        Some(
            crate::authority::commit::preparation::planning::strategy::PreparationStrategySelection::StagedParallel
        )
    );

    assert!(result.commit_log().events().iter().any(|event| matches!(
        event,
        CommitTraceEvent::InvariantEvaluated {
            execution_model: RelationalExecutionModel::StagedParallelPreparation,
            preparation_selected_mode: Some(
                crate::authority::commit::preparation::planning::strategy::PreparationStrategySelection::StagedParallel
            ),
            ..
        }
    )));
}

#[test]
fn staged_parallel_patch_preparation_matches_serial_patch_surface() {
    let mut serial_runtime =
        runtime_with_test_schema_execution_model(RelationalExecutionModel::SerialAuthority);
    let mut staged_runtime = runtime_with_test_schema_execution_model(
        RelationalExecutionModel::StagedParallelPreparation,
    );

    let serial = create_entity_outcome(&mut serial_runtime, "patch-parity");
    let staged = create_entity_outcome(&mut staged_runtime, "patch-parity");

    assert_eq!(serial.patch(), staged.patch());
    assert_eq!(serial.envelope().patch, staged.envelope().patch);
    assert!(staged.complexity_delta().preparation_packet_count >= serial.patch().len());
}

#[test]
fn entity_patch_aspects_follow_declared_contract_targets() {
    let mut runtime =
        runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("create").push(MutationIntent::Create(CreateIntent::Entity(
            crate::transactions::data::EntitySpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(1),
                client_key: crate::symbols::data::ClientKey::raw("aspect-entity"),
                fields: crate::tests::support::string_aspect_field_patch([(
                    crate::tests::support::aspect_key("name"),
                    crate::tests::support::field_key("name"),
                    "before",
                )]),
            },
        ))),
    );
    let created = txn.commit().unwrap();
    let entity = changed_entities(&created)[0];
    let created_patch = &created.patch()[0];
    let created_aspect_summary = created.aspect_summary().unwrap();

    let _ = assert_patch_truth_invariants(&created);

    assert_eq!(
        created_patch.structural_change,
        RecordStructuralChange::Created
    );
    assert_eq!(
        created_patch.authoritative_changed_aspects(),
        ordered_aspect_keys([
            AspectKey::new("lifecycle").unwrap(),
            AspectKey::new("name").unwrap(),
        ])
    );
    assert!(!created_patch.contains_opaque_aspect);
    assert_eq!(created_aspect_summary.changed_entity_aspect_count, 2);
    assert_eq!(created_aspect_summary.changed_relation_aspect_count, 0);

    let updated = {
        let mut txn = runtime.begin_transaction(TransactionOptions::default());
        txn.push_batch(
            WorkerIntentBatch::new("update").push(MutationIntent::Entity(
                EntityMutationIntent::UpdateFields(UpdateEntityFieldsIntent {
                    entity_id: entity,
                    fields: crate::tests::support::string_aspect_field_patch([(
                        crate::tests::support::aspect_key("name"),
                        crate::tests::support::field_key("name"),
                        "after",
                    )]),
                }),
            )),
        );
        txn.commit().unwrap()
    };
    let updated_patch = &updated.patch()[0];
    let updated_aspect_summary = updated.aspect_summary().unwrap();

    let _ = assert_patch_truth_invariants(&updated);

    assert_eq!(
        updated_patch.structural_change,
        RecordStructuralChange::Updated
    );
    assert_eq!(
        updated_patch.authoritative_changed_aspects(),
        ordered_aspect_keys([AspectKey::new("name").unwrap()])
    );
    let updated_read = runtime
        .read_truth()
        .read_snapshot(&updated.snapshot)
        .expect("updated snapshot should read");
    let updated_record = updated_read
        .get_entity(entity)
        .expect("updated entity should read");
    let authoritative_name = updated_record
        .authoritative_aspect_state
        .as_ref()
        .and_then(|state| state.get(&AspectKey::new("name").unwrap()))
        .expect("updated name aspect state");
    assert!(matches!(
        authoritative_name.view(),
        ContractValidatedAspectValueView::Scalar(AspectValue::String(value))
            if value == &"after".into()
    ));
    assert_eq!(updated_aspect_summary.changed_entity_aspect_count, 1);

    let idempotent_declared_update = {
        let mut txn = runtime.begin_transaction(TransactionOptions::default());
        txn.push_batch(WorkerIntentBatch::new("idempotent-declared-update").push(
            MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                UpdateEntityFieldsIntent {
                    entity_id: entity,
                    fields: crate::tests::support::single_string_aspect_field_patch(
                        crate::tests::support::aspect_key("name"),
                        crate::tests::support::field_key("name"),
                        "after",
                    ),
                },
            )),
        ));
        txn.commit().unwrap()
    };
    assert_eq!(
        idempotent_declared_update.patch()[0].authoritative_changed_aspects(),
        Vec::new()
    );
    assert_eq!(
        idempotent_declared_update
            .aspect_summary()
            .unwrap()
            .changed_entity_aspect_count,
        0
    );

    let deleted = delete_entity(&mut runtime, entity);
    let deleted_patch = &deleted.patch()[0];
    let deleted_aspect_summary = deleted.aspect_summary().unwrap();
    assert_eq!(
        deleted_patch.structural_change,
        RecordStructuralChange::Deleted
    );
    assert_eq!(
        deleted_patch.authoritative_changed_aspects(),
        ordered_aspect_keys([
            AspectKey::new("lifecycle").unwrap(),
            AspectKey::new("name").unwrap(),
        ])
    );
    assert_eq!(deleted_aspect_summary.changed_entity_aspect_count, 2);
}

#[test]
fn retained_relation_patch_only_emits_declared_lifecycle_delta_when_endpoints_and_aspects_stay_same(
) {
    let mut runtime =
        runtime_with_declared_aspect_schema(CascadeDeletePolicy::RetainDanglingForAudit);
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    let relation_outcome = create_relation_outcome(&mut runtime, source, target, "r-audit");
    let relation_patch = &relation_outcome.patch()[0];
    let relation_aspect_summary = relation_outcome.aspect_summary().unwrap();

    let _ = assert_patch_truth_invariants(&relation_outcome);

    assert_eq!(
        relation_patch.structural_change,
        RecordStructuralChange::Created
    );
    assert_eq!(
        relation_patch.authoritative_changed_aspects(),
        ordered_aspect_keys([
            AspectKey::new("label").unwrap(),
            AspectKey::new("lifecycle").unwrap(),
            AspectKey::new("source").unwrap(),
            AspectKey::new("target").unwrap(),
        ])
    );
    assert_eq!(relation_aspect_summary.changed_relation_aspect_count, 4);

    let deleted_source = delete_entity(&mut runtime, source);
    let retained_relation_patch = deleted_source
        .patch()
        .iter()
        .find(|record| matches!(record.target, RecordRef::Relation(_)))
        .expect("retained relation patch");
    let deleted_source_aspect_summary = deleted_source.aspect_summary().unwrap();

    let _ = assert_patch_truth_invariants(&deleted_source);

    assert_eq!(
        retained_relation_patch.structural_change,
        RecordStructuralChange::RetainedForAudit
    );
    assert_eq!(
        retained_relation_patch.authoritative_changed_aspects(),
        ordered_aspect_keys([AspectKey::new("lifecycle").unwrap()])
    );
    assert!(!retained_relation_patch.contains_opaque_aspect);
    assert_eq!(deleted_source_aspect_summary.changed_entity_aspect_count, 2);
    assert_eq!(
        deleted_source_aspect_summary.changed_relation_aspect_count,
        1
    );
}

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
fn entity_slot_reuse_increments_generation() {
    let mut runtime = runtime_with_test_schema_profile(RelationalRuntimeProfile::AiWorkflow);
    let create_outcome = create_entity_outcome(&mut runtime, "first");
    let entity_a = changed_entities(&create_outcome)[0];
    assert!(runtime
        .visibility_authority()
        .release_snapshot(&create_outcome.snapshot));
    let delete_outcome = delete_entity(&mut runtime, entity_a);
    assert!(runtime
        .visibility_authority()
        .release_snapshot(&delete_outcome.snapshot));
    let retention = runtime.retention().run_pass();
    let entity_b = create_entity(&mut runtime, "second");

    assert!(retention.entity_reclaimed <= 1);
    assert_eq!(
        runtime
            .storage_access()
            .storage_stats()
            .reusable_entity_slots,
        0
    );
    assert_eq!(entity_a.local_slot, entity_b.local_slot);
    assert!(entity_b.generation.0 > entity_a.generation.0);
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
            crate::transactions::data::RelationSpec {
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
            aspect_declarations: KindAspectDeclarations::default(),
        })
        .and_then(|registry| {
            registry.register_relation_kind(RelationKindRegistration {
                kind_id: KindId(2),
                kind_name: "test.relation".to_string(),
                schema_id: SchemaId("test".to_string()),
                schema_version_id: SchemaVersionId(1),
                cross_context_policy: CrossContextPolicy::AllowExplicit,
                cascade_delete_policy: CascadeDeletePolicy::RetainDanglingForAudit,
                aspect_declarations: KindAspectDeclarations::default(),
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

#[test]
fn snapshot_reads_are_immutable_after_later_mutation() {
    let mut runtime = runtime_with_test_schema();
    let first = create_entity(&mut runtime, "first");
    let snapshot = runtime.visibility_authority().snapshot();
    let _second = create_entity(&mut runtime, "second");
    let read = runtime.read_truth().read_snapshot(&snapshot).unwrap();

    assert!(read.get_entity(first).is_some());
    assert_eq!(read.entities().len(), 1);
}

#[test]
fn snapshots_resolve_historical_entity_aspects_by_version() {
    let mut runtime = runtime_with_test_schema();
    let create_outcome = create_entity_outcome(&mut runtime, "before");
    let entity = changed_entities(&create_outcome)[0];
    let snapshot = runtime.visibility_authority().snapshot();
    let update_outcome = update_entity(&mut runtime, entity, "after");

    let old_read = runtime.read_truth().read_snapshot(&snapshot).unwrap();
    let current_read = runtime
        .read_truth()
        .read_snapshot(&update_outcome.snapshot)
        .unwrap();
    let version_read = runtime.read_truth().read_version(create_outcome.version_id);

    assert_eq!(
        read_entity_name(old_read.get_entity(entity).unwrap()),
        Some("before".into())
    );
    assert_eq!(
        read_entity_name(current_read.get_entity(entity).unwrap()),
        Some("after".into())
    );
    assert_eq!(
        read_entity_name(version_read.get_entity(entity).unwrap()),
        Some("before".into())
    );
}

#[test]
fn historical_reads_preserve_generation_and_aspects_after_slot_reuse() {
    let mut runtime = runtime_with_test_schema_profile(RelationalRuntimeProfile::AiWorkflow);
    let created = create_entity_outcome(&mut runtime, "before");
    let original = changed_entities(&created)[0];
    let deleted = delete_entity(&mut runtime, original);
    assert!(runtime
        .visibility_authority()
        .release_snapshot(&created.snapshot));
    assert!(runtime
        .visibility_authority()
        .release_snapshot(&deleted.snapshot));
    assert!(runtime
        .history_authority()
        .retain_version_for_replay(created.version_id));
    let _ = runtime.retention().run_pass();
    let replacement = create_entity(&mut runtime, "after");

    let historical = runtime.read_truth().read_version(created.version_id);
    let record = historical.get_entity(original).unwrap();

    assert_eq!(record.entity_id, original);
    assert_eq!(read_entity_name(record), Some("before".into()));
    assert_eq!(original.local_slot, replacement.local_slot);
    assert!(replacement.generation.0 > original.generation.0);
    assert!(runtime
        .history_authority()
        .release_version_replay_retention(created.version_id));
}

#[test]
fn profile_resolution_and_provenance_are_explicit() {
    let runtime = RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::GeometryKernel)
        .schema_registry(test_schema_registry())
        .entity_capacity(999)
        .build();

    assert_eq!(
        runtime.config().profile,
        RelationalRuntimeProfile::GeometryKernel
    );
    assert_eq!(runtime.config().storage.initial_entity_capacity, 999);
    assert!(runtime.config().diagnostics.profile.detailed_traces_enabled);
    assert_eq!(runtime.config().storage.layout.entity_chunk_size, 2048);
    assert_eq!(
        runtime
            .config()
            .provenance
            .source_for("storage.initial_entity_capacity")
            .unwrap()
            .source,
        ConfigValueSource::BuilderOverride
    );
    assert_eq!(
        runtime
            .config()
            .provenance
            .source_for("storage.layout")
            .unwrap()
            .source,
        ConfigValueSource::ProfileDefault
    );
    assert_eq!(
        runtime
            .config()
            .provenance
            .source_for("visibility.cache_policy")
            .unwrap()
            .source,
        ConfigValueSource::ProfileDefault
    );
    assert!(runtime.config().visibility.cache_policy.enabled);
}

#[test]
fn snapshot_pins_block_reclaim_until_release() {
    let mut runtime = runtime_with_test_schema_profile(RelationalRuntimeProfile::AiWorkflow);
    let create_outcome = create_entity_outcome(&mut runtime, "pinned");
    let create_snapshot = runtime.visibility_authority().snapshot();
    let entity = changed_entities(&create_outcome)[0];
    let _delete_outcome = delete_entity(&mut runtime, entity);
    let delete_snapshot = runtime.visibility_authority().snapshot();
    let first_retention = runtime.retention().run_pass();

    assert_eq!(first_retention.entity_reclaimed, 0);
    assert_eq!(runtime.storage_access().storage_stats().deleted_entities, 1);
    assert_eq!(first_retention.entity_chunks_scanned, 1);

    assert!(runtime
        .visibility_authority()
        .release_snapshot(&create_snapshot));
    assert!(runtime
        .visibility_authority()
        .release_snapshot(&delete_snapshot));
    let second_retention = runtime.retention().run_pass();

    assert!(second_retention.entity_reclaimed <= 1);
    assert_eq!(
        runtime
            .storage_access()
            .storage_stats()
            .reusable_entity_slots,
        1
    );
}

#[test]
fn bulk_mutation_plan_normalizes_client_keys_and_tracks_locality() {
    let mut runtime = runtime_with_test_schema();
    let source = create_entity(&mut runtime, "source");
    let target = create_entity_in_partition(&mut runtime, "target", PartitionId(7));

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("bulk-plan")
            .with_partition_key("planner-main")
            .push(MutationIntent::Create(CreateIntent::BulkEntities(
                crate::facade::transactions::BulkEntityCreateIntent {
                    partition_id: PartitionId::main(),
                    kind_id: KindId(1),
                    client_keys: vec![
                        crate::symbols::data::ClientKey::raw("bulk-a"),
                        crate::symbols::data::ClientKey::raw("bulk-b"),
                    ],
                    field_patches: vec![
                        crate::tests::support::single_string_aspect_field_patch(
                            crate::tests::support::aspect_key("name"),
                            crate::tests::support::field_key("name"),
                            "bulk-a",
                        ),
                        crate::tests::support::single_string_aspect_field_patch(
                            crate::tests::support::aspect_key("name"),
                            crate::tests::support::field_key("name"),
                            "bulk-b",
                        ),
                    ],
                },
            )))
            .push(MutationIntent::Create(CreateIntent::BulkRelations(
                crate::facade::transactions::BulkRelationCreateIntent {
                    partition_id: PartitionId::main(),
                    kind_id: KindId(2),
                    client_keys: vec![crate::symbols::data::ClientKey::raw("cross-edge")],
                    endpoints: vec![(
                        crate::transactions::data::EntityReference::Existing(source),
                        crate::transactions::data::EntityReference::Existing(target),
                    )],
                    field_patches: vec![crate::transactions::data::AspectFieldPatch::default()],
                },
            ))),
    );

    let plan = txn.plan_bulk_mutation_batch().expect("planned batch");

    assert_eq!(
        plan.scope,
        crate::facade::transactions::BulkMutationScope::BulkMixedMutation
    );
    assert_eq!(plan.locality.entity_target_count, 2);
    assert_eq!(plan.locality.relation_target_count, 1);
    assert_eq!(plan.locality.cross_partition_relation_count, 1);
    assert_eq!(
        plan.locality.touched_partitions.as_ref(),
        &[PartitionId::main(), PartitionId(7)]
    );
    assert_eq!(
        plan.provenance.worker_batch_names.as_ref(),
        &["bulk-plan".to_string()]
    );
    assert_eq!(
        plan.provenance.worker_partition_keys.as_ref(),
        &[Some("planner-main".to_string())]
    );
    assert!(!plan.naming.naming_digest.is_empty());
    assert!(!plan.provenance.provenance_digest.is_empty());
    assert_eq!(plan.naming.normalized_client_keys.len(), 3);
    if runtime.config().identity.client_key_symbol_policy
        != crate::symbols::data::ClientKeySymbolPolicy::Disabled
    {
        assert!(plan
            .naming
            .normalized_client_keys
            .iter()
            .all(|value| value.as_symbol().is_some()));
    }
}

#[test]
fn bulk_mutation_plan_captures_lineage_and_provenance_for_topology_rewrite() {
    let mut runtime = runtime_with_test_schema();
    let original = create_entity(&mut runtime, "original");
    let peer = create_entity(&mut runtime, "peer");
    let relation = create_relation(&mut runtime, original, peer, "original-edge");

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("rewrite")
            .push(MutationIntent::Entity(EntityMutationIntent::Replace(
                ReplaceEntityIntent {
                    entity_id: original,
                    replacement: crate::transactions::data::EntitySpec {
                        partition_id: PartitionId(9),
                        kind_id: KindId(1),
                        client_key: crate::symbols::data::ClientKey::raw("replacement"),
                        fields: crate::tests::support::single_string_aspect_field_patch(
                            crate::tests::support::aspect_key("name"),
                            crate::tests::support::field_key("name"),
                            "replacement",
                        ),
                    },
                },
            )))
            .push(MutationIntent::Relation(RelationMutationIntent::Delete(
                DeleteRelationIntent {
                    relation_id: relation,
                },
            ))),
    );

    let plan = txn.plan_bulk_mutation_batch().expect("planned batch");

    assert_eq!(
        plan.scope,
        crate::facade::transactions::BulkMutationScope::TopologyRegionRewrite
    );
    assert!(plan.lineage.transitions.iter().any(|transition| {
        matches!(
            transition,
            crate::facade::transactions::PlannedLineageTransition::ReplaceEntity {
                entity_id,
                replacement_partition_id,
                ..
            } if *entity_id == original && *replacement_partition_id == PartitionId(9)
        )
    }));
    assert!(plan.lineage.transitions.iter().any(|transition| {
        matches!(
            transition,
            crate::facade::transactions::PlannedLineageTransition::DeleteRelation {
                relation_id
            } if *relation_id == relation
        )
    }));
    assert!(plan
        .provenance
        .worker_batch_names
        .iter()
        .any(|name| name == "rewrite"));
    assert!(!plan.lineage.lineage_scope_digest.is_empty());
}

#[test]
fn bulk_mutation_plan_is_absent_for_empty_staging() {
    let mut runtime = runtime_with_test_schema();
    let txn = runtime.begin_transaction(TransactionOptions::default());

    assert!(txn.plan_bulk_mutation_batch().is_none());
}

#[test]
fn bulk_mutation_commit_records_admission_counters() {
    let mut runtime = runtime_with_test_schema();
    let source = create_entity(&mut runtime, "source");
    let target = create_entity_in_partition(&mut runtime, "target", PartitionId(4));

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("bulk-counters").push(MutationIntent::Create(
            CreateIntent::BulkRelations(crate::facade::transactions::BulkRelationCreateIntent {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_keys: vec![crate::symbols::data::ClientKey::raw("edge-a")],
                endpoints: vec![(
                    crate::transactions::data::EntityReference::Existing(source),
                    crate::transactions::data::EntityReference::Existing(target),
                )],
                field_patches: vec![crate::transactions::data::AspectFieldPatch::default()],
            }),
        )),
    );
    let outcome = txn.commit().unwrap();

    assert_eq!(outcome.complexity_delta().bulk_mutation_batch_count, 1);
    assert_eq!(
        outcome.complexity_delta().bulk_mutation_entity_target_count,
        0
    );
    assert_eq!(
        outcome
            .complexity_delta()
            .bulk_mutation_relation_target_count,
        1
    );
    assert_eq!(
        outcome
            .complexity_delta()
            .bulk_mutation_cross_partition_relation_count,
        1
    );
    assert_eq!(
        outcome
            .complexity_delta()
            .bulk_mutation_naming_normalization_count,
        1
    );
    assert_eq!(
        outcome
            .complexity_delta()
            .bulk_mutation_lineage_transition_count,
        1
    );
    assert_eq!(
        outcome
            .complexity_delta()
            .bulk_mutation_provenance_record_count,
        1
    );
}

#[test]
fn same_commit_graph_creation_allows_relation_to_target_created_entities() {
    let mut runtime = runtime_with_test_schema();
    let source_key = crate::symbols::data::ClientKey::raw("same-commit-source");
    let target_key = crate::symbols::data::ClientKey::raw("same-commit-target");

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("same-commit-graph")
            .push(MutationIntent::Create(CreateIntent::Entity(EntitySpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(1),
                client_key: source_key.clone(),
                fields: crate::tests::support::single_string_aspect_field_patch(
                    crate::tests::support::aspect_key("name"),
                    crate::tests::support::field_key("name"),
                    "same-commit-source",
                ),
            })))
            .push(MutationIntent::Create(CreateIntent::Entity(EntitySpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(1),
                client_key: target_key.clone(),
                fields: crate::tests::support::single_string_aspect_field_patch(
                    crate::tests::support::aspect_key("name"),
                    crate::tests::support::field_key("name"),
                    "same-commit-target",
                ),
            })))
            .push(MutationIntent::Create(CreateIntent::Relation(
                RelationSpec {
                    partition_id: PartitionId::main(),
                    kind_id: KindId(2),
                    client_key: crate::symbols::data::ClientKey::raw("same-commit-edge"),
                    source: crate::facade::transactions::EntityReference::Created(
                        crate::facade::transactions::CreatedEntityRef {
                            partition_id: PartitionId::main(),
                            kind_id: KindId(1),
                            client_key: source_key.clone(),
                        },
                    ),
                    target: crate::facade::transactions::EntityReference::Created(
                        crate::facade::transactions::CreatedEntityRef {
                            partition_id: PartitionId::main(),
                            kind_id: KindId(1),
                            client_key: target_key.clone(),
                        },
                    ),
                    fields: crate::transactions::data::AspectFieldPatch::default(),
                },
            ))),
    );

    let outcome = txn
        .commit()
        .expect("same-commit graph creation should succeed");
    let created_entities = changed_entities(&outcome);
    let created_relations = changed_relations(&outcome);

    assert_eq!(created_entities.len(), 2);
    assert_eq!(created_relations.len(), 1);
}

#[test]
fn bulk_relation_create_can_target_same_commit_created_entities() {
    let mut runtime = runtime_with_test_schema();
    let source_key = crate::symbols::data::ClientKey::raw("bulk-created-source");
    let target_key = crate::symbols::data::ClientKey::raw("bulk-created-target");

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("bulk-graph")
            .push(MutationIntent::Create(CreateIntent::BulkEntities(
                crate::facade::transactions::BulkEntityCreateIntent {
                    partition_id: PartitionId::main(),
                    kind_id: KindId(1),
                    client_keys: vec![source_key.clone(), target_key.clone()],
                    field_patches: vec![
                        crate::tests::support::single_string_aspect_field_patch(
                            crate::tests::support::aspect_key("name"),
                            crate::tests::support::field_key("name"),
                            "bulk-created-source",
                        ),
                        crate::tests::support::single_string_aspect_field_patch(
                            crate::tests::support::aspect_key("name"),
                            crate::tests::support::field_key("name"),
                            "bulk-created-target",
                        ),
                    ],
                },
            )))
            .push(MutationIntent::Create(CreateIntent::BulkRelations(
                crate::facade::transactions::BulkRelationCreateIntent {
                    partition_id: PartitionId::main(),
                    kind_id: KindId(2),
                    client_keys: vec![crate::symbols::data::ClientKey::raw("bulk-created-edge")],
                    endpoints: vec![(
                        crate::facade::transactions::EntityReference::Created(
                            crate::facade::transactions::CreatedEntityRef {
                                partition_id: PartitionId::main(),
                                kind_id: KindId(1),
                                client_key: source_key.clone(),
                            },
                        ),
                        crate::facade::transactions::EntityReference::Created(
                            crate::facade::transactions::CreatedEntityRef {
                                partition_id: PartitionId::main(),
                                kind_id: KindId(1),
                                client_key: target_key.clone(),
                            },
                        ),
                    )],
                    field_patches: vec![crate::transactions::data::AspectFieldPatch::default()],
                },
            ))),
    );

    let outcome = txn
        .commit()
        .expect("bulk relation create against created refs should succeed");

    assert_eq!(changed_entities(&outcome).len(), 2);
    assert_eq!(changed_relations(&outcome).len(), 1);
}

#[test]
fn relation_create_rejects_created_entity_refs_missing_from_same_commit() {
    let mut runtime = runtime_with_test_schema();
    let missing_key = crate::symbols::data::ClientKey::raw("missing-created-endpoint");

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("invalid-created-ref").push(MutationIntent::Create(
            CreateIntent::Relation(RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: crate::symbols::data::ClientKey::raw("invalid-created-edge"),
                source: crate::facade::transactions::EntityReference::Created(
                    crate::facade::transactions::CreatedEntityRef {
                        partition_id: PartitionId::main(),
                        kind_id: KindId(1),
                        client_key: missing_key.clone(),
                    },
                ),
                target: crate::facade::transactions::EntityReference::Created(
                    crate::facade::transactions::CreatedEntityRef {
                        partition_id: PartitionId::main(),
                        kind_id: KindId(1),
                        client_key: missing_key,
                    },
                ),
                fields: crate::transactions::data::AspectFieldPatch::default(),
            }),
        )),
    );

    let error = txn
        .commit()
        .expect_err("missing created ref should fail closed");
    match error {
        TransactionCommitError::Conflict { error, .. } => {
            assert_eq!(error.code(), DiagnosticCode::InvalidRelationEndpoint);
        }
        other => panic!(
            "expected invalid relation endpoint conflict, got {:?}",
            other
        ),
    }
}

#[test]
fn epoch_retention_backend_preserves_snapshot_visibility_until_release() {
    let mut runtime = RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::ChipSimulation)
        .schema_registry(test_schema_registry())
        .mvcc(MvccConfig {
            track_visibility_metadata: true,
            snapshot_release_policy: SnapshotReleasePolicy::ExplicitRelease,
            auto_reclaim_deleted_records: true,
            reclaim_batch_size: 128,
            retention_backend: RetentionBackend::EpochChunkRetention,
        })
        .build();
    let create_outcome = create_entity_outcome(&mut runtime, "epoch-pinned");
    let create_snapshot = runtime.visibility_authority().snapshot();
    let entity = changed_entities(&create_outcome)[0];
    let _delete_outcome = delete_entity(&mut runtime, entity);
    let delete_snapshot = runtime.visibility_authority().snapshot();

    let first_retention = runtime.retention().run_pass();
    assert_eq!(
        runtime.config().storage.retention.backend,
        RetentionBackend::EpochChunkRetention
    );
    assert_eq!(first_retention.entity_reclaimed, 0);
    assert!(runtime
        .read_truth()
        .read_snapshot(&create_snapshot)
        .unwrap()
        .get_entity(entity)
        .is_some());

    assert!(runtime
        .visibility_authority()
        .release_snapshot(&create_snapshot));
    assert!(runtime
        .visibility_authority()
        .release_snapshot(&delete_snapshot));
    let second_retention = runtime.retention().run_pass();

    assert!(second_retention.entity_reclaimed <= 1);
    assert_eq!(
        runtime
            .storage_access()
            .storage_stats()
            .reusable_entity_slots,
        1
    );
}

#[test]
fn read_records_expose_visibility_metadata() {
    let mut runtime = runtime_with_test_schema();
    let outcome = create_entity_outcome(&mut runtime, "visible");
    let read = runtime
        .read_truth()
        .read_snapshot(&outcome.snapshot)
        .unwrap();
    let record = read.entities().first().unwrap();

    assert_eq!(record.created_at_version, outcome.version_id);
    assert_eq!(record.retired_at_version, None);
}
