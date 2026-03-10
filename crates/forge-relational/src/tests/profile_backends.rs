use super::support::*;

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
        .read_snapshot(&singular_runtime.latest_publication_bundle().unwrap().snapshot)
        .unwrap();

    assert_eq!(bulk_outcome.changed_records.len(), 2);
    assert_eq!(bulk_read.entities().len(), singular_read.entities().len());
    assert_eq!(
        bulk_read
            .entities()
            .iter()
            .map(read_entity_name)
            .collect::<Vec<_>>(),
        singular_read
            .entities()
            .iter()
            .map(read_entity_name)
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

    assert_eq!(partition_ids, vec![PartitionId(7), PartitionId(11), PartitionId(29)]);
    assert_eq!(stats.len(), 3);
    assert_eq!(
        stats.iter()
            .find(|entry| entry.partition_id == PartitionId(7))
            .unwrap()
            .live_entities,
        1
    );
    assert_eq!(
        stats.iter()
            .find(|entry| entry.partition_id == PartitionId(11))
            .unwrap()
            .live_entities,
        1
    );
    assert_eq!(
        stats.iter()
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
    delete_txn.push_batch(
        WorkerIntentBatch::new("delete-relation")
            .push(TransactionIntent::DeleteRelation { relation_id: relation }),
    );
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
    right_txn.push_batch(
        WorkerIntentBatch::new("right-json").push(TransactionIntent::CreateEntity(
            crate::transactions::data::EntitySpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(1),
                client_key: InternedString::Raw("entity".to_string()),
                payload: RecordPayload::StructuredJson(json!({"a": 1, "b": 2})),
            },
        )),
    );
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

    assert_eq!(patch.compatibility, PatchCompatibilityClass::DenseCompatible);
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
