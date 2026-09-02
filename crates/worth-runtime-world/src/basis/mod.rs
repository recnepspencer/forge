mod admission;
mod composite;
mod equivalence;

pub use admission::AdmittedCompositeRuntimeWorldBasis;
pub(crate) use admission::{admit_current, CompositeBasisAdmissionDenial};
pub(crate) use equivalence::compare_exact;
