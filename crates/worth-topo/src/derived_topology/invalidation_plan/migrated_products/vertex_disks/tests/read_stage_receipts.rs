use super::support::{
    query_native_shared_vertex_view, selected_vertex_disk_touched_closure,
    selected_vertex_disks_plan_with_query_read_digest,
};
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;
use crate::derived_topology::invalidation_plan::migrated_products::vertex_disks::{
    VertexDiskMigrationError, VertexDiskReadSource, VertexDiskReadStageExecutor,
};
use crate::derived_topology::invalidation_plan::selection::selection_test_fixtures::{
    admitted_legality_support, admitted_query_support, catalog_closeout,
    legality_support_missing_selected_legality_plan, query_support_missing_native_read,
};
use crate::derived_topology::invalidation_plan::selection::{
    DerivedInvalidationDenialKind, DerivedInvalidationDensityPolicy,
    DerivedInvalidationExecutionAdmission, DerivedInvalidationSelectedPlan,
};

#[test]
fn selected_plan_query_receipt_must_match_query_read_source_digest() {
    let fixture = query_native_shared_vertex_view("vertex-disk.phase-15.receipt-mismatch");
    let plan = selected_vertex_disks_plan_with_query_read_digest(
        "vertex-disk-receipt-mismatch",
        "different-query-read-digest",
    );
    let touched_closure = selected_vertex_disk_touched_closure("vertex-disk-receipt-mismatch");
    let read_source = VertexDiskReadSource::from_query_shared_vertex_neighborhood_views(
        &plan,
        &touched_closure,
        &[fixture.shared_vertex],
    )
    .unwrap();

    let error = VertexDiskReadStageExecutor::execute(&plan, read_source).unwrap_err();

    assert_eq!(
        error,
        VertexDiskMigrationError::ReadStageQueryReceiptNotBoundToSource
    );
}

#[test]
fn vertex_disk_missing_query_read_receipt_denies_before_product_construction() {
    let plan = DerivedInvalidationSelectedPlan::lower(
        &catalog_closeout(),
        &selected_vertex_disk_touched_closure("vertex-disk-missing-query-read"),
        &query_support_missing_native_read(),
        &admitted_legality_support(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .unwrap();

    assert_vertex_disk_denied_before_product_construction(
        &plan,
        DerivedInvalidationDenialKind::MissingQuerySupport,
    );
}

#[test]
fn vertex_disk_missing_legality_receipt_denies_before_product_construction() {
    let plan = DerivedInvalidationSelectedPlan::lower(
        &catalog_closeout(),
        &selected_vertex_disk_touched_closure("vertex-disk-missing-legality"),
        &admitted_query_support(),
        &legality_support_missing_selected_legality_plan(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .unwrap();

    assert_vertex_disk_denied_before_product_construction(
        &plan,
        DerivedInvalidationDenialKind::MissingLegalitySupport,
    );
}

fn assert_vertex_disk_denied_before_product_construction(
    plan: &DerivedInvalidationSelectedPlan,
    expected_kind: DerivedInvalidationDenialKind,
) {
    assert_eq!(
        plan.execution_admission(),
        DerivedInvalidationExecutionAdmission::Denied
    );
    assert!(plan
        .selected_rows()
        .iter()
        .all(|row| row.family_identity() != DerivedTopologyProductFamilyIdentity::VertexDisks));
    assert!(plan.denied_rows().iter().any(|row| {
        row.kind() == expected_kind
            && row.family_identity() == DerivedTopologyProductFamilyIdentity::VertexDisks
    }));
}
