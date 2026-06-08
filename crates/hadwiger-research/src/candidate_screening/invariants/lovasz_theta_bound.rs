use super::{definition_row, A, F, T};
use crate::candidate_screening::definitions::CandidateScreeningInvariantDefinition;

pub(crate) fn definition() -> CandidateScreeningInvariantDefinition {
    definition_row(
        F::LovaszThetaBound,
        "lovasz_theta_bound",
        "Lovasz theta bound",
        T::CertificateRequired,
        A::FiniteConflictGraph,
        "Lovasz theta of the complement can lower-bound chromatic number.",
        "theta(complement(G)) > 6",
        "semidefinite certificate or independently checked bound",
    )
}
