use crate::facade::history::BranchId;
use crate::facade::merge::{MergeExecutionRequest, MergeIntent};
use crate::facade::transactions::TransactionOptions;
use crate::tests::support::{
    create_branch_from_main, create_entity_outcome, create_entity_outcome_on_branch,
    persisted_runtime_with_test_schema,
};

#[test]
fn merge_commit_context_rejects_mismatched_parent_branch_metadata() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "root");
    create_branch_from_main(&mut runtime, "feature");
    create_entity_outcome_on_branch(
        &mut runtime,
        "feature-only",
        BranchId("feature".to_string()),
    );

    let prepared = runtime
        .prepare_merge_execution(MergeExecutionRequest {
            target_branch: BranchId("main".to_string()),
            source_branch: BranchId("feature".to_string()),
            merge_intent: MergeIntent::ReconcileIntoTarget,
        })
        .expect("prepared merge execution");
    let mutation_plan =
        prepared.bind_mutation_plan_for_test(crate::facade::transactions::TransactionId(999));

    let error = crate::authority::commit::pipeline::AuthoritativeCommitContext::from_merge(
        TransactionOptions {
            target_branch: Some(BranchId("main".to_string())),
            merge_parent_branches: vec![BranchId("wrong".to_string())],
            ..TransactionOptions::default()
        },
        mutation_plan,
    )
    .expect_err("mismatched merge context should be rejected");

    match error {
        crate::facade::transactions::TransactionCommitError::Conflict { error, .. } => {
            assert!(matches!(
                error.class,
                crate::facade::transactions::ConflictClass::InvalidMergeParent { .. }
            ));
        }
        other => panic!("expected conflict error, got {other:?}"),
    }
}
