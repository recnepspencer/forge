use super::{definition_row, A, F, T};
use crate::candidate_screening::definitions::CandidateScreeningInvariantDefinition;

pub(crate) fn definition() -> CandidateScreeningInvariantDefinition {
    definition_row(F::ExhaustiveLocalNeighborhood, "exhaustive_local_neighborhood", "Exhaustive local-neighborhood test", T::ExactCheckerReady, A::PointEmbedding, "Visible neighbors are not enough; all bounded-radius unit-distance interactions must be checked.", "a generated local unit-distance neighbor has the same color", "bounded neighborhood generation certificate")
}
