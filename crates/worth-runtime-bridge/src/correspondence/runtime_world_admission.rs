mod admission;
mod basis;
mod denial;
#[cfg(test)]
mod tests;

pub use basis::AdmittedRuntimeWorldCorrespondenceBasis;
pub use denial::RuntimeWorldCorrespondenceAdmissionDenial;

pub(crate) use admission::{admit_installed_basis, compare_current_basis};
