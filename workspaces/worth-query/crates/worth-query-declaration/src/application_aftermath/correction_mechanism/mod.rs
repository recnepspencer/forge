//! Declared correction-mechanism axis.
//!
//! Populated with exactly `RecordedInverse` and `Compensation` for Milestone
//! 9.16. Deterministic re-derivation is a committed future sibling under this
//! parent axis and must not appear as an empty placeholder file.

mod compensation;
mod recorded_inverse;

pub use compensation::DeclaredCompensation;
pub use recorded_inverse::{
    DeclaredLoweringCorrespondenceRef, DeclaredPreImageDemand, DeclaredPreImageDemandDenial,
    DeclaredRecordedInverse,
};

/// How the corrected state is produced when correction authority permits it.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum DeclaredCorrectionMechanism {
    RecordedInverse(DeclaredRecordedInverse),
    Compensation(DeclaredCompensation),
}
