use super::{definition_row, A, F, T};
use crate::candidate_screening::definitions::CandidateScreeningInvariantDefinition;

pub(crate) fn definition() -> CandidateScreeningInvariantDefinition {
    definition_row(
        F::FractionalChromaticNumber,
        "fractional_chromatic_number",
        "Fractional chromatic number",
        T::GraphTheoreticBound,
        A::FiniteConflictGraph,
        "Fractional chromatic number lower-bounds chromatic number.",
        "chi_f(G) > 6",
        "certified fractional-coloring lower bound",
    )
}
