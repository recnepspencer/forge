mod admission;
mod artifact_equivalence;
mod digest;
mod identity;
mod impact;
mod invalid_candidate;
mod narrowing;

pub(crate) use admission::diagnostic_for_candidate_admission;
pub(crate) use artifact_equivalence::diagnostic_for_artifact_equivalence;
pub(crate) use identity::diagnostic_for_identity_matching;
pub(crate) use impact::diagnostic_for_replacement_impact;
pub(crate) use invalid_candidate::diagnostic_for_invalid_candidate;
pub(crate) use narrowing::diagnostic_for_impact_narrowing;
