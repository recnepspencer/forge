use super::{definition_row, A, F, T};
use crate::candidate_screening::definitions::CandidateScreeningInvariantDefinition;

pub(crate) fn definition() -> CandidateScreeningInvariantDefinition {
    definition_row(
        F::UnitDistanceEmbeddability,
        "unit_distance_embeddability",
        "Unit-distance embeddability test",
        T::ExactCheckerReady,
        A::PointEmbedding,
        "Chromatic graphs matter only if they realize actual unit distances in the plane.",
        "any edge fails |p_i-p_j|^2=1, or optional non-edge exclusions fail",
        "exact coordinate and edge-distance certificate",
    )
}
