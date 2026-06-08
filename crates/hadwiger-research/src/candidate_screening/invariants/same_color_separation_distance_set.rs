use super::{definition_row, A, F, T};
use crate::candidate_screening::definitions::CandidateScreeningInvariantDefinition;

pub(crate) fn definition() -> CandidateScreeningInvariantDefinition {
    definition_row(F::SameColorSeparationDistanceSet, "same_color_separation_distance_set", "Same-color separation distance-set test", T::ExactCheckerReady, A::RegionGeometry, "Same-color rejection requires the exact distance set to contain 1, not only minimum distance <= 1.", "d_min <= 1 <= d_max for a certified connected compact pair", "exact distance-set certificate")
}
