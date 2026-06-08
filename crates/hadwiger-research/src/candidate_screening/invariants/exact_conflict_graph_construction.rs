use super::{definition_row, A, F, T};
use crate::candidate_screening::definitions::CandidateScreeningInvariantDefinition;

pub(crate) fn definition() -> CandidateScreeningInvariantDefinition {
    definition_row(
        F::ExactConflictGraphConstruction,
        "exact_conflict_graph_construction",
        "Exact conflict graph construction",
        T::ExactCheckerReady,
        A::TileConflictGraph,
        "Conflict graph edges are certified by unit-distance possibility, not vague adjacency.",
        "an edge is missing or present contrary to 1 in Delta(T_i,T_j)",
        "exact tile-pair conflict certificate",
    )
}
