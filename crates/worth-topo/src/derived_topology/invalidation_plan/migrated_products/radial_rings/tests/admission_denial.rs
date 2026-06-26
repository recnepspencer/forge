use super::super::{
    RadialRingMigrationError, RadialRingReadSource, RadialRingReadStageExecutor,
    RadialRingTouchedBoundaryRows,
};
use super::support::{
    selected_radial_ring_touched_closure, selected_radial_rings_plan, source_row,
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
fn unrelated_touched_closure_cannot_close_radial_ring_migration() {
    let plan = unrelated_geometry_selected_plan();

    let read_source =
        RadialRingReadSource::from_rows(vec![source_row(1, 10, 3, 2, false, false)], 1).unwrap();

    let error = RadialRingReadStageExecutor::execute(&plan, read_source).unwrap_err();

    assert_eq!(
        error,
        RadialRingMigrationError::SelectedPlanMissingRadialRingRow
    );
}

#[test]
fn selected_plan_without_radial_ring_row_cannot_admit_radial_ring_input() {
    let plan = unrelated_geometry_selected_plan();

    let read_source =
        RadialRingReadSource::from_rows(vec![source_row(1, 10, 3, 2, false, false)], 1).unwrap();

    let error = RadialRingReadStageExecutor::execute(&plan, read_source).unwrap_err();

    assert_eq!(
        error,
        RadialRingMigrationError::SelectedPlanMissingRadialRingRow
    );
}

#[test]
fn read_source_rejects_touched_closure_from_different_selected_plan() {
    let plan = selected_radial_rings_plan("loop-touch-a");
    let unrelated_closure = unrelated_geometry_touched_closure();
    let topology =
        crate::test_support::hostile_neighborhoods::interpretation_neighborhoods::open_shell_nmt_fan_view(4);

    let error =
        RadialRingReadSource::select_from_touched_closure(&plan, &unrelated_closure, &topology)
            .unwrap_err();

    assert_eq!(
        error,
        RadialRingMigrationError::ReadStageTouchedClosureNotBoundToSelectedPlan
    );
}

#[test]
fn read_stage_rejects_zero_selected_radial_ring_rows() {
    let plan = selected_radial_rings_plan("loop-touch");
    let read_source = RadialRingReadSource::from_rows(Vec::new(), 1).unwrap();

    let error = RadialRingReadStageExecutor::execute(&plan, read_source).unwrap_err();

    assert_eq!(
        error,
        RadialRingMigrationError::ReadStageTouchedClosureSelectedNoRadialRingRows
    );
}

#[test]
fn source_rows_cannot_claim_more_selected_than_available() {
    let error = RadialRingTouchedBoundaryRows::from_selected_rows_with_available_count(
        vec![
            source_row(1, 10, 3, 2, false, false),
            source_row(2, 10, 1, 2, false, false),
        ],
        1,
    )
    .unwrap_err();

    assert_eq!(
        error,
        RadialRingMigrationError::SelectedRowsExceedAvailableRows
    );
}

#[test]
fn radial_family_missing_query_read_receipt_denies_before_product_construction() {
    let plan = DerivedInvalidationSelectedPlan::lower(
        &catalog_closeout(),
        &selected_radial_ring_touched_closure("missing-query-read"),
        &query_support_missing_native_read(),
        &admitted_legality_support(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .unwrap();

    assert_radial_denied_before_product_construction(
        &plan,
        DerivedInvalidationDenialKind::MissingQuerySupport,
    );
}

#[test]
fn radial_family_missing_legality_receipt_denies_before_product_construction() {
    let plan = DerivedInvalidationSelectedPlan::lower(
        &catalog_closeout(),
        &selected_radial_ring_touched_closure("missing-legality"),
        &admitted_query_support(),
        &legality_support_missing_selected_legality_plan(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .unwrap();

    assert_radial_denied_before_product_construction(
        &plan,
        DerivedInvalidationDenialKind::MissingLegalitySupport,
    );
}

fn assert_radial_denied_before_product_construction(
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
            && row.family_identity() == DerivedTopologyProductFamilyIdentity::RadialRings
    }));
}
