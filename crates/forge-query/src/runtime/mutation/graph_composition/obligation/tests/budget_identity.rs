use super::{
    rule, ForgeQueryGraphObligationBudgetExceededPolicy, ForgeQueryGraphObligationDispatchPlan,
    ForgeQueryGraphObligationExecutionBudget, ForgeQueryGraphObligationExecutionCostClass,
    ForgeQueryGraphObligationExecutionScope, ForgeQueryGraphObligationExecutionStatus,
    ForgeQueryGraphObligationVerdict,
};

#[test]
fn equivalent_execution_budgets_have_stable_identity() {
    let first = sparse_relation_budget();
    let replay = sparse_relation_budget();

    assert_eq!(first.budget_digest(), replay.budget_digest());
    assert_eq!(
        first.budget_exceeded_policy(),
        ForgeQueryGraphObligationBudgetExceededPolicy::FailClosed
    );
}

#[test]
fn budget_shape_changes_digest() {
    let sparse = sparse_relation_budget();
    let dense = ForgeQueryGraphObligationExecutionBudget::declared(
        ForgeQueryGraphObligationExecutionCostClass::DenseTopology,
        ForgeQueryGraphObligationExecutionScope::TouchedRelationKind,
        ForgeQueryGraphObligationBudgetExceededPolicy::FailClosed,
    );
    let advisory = ForgeQueryGraphObligationExecutionBudget::bounded_sparse(
        ForgeQueryGraphObligationExecutionScope::TouchedRelationKind,
        ForgeQueryGraphObligationBudgetExceededPolicy::Advisory,
    );
    let capped = sparse_relation_budget().with_max_state_scope(8);

    assert_ne!(sparse.budget_digest(), dense.budget_digest());
    assert_ne!(sparse.budget_digest(), advisory.budget_digest());
    assert_ne!(sparse.budget_digest(), capped.budget_digest());
}

#[test]
fn dispatch_plan_digest_changes_when_budget_changes() {
    let default_budget_plan = blocking_plan_with_budget(
        ForgeQueryGraphObligationExecutionBudget::selection_only_deferred_execution(),
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
    assert!(ForgeQueryGraphObligationExecutionStatus::BudgetExceeded.is_budget_denial());
    assert!(!ForgeQueryGraphObligationExecutionStatus::BudgetExceeded.is_execution_failure());
    assert!(ForgeQueryGraphObligationExecutionStatus::ExecutorError.is_execution_failure());
    assert!(!ForgeQueryGraphObligationExecutionStatus::ExecutorError.is_budget_denial());
}

fn sparse_relation_budget() -> ForgeQueryGraphObligationExecutionBudget {
    ForgeQueryGraphObligationExecutionBudget::bounded_sparse(
        ForgeQueryGraphObligationExecutionScope::TouchedRelationKind,
        ForgeQueryGraphObligationBudgetExceededPolicy::FailClosed,
    )
}

fn blocking_plan_with_budget(
    execution_budget: ForgeQueryGraphObligationExecutionBudget,
) -> ForgeQueryGraphObligationDispatchPlan {
    ForgeQueryGraphObligationDispatchPlan::blocking_invariant("topology.loop-wiring")
        .with_rule_identity(rule("topology", "loop-wiring", "v1"))
        .with_execution_budget(execution_budget)
        .verdict(
            ForgeQueryGraphObligationVerdict::block(
                "loop successor would break closed-loop continuity",
            )
            .expect("blocking verdict"),
        )
        .expect("blocking plan")
}
