use super::{definition_row, A, F, T};
use crate::candidate_screening::definitions::CandidateScreeningInvariantDefinition;

pub(crate) fn definition() -> CandidateScreeningInvariantDefinition {
    definition_row(
        F::TranslationRotationClosure,
        "translation_rotation_closure",
        "Translation / rotation closure test",
        T::ExactCheckerReady,
        A::GeneratedPattern,
        "Infinite extension generators must preserve all unit-distance constraints.",
        "a generated transform creates a same-color unit-distance conflict",
        "checked generator-closure certificate",
    )
}
