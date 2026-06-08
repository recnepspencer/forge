use super::{definition_row, A, F, T};
use crate::candidate_screening::definitions::CandidateScreeningInvariantDefinition;

pub(crate) fn definition() -> CandidateScreeningInvariantDefinition {
    definition_row(
        F::CriticalSubgraphExtraction,
        "critical_subgraph_extraction",
        "Critical-subgraph extraction",
        T::DiscoverySupport,
        A::FiniteConflictGraph,
        "Non-6-colorable graphs should be minimized into reusable obstruction evidence.",
        "a smaller non-6-colorable subgraph exists or criticality is untested",
        "checked minimality or obstruction extraction record",
    )
}
