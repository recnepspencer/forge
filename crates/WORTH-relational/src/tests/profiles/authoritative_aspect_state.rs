use crate::facade::transactions::MutationIntent;
use crate::tests::support::*;

#[test]
fn fieldless_entity_create_commits_with_absent_authoritative_aspect_state() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .build();
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("opaque").push(MutationIntent::Create(CreateIntent::Entity(
            crate::transactions::data::EntitySpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(1),
                client_key: crate::symbols::data::ClientKey::raw("opaque"),
                fields: crate::transactions::data::AspectFieldPatch::default(),
            },
        ))),
    );
    let outcome = txn.commit().unwrap();
    let read = runtime
        .read_truth()
        .read_snapshot(&outcome.snapshot)
        .unwrap();
    let entity_id = read.entities().first().unwrap().entity_id;

    assert_eq!(runtime.entity_history_len_for_test(entity_id), 1);
    assert!(read.entities()[0].authoritative_aspect_state.is_none());
}

#[test]
fn authoritative_field_patches_are_order_independent_in_patch_output() {
    let order_independent_schema = AspectSchemaFixture {
        entity_aspects: vec![
            entity_field_aspect(
                crate::tests::support::aspect_key("a"),
                crate::tests::support::field_key("a"),
            ),
            entity_field_aspect(
                crate::tests::support::aspect_key("b"),
                crate::tests::support::field_key("b"),
            ),
            lifecycle_aspect(),
        ],
        relation_aspects: vec![
            lifecycle_aspect(),
            relation_source_aspect(),
            relation_target_aspect(),
        ],
        ..AspectSchemaFixture::default()
    }
    .build_registry();
    let mut left_runtime = RelationalRuntimeApi::builder()
        .schema_registry(order_independent_schema.clone())
        .build();
    let mut right_runtime = RelationalRuntimeApi::builder()
        .schema_registry(order_independent_schema)
        .build();

    let mut left_txn = left_runtime.begin_transaction(TransactionOptions::default());
    left_txn.push_batch(
        WorkerIntentBatch::new("left-field-patch").push(MutationIntent::Create(
            CreateIntent::Entity(crate::transactions::data::EntitySpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(1),
                client_key: crate::symbols::data::ClientKey::raw("entity"),
                fields: crate::tests::support::string_aspect_field_patch([
                    (
                        crate::tests::support::aspect_key("b"),
                        crate::tests::support::field_key("b"),
                        "second",
                    ),
                    (
                        crate::tests::support::aspect_key("a"),
                        crate::tests::support::field_key("a"),
                        "first",
                    ),
                ]),
            }),
        )),
    );
    left_txn.commit().unwrap();

    let mut right_txn = right_runtime.begin_transaction(TransactionOptions::default());
    right_txn.push_batch(
        WorkerIntentBatch::new("right-field-patch").push(MutationIntent::Create(
            CreateIntent::Entity(crate::transactions::data::EntitySpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(1),
                client_key: crate::symbols::data::ClientKey::raw("entity"),
                fields: crate::tests::support::string_aspect_field_patch([
                    (
                        crate::tests::support::aspect_key("a"),
                        crate::tests::support::field_key("a"),
                        "first",
                    ),
                    (
                        crate::tests::support::aspect_key("b"),
                        crate::tests::support::field_key("b"),
                        "second",
                    ),
                ]),
            }),
        )),
    );
    right_txn.commit().unwrap();

    assert_eq!(
        left_runtime.publication().artifacts().latest_patch(),
        right_runtime.publication().artifacts().latest_patch()
    );
}
