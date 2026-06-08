use super::{definition_row, A, F, T};
use crate::candidate_screening::definitions::CandidateScreeningInvariantDefinition;

pub(crate) fn definition() -> CandidateScreeningInvariantDefinition {
    definition_row(
        F::SubstitutionConsistency,
        "substitution_consistency",
        "Substitution consistency test",
        T::CertificateRequired,
        A::GeneratedPattern,
        "Recursive substitutions must preserve internal, boundary, and cross-level legality.",
        "legality holds at one level but fails at the next or parent-child colors are incompatible",
        "substitution-level replay certificate",
    )
}
