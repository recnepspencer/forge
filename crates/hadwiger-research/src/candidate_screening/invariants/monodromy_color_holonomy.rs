use super::{definition_row, A, F, T};
use crate::candidate_screening::definitions::CandidateScreeningInvariantDefinition;

pub(crate) fn definition() -> CandidateScreeningInvariantDefinition {
    definition_row(
        F::MonodromyColorHolonomy,
        "monodromy_color_holonomy",
        "Monodromy / color-holonomy test",
        T::ExactCheckerReady,
        A::GeneratedPattern,
        "Closed loops of transformations must return compatible color permutations.",
        "a closed loop forces a tile/color to return with an incompatible permutation",
        "checked loop generator and permutation certificate",
    )
}
