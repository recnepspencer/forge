use super::{definition_row, A, F, T};
use crate::candidate_screening::definitions::CandidateScreeningInvariantDefinition;

pub(crate) fn definition() -> CandidateScreeningInvariantDefinition {
    definition_row(
        F::ExactUnitDistanceConflict,
        "exact_unit_distance_conflict",
        "Exact unit-distance conflict test",
        T::ExactCheckerReady,
        A::RegionGeometry,
        "Reject same-color regions exactly when their distance set contains 1.",
        "1 in Delta(A,B), or a certified compact interval crosses 1.",
        "exact geometry or interval certificate",
    )
}
