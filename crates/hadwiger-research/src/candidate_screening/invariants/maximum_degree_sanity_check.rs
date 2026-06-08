use super::{definition_row, A, F, T};
use crate::candidate_screening::definitions::CandidateScreeningInvariantDefinition;

pub(crate) fn definition() -> CandidateScreeningInvariantDefinition {
    definition_row(
        F::MaximumDegreeSanityCheck,
        "maximum_degree_sanity_check",
        "Maximum-degree sanity check",
        T::HeuristicRanking,
        A::FiniteConflictGraph,
        "Low maximum degree without K7-type pressure is usually a poor 7-obstruction priority.",
        "Delta(G) <= 6 and no stronger obstruction witness is present",
        "ranking only; never proof authority",
    )
}
