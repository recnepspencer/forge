use forge_harness::facade::{
    DiagnosticsHarnessAdapter, ExecutionProfile, ExecutionRequest, HarnessAdapter, MutationBatch,
    ReplayHarnessAdapter, ReplayRequest, ScenarioPlan,
};
use serde_json::json;

use crate::facade::{
    CommitOutcome, DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope,
    EntityKindRegistration, EntityReadRecord, InvariantCatalog, InvariantClass,
    InvariantExecutionPoint, InvariantRule, KindId, PublicationStage, PublicationStatus,
    QueryWorkPacket, ReadTarget, RelationId, RelationKindRegistration, RelationalHarnessAdapter,
    RelationalMutation, RelationalRuntime, RelationalRuntimeApi, RelationalSchemaRegistry,
    SchemaId, SchemaVersionId, TransactionCommitError, TransactionIntent, TransactionOptions,
    WorkerIntentBatch,
};

#[test]
fn runtime_defaults_to_serialized_authority() {
    let runtime = runtime_with_test_schema();

    assert_eq!(
        runtime.config().execution_model,
        crate::facade::RelationalExecutionModel::SerialAuthority
    );
    assert_eq!(
        runtime.config().commit_authority.authority.mode,
        crate::facade::AuthorityMode::SerializedCommit
    );
}

#[test]
fn harness_defaults_require_determinism_and_parity() {
    let expectations = crate::facade::default_harness_expectations();
    assert!(expectations.serial_parallel_parity_required);
}

#[test]
fn entity_slot_reuse_increments_generation() {
    let mut runtime = runtime_with_test_schema();
    let entity_a = create_entity(&mut runtime, "first");
    delete_entity(&mut runtime, entity_a);
    let entity_b = create_entity(&mut runtime, "second");

    assert_eq!(entity_a.slot, entity_b.slot);
    assert!(entity_b.generation.0 > entity_a.generation.0);
}

#[test]
fn stale_entity_ids_are_rejected() {
    let mut runtime = runtime_with_test_schema();
    let entity = create_entity(&mut runtime, "first");
    delete_entity(&mut runtime, entity);
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("update").push(TransactionIntent::UpdateEntity {
            entity_id: entity,
            payload: json!({"name":"stale"}),
        }),
    );
    let error = txn.commit().unwrap_err();

    assert!(matches!(
        error,
        TransactionCommitError::Conflict(ref conflict) if conflict.code == DiagnosticCode::StaleHandle
    ));
}

#[test]
fn unknown_entity_kind_fails_explicitly() {
    let mut runtime = runtime_with_test_schema();
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("unknown-kind").push(TransactionIntent::CreateEntity(
            crate::data::transaction::EntitySpec {
                kind_id: KindId(999),
                client_key: "bad".to_string(),
                payload: json!({"name":"bad"}),
            },
        )),
    );
    let error = txn.commit().unwrap_err();

    assert!(matches!(
        error,
        TransactionCommitError::Conflict(ref conflict) if conflict.code == DiagnosticCode::InvariantViolation
    ));
}

#[test]
fn duplicate_relation_identity_is_rejected() {
    let mut runtime = runtime_with_test_schema();
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    create_relation(&mut runtime, source, target, "r1");

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("duplicate").push(TransactionIntent::CreateRelation(
            crate::data::transaction::RelationSpec {
                kind_id: KindId(2),
                client_key: "r2".to_string(),
                source,
                target,
                payload: json!({"label":"rel"}),
            },
        )),
    );
    let error = txn.commit().unwrap_err();

    assert!(matches!(
        error,
        TransactionCommitError::Conflict(ref conflict)
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
    let read = runtime.read_snapshot(&outcome.snapshot).unwrap();

    assert!(!rollback.restored_records.is_empty());
    assert_eq!(read.entities().len(), 1);
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
    let snapshot = runtime.snapshot();
    let _second = create_entity(&mut runtime, "second");
    let read = runtime.read_snapshot(&snapshot).unwrap();

    assert!(read.get_entity(first).is_some());
    assert_eq!(read.entities().len(), 1);
}

#[test]
fn diagnostics_and_replay_are_emitted_for_commit() {
    let mut runtime = runtime_with_test_schema();
    let _entity = create_entity(&mut runtime, "first");
    let diagnostics = runtime.diagnostics();

    assert!(diagnostics.artifacts().iter().any(|artifact| {
        artifact.scope == DiagnosticsScope::Transaction
            && artifact.kind == DiagnosticsArtifactKind::MinimalSummary
    }));
    assert!(diagnostics
        .minimal_summaries()
        .iter()
        .flat_map(|artifact| artifact.entries.iter())
        .any(|entry| entry.code == DiagnosticCode::EntityCreated));
    assert!(runtime.latest_patch().is_some());
    assert!(runtime.latest_replay().is_some());
    assert_eq!(
        runtime.latest_replay().unwrap().schema_registry,
        test_schema_registry()
    );
}

#[test]
fn publication_bundle_is_the_single_visible_commit_surface() {
    let mut runtime = runtime_with_test_schema();
    let outcome = create_entity_outcome(&mut runtime, "first");
    let bundle = runtime.latest_publication_bundle().unwrap();

    assert_eq!(outcome.publication_status, PublicationStatus::Published);
    assert_eq!(bundle.snapshot, outcome.snapshot);
    assert_eq!(bundle.commit, *runtime.latest_commit().unwrap());
    assert_eq!(bundle.patch, *runtime.latest_patch().unwrap());
    assert_eq!(bundle.replay, *runtime.latest_replay().unwrap());
}

#[test]
fn snapshot_audit_failure_blocks_publication() {
    let mut runtime = runtime_with_test_schema_and_invariants(InvariantCatalog {
        snapshot_audit: vec![InvariantRule::MaxSnapshotEntities(0)],
        ..InvariantCatalog::default()
    });
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(batch_create("blocked"));
    let error = txn.commit().unwrap_err();

    assert!(matches!(
        error,
        TransactionCommitError::Publication(ref publication)
            if publication.stage == PublicationStage::InvariantCheck
    ));
    assert!(runtime.latest_publication_bundle().is_none());
}

#[test]
fn bulk_packets_are_the_primary_read_surface() {
    let mut runtime = runtime_with_test_schema();
    let entity = create_entity(&mut runtime, "first");
    let snapshot = runtime.snapshot();
    let result = runtime
        .execute_read_packet(
            &snapshot,
            &QueryWorkPacket::bulk("entities", vec![ReadTarget::Entity(entity)]),
        )
        .unwrap();

    assert_eq!(result.entities.len(), 1);
}

#[test]
fn harness_runner_captures_snapshot_diagnostics_and_replay() {
    let adapter = RelationalHarnessAdapter;
    let fixture = ScenarioPlan::new(
        "fixture",
        crate::presentation::harness::RelationalFixture {
            entities: Vec::new(),
            relations: Vec::new(),
        },
    )
    .compile();
    let batch =
        MutationBatch::new("mutate").push(RelationalMutation::Batch(batch_create("from-harness")));
    let request = ExecutionRequest::target("inspect", "entity:0:1".to_string());
    let profile = ExecutionProfile::forensic("forensic");
    let mut runtime = adapter.create_runtime().unwrap();
    adapter.load_fixture(&mut runtime, &fixture).unwrap();
    adapter.apply_mutation_batch(&mut runtime, &batch).unwrap();
    let run = adapter
        .execute(&mut runtime, &fixture, &request, &profile)
        .unwrap();
    let snapshot = adapter
        .capture_snapshot(&runtime, &fixture, &request, &profile)
        .unwrap();
    let diagnostics = adapter
        .capture_diagnostics(&runtime, &fixture, &profile)
        .unwrap();
    let replay_request = ReplayRequest {
        name: "replay".to_string(),
        source_run: run.clone(),
        request: request.clone(),
        profile: profile.clone(),
    };
    let replay = adapter
        .capture_replay(&runtime, &fixture, &replay_request)
        .unwrap();

    assert_eq!(snapshot.observations.len(), 1);
    assert!(diagnostics.summary.is_object());
    assert!(replay.summary.is_object());
}

#[test]
fn runtime_packet_execution_and_storage_stats_are_readable() {
    let mut runtime = runtime_with_test_schema();
    let entity = create_entity(&mut runtime, "first");
    let snapshot = runtime.snapshot();
    let packet = QueryWorkPacket::bulk("entities", vec![ReadTarget::Entity(entity)]);
    let result = runtime.execute_read_packet(&snapshot, &packet).unwrap();
    let stats = runtime.storage_stats();

    assert_eq!(result.entities.len(), 1);
    assert_eq!(stats.live_entities, 1);
    assert!(stats.snapshot_count >= 1);
}

#[test]
fn repeated_serial_runs_are_harness_comparable() {
    let adapter = RelationalHarnessAdapter;
    let runner = forge_harness::facade::HarnessRunner::new(adapter);
    let fixture = ScenarioPlan::new(
        "fixture",
        crate::presentation::harness::RelationalFixture {
            entities: Vec::new(),
            relations: Vec::new(),
        },
    )
    .compile();
    let batch =
        MutationBatch::new("mutate").push(RelationalMutation::Batch(batch_create("stable")));
    let request = ExecutionRequest::target("inspect", "entity:0:1".to_string());
    let profile = ExecutionProfile::forensic("forensic");
    let run_a = runner
        .execute_core(&fixture, Some(&batch), &request, &profile)
        .unwrap();
    let run_b = runner
        .execute_core(&fixture, Some(&batch), &request, &profile)
        .unwrap();
    let comparison = runner
        .compare_runs(
            &run_a.run,
            &run_b.run,
            &forge_harness::facade::ComparisonProfile::default(),
        )
        .unwrap();

    assert!(comparison.mismatches.is_empty());
}

#[test]
fn harness_heavy_invariants_are_opt_in() {
    let runtime = runtime_with_test_schema_and_invariants(InvariantCatalog {
        harness_heavy: vec![InvariantRule::UniqueEntityPayloadField("name".to_string())],
        ..InvariantCatalog::default()
    });

    let default_results = runtime.run_invariants(InvariantExecutionPoint::HarnessAudit, false);
    let enabled_results = runtime.run_invariants(InvariantExecutionPoint::HarnessAudit, true);

    assert!(default_results.is_empty());
    assert_eq!(enabled_results.len(), 1);
    assert_eq!(enabled_results[0].class, InvariantClass::HarnessHeavy);
}

#[test]
fn cross_order_equivalent_mutations_converge() {
    let runtime_a = apply_batches(vec![batch_create("a"), batch_create("b")]);
    let runtime_b = apply_batches(vec![batch_create("b"), batch_create("a")]);

    assert_eq!(runtime_a.latest_patch(), runtime_b.latest_patch());
    assert_eq!(runtime_a.latest_replay(), runtime_b.latest_replay());
    assert_eq!(runtime_a.diagnostics(), runtime_b.diagnostics());
}

fn test_schema_registry() -> RelationalSchemaRegistry {
    RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "test.entity".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
        })
        .and_then(|registry| {
            registry.register_relation_kind(RelationKindRegistration {
                kind_id: KindId(2),
                kind_name: "test.relation".to_string(),
                schema_id: SchemaId("test".to_string()),
                schema_version_id: SchemaVersionId(1),
            })
        })
        .unwrap()
}

fn runtime_with_test_schema() -> RelationalRuntime {
    RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .build()
}

fn runtime_with_test_schema_and_invariants(
    invariant_catalog: InvariantCatalog,
) -> RelationalRuntime {
    RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .invariant_catalog(invariant_catalog)
        .build()
}

fn batch_create(name: &str) -> WorkerIntentBatch {
    WorkerIntentBatch::new(format!("batch-{name}")).push(TransactionIntent::CreateEntity(
        crate::data::transaction::EntitySpec {
            kind_id: KindId(1),
            client_key: name.to_string(),
            payload: json!({ "name": name }),
        },
    ))
}

fn create_entity(runtime: &mut RelationalRuntime, name: &str) -> crate::facade::EntityId {
    changed_entities(&create_entity_outcome(runtime, name))[0]
}

fn create_entity_outcome(runtime: &mut RelationalRuntime, name: &str) -> CommitOutcome {
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(batch_create(name));
    txn.commit().unwrap()
}

fn delete_entity(
    runtime: &mut RelationalRuntime,
    entity_id: crate::facade::EntityId,
) -> CommitOutcome {
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("delete").push(TransactionIntent::DeleteEntity { entity_id }),
    );
    txn.commit().unwrap()
}

fn create_relation(
    runtime: &mut RelationalRuntime,
    source: crate::facade::EntityId,
    target: crate::facade::EntityId,
    client_key: &str,
) -> RelationId {
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("relation").push(TransactionIntent::CreateRelation(
            crate::data::transaction::RelationSpec {
                kind_id: KindId(2),
                client_key: client_key.to_string(),
                source,
                target,
                payload: json!({"label":"rel"}),
            },
        )),
    );
    let outcome = txn.commit().unwrap();
    changed_relations(&outcome)[0]
}

fn changed_entities(outcome: &CommitOutcome) -> Vec<crate::facade::EntityId> {
    outcome
        .changed_records
        .iter()
        .filter_map(|record| match record {
            crate::facade::RecordRef::Entity(entity_id) => Some(*entity_id),
            crate::facade::RecordRef::Relation(_) => None,
        })
        .collect()
}

fn changed_relations(outcome: &CommitOutcome) -> Vec<RelationId> {
    outcome
        .changed_records
        .iter()
        .filter_map(|record| match record {
            crate::facade::RecordRef::Relation(relation_id) => Some(*relation_id),
            crate::facade::RecordRef::Entity(_) => None,
        })
        .collect()
}

fn apply_batches(batches: Vec<WorkerIntentBatch>) -> RelationalRuntime {
    let mut runtime = runtime_with_test_schema();
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    for batch in batches {
        txn.push_batch(batch);
    }
    txn.commit().unwrap();
    runtime
}

#[allow(dead_code)]
fn _read_entity_name(record: &EntityReadRecord) -> Option<&str> {
    record.payload.get("name").and_then(|value| value.as_str())
}
