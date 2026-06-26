use super::touched_closure::selected_wire_view_touched_closure;
use crate::derived_topology::invalidation_plan::selection::selection_test_fixtures::{
    admitted_legality_support, catalog_closeout,
};
use crate::derived_topology::invalidation_plan::selection::{
    DerivedInvalidationDensityPolicy, DerivedInvalidationQuerySupportEvidence,
    DerivedInvalidationSelectedPlan,
};

pub(in super::super) fn selected_wire_view_plan(
    operator_family: &'static str,
) -> DerivedInvalidationSelectedPlan {
    selected_wire_view_plan_with_query_read_digest(operator_family, "query.native.read.receipt")
}

pub(in super::super) fn selected_wire_view_plan_with_query_read_digest(
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
        &selected_wire_view_touched_closure(operator_family),
        &query_support,
        &admitted_legality_support(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .unwrap()
}
