use super::super::{
    TraversalViewsExecutionInput, TraversalViewsMigrationError, TraversalViewsReadSource,
    TraversalViewsReadStageExecutor,
};
use super::support::{
    selected_traversal_receipt, selected_traversal_touched_closure, selected_traversal_views_plan,
    traversal_views_plan_missing_legality, traversal_views_plan_missing_native_read,
};

#[test]
fn sparse_traversal_view_execution_counts_selected_rows_not_available_rows() {
    let plan = selected_traversal_views_plan();
    let touched_closure = selected_traversal_touched_closure();
    let topology =
        crate::test_support::hostile_neighborhoods::interpretation_neighborhoods::open_shell_nmt_fan_view(16);
    let read_source =
        TraversalViewsReadSource::select_from_touched_closure(&plan, &touched_closure, &topology)
            .unwrap();
    let available_traversal_count = read_source.available_traversal_count();
    let receipt = TraversalViewsReadStageExecutor::execute(&plan, read_source).unwrap();

    let closeout = super::support::close_traversal_views_slice(receipt);

    assert_eq!(
        closeout.counters().touched_closure_traversal_bound(),
        touched_closure_traversal_bound(&plan)
    );
    assert_eq!(closeout.counters().selected_traversal_count(), 2);
    assert_eq!(
        closeout.counters().available_traversal_count(),
        available_traversal_count
    );
    assert!(closeout.counters().available_traversal_count() > 2);
    assert_eq!(closeout.counters().execution_work_count(), 2);
    assert_eq!(closeout.counters().whole_view_fallback_count(), 0);
}

#[test]
fn traversal_read_stage_cannot_select_more_rows_than_touched_closure_bound() {
    let plan = selected_traversal_views_plan();
    let topology =
        crate::test_support::hostile_neighborhoods::interpretation_neighborhoods::open_shell_nmt_fan_view(16);
    let touched_bound = touched_closure_traversal_bound(&plan);
    let read_source = TraversalViewsReadSource::from_topology_view_with_selected_prefix(
        &topology,
        touched_bound + 1,
    )
    .unwrap();

    let error = TraversalViewsReadStageExecutor::execute(&plan, read_source).unwrap_err();

    assert_eq!(
        error,
        TraversalViewsMigrationError::ReadStageSelectedRowsExceedTouchedClosure
    );
}

#[test]
fn selected_traversal_rows_cannot_exceed_available_read_rows() {
    let topology =
        crate::test_support::hostile_neighborhoods::interpretation_neighborhoods::open_shell_nmt_fan_view(4);
    let available_traversals =
        TraversalViewsReadSource::from_topology_view_with_selected_prefix(&topology, 1)
            .unwrap()
            .available_traversal_count();

    let error = TraversalViewsReadSource::from_topology_view_with_selected_prefix(
        &topology,
        available_traversals + 1,
    )
    .unwrap_err();

    assert_eq!(
        error,
        TraversalViewsMigrationError::ReadStageSelectedRowsExceedAvailableRows
    );
}

fn touched_closure_traversal_bound(
    plan: &crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedPlan,
) -> usize {
    let counters = plan.counters();
    counters.touched_entity_count()
        + counters.touched_relation_count()
        + counters.touched_relation_kind_count()
        + counters.touched_aspect_count()
        + counters.touched_scope_count()
}

#[test]
fn traversal_read_source_requires_same_touched_closure_as_selected_plan() {
    let plan = selected_traversal_views_plan();
    let unrelated_plan = traversal_views_plan_missing_native_read();
    let topology =
        crate::test_support::hostile_neighborhoods::interpretation_neighborhoods::open_shell_nmt_fan_view(4);
    let unrelated_closure = selected_traversal_touched_closure();

    let error = TraversalViewsReadSource::select_from_touched_closure(
        &unrelated_plan,
        &unrelated_closure,
        &topology,
    )
    .unwrap_err();

    assert_eq!(
        error,
        TraversalViewsMigrationError::ReadStageTouchedClosureNotBoundToSelectedPlan
    );
    assert_ne!(
        plan.touched_closure_digest(),
        unrelated_plan.touched_closure_digest()
    );
}

#[test]
fn traversal_read_source_rejects_selected_plan_with_no_touched_traversal_rows() {
    let plan = selected_traversal_views_plan();
    let touched_closure = selected_traversal_touched_closure();
    let unrelated_topology =
        crate::test_support::hostile_neighborhoods::interpretation_neighborhoods::open_wire_chain_view(
            4,
        );

    let error = TraversalViewsReadSource::select_from_touched_closure(
        &plan,
        &touched_closure,
        &unrelated_topology,
    )
    .unwrap_err();

    assert_eq!(
        error,
        TraversalViewsMigrationError::ReadStageTouchedClosureSelectedNoTraversalRows
    );
}

#[test]
fn execution_input_requires_read_stage_to_match_selected_plan() {
    let plan = selected_traversal_views_plan();
    let unrelated_receipt =
        selected_traversal_receipt().with_selected_plan_digest_for_tests("forged-plan");

    let error =
        TraversalViewsExecutionInput::from_selected_plan_and_read_stage(&plan, unrelated_receipt)
            .unwrap_err();

    assert_eq!(
        error,
        TraversalViewsMigrationError::ReadStageReceiptNotBoundToSelectedPlan
    );
}

#[test]
fn execution_input_requires_read_stage_query_receipt_to_match_selected_row() {
    let plan = selected_traversal_views_plan();
    let forged_receipt = selected_traversal_receipt()
        .with_native_query_read_receipt_digest_for_tests("forged-query-read");

    let error =
        TraversalViewsExecutionInput::from_selected_plan_and_read_stage(&plan, forged_receipt)
            .unwrap_err();

    assert_eq!(
        error,
        TraversalViewsMigrationError::ReadStageReceiptNotBoundToSelectedPlan
    );
}

#[test]
fn missing_query_read_support_denies_before_traversal_read_stage() {
    let plan = traversal_views_plan_missing_native_read();

    let error = TraversalViewsReadStageExecutor::execute(
        &plan,
        super::support::selected_traversal_read_source(),
    )
    .unwrap_err();

    assert_eq!(
        error,
        TraversalViewsMigrationError::SelectedPlanMissingTraversalViewsRow
    );
}

#[test]
fn missing_legality_support_denies_before_traversal_read_stage() {
    let plan = traversal_views_plan_missing_legality();

    let error = TraversalViewsReadStageExecutor::execute(
        &plan,
        super::support::selected_traversal_read_source(),
    )
    .unwrap_err();

    assert_eq!(
        error,
        TraversalViewsMigrationError::SelectedPlanMissingTraversalViewsRow
    );
}
