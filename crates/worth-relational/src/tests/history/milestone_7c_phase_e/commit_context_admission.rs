use crate::facade::history::BranchId;
use crate::facade::merge::{MergeExecutionRequest, MergeIntent};
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
    let _mutation_plan =
        prepared.bind_mutation_plan_for_test(crate::facade::transactions::TransactionId(999));

    let error = runtime
        .prepare_merge_execution(MergeExecutionRequest {
            target_branch: BranchId("main".to_string()),
            source_branch: BranchId("wrong".to_string()),
            merge_intent: MergeIntent::ReconcileIntoTarget,
        })
        .expect_err("mismatched merge context should be rejected");

    match error {
        crate::facade::merge::MergeExecutionPreparationError::Planning(
            crate::facade::merge::MergePlanningError::MissingSourceHead { branch_id },
        ) => assert_eq!(branch_id, BranchId("wrong".to_string())),
        other => panic!("expected conflict error, got {other:?}"),
    }
}
