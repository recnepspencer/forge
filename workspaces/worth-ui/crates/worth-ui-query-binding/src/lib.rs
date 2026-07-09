//! Query binding surfaces grouped by lifecycle: subsystem entry → prerequisite boundary.

pub mod entry;
pub mod prerequisites;

// Subsystem entry lane
pub use entry::WorthUiQueryBindingSubsystem;
// Prerequisite boundary lane
pub use prerequisites::{
    WorthUiQueryBasisPosture, WorthUiQueryCausalExplanationLane, WorthUiQueryInspectionLane,
    WorthUiQueryMeasurementFactEligibility, WorthUiQueryMeasurementFactEligibilityError,
    WorthUiQueryMeasurementFactFamily, WorthUiQueryMeasurementFactObservation,
    WorthUiQueryMeasurementFactObservationError, WorthUiQueryMeasurementFactReceipt,
    WorthUiQueryMeasurementFactReceiptError, WorthUiQueryPrerequisiteBoundary,
    WorthUiQueryPrerequisiteEvidence, WorthUiQueryPrerequisiteEvidenceError,
    WorthUiQueryProjectionConsumptionLane,
};
