use super::{definition_row, A, F, T};
use crate::candidate_screening::definitions::CandidateScreeningInvariantDefinition;

pub(crate) fn definition() -> CandidateScreeningInvariantDefinition {
    definition_row(
        F::CliqueNumberLowerBound,
        "clique_number_lower_bound",
        "Clique-number lower bound",
        T::GraphTheoreticBound,
        A::FiniteConflictGraph,
        "Clique number lower-bounds chromatic number.",
        "omega(G) > 6 for a six-color candidate",
        "certified clique witness",
    )
}
