use super::bundles::canonical_row;
use super::fixtures;
use crate::harness::planning_matrix::{
    PlanningCertificationRow, PlanningHostileExpectation, PlanningPerturbationClass,
};

pub(super) fn canonical_rows() -> Vec<PlanningCertificationRow> {
    vec![
        canonical_row(
            "direct-runtime-plan-parity",
            PlanningPerturbationClass::DirectRuntimeParity,
            PlanningHostileExpectation::EquivalentToControl,
            fixtures::direct_preflight(),
            fixtures::direct_preflight(),
            fixtures::replay_preflight(),
        ),
        canonical_row(
            "replanned-runtime-parity",
            PlanningPerturbationClass::ReplayParity,
            PlanningHostileExpectation::EquivalentToControl,
            fixtures::direct_preflight(),
            fixtures::replay_preflight(),
            fixtures::direct_preflight(),
        ),
        canonical_row(
            "type-bound-runtime-parity",
            PlanningPerturbationClass::BindingParity,
            PlanningHostileExpectation::EquivalentToControl,
            fixtures::bound_preflight(),
            fixtures::pre_resolved_bound_preflight(),
            fixtures::bound_preflight(),
        ),
        canonical_row(
            "runtime-basis-repeatability",
            PlanningPerturbationClass::BasisRepeatability,
            PlanningHostileExpectation::EquivalentToControl,
            fixtures::direct_preflight(),
            fixtures::replay_preflight(),
            fixtures::direct_preflight(),
        ),
        canonical_row(
            "identity-bearing-binding-difference",
            PlanningPerturbationClass::BindingParity,
            PlanningHostileExpectation::DistinctFromControl,
            fixtures::bound_preflight(),
            fixtures::alternate_bound_preflight(),
            fixtures::pre_resolved_bound_preflight(),
        ),
        canonical_row(
            "basis-difference",
            PlanningPerturbationClass::BasisDifference,
            PlanningHostileExpectation::DistinctFromControl,
            fixtures::direct_preflight(),
            fixtures::alternate_basis_preflight(),
            fixtures::replay_preflight(),
        ),
        canonical_row(
            "route-semantic-difference",
            PlanningPerturbationClass::RouteSemanticDifference,
            PlanningHostileExpectation::DistinctFromControl,
            fixtures::direct_preflight(),
            fixtures::expanded_runtime_preflight(),
            fixtures::replay_preflight(),
        ),
    ]
}
