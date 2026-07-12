//! Query binding surfaces grouped by lifecycle: subsystem entry → prerequisite boundary.

pub mod entry;
pub mod prerequisites;

// Subsystem entry lane
pub use entry::{WorthUiQueryAllocationAdmission, WorthUiQueryBindingSubsystem};
// Prerequisite boundary lane
pub use prerequisites::{
    WorthUiQueryAllocationConsumptionIdentity, WorthUiQueryAllocationInvalidationBasis,
    WorthUiQueryAllocationSourceGeneration, WorthUiQueryAllocationSourceIdentity,
    WorthUiQueryAllocationSourceOrder, WorthUiQueryAuthorityHandle, WorthUiQueryAuthorityIndexKey,
    WorthUiQueryBasisPosture, WorthUiQueryCausalExplanationLane, WorthUiQueryInspectionLane,
    WorthUiQueryMeasurementFactEligibility, WorthUiQueryMeasurementFactEligibilityError,
    WorthUiQueryMeasurementFactFamily, WorthUiQueryMeasurementFactObservation,
    WorthUiQueryMeasurementFactObservationError, WorthUiQueryMeasurementFactReceipt,
    WorthUiQueryMeasurementFactReceiptError, WorthUiQueryMeasurementFactSettlement,
    WorthUiQueryMeasurementFactSettlementDenial, WorthUiQueryPrerequisiteBoundary,
    WorthUiQueryPrerequisiteEvidence, WorthUiQueryPrerequisiteEvidenceError,
    WorthUiQueryProjectionConsumptionLane,
};
