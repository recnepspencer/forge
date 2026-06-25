use super::super::{
    ShellViewMigrationError, ShellViewReadSource, ShellViewReadStageCounters,
    ShellViewReadStageExecutor, ShellViewTouchedBoundaryRows,
};
use super::support::{
    selected_shell_view_touched_closure, selected_shell_views_plan, source_row,
    unrelated_geometry_selected_plan,
};
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;
use crate::derived_topology::invalidation_plan::selection::selection_test_fixtures::{
    admitted_legality_support, admitted_query_support, catalog_closeout,
    legality_support_missing_selected_legality_plan, query_support_missing_native_read,
    unrelated_geometry_touched_closure,
};
use crate::derived_topology::invalidation_plan::selection::{
    DerivedInvalidationDenialKind, DerivedInvalidationDensityPolicy,
    DerivedInvalidationExecutionAdmission, DerivedInvalidationSelectedPlan,
};

#[test]
fn unrelated_touched_closure_cannot_close_shell_view_migration() {
    let plan = unrelated_geometry_selected_plan();

    let read_source =
        ShellViewReadSource::from_rows(vec![source_row(1, 10, 3, 2, false, false)], 1).unwrap();

    let error = ShellViewReadStageExecutor::execute(&plan, read_source).unwrap_err();

    assert_eq!(
        error,
        ShellViewMigrationError::SelectedPlanMissingShellViewRow
    );
}

#[test]
fn selected_plan_without_shell_view_row_cannot_admit_shell_view_input() {
    let plan = unrelated_geometry_selected_plan();

    let read_source =
        ShellViewReadSource::from_rows(vec![source_row(1, 10, 3, 2, false, false)], 1).unwrap();

    let error = ShellViewReadStageExecutor::execute(&plan, read_source).unwrap_err();

    assert_eq!(
        error,
        ShellViewMigrationError::SelectedPlanMissingShellViewRow
    );
}

#[test]
fn read_source_rejects_touched_closure_from_different_selected_plan() {
    let plan = selected_shell_views_plan("loop-touch-a");
    let unrelated_closure = unrelated_geometry_touched_closure();
    let topology =
        crate::test_support::hostile_neighborhoods::interpretation_neighborhoods::open_shell_nmt_fan_view(4);

    let error =
        ShellViewReadSource::select_from_touched_closure(&plan, &unrelated_closure, &topology)
            .unwrap_err();

    assert_eq!(
        error,
        ShellViewMigrationError::ReadStageTouchedClosureNotBoundToSelectedPlan
    );
}

#[test]
fn read_stage_rejects_zero_selected_shell_view_rows() {
    let plan = selected_shell_views_plan("loop-touch");
    let read_source = ShellViewReadSource::from_rows(Vec::new(), 1).unwrap();

    let error = ShellViewReadStageExecutor::execute(&plan, read_source).unwrap_err();

    assert_eq!(
        error,
        ShellViewMigrationError::ReadStageTouchedClosureSelectedNoShellViewRows
    );
}

#[test]
fn read_stage_rejects_query_receipt_not_bound_to_read_source() {
    let plan = selected_shell_views_plan("query-proof-mismatch");
    let read_source = ShellViewReadSource::from_rows_with_counters_and_query_reports(
        vec![source_row(1, 10, 3, 2, false, false)],
        1,
        ShellViewReadStageCounters::for_selected_rows(1, 1),
        vec!["different.query.read.receipt".to_string()],
    )
    .unwrap();

    let error = ShellViewReadStageExecutor::execute(&plan, read_source).unwrap_err();

    assert_eq!(
        error,
        ShellViewMigrationError::ReadStageQueryReceiptNotBoundToSource
    );
}

#[test]
fn source_rows_cannot_claim_more_selected_than_available() {
    let error = ShellViewTouchedBoundaryRows::from_selected_rows_with_available_count(
        vec![
            source_row(1, 10, 3, 2, false, false),
            source_row(2, 10, 1, 2, false, false),
        ],
        1,
    )
    .unwrap_err();

    assert_eq!(
        error,
        ShellViewMigrationError::SelectedRowsExceedAvailableRows
    );
}

#[test]
fn read_source_rejects_empty_shell_or_source_identity() {
    let error = ShellViewReadSource::from_rows(
        vec![super::super::ShellViewBoundarySourceRow::new(
            "",
            "entity:0:1:1",
            "entity:0:1:1",
            "entity:0:10:1",
            "entity:0:3:1",
            "entity:0:10:1",
            "relation:0:50001:1",
            2,
            false,
            false,
        )],
        1,
    )
    .unwrap_err();

    assert_eq!(error, ShellViewMigrationError::ReadStageQueryProofInvalid);
}

#[test]
fn shell_view_family_missing_query_read_receipt_denies_before_product_construction() {
    let plan = DerivedInvalidationSelectedPlan::lower(
        &catalog_closeout(),
        &selected_shell_view_touched_closure("missing-query-read"),
        &query_support_missing_native_read(),
        &admitted_legality_support(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .unwrap();

    assert_shell_view_denied_before_product_construction(
        &plan,
        DerivedInvalidationDenialKind::MissingQuerySupport,
    );
}

#[test]
fn shell_view_family_missing_legality_receipt_denies_before_product_construction() {
    let plan = DerivedInvalidationSelectedPlan::lower(
        &catalog_closeout(),
        &selected_shell_view_touched_closure("missing-legality"),
        &admitted_query_support(),
        &legality_support_missing_selected_legality_plan(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .unwrap();

    assert_shell_view_denied_before_product_construction(
        &plan,
        DerivedInvalidationDenialKind::MissingLegalitySupport,
    );
}

fn assert_shell_view_denied_before_product_construction(
    plan: &DerivedInvalidationSelectedPlan,
    expected_kind: DerivedInvalidationDenialKind,
) {
    assert_eq!(
        plan.execution_admission(),
        DerivedInvalidationExecutionAdmission::Denied
    );
    assert!(plan.selected_rows().is_empty());
    assert!(plan.denied_rows().iter().any(|row| {
        row.kind() == expected_kind
            && row.family_identity() == DerivedTopologyProductFamilyIdentity::ShellViews
    }));
}
