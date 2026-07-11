mod classification;
mod denial;
mod facade;
mod input;
mod outcome;
mod quarantine;
mod quarantine_authority;
mod readmission;
#[cfg(test)]
mod readmission_test_support;
#[cfg(test)]
pub(crate) mod readmission_tests;
#[cfg(test)]
pub(crate) mod tests;

pub use classification::{S8LayoutCorruptionClass, S8LayoutReadmissionSource};
pub use denial::S8CorruptionDenial;
pub use facade::{layout_corruption, LayoutCorruptionFacade};
pub use input::S8LayoutCorruptionInput;
pub use outcome::{
    S8LayoutCorruptionOutcome, S8LayoutCorruptionView, S8LayoutReadmissionOutcome,
    S8LayoutReadmissionView, S8QuarantineReadmissionRequirement, S8ReadmissionDenied,
    S8ReadmissionRequirement, S8UnsupportedCorruptionState,
};
pub use quarantine::S8LayoutQuarantineWitness;
pub use readmission::{S8LayoutReadmissionWitness, S8NativeReadmissionInput};
