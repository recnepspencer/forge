use super::fixtures::source_max_one_runtime;
use crate::tests::support::*;

#[test]
fn relation_integrity_rejected_branch_local_commit_does_not_advance_truth_or_leak_to_main() {
    let mut runtime = source_max_one_runtime();
    let source = create_entity(&mut runtime, "source");
    let target_a = create_entity(&mut runtime, "target-a");
    let target_b = create_entity(&mut runtime, "target-b");
    let accepted = create_relation_outcome(&mut runtime, source, target_a, "accepted");
    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let main_head_before = runtime
        .history()
        .branch_head(&BranchId("main".to_string()))
        .cloned();
    let feature_head_before = runtime
        .history()
        .branch_head(&BranchId("feature".to_string()))
        .cloned();
    let main_digest_before = relation_aspect_history_digest_on_branch(
        &runtime,
        &BranchId("main".to_string()),
        changed_relations(&accepted)[0],
        None,
    );
    let feature_digest_before = relation_aspect_history_digest_on_branch(
        &runtime,
        &BranchId("feature".to_string()),
        changed_relations(&accepted)[0],
        None,
    );
    let latest_patch_before = runtime
        .publication()
        .artifacts()
        .latest_patch()
        .unwrap()
        .position;

    let mut txn = runtime.begin_transaction(TransactionOptions {
        target_branch: Some(BranchId("feature".to_string())),
        ..TransactionOptions::default()
    });
    txn.push_batch(WorkerIntentBatch::new("illegal-feature-relation").push(
        MutationIntent::Create(CreateIntent::Relation(
            crate::transactions::data::RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: crate::symbols::data::ClientKey::raw("illegal-feature"),
                source: crate::transactions::data::EntityReference::Existing(source),
                target: crate::transactions::data::EntityReference::Existing(target_b),
                fields: crate::transactions::data::AspectFieldPatch::default(),
            },
        )),
    ));

    let error = txn.commit().unwrap_err();
    match error {
        TransactionCommitError::Conflict { error, .. } => {
            assert_eq!(error.code(), DiagnosticCode::RelationCardinalityViolation);
        }
        other => panic!("expected conflict, got {:?}", other),
    }

    assert_eq!(
        runtime.history().branch_head(&BranchId("main".to_string())),
        main_head_before.as_ref()
    );
    assert_eq!(
        runtime
            .history()
            .branch_head(&BranchId("feature".to_string())),
        feature_head_before.as_ref()
    );
    assert_eq!(
        relation_aspect_history_digest_on_branch(
            &runtime,
            &BranchId("main".to_string()),
            changed_relations(&accepted)[0],
            None,
        ),
        main_digest_before
    );
    assert_eq!(
        relation_aspect_history_digest_on_branch(
            &runtime,
            &BranchId("feature".to_string()),
            changed_relations(&accepted)[0],
            None,
        ),
        feature_digest_before
    );
    assert_eq!(
        runtime
            .publication()
            .artifacts()
            .latest_patch()
            .unwrap()
            .position,
        latest_patch_before
    );
    assert_eq!(
        runtime
            .publication()
            .artifacts()
            .latest_bundle()
            .unwrap()
            .commit,
        accepted.commit
    );
}
