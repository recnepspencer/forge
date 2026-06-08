use super::{definition_row, A, F, T};
use crate::candidate_screening::definitions::CandidateScreeningInvariantDefinition;

pub(crate) fn definition() -> CandidateScreeningInvariantDefinition {
    definition_row(
        F::NumericalMargin,
        "numerical_margin",
        "Numerical margin test",
        T::CertificateRequired,
        A::RegionGeometry,
        "Floating near misses are not proof; unresolved near-unit distances must be quarantined.",
        "same-color safety relies on floating distance without interval/exact clearance",
        "interval arithmetic or exact geometry margin certificate",
    )
}
