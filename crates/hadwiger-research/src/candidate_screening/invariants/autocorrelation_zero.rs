use super::{definition_row, A, F, T};
use crate::candidate_screening::definitions::CandidateScreeningInvariantDefinition;

pub(crate) fn definition() -> CandidateScreeningInvariantDefinition {
    definition_row(
        F::AutocorrelationZero,
        "autocorrelation_zero",
        "Autocorrelation zero test",
        T::CertificateRequired,
        A::GeneratedPattern,
        "Each color class must have zero autocorrelation on every unit vector.",
        "area(C_i intersect (C_i+u)) > 0 for some |u|=1",
        "measure/raster certificate with exact or interval replay",
    )
}
