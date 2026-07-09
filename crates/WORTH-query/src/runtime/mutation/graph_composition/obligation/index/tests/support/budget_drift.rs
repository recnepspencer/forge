use crate::runtime::{
    WorthQueryGraphObligationBudgetExceededPolicy, WorthQueryGraphObligationExecutionBudget,
    WorthQueryGraphObligationExecutionScope, WorthQueryGraphObligationIndex,
    WorthQueryGraphObligationOperatingWorldDescriptor,
    WorthQueryGraphObligationOperatingWorldSelector,
};

use super::super::fixtures::{
    catalog, relation_kind_id_selector, schema_registration,
    symbolic_relation_retirement_descriptor,
};

#[test]
fn changing_registration_budget_changes_index_and_selection_identity() {
    let world_selector = WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority();
    let default_registration =
        schema_registration("schema", relation_kind_id_selector(), world_selector);
    let budgeted_registration =
        schema_registration("schema", relation_kind_id_selector(), world_selector)
            .with_execution_budget(sparse_relation_budget());

    let default_index =
        WorthQueryGraphObligationIndex::from_catalog(&catalog(vec![default_registration]));
    let budgeted_index =
        WorthQueryGraphObligationIndex::from_catalog(&catalog(vec![budgeted_registration]));
    let descriptor = symbolic_relation_retirement_descriptor();
    let world = WorthQueryGraphObligationOperatingWorldDescriptor::any_committed_authority();
    let default_selection = default_index.select_for_touch(&descriptor, &world);
    let budgeted_selection = budgeted_index.select_for_touch(&descriptor, &world);

    assert_ne!(default_index.index_digest(), budgeted_index.index_digest());
    assert_ne!(
        default_selection.selection_digest(),
        budgeted_selection.selection_digest()
    );
    assert_eq!(
        budgeted_selection
            .matched_execution_budgets()
            .next()
            .expect("selected budget")
            .budget_digest(),
        sparse_relation_budget().budget_digest()
    );
    assert_eq!(
        budgeted_selection.counters().registration_full_scan_count(),
        0,
        "budget projection must not require a full catalog scan"
    );
}

fn sparse_relation_budget() -> WorthQueryGraphObligationExecutionBudget {
    WorthQueryGraphObligationExecutionBudget::bounded_sparse(
        WorthQueryGraphObligationExecutionScope::TouchedRelationKind,
        WorthQueryGraphObligationBudgetExceededPolicy::FailClosed,
    )
}
