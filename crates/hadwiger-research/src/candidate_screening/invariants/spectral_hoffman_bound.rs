use super::{definition_row, A, F, T};
use crate::candidate_screening::definitions::CandidateScreeningInvariantDefinition;

pub(crate) fn definition() -> CandidateScreeningInvariantDefinition {
    definition_row(
        F::SpectralHoffmanBound,
        "spectral_hoffman_bound",
        "Spectral / Hoffman bound",
        T::GraphTheoreticBound,
        A::FiniteConflictGraph,
        "For regular or near-regular graphs, spectral bounds can certify color pressure.",
        "1 - d / lambda_min > 6 in the valid Hoffman regime",
        "checked eigenvalue and regularity certificate",
    )
}
