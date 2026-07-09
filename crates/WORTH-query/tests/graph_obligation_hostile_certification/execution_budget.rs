use worth_query::facade::runtime::{
    WorthQueryGraphObligationBudgetExceededPolicy, WorthQueryGraphObligationExecutionStatus,
    WorthQueryGraphObligationIndex, WorthQueryGraphObligationMaterializedDispatch,
    WorthQueryGraphObligationRegistrationCatalog,
};

use super::support::{budget_limited_registration, committed_world, graph_mutation_touch};

#[test]
fn budget_denial_happens_before_unbounded_state_load() {
    let registration = budget_limited_registration();
    let budget = registration.execution_budget().clone();
    let catalog =
        WorthQueryGraphObligationRegistrationCatalog::from_registrations(vec![registration])
            .expect("budget-limited catalog");
    let selection = WorthQueryGraphObligationIndex::from_catalog(&catalog)
        .select_for_touch(&graph_mutation_touch(), &committed_world());
    let envelope = WorthQueryGraphObligationMaterializedDispatch::from_selection(selection)
        .selected_result_envelope();

    let row = envelope.rows().first().expect("budget result row");

    assert_eq!(
        row.status(),
        WorthQueryGraphObligationExecutionStatus::BudgetExceeded
    );
    assert_eq!(
        budget.budget_exceeded_policy(),
        WorthQueryGraphObligationBudgetExceededPolicy::FailClosed
    );
    assert_eq!(row.state_load_counters().loaded_state_scope_count(), 1);
    assert_eq!(row.state_load_counters().traversed_edge_count(), 0);
    assert_eq!(row.state_load_counters().materialized_row_count(), 0);
    assert_eq!(
        row.verdict().and_then(|verdict| verdict.context()),
        Some("obligation-execution-budget-exceeded")
    );
}
