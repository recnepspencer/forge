use forge_query::facade::runtime::{
    ForgeQueryGraphObligationBudgetExceededPolicy, ForgeQueryGraphObligationExecutionStatus,
    ForgeQueryGraphObligationIndex, ForgeQueryGraphObligationMaterializedDispatch,
    ForgeQueryGraphObligationRegistrationCatalog,
};

use super::support::{budget_limited_registration, committed_world, graph_mutation_touch};

#[test]
fn budget_denial_happens_before_unbounded_state_load() {
    let registration = budget_limited_registration();
    let budget = registration.execution_budget().clone();
    let catalog =
        ForgeQueryGraphObligationRegistrationCatalog::from_registrations(vec![registration])
            .expect("budget-limited catalog");
    let selection = ForgeQueryGraphObligationIndex::from_catalog(&catalog)
        .select_for_touch(&graph_mutation_touch(), &committed_world());
    let envelope = ForgeQueryGraphObligationMaterializedDispatch::from_selection(selection)
        .selected_result_envelope();

    let row = envelope.rows().first().expect("budget result row");

    assert_eq!(
        row.status(),
        ForgeQueryGraphObligationExecutionStatus::BudgetExceeded
    );
    assert_eq!(
        budget.budget_exceeded_policy(),
        ForgeQueryGraphObligationBudgetExceededPolicy::FailClosed
    );
    assert_eq!(row.state_load_counters().loaded_state_scope_count(), 1);
    assert_eq!(row.state_load_counters().traversed_edge_count(), 0);
    assert_eq!(row.state_load_counters().materialized_row_count(), 0);
    assert_eq!(
        row.verdict().and_then(|verdict| verdict.context()),
        Some("obligation-execution-budget-exceeded")
    );
}
