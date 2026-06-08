use super::{definition_row, A, F, T};
use crate::candidate_screening::definitions::CandidateScreeningInvariantDefinition;

pub(crate) fn definition() -> CandidateScreeningInvariantDefinition {
    definition_row(
        F::ForbiddenDisplacementSet,
        "forbidden_displacement_set",
        "Forbidden displacement set",
        T::ExactCheckerReady,
        A::PeriodicTiling,
        "Repeated tile copies conflict by forbidden displacement, not center distance alone.",
        "a same-color displacement vector lies in F_P",
        "exact Minkowski/difference displacement certificate",
    )
}
