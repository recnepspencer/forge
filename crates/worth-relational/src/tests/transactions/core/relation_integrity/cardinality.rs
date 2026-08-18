use super::fixtures::source_max_one_runtime;
use crate::tests::support::*;

#[test]
fn relation_integrity_commit_boundary_rejects_source_cardinality_overflow() {
    let mut runtime = source_max_one_runtime();
    let source = create_entity(&mut runtime, "source");
    let target_a = create_entity(&mut runtime, "target-a");
    let target_b = create_entity(&mut runtime, "target-b");

    create_relation(&mut runtime, source, target_a, "a");

    let mut txn = crate::tests::support::test_owner_begin_transaction_for_main(&mut runtime);
    txn.push_batch(
        WorkerIntentBatch::new("relation").push(MutationIntent::Create(CreateIntent::Relation(
            crate::transactions::data::RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: crate::symbols::data::ClientKey::raw("b"),
                source: crate::transactions::data::EntityReference::Existing(source),
                target: crate::transactions::data::EntityReference::Existing(target_b),
                fields: crate::transactions::data::AspectFieldPatch::default(),
            },
        ))),
    );

    let error = txn.commit().unwrap_err();
    match error {
        TransactionCommitError::Conflict { error, .. } => {
            assert_eq!(error.code(), DiagnosticCode::RelationCardinalityViolation);
        }
        other => panic!("expected conflict, got {:?}", other),
    }
}
