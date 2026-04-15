use super::bundles::{
    binding_conflict_hostile, rejection_row, snapshot_basis_resolution_failure_hostile,
    unsupported_backend_route_hostile, unsupported_fallback_shape_hostile,
};
use super::fixtures;
use crate::harness::planning_matrix::{PlanningPerturbationClass, PlanningRejectionRow};

pub(super) fn rejection_rows() -> Vec<PlanningRejectionRow> {
    vec![
        rejection_row(
            "unsupported-backend-route",
            PlanningPerturbationClass::RouteSemanticDifference,
            &fixtures::direct_preflight(),
            unsupported_backend_route_hostile(),
        ),
        rejection_row(
            "unsupported-fallback-shape",
            PlanningPerturbationClass::FallbackRejection,
            &fixtures::direct_preflight(),
            unsupported_fallback_shape_hostile(),
        ),
        rejection_row(
            "binding-fulfillment-conflict",
            PlanningPerturbationClass::BindingRejection,
            &fixtures::direct_preflight(),
            binding_conflict_hostile(),
        ),
        rejection_row(
            "snapshot-basis-resolution-failure",
            PlanningPerturbationClass::BasisResolutionFailure,
            &fixtures::direct_preflight(),
            snapshot_basis_resolution_failure_hostile(),
        ),
    ]
}
