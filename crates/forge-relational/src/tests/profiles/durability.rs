use crate::tests::support::*;

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
