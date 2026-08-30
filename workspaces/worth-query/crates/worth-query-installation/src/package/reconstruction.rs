//! Bounded intake progression for untrusted typed package records.

mod candidate;
mod canonical_order;
mod denial;
mod expected_identity;
mod fresh_validation;
mod limits;
mod materialization;
mod progression;
mod semantic_candidate;
pub(crate) mod work_observation;

pub use candidate::WorthQueryPortablePackageReconstructionCandidate;
pub use denial::WorthQueryPortablePackageReconstructionDenial;
pub use expected_identity::WorthQueryExpectedPortablePackageIdentity;
pub use limits::{
    WorthQueryPortablePackageReconstructionLimits,
    WORTH_QUERY_PORTABLE_PACKAGE_MAX_RECONSTRUCTION_CANONICAL_BYTES,
    WORTH_QUERY_PORTABLE_PACKAGE_MAX_RECONSTRUCTION_ENTRIES,
};
pub use progression::WorthQueryPortablePackageReconstruction;
pub use semantic_candidate::WorthQueryReconstructedPortablePackageCandidate;
pub use work_observation::WorthQueryPortablePackageReconstructionWork;

#[cfg(test)]
mod exact_semantic_readmission_tests;
#[cfg(test)]
mod recursive_work_ceiling_tests;
#[cfg(test)]
mod recursive_work_tests;
#[cfg(test)]
mod test_support;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod work_budget_tests;
