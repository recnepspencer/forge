use super::touched_closures::{
    selected_shell_view_touched_closure, selected_shell_view_touched_closure_for_shell,
};
use crate::derived_topology::invalidation_plan::selection::selection_test_fixtures::{
    admitted_legality_support, admitted_query_support, catalog_closeout,
    unrelated_geometry_touched_closure,
};
use crate::derived_topology::invalidation_plan::selection::{
    DerivedInvalidationDensityPolicy, DerivedInvalidationQuerySupportEvidence,
    DerivedInvalidationSelectedPlan,
};

pub(crate) fn selected_shell_views_plan(
    operator_family: &'static str,
) -> DerivedInvalidationSelectedPlan {
    DerivedInvalidationSelectedPlan::lower(
        &catalog_closeout(),
        &selected_shell_view_touched_closure(operator_family),
        &admitted_query_support(),
        &admitted_legality_support(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .unwrap()
}

pub(crate) fn selected_shell_views_plan_with_query_read_digest(
    operator_family: &'static str,
    native_read_receipt_digest: &str,
) -> DerivedInvalidationSelectedPlan {
    let query_support = DerivedInvalidationQuerySupportEvidence::from_receipt_digests_for_tests(
        Some("query.projection.consumption.receipt".to_string()),
        Some(native_read_receipt_digest.to_string()),
        Some("query.native.write.receipt".to_string()),
    );
    DerivedInvalidationSelectedPlan::lower(
        &catalog_closeout(),
        &selected_shell_view_touched_closure(operator_family),
        &query_support,
        &admitted_legality_support(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .unwrap()
}

pub(crate) fn selected_shell_views_plan_for_shell(
    operator_family: &'static str,
    shell_slot: u64,
) -> DerivedInvalidationSelectedPlan {
    DerivedInvalidationSelectedPlan::lower(
        &catalog_closeout(),
        &selected_shell_view_touched_closure_for_shell(operator_family, shell_slot),
        &admitted_query_support(),
        &admitted_legality_support(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .unwrap()
}

pub(crate) fn unrelated_geometry_selected_plan() -> DerivedInvalidationSelectedPlan {
    DerivedInvalidationSelectedPlan::lower(
        &catalog_closeout(),
        &unrelated_geometry_touched_closure(),
        &admitted_query_support(),
        &admitted_legality_support(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .unwrap()
}
