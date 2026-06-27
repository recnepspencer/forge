mod snapshot_reference_validation_report;
mod snapshot_reference_validator;
mod snapshot_reference_violation;

pub use snapshot_reference_validation_report::SnapshotReferenceValidationReport;
pub(crate) use snapshot_reference_validator::validate_snapshot_references;
pub use snapshot_reference_violation::{
    SnapshotReferenceViolation, SnapshotReferenceViolationKind,
};
