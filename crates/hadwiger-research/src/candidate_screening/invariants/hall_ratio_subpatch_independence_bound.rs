use super::{definition_row, A, F, T};
use crate::candidate_screening::definitions::CandidateScreeningInvariantDefinition;

pub(crate) fn definition() -> CandidateScreeningInvariantDefinition {
    definition_row(
        F::HallRatioSubpatchIndependenceBound,
        "hall_ratio_subpatch_independence_bound",
        "Hall-ratio subpatch independence bound",
        T::GraphTheoreticBound,
        A::FiniteConflictGraph,
        "Dense subgraphs can force more colors even when the whole graph looks mild.",
        "max_H |V(H)| / alpha(H) > 6, or weighted analogue",
        "certified dense subpatch witness",
    )
}
