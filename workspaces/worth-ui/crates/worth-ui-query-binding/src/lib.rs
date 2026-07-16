//! Query binding surfaces grouped by lifecycle: subsystem entry → prerequisite boundary.

mod domain_marker;
mod domain_package;
pub mod entry;
mod installed_measurements;
mod native_aspect_contracts;
pub mod prerequisites;

// Subsystem entry lane
pub use domain_marker::WorthUiDomainEntry;
pub use domain_package::worth_ui_domain_package;
pub use entry::{WorthUiQueryAllocationAdmission, WorthUiQueryBindingSubsystem};
pub use installed_measurements::{WorthUiMeasurementContribution, WorthUiQueryExt};
pub use native_aspect_contracts::worth_ui_native_aspect_contracts;
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
mod installed_measurements_tests;
