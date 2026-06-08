use super::{definition_row, A, F, T};
use crate::candidate_screening::definitions::CandidateScreeningInvariantDefinition;

pub(crate) fn definition() -> CandidateScreeningInvariantDefinition {
    definition_row(
        F::FinitePatchBoundaryExtension,
        "finite_patch_boundary_extension",
        "Finite patch boundary-extension test",
        T::DiscoverySupport,
        A::GeneratedPattern,
        "Finite colorable patches may fail to extend to forced neighborhoods.",
        "boundary colorings do not extend to required larger patches",
        "bounded extension search certificate",
    )
}
