use super::{
    rule, WorthQueryGraphObligationBudgetExceededPolicy, WorthQueryGraphObligationDispatchPlan,
    WorthQueryGraphObligationExecutionBudget, WorthQueryGraphObligationExecutionCostClass,
    WorthQueryGraphObligationExecutionScope, WorthQueryGraphObligationExecutionStatus,
    WorthQueryGraphObligationVerdict,
};

#[test]
fn equivalent_execution_budgets_have_stable_identity() {
    let first = sparse_relation_budget();
    let replay = sparse_relation_budget();

    assert_eq!(first.budget_digest(), replay.budget_digest());
    assert_eq!(
        first.budget_exceeded_policy(),
        WorthQueryGraphObligationBudgetExceededPolicy::FailClosed
    );
}

#[test]
fn budget_shape_changes_digest() {
    let sparse = sparse_relation_budget();
    let dense = WorthQueryGraphObligationExecutionBudget::declared(
        WorthQueryGraphObligationExecutionCostClass::DenseTopology,
        WorthQueryGraphObligationExecutionScope::TouchedRelationKind,
        WorthQueryGraphObligationBudgetExceededPolicy::FailClosed,
    );
    let advisory = WorthQueryGraphObligationExecutionBudget::bounded_sparse(
        WorthQueryGraphObligationExecutionScope::TouchedRelationKind,
        WorthQueryGraphObligationBudgetExceededPolicy::Advisory,
    );
    let capped = sparse_relation_budget().with_max_state_scope(8);

    assert_ne!(sparse.budget_digest(), dense.budget_digest());
    assert_ne!(sparse.budget_digest(), advisory.budget_digest());
    assert_ne!(sparse.budget_digest(), capped.budget_digest());
}

#[test]
fn dispatch_plan_digest_changes_when_budget_changes() {
    let default_budget_plan = blocking_plan_with_budget(
        WorthQueryGraphObligationExecutionBudget::selection_only_deferred_execution(),
    );
    let sparse_budget_plan = blocking_plan_with_budget(sparse_relation_budget());

    assert_ne!(
        default_budget_plan.plan_digest(),
        sparse_budget_plan.plan_digest()
    );
    assert_eq!(
        sparse_budget_plan.execution_budget().budget_digest(),
        sparse_relation_budget().budget_digest()
    );
}

#[test]
fn budget_exceeded_status_is_not_executor_error() {
    assert!(WorthQueryGraphObligationExecutionStatus::BudgetExceeded.is_budget_denial());
    assert!(!WorthQueryGraphObligationExecutionStatus::BudgetExceeded.is_execution_failure());
    assert!(WorthQueryGraphObligationExecutionStatus::ExecutorError.is_execution_failure());
    assert!(!WorthQueryGraphObligationExecutionStatus::ExecutorError.is_budget_denial());
}

fn sparse_relation_budget() -> WorthQueryGraphObligationExecutionBudget {
    WorthQueryGraphObligationExecutionBudget::bounded_sparse(
        WorthQueryGraphObligationExecutionScope::TouchedRelationKind,
        WorthQueryGraphObligationBudgetExceededPolicy::FailClosed,
    )
}

fn blocking_plan_with_budget(
    execution_budget: WorthQueryGraphObligationExecutionBudget,
) -> WorthQueryGraphObligationDispatchPlan {
    WorthQueryGraphObligationDispatchPlan::blocking_invariant("topology.loop-wiring")
        .with_rule_identity(rule("topology", "loop-wiring", "v1"))
        .with_execution_budget(execution_budget)
        .verdict(
            WorthQueryGraphObligationVerdict::block(
                "loop successor would break closed-loop continuity",
            )
            .expect("blocking verdict"),
        )
        .expect("blocking plan")
}
