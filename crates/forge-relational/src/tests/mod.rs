mod complexity_contracts;
mod durability_contracts;
mod index_contracts;
mod lineage_contracts;
mod replay_contracts;

use forge_harness::facade::{
    DiagnosticsHarnessAdapter, ExecutionProfile, ExecutionRequest, HarnessAdapter, MutationBatch,
    ReplayHarnessAdapter, ReplayRequest, ScenarioPlan,
};
use serde_json::json;

use crate::config::data::{CascadeDeletePolicy, CrossContextPolicy};
use crate::config::data::{DurableLogPolicy, DurableLogRetentionMode};
use crate::facade::{
    BranchCreateError, BranchId, CommitOutcome, DiagnosticCode, DiagnosticsArtifactKind,
    DiagnosticsScope, EntityKindRegistration, EntityReadRecord, InvariantCatalog, InvariantClass,
    InvariantExecutionPoint, InvariantRule, KindId, PartitionId, PublicationStage,
    PublicationStatus, QueryWorkPacket, ReadTarget, RelationId, RelationKindRegistration,
    RelationalHarnessAdapter, RelationalMutation, RelationalRuntime, RelationalRuntimeApi,
    RelationalRuntimeProfile, RelationalSchemaRegistry, SchemaId, SchemaVersionId,
    StorageLayoutConfig, TransactionCommitError, TransactionIntent, TransactionOptions,
    WorkerIntentBatch,
};
use crate::payloads::data::RecordPayload;
use crate::publication::data::diff::{PatchCompatibilityClass, PatchDetail};
use crate::schema::data::RelationPayloadClass;
use crate::symbols::data::{InternedString, SymbolPolicy};

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
    let mut runtime = runtime_with_test_schema_profile(RelationalRuntimeProfile::AiWorkflow);
    let create_outcome = create_entity_outcome(&mut runtime, "first");
    let entity_a = changed_entities(&create_outcome)[0];
    assert!(runtime.release_snapshot(&create_outcome.snapshot));
    let delete_outcome = delete_entity(&mut runtime, entity_a);
    assert!(runtime.release_snapshot(&delete_outcome.snapshot));
    let retention = runtime.run_retention_pass();
    let entity_b = create_entity(&mut runtime, "second");

    assert!(retention.entity_reclaimed <= 1);
    assert_eq!(runtime.storage_stats().reusable_entity_slots, 0);
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
        WorkerIntentBatch::new("update").push(TransactionIntent::UpdateEntity {
            entity_id: entity,
            payload: RecordPayload::StructuredJson(json!({"name":"stale"})),
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
            crate::transactions::data::EntitySpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(999),
                client_key: InternedString::Raw("bad".to_string()),
                payload: RecordPayload::StructuredJson(json!({"name":"bad"})),
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
    let source_outcome = create_entity_outcome(&mut runtime, "source");
    let target_outcome = create_entity_outcome(&mut runtime, "target");
    let source = changed_entities(&source_outcome)[0];
    let target = changed_entities(&target_outcome)[0];
    create_relation(&mut runtime, source, target, "r1");

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("duplicate").push(TransactionIntent::CreateRelation(
            crate::transactions::data::RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: InternedString::Raw("r2".to_string()),
                source,
                target,
                payload: Some(RecordPayload::StructuredJson(json!({"label":"rel"}))),
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
fn snapshots_resolve_historical_entity_payloads_by_version() {
    let mut runtime = runtime_with_test_schema();
    let create_outcome = create_entity_outcome(&mut runtime, "before");
    let entity = changed_entities(&create_outcome)[0];
    let snapshot = runtime.snapshot();
    let update_outcome = update_entity(&mut runtime, entity, "after");

    let old_read = runtime.read_snapshot(&snapshot).unwrap();
    let current_read = runtime.read_snapshot(&update_outcome.snapshot).unwrap();
    let version_read = runtime.read_version(create_outcome.version_id);

    assert_eq!(
        _read_entity_name(old_read.get_entity(entity).unwrap()),
        Some("before")
    );
    assert_eq!(
        _read_entity_name(current_read.get_entity(entity).unwrap()),
        Some("after")
    );
    assert_eq!(
        _read_entity_name(version_read.get_entity(entity).unwrap()),
        Some("before")
    );
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
    assert_eq!(runtime.config().initial_entity_capacity, 999);
    assert!(runtime.config().diagnostics.detailed_traces_enabled);
    assert_eq!(runtime.config().storage_layout.entity_chunk_size, 2048);
    assert_eq!(
        runtime
            .config()
            .config_provenance
            .source_for("initial_entity_capacity")
            .unwrap()
            .source,
        crate::facade::ConfigValueSource::BuilderOverride
    );
    assert_eq!(
        runtime
            .config()
            .config_provenance
            .source_for("storage_layout")
            .unwrap()
            .source,
        crate::facade::ConfigValueSource::ProfileDefault
    );
}

#[test]
fn snapshot_pins_block_reclaim_until_release() {
    let mut runtime = runtime_with_test_schema_profile(RelationalRuntimeProfile::AiWorkflow);
    let create_outcome = create_entity_outcome(&mut runtime, "pinned");
    let entity = changed_entities(&create_outcome)[0];
    let delete_outcome = delete_entity(&mut runtime, entity);
    let first_retention = runtime.run_retention_pass();

    assert_eq!(first_retention.entity_reclaimed, 0);
    assert_eq!(runtime.storage_stats().deleted_entities, 1);
    assert_eq!(first_retention.entity_chunks_scanned, 1);

    assert!(runtime.release_snapshot(&create_outcome.snapshot));
    assert!(runtime.release_snapshot(&delete_outcome.snapshot));
    let second_retention = runtime.run_retention_pass();

    assert!(second_retention.entity_reclaimed <= 1);
    assert_eq!(runtime.storage_stats().reusable_entity_slots, 1);
}

#[test]
fn epoch_retention_backend_preserves_snapshot_visibility_until_release() {
    let mut runtime = RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::ChipSimulation)
        .schema_registry(test_schema_registry())
        .mvcc(crate::facade::MvccConfig {
            track_visibility_metadata: true,
            snapshot_release_policy: crate::facade::SnapshotReleasePolicy::ExplicitRelease,
            auto_reclaim_deleted_records: true,
            reclaim_batch_size: 128,
            retention_backend: crate::facade::RetentionBackend::EpochChunkRetention,
        })
        .build();
    let create_outcome = create_entity_outcome(&mut runtime, "epoch-pinned");
    let entity = changed_entities(&create_outcome)[0];
    let delete_outcome = delete_entity(&mut runtime, entity);

    let first_retention = runtime.run_retention_pass();
    assert_eq!(
        runtime.config().retention_policy.backend,
        crate::facade::RetentionBackend::EpochChunkRetention
    );
    assert_eq!(first_retention.entity_reclaimed, 0);
    assert!(runtime
        .read_snapshot(&create_outcome.snapshot)
        .unwrap()
        .get_entity(entity)
        .is_some());

    assert!(runtime.release_snapshot(&create_outcome.snapshot));
    assert!(runtime.release_snapshot(&delete_outcome.snapshot));
    let second_retention = runtime.run_retention_pass();

    assert!(second_retention.entity_reclaimed <= 1);
    assert_eq!(runtime.storage_stats().reusable_entity_slots, 1);
}

#[test]
fn read_records_expose_visibility_metadata() {
    let mut runtime = runtime_with_test_schema();
    let outcome = create_entity_outcome(&mut runtime, "visible");
    let read = runtime.read_snapshot(&outcome.snapshot).unwrap();
    let record = read.entities().first().unwrap();

    assert_eq!(record.created_at_version, outcome.version_id);
    assert_eq!(record.retired_at_version, None);
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
    assert_eq!(bundle.commit, outcome.commit);
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
    let plan = runtime
        .plan_read_packet(
            &snapshot,
            &QueryWorkPacket::bulk("entities", vec![ReadTarget::Entity(entity)]),
        )
        .unwrap();
    let result = runtime
        .execute_read_packet(
            &snapshot,
            &QueryWorkPacket::bulk("entities", vec![ReadTarget::Entity(entity)]),
        )
        .unwrap();

    assert_eq!(plan.entity_chunk_indexes, vec![0]);
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
    assert!(stats.entity_chunks >= 1);
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

#[test]
fn opaque_payloads_round_trip_through_commit_and_read() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .payload_policy(crate::payloads::data::PayloadPolicy {
            default_class: crate::payloads::data::PayloadClass::OpaqueBytes,
            allow_opaque_bytes: true,
        })
        .build();
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("opaque").push(TransactionIntent::CreateEntity(
            crate::transactions::data::EntitySpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(1),
                client_key: InternedString::Raw("opaque".to_string()),
                payload: RecordPayload::OpaqueBytes(vec![1, 2, 3, 4]),
            },
        )),
    );
    let outcome = txn.commit().unwrap();
    let read = runtime.read_snapshot(&outcome.snapshot).unwrap();

    assert_eq!(
        read.entities().first().unwrap().payload,
        RecordPayload::OpaqueBytes(vec![1, 2, 3, 4])
    );
}

#[test]
fn symbol_policy_interns_client_keys_before_merge() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .symbol_policy(SymbolPolicy::RequireInterned)
        .build();
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(batch_create("intern-me"));
    let plan = txn.merged_plan().unwrap().clone();

    match &plan.merged_intents[0] {
        TransactionIntent::CreateEntity(spec) => {
            assert!(matches!(spec.client_key, InternedString::Symbol(_)));
        }
        other => panic!("expected create entity intent, got {other:?}"),
    }
    assert!(!runtime.config().symbol_table.entries.is_empty());
}

#[test]
fn bulk_create_entities_match_equivalent_singular_creates() {
    let mut bulk_runtime = runtime_with_test_schema();
    let mut bulk_txn = bulk_runtime.begin_transaction(TransactionOptions::default());
    bulk_txn.push_batch(WorkerIntentBatch::new("bulk").push(
        TransactionIntent::BulkCreateEntities {
            partition_id: PartitionId::main(),
            kind_id: KindId(1),
            client_keys: vec![
                InternedString::Raw("a".to_string()),
                InternedString::Raw("b".to_string()),
            ],
            payloads: vec![
                RecordPayload::StructuredJson(json!({"name":"a"})),
                RecordPayload::StructuredJson(json!({"name":"b"})),
            ],
        },
    ));
    let bulk_outcome = bulk_txn.commit().unwrap();

    let singular_runtime = apply_batches(vec![batch_create("a"), batch_create("b")]);
    let bulk_read = bulk_runtime.read_snapshot(&bulk_outcome.snapshot).unwrap();
    let singular_read = singular_runtime
        .read_snapshot(
            &singular_runtime
                .latest_publication_bundle()
                .unwrap()
                .snapshot,
        )
        .unwrap();

    assert_eq!(bulk_outcome.changed_records.len(), 2);
    assert_eq!(bulk_read.entities().len(), singular_read.entities().len());
    assert_eq!(
        bulk_read
            .entities()
            .iter()
            .map(_read_entity_name)
            .collect::<Vec<_>>(),
        singular_read
            .entities()
            .iter()
            .map(_read_entity_name)
            .collect::<Vec<_>>()
    );
}

#[test]
fn cross_context_relations_preserve_partitioned_endpoints() {
    let mut runtime = runtime_with_test_schema();
    let source = create_entity_in_partition(&mut runtime, "left", PartitionId(7));
    let target = create_entity_in_partition(&mut runtime, "right", PartitionId(11));
    let relation =
        create_relation_in_partition(&mut runtime, source, target, "bridge", PartitionId(29));
    let snapshot = runtime.snapshot();
    let read = runtime.read_snapshot(&snapshot).unwrap();
    let relation_record = read.get_relation(relation).unwrap();

    assert_eq!(relation.partition_id, PartitionId(29));
    assert_eq!(relation_record.source.partition_id, PartitionId(7));
    assert_eq!(relation_record.target.partition_id, PartitionId(11));
    assert_eq!(relation_record.relation_id.partition_id, PartitionId(29));
}

#[test]
fn cross_context_relations_respect_relation_kind_policy() {
    let schema_registry = RelationalSchemaRegistry::new()
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
                payload_class: RelationPayloadClass::PayloadBearingRelation,
                cross_context_policy: CrossContextPolicy::SchemaControlled,
                cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
            })
        })
        .unwrap();
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(schema_registry)
        .cross_context_policy(CrossContextPolicy::SchemaControlled)
        .build();
    let source = create_entity_in_partition(&mut runtime, "left", PartitionId(7));
    let target = create_entity_in_partition(&mut runtime, "right", PartitionId(11));
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(WorkerIntentBatch::new("forbidden-cross-context").push(
        TransactionIntent::CreateRelation(crate::transactions::data::RelationSpec {
            partition_id: PartitionId(29),
            kind_id: KindId(2),
            client_key: InternedString::Raw("bridge".to_string()),
            source,
            target,
            payload: None,
        }),
    ));

    let error = txn.commit().unwrap_err();

    assert!(matches!(
        error,
        crate::facade::TransactionCommitError::Conflict(crate::facade::CommitConflict {
            code: crate::facade::DiagnosticCode::InvalidRelationEndpoint,
            ..
        })
    ));
}

#[test]
fn partition_registry_and_stats_expose_partition_owned_state() {
    let mut runtime = runtime_with_test_schema_profile(RelationalRuntimeProfile::GeometryKernel);
    let left = create_entity_in_partition(&mut runtime, "left", PartitionId(7));
    let right = create_entity_in_partition(&mut runtime, "right", PartitionId(11));
    let _ = create_relation_in_partition(&mut runtime, left, right, "bridge", PartitionId(29));

    let partition_ids = runtime.partition_ids();
    let stats = runtime.partition_storage_stats();

    assert_eq!(
        partition_ids,
        vec![PartitionId(7), PartitionId(11), PartitionId(29)]
    );
    assert_eq!(stats.len(), 3);
    assert_eq!(
        stats
            .iter()
            .find(|entry| entry.partition_id == PartitionId(7))
            .unwrap()
            .live_entities,
        1
    );
    assert_eq!(
        stats
            .iter()
            .find(|entry| entry.partition_id == PartitionId(11))
            .unwrap()
            .live_entities,
        1
    );
    assert_eq!(
        stats
            .iter()
            .find(|entry| entry.partition_id == PartitionId(29))
            .unwrap()
            .live_relations,
        1
    );
}

#[test]
fn durable_log_compaction_respects_checkpoint_policy() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .durable_log_policy(DurableLogPolicy {
            retention_mode: DurableLogRetentionMode::CompactAfterCheckpoint,
            max_in_memory_envelopes: 1,
            compact_after_checkpoint: true,
        })
        .build();

    create_entity(&mut runtime, "first");
    runtime.checkpoint().unwrap();
    create_entity(&mut runtime, "second");
    create_entity(&mut runtime, "third");

    assert!(runtime.durable_log().len() <= 1);
}

#[test]
fn relation_payload_history_is_removed_after_reclaim() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .mvcc(crate::facade::MvccConfig {
            track_visibility_metadata: true,
            snapshot_release_policy: crate::facade::SnapshotReleasePolicy::ExplicitRelease,
            auto_reclaim_deleted_records: true,
            reclaim_batch_size: 32,
            retention_backend: crate::facade::RetentionBackend::PinTrackedRetention,
        })
        .build();
    let source_outcome = create_entity_outcome(&mut runtime, "source");
    let target_outcome = create_entity_outcome(&mut runtime, "target");
    let source = changed_entities(&source_outcome)[0];
    let target = changed_entities(&target_outcome)[0];
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(WorkerIntentBatch::new("payload-bearing-relation").push(
        TransactionIntent::CreateRelation(crate::transactions::data::RelationSpec {
            partition_id: PartitionId::main(),
            kind_id: KindId(2),
            client_key: InternedString::Raw("r1".to_string()),
            source,
            target,
            payload: Some(RecordPayload::StructuredJson(
                json!({"weight": 1, "name": "r1"}),
            )),
        }),
    ));
    let created = txn.commit().unwrap();
    let relation = changed_relations(&created)[0];
    assert_eq!(runtime.relation_history_len_for_test(relation), 1);

    let mut delete_txn = runtime.begin_transaction(TransactionOptions::default());
    delete_txn.push_batch(WorkerIntentBatch::new("delete-relation").push(
        TransactionIntent::DeleteRelation {
            relation_id: relation,
        },
    ));
    let deleted = delete_txn.commit().unwrap();
    runtime.release_snapshot(&source_outcome.snapshot);
    runtime.release_snapshot(&target_outcome.snapshot);
    runtime.release_snapshot(&created.snapshot);
    runtime.release_snapshot(&deleted.snapshot);
    runtime.run_retention_pass();

    assert_eq!(runtime.relation_history_len_for_test(relation), 0);
}

#[test]
fn structured_json_payloads_are_canonicalized_in_patch_output() {
    let mut left_runtime = runtime_with_test_schema();
    let mut right_runtime = runtime_with_test_schema();

    let mut left_txn = left_runtime.begin_transaction(TransactionOptions::default());
    left_txn.push_batch(
        WorkerIntentBatch::new("left-json").push(TransactionIntent::CreateEntity(
            crate::transactions::data::EntitySpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(1),
                client_key: InternedString::Raw("entity".to_string()),
                payload: RecordPayload::StructuredJson(json!({"b": 2, "a": 1})),
            },
        )),
    );
    left_txn.commit().unwrap();

    let mut right_txn = right_runtime.begin_transaction(TransactionOptions::default());
    right_txn.push_batch(WorkerIntentBatch::new("right-json").push(
        TransactionIntent::CreateEntity(crate::transactions::data::EntitySpec {
            partition_id: PartitionId::main(),
            kind_id: KindId(1),
            client_key: InternedString::Raw("entity".to_string()),
            payload: RecordPayload::StructuredJson(json!({"a": 1, "b": 2})),
        }),
    ));
    right_txn.commit().unwrap();

    assert_eq!(left_runtime.latest_patch(), right_runtime.latest_patch());
}

#[test]
fn chip_profile_emits_dense_patch_surface_details() {
    let mut runtime = runtime_with_test_schema_profile(RelationalRuntimeProfile::ChipSimulation);
    let left = create_entity_in_partition(&mut runtime, "left", PartitionId(7));
    let right = create_entity_in_partition(&mut runtime, "right", PartitionId(11));
    let _ = create_relation_in_partition(&mut runtime, left, right, "bridge", PartitionId(29));
    let patch = runtime.latest_patch().unwrap();

    assert_eq!(
        patch.compatibility,
        PatchCompatibilityClass::DenseCompatible
    );
    assert!(patch
        .records
        .iter()
        .all(|record| matches!(record.detail, PatchDetail::DenseBitset(_))));
}

#[test]
fn chip_profile_preserves_relation_traversal_with_compressed_adjacency_backend() {
    let mut runtime = runtime_with_test_schema_profile(RelationalRuntimeProfile::ChipSimulation);
    let source = create_entity_in_partition(&mut runtime, "source", PartitionId(7));
    let target_a = create_entity_in_partition(&mut runtime, "target-a", PartitionId(7));
    let target_b = create_entity_in_partition(&mut runtime, "target-b", PartitionId(9));
    let relation_a =
        create_relation_in_partition(&mut runtime, source, target_a, "r-a", PartitionId(7));
    let relation_b =
        create_relation_in_partition(&mut runtime, source, target_b, "r-b", PartitionId(12));
    let version_id = runtime.latest_commit().unwrap().version_id;

    assert_eq!(
        runtime.config().adjacency_policy.backend,
        crate::facade::AdjacencyBackend::CompressedFanoutAdjacency
    );
    assert_eq!(
        runtime.outgoing_relations_for_entity(source, version_id),
        vec![relation_a, relation_b]
    );
    assert_eq!(
        runtime.incoming_relations_for_entity(target_b, version_id),
        vec![relation_b]
    );
}

#[test]
fn chip_profile_compiled_artifacts_are_derived_from_committed_truth() {
    let mut runtime = runtime_with_test_schema_profile(RelationalRuntimeProfile::ChipSimulation);
    let left = create_entity_in_partition(&mut runtime, "left", PartitionId(7));
    let right = create_entity_in_partition(&mut runtime, "right", PartitionId(11));
    let _ = create_relation_in_partition(&mut runtime, left, right, "bridge", PartitionId(29));
    let commit = runtime.latest_commit().unwrap().clone();

    let artifact = runtime
        .compile_execution_artifact(
            commit.commit_id,
            vec![PartitionId(7), PartitionId(11), PartitionId(29)],
        )
        .unwrap();

    assert_eq!(artifact.source_commit_id, commit.commit_id);
    assert_eq!(artifact.source_version_id, commit.version_id);
    assert_eq!(artifact.source_branch_id, BranchId("main".to_string()));
    assert_eq!(
        runtime.compiled_artifact_compatibility(artifact.artifact_id),
        crate::facade::CompiledArtifactCompatibility::Compatible
    );
}

#[test]
fn compiled_artifact_rejects_stale_topology_after_later_commit() {
    let mut runtime = runtime_with_test_schema_profile(RelationalRuntimeProfile::ChipSimulation);
    let original = create_entity_outcome(&mut runtime, "seed");
    let artifact = runtime
        .compile_execution_artifact(original.commit.commit_id, vec![PartitionId::main()])
        .unwrap();
    let _later = create_entity_outcome(&mut runtime, "later");

    assert_eq!(
        runtime.compiled_artifact_compatibility(artifact.artifact_id),
        crate::facade::CompiledArtifactCompatibility::StaleVersion
    );
}

#[test]
fn branch_creation_and_branch_targeted_commits_build_a_version_graph() {
    let mut runtime = runtime_with_test_schema();
    let main_outcome = create_entity_outcome(&mut runtime, "main-a");
    runtime
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let feature_outcome =
        create_entity_outcome_on_branch(&mut runtime, "feature-a", BranchId("feature".to_string()));
    let main_second =
        create_entity_outcome_on_branch(&mut runtime, "main-b", BranchId("main".to_string()));
    let graph = runtime.version_graph();

    assert_eq!(
        runtime
            .branch_head(&BranchId("feature".to_string()))
            .unwrap(),
        &feature_outcome.commit
    );
    assert_eq!(
        runtime.branch_head(&BranchId("main".to_string())).unwrap(),
        &main_second.commit
    );
    assert_eq!(
        feature_outcome.commit.parents,
        vec![main_outcome.commit.commit_id]
    );
    assert_eq!(
        main_second.commit.parents,
        vec![main_outcome.commit.commit_id]
    );
    assert_eq!(graph.branches.len(), 2);
    assert_eq!(graph.commits.len(), 3);
}

#[test]
fn merge_commit_uses_deterministic_parent_order_and_advances_target_branch() {
    let mut runtime = runtime_with_test_schema();
    let main_outcome = create_entity_outcome(&mut runtime, "main-a");
    runtime
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let feature_outcome =
        create_entity_outcome_on_branch(&mut runtime, "feature-a", BranchId("feature".to_string()));
    let merge_outcome = merge_commit_from_branches(
        &mut runtime,
        BranchId("main".to_string()),
        vec![BranchId("feature".to_string())],
    );

    assert_eq!(
        merge_outcome.commit.parents,
        vec![
            main_outcome.commit.commit_id,
            feature_outcome.commit.commit_id
        ]
    );
    assert_eq!(
        runtime.branch_head(&BranchId("main".to_string())),
        Some(&merge_outcome.commit)
    );
    assert_eq!(
        runtime.branch_head(&BranchId("feature".to_string())),
        Some(&feature_outcome.commit)
    );
    let envelope = runtime
        .canonical_commit_envelope(merge_outcome.commit.commit_id)
        .unwrap();
    assert_eq!(
        envelope.merge_parent_branches,
        vec![BranchId("feature".to_string())]
    );
    assert_eq!(
        envelope.merge_base_commits,
        vec![main_outcome.commit.commit_id]
    );
    assert!(runtime
        .diagnostics()
        .by_scope(DiagnosticsScope::PatchPublication)
        .iter()
        .flat_map(|artifact| artifact.entries.iter())
        .any(|entry| entry.code == DiagnosticCode::MergeCommitPublished));
    assert!(runtime
        .diagnostics()
        .by_scope(DiagnosticsScope::PatchPublication)
        .iter()
        .flat_map(|artifact| artifact.entries.iter())
        .any(|entry| entry.code == DiagnosticCode::MergeBaseResolved));
}

#[test]
fn merge_commit_requires_existing_parent_branch_heads() {
    let mut runtime = runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "main-a");
    let txn = runtime.begin_transaction(
        TransactionOptions::default().merge_from_branches(vec![BranchId("missing".to_string())]),
    );
    let error = txn.commit().unwrap_err();

    assert!(matches!(
        error,
        TransactionCommitError::Conflict(ref conflict)
            if conflict.code == DiagnosticCode::InvalidMergeParent
    ));
}

#[test]
fn branch_history_helpers_expose_ancestor_and_merge_base_reasoning() {
    let mut runtime = runtime_with_test_schema();
    let main = create_entity_outcome(&mut runtime, "main");
    runtime
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let feature =
        create_entity_outcome_on_branch(&mut runtime, "feature", BranchId("feature".to_string()));
    let chain = runtime.ancestor_chain(feature.commit.commit_id);
    let merge_base = runtime.latest_common_ancestor_between_branches(
        &BranchId("main".to_string()),
        &BranchId("feature".to_string()),
    );

    assert_eq!(chain, vec![main.commit.commit_id, feature.commit.commit_id]);
    assert_eq!(merge_base, Some(main.commit.commit_id));
    assert!(runtime.can_merge_branch_into(
        &BranchId("feature".to_string()),
        &BranchId("main".to_string())
    ));
}

#[test]
fn merge_inspection_reports_overlapping_authority() {
    let mut runtime = runtime_with_test_schema();
    let base = create_entity_outcome(&mut runtime, "shared");
    let shared = changed_entities(&base)[0];
    runtime
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let _main_update = update_entity(&mut runtime, shared, "main-updated");
    let _feature_update = update_entity_on_branch(
        &mut runtime,
        shared,
        "feature-updated",
        BranchId("feature".to_string()),
    );
    let inspection = runtime.inspect_merge(
        &BranchId("feature".to_string()),
        &BranchId("main".to_string()),
    );

    assert_eq!(inspection.merge_base, Some(base.commit.commit_id));
    assert!(!inspection.can_merge);
    assert_eq!(
        inspection.conflicting_records,
        vec![crate::facade::MergeConflictRecord::Entity(shared)]
    );
}

#[test]
fn merge_commit_rejects_overlapping_authority_since_merge_base() {
    let mut runtime = runtime_with_test_schema();
    let base = create_entity_outcome(&mut runtime, "shared");
    let shared = changed_entities(&base)[0];
    runtime
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let _main_update = update_entity(&mut runtime, shared, "main-updated");
    let _feature_update = update_entity_on_branch(
        &mut runtime,
        shared,
        "feature-updated",
        BranchId("feature".to_string()),
    );
    let txn = runtime.begin_transaction(TransactionOptions {
        target_branch: Some(BranchId("main".to_string())),
        merge_parent_branches: vec![BranchId("feature".to_string())],
        ..TransactionOptions::default()
    });
    let error = txn.commit().unwrap_err();

    assert!(matches!(
        error,
        TransactionCommitError::Conflict(ref conflict)
            if conflict.code == DiagnosticCode::MergeConflictOverlap
    ));
    assert!(runtime
        .diagnostics()
        .by_scope(DiagnosticsScope::History)
        .iter()
        .flat_map(|artifact| artifact.entries.iter())
        .any(|entry| entry.code == DiagnosticCode::MergeConflictOverlap));
}

#[test]
fn duplicate_branch_creation_is_rejected() {
    let mut runtime = runtime_with_test_schema();
    runtime
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let error = runtime
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap_err();

    assert_eq!(error, BranchCreateError::BranchAlreadyExists);
}

#[test]
fn chunked_storage_summary_tracks_visibility_boundaries() {
    let mut runtime = runtime_with_test_schema_and_chunks(2, 2);
    let first = create_entity_outcome(&mut runtime, "e0");
    let entity_a = changed_entities(&first)[0];
    let _second = create_entity_outcome(&mut runtime, "e1");
    let snapshot = runtime.snapshot();
    let _third = create_entity_outcome(&mut runtime, "e2");
    let _update = update_entity(&mut runtime, entity_a, "e0-updated");

    let summary_before_update = runtime.chunked_storage_summary(snapshot.version_id);
    let summary_current =
        runtime.chunked_storage_summary(runtime.latest_commit().unwrap().version_id);

    assert_eq!(summary_before_update.entity_chunks.len(), 2);
    assert_eq!(summary_before_update.entity_chunks[0].visible_records, 2);
    assert_eq!(summary_before_update.entity_chunks[1].visible_records, 0);
    assert_eq!(summary_current.entity_chunks[1].visible_records, 1);
    assert_eq!(summary_current.entity_chunks[0].slot_len, 2);
}

#[test]
fn chunk_diagnostics_and_packet_plans_are_public_and_stable() {
    let mut runtime = runtime_with_test_schema_and_chunks(2, 2);
    let first = create_entity_outcome(&mut runtime, "e0");
    let second = create_entity_outcome(&mut runtime, "e1");
    let entity_a = changed_entities(&first)[0];
    let entity_b = changed_entities(&second)[0];
    let snapshot = runtime.snapshot();
    let packet = QueryWorkPacket::bulk(
        "pair",
        vec![ReadTarget::Entity(entity_a), ReadTarget::Entity(entity_b)],
    );

    let plan = runtime.plan_read_packet(&snapshot, &packet).unwrap();
    let diagnostics = runtime.chunk_diagnostics(snapshot.version_id);

    assert_eq!(plan.target_count, 2);
    assert_eq!(plan.entity_chunk_indexes, vec![0]);
    assert_eq!(diagnostics.entity_chunks_total, 1);
    assert_eq!(diagnostics.entity_chunks_with_visible_records, 1);
}

pub(super) fn test_schema_registry() -> RelationalSchemaRegistry {
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
                payload_class: RelationPayloadClass::PayloadBearingRelation,
                cross_context_policy: CrossContextPolicy::AllowExplicit,
                cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
            })
        })
        .unwrap()
}

pub(super) fn runtime_with_test_schema() -> RelationalRuntime {
    runtime_with_test_schema_profile(RelationalRuntimeProfile::CertificationCore)
}

pub(super) fn runtime_with_test_schema_profile(
    profile: RelationalRuntimeProfile,
) -> RelationalRuntime {
    RelationalRuntimeApi::builder()
        .profile(profile)
        .schema_registry(test_schema_registry())
        .build()
}

pub(super) fn runtime_with_test_schema_and_chunks(
    entity_chunk_size: usize,
    relation_chunk_size: usize,
) -> RelationalRuntime {
    RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::CertificationCore)
        .schema_registry(test_schema_registry())
        .storage_layout(StorageLayoutConfig {
            entity_chunk_size,
            relation_chunk_size,
            scan_packet_size: 64,
        })
        .build()
}

pub(super) fn runtime_with_test_schema_and_invariants(
    invariant_catalog: InvariantCatalog,
) -> RelationalRuntime {
    RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .invariant_catalog(invariant_catalog)
        .build()
}

pub(super) fn batch_create(name: &str) -> WorkerIntentBatch {
    WorkerIntentBatch::new(format!("batch-{name}")).push(TransactionIntent::CreateEntity(
        crate::transactions::data::EntitySpec {
            partition_id: PartitionId::main(),
            kind_id: KindId(1),
            client_key: InternedString::Raw(name.to_string()),
            payload: RecordPayload::StructuredJson(json!({ "name": name })),
        },
    ))
}

pub(super) fn create_entity(
    runtime: &mut RelationalRuntime,
    name: &str,
) -> crate::facade::EntityId {
    changed_entities(&create_entity_outcome(runtime, name))[0]
}

pub(super) fn create_entity_in_partition(
    runtime: &mut RelationalRuntime,
    name: &str,
    partition_id: PartitionId,
) -> crate::facade::EntityId {
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(WorkerIntentBatch::new(format!("batch-{name}")).push(
        TransactionIntent::CreateEntity(crate::transactions::data::EntitySpec {
            partition_id,
            kind_id: KindId(1),
            client_key: InternedString::Raw(name.to_string()),
            payload: RecordPayload::StructuredJson(json!({ "name": name })),
        }),
    ));
    changed_entities(&txn.commit().unwrap())[0]
}

pub(super) fn create_entity_outcome(runtime: &mut RelationalRuntime, name: &str) -> CommitOutcome {
    create_entity_outcome_on_branch(runtime, name, BranchId("main".to_string()))
}

pub(super) fn create_entity_outcome_on_branch(
    runtime: &mut RelationalRuntime,
    name: &str,
    branch_id: BranchId,
) -> CommitOutcome {
    let mut txn = runtime.begin_transaction(TransactionOptions {
        target_branch: Some(branch_id),
        ..TransactionOptions::default()
    });
    txn.push_batch(batch_create(name));
    txn.commit().unwrap()
}

pub(super) fn delete_entity(
    runtime: &mut RelationalRuntime,
    entity_id: crate::facade::EntityId,
) -> CommitOutcome {
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("delete").push(TransactionIntent::DeleteEntity { entity_id }),
    );
    txn.commit().unwrap()
}

pub(super) fn update_entity(
    runtime: &mut RelationalRuntime,
    entity_id: crate::facade::EntityId,
    name: &str,
) -> CommitOutcome {
    update_entity_on_branch(runtime, entity_id, name, BranchId("main".to_string()))
}

pub(super) fn update_entity_on_branch(
    runtime: &mut RelationalRuntime,
    entity_id: crate::facade::EntityId,
    name: &str,
    branch_id: BranchId,
) -> CommitOutcome {
    let mut txn = runtime.begin_transaction(TransactionOptions {
        target_branch: Some(branch_id),
        ..TransactionOptions::default()
    });
    txn.push_batch(
        WorkerIntentBatch::new("update").push(TransactionIntent::UpdateEntity {
            entity_id,
            payload: RecordPayload::StructuredJson(json!({ "name": name })),
        }),
    );
    txn.commit().unwrap()
}

pub(super) fn create_relation(
    runtime: &mut RelationalRuntime,
    source: crate::facade::EntityId,
    target: crate::facade::EntityId,
    client_key: &str,
) -> RelationId {
    create_relation_in_partition(runtime, source, target, client_key, PartitionId::main())
}

pub(super) fn create_relation_in_partition(
    runtime: &mut RelationalRuntime,
    source: crate::facade::EntityId,
    target: crate::facade::EntityId,
    client_key: &str,
    partition_id: PartitionId,
) -> RelationId {
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("relation").push(TransactionIntent::CreateRelation(
            crate::transactions::data::RelationSpec {
                partition_id,
                kind_id: KindId(2),
                client_key: InternedString::Raw(client_key.to_string()),
                source,
                target,
                payload: Some(RecordPayload::StructuredJson(json!({"label":"rel"}))),
            },
        )),
    );
    let outcome = txn.commit().unwrap();
    changed_relations(&outcome)[0]
}

pub(super) fn changed_entities(outcome: &CommitOutcome) -> Vec<crate::facade::EntityId> {
    outcome
        .changed_records
        .iter()
        .filter_map(|record| match record {
            crate::facade::RecordRef::Entity(entity_id) => Some(*entity_id),
            crate::facade::RecordRef::Relation(_) => None,
        })
        .collect()
}

pub(super) fn changed_relations(outcome: &CommitOutcome) -> Vec<RelationId> {
    outcome
        .changed_records
        .iter()
        .filter_map(|record| match record {
            crate::facade::RecordRef::Relation(relation_id) => Some(*relation_id),
            crate::facade::RecordRef::Entity(_) => None,
        })
        .collect()
}

pub(super) fn apply_batches(batches: Vec<WorkerIntentBatch>) -> RelationalRuntime {
    let mut runtime = runtime_with_test_schema();
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    for batch in batches {
        txn.push_batch(batch);
    }
    txn.commit().unwrap();
    runtime
}

pub(super) fn merge_commit_from_branches(
    runtime: &mut RelationalRuntime,
    target_branch: BranchId,
    merge_parent_branches: Vec<BranchId>,
) -> CommitOutcome {
    let txn = runtime.begin_transaction(TransactionOptions {
        target_branch: Some(target_branch),
        merge_parent_branches,
        ..TransactionOptions::default()
    });
    txn.commit().unwrap()
}

#[allow(dead_code)]
fn _read_entity_name(record: &EntityReadRecord) -> Option<&str> {
    record
        .payload
        .as_json()
        .and_then(|value| value.get("name"))
        .and_then(|value| value.as_str())
}
