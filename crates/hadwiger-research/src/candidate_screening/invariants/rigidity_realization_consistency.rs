use super::{definition_row, A, F, T};
use crate::candidate_screening::definitions::CandidateScreeningInvariantDefinition;

pub(crate) fn definition() -> CandidateScreeningInvariantDefinition {
    definition_row(F::RigidityRealizationConsistency, "rigidity_realization_consistency", "Rigidity / realization consistency test", T::CertificateRequired, A::PointEmbedding, "Point candidates should classify realization as impossible, flexible, locally rigid, or globally rigid.", "distance constraints are impossible or certification status is too weak for proof use", "rigidity matrix, interval solving, or realization certificate")
}
