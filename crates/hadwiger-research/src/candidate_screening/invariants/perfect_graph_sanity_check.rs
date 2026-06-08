use super::{definition_row, A, F, T};
use crate::candidate_screening::definitions::CandidateScreeningInvariantDefinition;

pub(crate) fn definition() -> CandidateScreeningInvariantDefinition {
    definition_row(
        F::PerfectGraphSanityCheck,
        "perfect_graph_sanity_check",
        "Perfect-graph sanity check",
        T::GraphTheoreticBound,
        A::FiniteConflictGraph,
        "Perfect graphs satisfy chi(G)=omega(G).",
        "G is perfect and omega(G) <= 6 for a claimed 7-lower-bound witness",
        "perfectness and clique certificate",
    )
}
