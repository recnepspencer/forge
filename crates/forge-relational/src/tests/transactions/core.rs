use crate::facade::MutationIntent;
use crate::tests::support::*;

#[test]
fn runtime_defaults_to_serialized_authority() {
    let runtime = runtime_with_test_schema();

    assert_eq!(
        runtime.config().execution.execution_model,
        crate::facade::RelationalExecutionModel::SerialAuthority
    );
    assert_eq!(
        runtime.config().execution.commit_authority.authority.mode,
        crate::facade::AuthorityMode::SerializedCommit
    );
}

#[test]
fn harness_defaults_require_determinism_and_parity() {
    let expectations = crate::facade::default_harness_expectations();
    assert!(expectations.serial_parallel_parity_required);
}

#[test]
fn tagged_record_ids_preserve_storage_identity() {
    let entity_id = crate::facade::EntityId::new(PartitionId(7), 11, 3);
    let relation_id = crate::facade::RelationId::new(PartitionId(9), 13, 4);

    let entity_storage: crate::facade::EntityStorageId = entity_id.storage_id();
    let relation_storage: crate::facade::RelationStorageId = relation_id.storage_id();

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
    let wrapped: crate::facade::RelationalError = transaction_error.into();
    assert!(matches!(wrapped, crate::facade::RelationalError::Transaction(_)));
    assert_eq!(
        wrapped.context().subsystem,
        crate::facade::RelationalSubsystem::Transaction
    );

    let wrapped: crate::facade::RelationalError =
        crate::facade::SchemaRegistryError::unknown_entity_kind(KindId(999)).into();
    assert!(matches!(wrapped, crate::facade::RelationalError::Schema(_)));
    assert_eq!(
        wrapped.context().subsystem,
        crate::facade::RelationalSubsystem::Schema
    );

    let wrapped: crate::facade::RelationalError =
        crate::facade::BranchCreateError::branch_already_exists().into();
    assert!(matches!(wrapped, crate::facade::RelationalError::History(_)));
    assert_eq!(
        wrapped.context().subsystem,
        crate::facade::RelationalSubsystem::History
    );

    let wrapped: crate::facade::RelationalError = crate::facade::PublicationError::new(
        crate::facade::PublicationStage::Visibility,
        "publication failed",
    )
    .into();
    assert!(matches!(wrapped, crate::facade::RelationalError::Publication(_)));

    let wrapped: crate::facade::RelationalError = crate::facade::DurabilityError::new(
        crate::facade::RecoveryFailureClass::DurableIoFailure,
        "durability failed",
    )
    .into();
    assert!(matches!(wrapped, crate::facade::RelationalError::Durability(_)));

    let wrapped: crate::facade::RelationalError = crate::facade::ReplayError::new(
        crate::facade::ReplayFailureClass::SchemaMismatch,
        "replay failed",
    )
    .into();
    assert!(matches!(wrapped, crate::facade::RelationalError::Replay(_)));
}

#[test]
fn transaction_intent_is_the_shared_mutation_intent_type() {
    let create = MutationIntent::Create(CreateIntent::Entity(crate::transactions::data::EntitySpec {
        partition_id: PartitionId::main(),
        kind_id: KindId(1),
        client_key: InternedString::Raw("alias".to_string()),
        payload: RecordPayload::StructuredJson(json!({"name":"alias"})),
    }));
    let transaction_intent: MutationIntent = create.clone();

    assert_eq!(transaction_intent, create);
}

#[test]
fn entity_slot_reuse_increments_generation() {
    let mut runtime = runtime_with_test_schema_profile(RelationalRuntimeProfile::AiWorkflow);
    let create_outcome = create_entity_outcome(&mut runtime, "first");
    let entity_a = changed_entities(&create_outcome)[0];
    assert!(runtime.snapshot_access().release_snapshot(&create_outcome.snapshot));
    let delete_outcome = delete_entity(&mut runtime, entity_a);
    assert!(runtime.snapshot_access().release_snapshot(&delete_outcome.snapshot));
    let retention = runtime.retention_access().run_pass();
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
        TransactionCommitError::Conflict(ref conflict) if conflict.code == DiagnosticCode::StaleHandle
    ));
}

#[test]
fn unknown_entity_kind_fails_explicitly() {
    let mut runtime = runtime_with_test_schema();
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("unknown-kind").push(MutationIntent::Create(
            CreateIntent::Entity(crate::transactions::data::EntitySpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(999),
                client_key: InternedString::Raw("bad".to_string()),
                payload: RecordPayload::StructuredJson(json!({"name":"bad"})),
            }),
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
        WorkerIntentBatch::new("duplicate").push(MutationIntent::Create(
            CreateIntent::Relation(crate::transactions::data::RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: InternedString::Raw("r2".to_string()),
                source,
                target,
                payload: Some(RecordPayload::StructuredJson(json!({"label":"rel"}))),
            }),
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
    let read = runtime.visibility_reads().read_snapshot(&outcome.snapshot).unwrap();

    assert!(rollback.effects.iter().any(|effect| matches!(
        effect,
        crate::facade::RollbackEffect::DiscardedEntityCreation
    )));
    assert_eq!(read.entities().len(), 1);
}

#[test]
fn snapshot_audit_failure_discards_only_touched_overlay() {
    let mut runtime = runtime_with_test_schema_and_invariants(InvariantCatalog {
        snapshot_audit: vec![InvariantRule::MaxSnapshotEntities(1)],
        ..InvariantCatalog::default()
    });
    let baseline = create_entity_outcome(&mut runtime, "baseline");

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(batch_create("blocked"));
    let error = txn.commit().unwrap_err();
    let committed_read = runtime.visibility_reads().read_snapshot(&baseline.snapshot).unwrap();

    assert!(matches!(
        error,
        TransactionCommitError::Publication(ref publication)
            if publication.stage == PublicationStage::InvariantCheck
    ));
    assert_eq!(committed_read.entities().len(), 1);
    assert!(committed_read.entities().iter().any(|record| read_entity_name(record) == Some("baseline")));
    assert_eq!(runtime.history_access().latest_commit().unwrap().commit_id, baseline.commit.commit_id);
}

#[test]
fn audit_retained_relations_remain_visible_after_endpoint_delete() {
    let schema = RelationalSchemaRegistry::new()
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
                cascade_delete_policy: CascadeDeletePolicy::RetainDanglingForAudit,
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
    let read = runtime.visibility_reads().read_snapshot(&deleted.snapshot).unwrap();
    let relation = read.get_relation(relation).unwrap();

    assert_eq!(
        relation.lifecycle,
        crate::facade::RecordLifecycleState::RetainedDanglingForAudit
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
    let snapshot = runtime.snapshot_access().snapshot();
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
    let snapshot = runtime.snapshot_access().snapshot();
    let update_outcome = update_entity(&mut runtime, entity, "after");

    let old_read = runtime.visibility_reads().read_snapshot(&snapshot).unwrap();
    let current_read = runtime.visibility_reads().read_snapshot(&update_outcome.snapshot).unwrap();
    let version_read = runtime.visibility_reads().read_version(create_outcome.version_id);

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
    assert!(runtime.snapshot_access().release_snapshot(&created.snapshot));
    assert!(runtime.snapshot_access().release_snapshot(&deleted.snapshot));
    let _ = runtime.retention_access().run_pass();
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
            .config_provenance
            .source_for("storage.initial_entity_capacity")
            .unwrap()
            .source,
        crate::facade::ConfigValueSource::BuilderOverride
    );
    assert_eq!(
        runtime
            .config()
            .config_provenance
            .source_for("storage.layout")
            .unwrap()
            .source,
        crate::facade::ConfigValueSource::ProfileDefault
    );
    assert_eq!(
        runtime
            .config()
            .config_provenance
            .source_for("visibility.cache_policy")
            .unwrap()
            .source,
        crate::facade::ConfigValueSource::ProfileDefault
    );
    assert!(runtime.config().visibility.cache_policy.enabled);
}

#[test]
fn snapshot_pins_block_reclaim_until_release() {
    let mut runtime = runtime_with_test_schema_profile(RelationalRuntimeProfile::AiWorkflow);
    let create_outcome = create_entity_outcome(&mut runtime, "pinned");
    let create_snapshot = runtime.snapshot_access().snapshot();
    let entity = changed_entities(&create_outcome)[0];
    let _delete_outcome = delete_entity(&mut runtime, entity);
    let delete_snapshot = runtime.snapshot_access().snapshot();
    let first_retention = runtime.retention_access().run_pass();

    assert_eq!(first_retention.entity_reclaimed, 0);
    assert_eq!(runtime.storage_stats().deleted_entities, 1);
    assert_eq!(first_retention.entity_chunks_scanned, 1);

    assert!(runtime.snapshot_access().release_snapshot(&create_snapshot));
    assert!(runtime.snapshot_access().release_snapshot(&delete_snapshot));
    let second_retention = runtime.retention_access().run_pass();

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
    let create_snapshot = runtime.snapshot_access().snapshot();
    let entity = changed_entities(&create_outcome)[0];
    let _delete_outcome = delete_entity(&mut runtime, entity);
    let delete_snapshot = runtime.snapshot_access().snapshot();

    let first_retention = runtime.retention_access().run_pass();
    assert_eq!(
        runtime.config().storage.retention.backend,
        crate::facade::RetentionBackend::EpochChunkRetention
    );
    assert_eq!(first_retention.entity_reclaimed, 0);
    assert!(runtime
        .visibility_reads().read_snapshot(&create_snapshot)
        .unwrap()
        .get_entity(entity)
        .is_some());

    assert!(runtime.snapshot_access().release_snapshot(&create_snapshot));
    assert!(runtime.snapshot_access().release_snapshot(&delete_snapshot));
    let second_retention = runtime.retention_access().run_pass();

    assert!(second_retention.entity_reclaimed <= 1);
    assert_eq!(runtime.storage_stats().reusable_entity_slots, 1);
}

#[test]
fn read_records_expose_visibility_metadata() {
    let mut runtime = runtime_with_test_schema();
    let outcome = create_entity_outcome(&mut runtime, "visible");
    let read = runtime.visibility_reads().read_snapshot(&outcome.snapshot).unwrap();
    let record = read.entities().first().unwrap();

    assert_eq!(record.created_at_version, outcome.version_id);
    assert_eq!(record.retired_at_version, None);
}
