use super::{definition_row, A, F, T};
use crate::candidate_screening::definitions::CandidateScreeningInvariantDefinition;

pub(crate) fn definition() -> CandidateScreeningInvariantDefinition {
    definition_row(
        F::WeightedIndependenceNumberBound,
        "weighted_independence_number_bound",
        "Weighted independence-number bound",
        T::GraphTheoreticBound,
        A::TileConflictGraph,
        "Weighted independence catches unequal tile-density hiding.",
        "total weight / alpha_w(G) > 6",
        "certified weighted independent-set bound",
    )
}
