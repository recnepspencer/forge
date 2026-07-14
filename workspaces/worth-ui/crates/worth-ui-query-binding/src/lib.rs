//! Query binding surfaces grouped by lifecycle: subsystem entry → prerequisite boundary.

pub mod entry;
pub mod ordinary_query;
pub mod prerequisites;

// Subsystem entry lane
pub use entry::{WorthUiQueryAllocationAdmission, WorthUiQueryBindingSubsystem};
pub use ordinary_query::{
    declare_measurement_comparison, declare_measurement_history, declare_measurement_live,
    declare_measurement_read, inspect_measurement_read,
};
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

#[cfg(test)]
mod ordinary_query_tests;
