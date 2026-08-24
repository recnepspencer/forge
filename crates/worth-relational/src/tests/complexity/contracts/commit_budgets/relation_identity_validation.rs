use super::*;

#[test]
fn complexity_budget_relation_identity_validation_avoids_partition_scan() {
    let mut runtime = runtime_with_test_schema();
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    let _existing = create_relation(&mut runtime, source, target, "existing");
    for index in 0..12 {
        let other_source = create_entity(&mut runtime, &format!("other-source-{index}"));
        let other_target = create_entity(&mut runtime, &format!("other-target-{index}"));
        let _ = create_relation(
            &mut runtime,
            other_source,
            other_target,
            &format!("r{index}"),
        );
    }

    runtime.performance_access().reset_counters();
    let mut txn = crate::tests::support::test_owner_begin_transaction_for_main(&mut runtime);
    txn.push_batch(
        WorkerIntentBatch::new("duplicate").push(MutationIntent::Create(CreateIntent::Relation(
            crate::transactions::data::RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: crate::symbols::data::ClientKey::raw("dup"),
                source: crate::transactions::data::EntityReference::Existing(source),
                target: crate::transactions::data::EntityReference::Existing(target),
                fields: crate::transactions::data::AspectFieldPatch::default(),
            },
        ))),
    );
    let error = txn.commit(&mut runtime).unwrap_err();
    let counters = runtime.performance_access().counters();

    assert!(matches!(
        error,
        TransactionCommitError::Conflict { error: ref conflict, .. }
            if conflict.code == DiagnosticCode::DuplicateRelationIdentity
    ));
    assert_eq!(counters.relation_identity_candidates_scanned, 1);
}
