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
    AuthorityMode, CommitPatchBudgetSummary, CommitPhase, CommitTopology, CommitTraceEvent,
    MutationIntent,
};
use crate::tests::support::*;

mod relation_integrity;

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
            EntityMutationIntent::Update(UpdateEntityIntent {
                entity_id: entity,
                payload: RecordPayload::StructuredJson(json!({"name":"stale"})),
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
            client_key: InternedString::Raw("alias".to_string()),
            payload: RecordPayload::StructuredJson(json!({"name":"alias"})),
        },
    ));
    let transaction_intent: MutationIntent = create.clone();

    assert_eq!(transaction_intent, create);
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
    assert_eq!(aspect_summary.changed_entity_aspect_count, 0);
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
    assert_eq!(aspect_summary.changed_entity_aspect_count, 0);
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
    let entity = create_entity(&mut runtime, "alpha");
    let updated = update_entity(&mut runtime, entity, "beta");
    let versions = runtime
        .visibility_reads()
        .entity_aspect_versions(entity)
        .unwrap();

    assert_eq!(
        versions
            .iter()
            .map(|(aspect, _)| aspect.clone())
            .collect::<Vec<_>>(),
        vec![
            AspectKey(InternedString::Raw("lifecycle".to_string())),
            AspectKey(InternedString::Raw("name".to_string())),
        ]
    );
    assert!(versions
        .iter()
        .all(|(_, version)| *version == updated.version_id.0));

    let relation = create_relation(&mut runtime, entity, entity, "edge");
    let relation_versions = runtime
        .visibility_reads()
        .relation_aspect_versions(relation)
        .unwrap();
    assert_eq!(
        relation_versions
            .iter()
            .map(|(aspect, _)| aspect.clone())
            .collect::<Vec<_>>(),
        vec![
            AspectKey(InternedString::Raw("label".to_string())),
            AspectKey(InternedString::Raw("lifecycle".to_string())),
            AspectKey(InternedString::Raw("source".to_string())),
            AspectKey(InternedString::Raw("target".to_string())),
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

    assert!(runtime
        .visibility_reads()
        .entity_aspect_versions(stale)
        .is_none());
    assert!(runtime
        .visibility_reads()
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
        CanonicalAspectSet::new([
            AspectKey(InternedString::Raw("lifecycle".to_string())),
            AspectKey(InternedString::Raw("name".to_string())),
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
            entity_payload_aspect("name", "name"),
            entity_payload_aspect("status", "status"),
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
                client_key: InternedString::Raw("row".to_string()),
                payload: RecordPayload::StructuredJson(json!({
                    "name": "before",
                    "status": "stable"
                })),
            },
        ))),
    );
    let created = txn.commit().unwrap();
    let entity = changed_entities(&created)[0];

    let mut update_txn = runtime.begin_transaction(TransactionOptions::default());
    update_txn.push_batch(
        WorkerIntentBatch::new("update-name-only").push(MutationIntent::Entity(
            EntityMutationIntent::Update(UpdateEntityIntent {
                entity_id: entity,
                payload: RecordPayload::StructuredJson(json!({
                    "name": "after",
                    "status": "stable"
                })),
            }),
        )),
    );
    let result = update_txn.commit().unwrap();
    let trace = &result.aspect_evaluation_traces()[0];
    let status_key = AspectKey(InternedString::Raw("status".to_string()));
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
fn entity_patch_aspects_follow_declared_semantics_not_payload_keys() {
    let mut runtime =
        runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("create").push(MutationIntent::Create(CreateIntent::Entity(
            crate::transactions::data::EntitySpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(1),
                client_key: InternedString::Raw("aspect-entity".to_string()),
                payload: RecordPayload::StructuredJson(json!({
                    "name": "before",
                    "ignored": "not-an-aspect"
                })),
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
        created_patch.aspects,
        CanonicalAspectSet::new([
            AspectKey(InternedString::Raw("lifecycle".to_string())),
            AspectKey(InternedString::Raw("name".to_string())),
        ])
    );
    assert!(!created_patch.contains_degraded_precision);
    assert_eq!(created_aspect_summary.changed_entity_aspect_count, 2);
    assert_eq!(created_aspect_summary.changed_relation_aspect_count, 0);

    let updated = {
        let mut txn = runtime.begin_transaction(TransactionOptions::default());
        txn.push_batch(
            WorkerIntentBatch::new("update").push(MutationIntent::Entity(
                EntityMutationIntent::Update(UpdateEntityIntent {
                    entity_id: entity,
                    payload: RecordPayload::StructuredJson(json!({
                        "name": "after",
                        "ignored": "still-not-an-aspect"
                    })),
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
        updated_patch.aspects,
        CanonicalAspectSet::new([
            AspectKey(InternedString::Raw("lifecycle".to_string())),
            AspectKey(InternedString::Raw("name".to_string())),
        ])
    );
    assert!(!updated_patch
        .aspects
        .iter()
        .any(|aspect| { *aspect == AspectKey(InternedString::Raw("ignored".to_string())) }));
    assert_eq!(updated_aspect_summary.changed_entity_aspect_count, 2);

    let deleted = delete_entity(&mut runtime, entity);
    let deleted_patch = &deleted.patch()[0];
    let deleted_aspect_summary = deleted.aspect_summary().unwrap();
    assert_eq!(
        deleted_patch.structural_change,
        RecordStructuralChange::Deleted
    );
    assert_eq!(
        deleted_patch.aspects,
        CanonicalAspectSet::new([
            AspectKey(InternedString::Raw("lifecycle".to_string())),
            AspectKey(InternedString::Raw("name".to_string())),
        ])
    );
    assert_eq!(deleted_aspect_summary.changed_entity_aspect_count, 2);
}

#[test]
fn retained_relation_patch_only_emits_declared_lifecycle_delta_when_endpoints_and_payload_stay_same(
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
        relation_patch.aspects,
        CanonicalAspectSet::new([
            AspectKey(InternedString::Raw("label".to_string())),
            AspectKey(InternedString::Raw("lifecycle".to_string())),
            AspectKey(InternedString::Raw("source".to_string())),
            AspectKey(InternedString::Raw("target".to_string())),
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
        retained_relation_patch.aspects,
        CanonicalAspectSet::new([AspectKey(InternedString::Raw("lifecycle".to_string()))])
    );
    assert!(!retained_relation_patch.contains_degraded_precision);
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
            EntityMutationIntent::Update(UpdateEntityIntent {
                entity_id: entity,
                payload: RecordPayload::StructuredJson(json!({"name":"stale"})),
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
            patch_surface_policy: PatchSurfacePolicy::StructuredPatchSurface,
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
    let retention = runtime.retention_authority().run_pass();
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
            EntityMutationIntent::Update(UpdateEntityIntent {
                entity_id: entity,
                payload: RecordPayload::StructuredJson(json!({"name":"stale"})),
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
                client_key: InternedString::Raw("bad".to_string()),
                payload: RecordPayload::StructuredJson(json!({"name":"bad"})),
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
                client_key: InternedString::Raw("r2".to_string()),
                source,
                target,
                payload: Some(RecordPayload::StructuredJson(json!({"label":"rel"}))),
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
        .visibility_reads()
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
        .visibility_reads()
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
        .any(|record| read_entity_name(record) == Some("baseline")));
    assert_eq!(
        runtime.history_access().latest_commit().unwrap().commit_id,
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
                payload_class: RelationPayloadClass::PayloadBearingRelation,
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
        .visibility_reads()
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
    let read = runtime.visibility_reads().read_snapshot(&snapshot).unwrap();

    assert!(read.get_entity(first).is_some());
    assert_eq!(read.entities().len(), 1);
}

#[test]
fn snapshots_resolve_historical_entity_payloads_by_version() {
    let mut runtime = runtime_with_test_schema();
    let create_outcome = create_entity_outcome(&mut runtime, "before");
    let entity = changed_entities(&create_outcome)[0];
    let snapshot = runtime.visibility_authority().snapshot();
    let update_outcome = update_entity(&mut runtime, entity, "after");

    let old_read = runtime.visibility_reads().read_snapshot(&snapshot).unwrap();
    let current_read = runtime
        .visibility_reads()
        .read_snapshot(&update_outcome.snapshot)
        .unwrap();
    let version_read = runtime
        .visibility_reads()
        .read_version(create_outcome.version_id);

    assert_eq!(
        read_entity_name(old_read.get_entity(entity).unwrap()),
        Some("before")
    );
    assert_eq!(
        read_entity_name(current_read.get_entity(entity).unwrap()),
        Some("after")
    );
    assert_eq!(
        read_entity_name(version_read.get_entity(entity).unwrap()),
        Some("before")
    );
}

#[test]
fn historical_reads_preserve_generation_and_payload_after_slot_reuse() {
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
    let _ = runtime.retention_authority().run_pass();
    let replacement = create_entity(&mut runtime, "after");

    let historical = runtime.visibility_reads().read_version(created.version_id);
    let record = historical.get_entity(original).unwrap();

    assert_eq!(record.entity_id, original);
    assert_eq!(read_entity_name(record), Some("before"));
    assert_eq!(original.local_slot, replacement.local_slot);
    assert!(replacement.generation.0 > original.generation.0);
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
    let first_retention = runtime.retention_authority().run_pass();

    assert_eq!(first_retention.entity_reclaimed, 0);
    assert_eq!(runtime.storage_access().storage_stats().deleted_entities, 1);
    assert_eq!(first_retention.entity_chunks_scanned, 1);

    assert!(runtime
        .visibility_authority()
        .release_snapshot(&create_snapshot));
    assert!(runtime
        .visibility_authority()
        .release_snapshot(&delete_snapshot));
    let second_retention = runtime.retention_authority().run_pass();

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

    let first_retention = runtime.retention_authority().run_pass();
    assert_eq!(
        runtime.config().storage.retention.backend,
        RetentionBackend::EpochChunkRetention
    );
    assert_eq!(first_retention.entity_reclaimed, 0);
    assert!(runtime
        .visibility_reads()
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
    let second_retention = runtime.retention_authority().run_pass();

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
        .visibility_reads()
        .read_snapshot(&outcome.snapshot)
        .unwrap();
    let record = read.entities().first().unwrap();

    assert_eq!(record.created_at_version, outcome.version_id);
    assert_eq!(record.retired_at_version, None);
}
