mod registration_candidate;
mod registration_validation;
mod registration_validation_report;
#[cfg(test)]
mod registration_validation_tests;

pub(crate) use registration_candidate::{
    RegistrationCandidate, RegistrationCandidateDiagnostic, RegistrationDependency,
};
pub(crate) use registration_validation::validate_registration_candidates;
