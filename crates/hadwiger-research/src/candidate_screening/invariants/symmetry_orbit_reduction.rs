use super::{definition_row, A, F, T};
use crate::candidate_screening::definitions::CandidateScreeningInvariantDefinition;

pub(crate) fn definition() -> CandidateScreeningInvariantDefinition {
    definition_row(
        F::SymmetryOrbitReduction,
        "symmetry_orbit_reduction",
        "Symmetry-orbit reduction test",
        T::DiscoverySupport,
        A::FiniteConflictGraph,
        "Symmetry quotients reduce search and expose hidden constraints.",
        "symmetry-reduced constraints are inconsistent or contradict full graph constraints",
        "group action, orbit, and stabilizer certificate",
    )
}
