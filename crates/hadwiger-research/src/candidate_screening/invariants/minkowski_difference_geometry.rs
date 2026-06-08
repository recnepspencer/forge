use super::{definition_row, A, F, T};
use crate::candidate_screening::definitions::CandidateScreeningInvariantDefinition;

pub(crate) fn definition() -> CandidateScreeningInvariantDefinition {
    definition_row(
        F::MinkowskiDifferenceGeometry,
        "minkowski_difference_geometry",
        "Minkowski-difference geometry test",
        T::ExactCheckerReady,
        A::RegionGeometry,
        "Two regions conflict iff their Minkowski difference intersects the unit circle.",
        "(A-B) intersects S^1",
        "exact region-difference intersection certificate",
    )
}
