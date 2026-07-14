mod absence;
mod readiness;
mod support_rows;
mod support_witness;

pub use absence::{SignalMergeSupportInspectionAbsence, SignalMergeSupportInspectionAbsenceKind};
pub use readiness::SignalMergeSupportReadinessPosture;
pub use support_rows::{
    SignalBranchBasisInspectionRow, SignalCompatibilityInspectionRow,
    SignalScopedMergeInspectionRow, SignalStrategyInspectionRow,
};
pub use support_witness::{
    SignalMergeSupportInspectionOutcome, SignalMergeSupportInspectionWitness,
};
