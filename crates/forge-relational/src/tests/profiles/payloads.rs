use crate::tests::support::*;

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
