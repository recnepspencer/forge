use super::{definition_row, A, F, T};
use crate::candidate_screening::definitions::CandidateScreeningInvariantDefinition;

pub(crate) fn definition() -> CandidateScreeningInvariantDefinition {
    definition_row(
        F::IndependenceNumberLowerBound,
        "independence_number_lower_bound",
        "Independence-number lower bound",
        T::GraphTheoreticBound,
        A::FiniteConflictGraph,
        "|V| / alpha(G) lower-bounds chromatic number.",
        "|V| / alpha(G) > 6",
        "certified maximum independent-set bound",
    )
}
