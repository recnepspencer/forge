use super::{definition_row, A, F, T};
use crate::candidate_screening::definitions::CandidateScreeningInvariantDefinition;

pub(crate) fn definition() -> CandidateScreeningInvariantDefinition {
    definition_row(F::BoundaryOwnership, "boundary_ownership", "Boundary ownership test", T::CertificateRequired, A::RegionGeometry, "Every boundary point must have exactly one defined color unless overlap ownership is explicitly modeled.", "uncovered, ambiguously owned, or same-color unit-distance boundary points exist", "boundary ownership and boundary conflict certificate")
}
