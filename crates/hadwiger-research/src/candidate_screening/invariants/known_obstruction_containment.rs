use super::{definition_row, A, F, T};
use crate::candidate_screening::definitions::CandidateScreeningInvariantDefinition;

pub(crate) fn definition() -> CandidateScreeningInvariantDefinition {
    definition_row(
        F::KnownObstructionContainment,
        "known_obstruction_containment",
        "Known obstruction containment test",
        T::DiscoverySupport,
        A::DiscoveryMemory,
        "Known non-6-colorable or high-pressure subgraphs should kill repeats early.",
        "a retained known obstruction embeds in the candidate",
        "typed obstruction library embedding certificate",
    )
}
