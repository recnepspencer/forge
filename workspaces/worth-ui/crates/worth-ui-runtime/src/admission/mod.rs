mod boundary;
mod criteria;
mod inspection;
mod legality;
mod report;
mod support;

pub use boundary::{UiAdmissionBoundary, UiAdmissionTarget, UiAdmissionWorld};
pub use criteria::{
    UiAdmissionFamily, UiAdmissionHostCapability, UiAdmissionQueryBasis,
    UiAdmissionSelectionBudget, UiAdmissionStaleEvidence,
};
pub use legality::{UiLegalityDecision, UiLegalityPosture, UiLegalityReason};
pub use report::{
    UiAdmissionAggregation, UiAdmissionDecision, UiAdmissionOutcome, UiAdmissionReport,
};
pub use support::{
    UiMeasurementAdmission, UiMeasurementAdmissionPosture, UiMeasurementCapabilityGateReason,
    UiMeasurementUnsupportedReason,
};
pub(crate) use support::{UiMeasurementAdmissionInput, UiQueryMeasurementEligibilityInput};
pub use support::{
    UiQueryMeasurementBasisAuthority, UiQueryMeasurementEligibility,
    UiQueryMeasurementEligibilityPosture, UiQueryMeasurementUnsupportedQueryReason,
};
pub use support::{UiSupportPosture, UiSupportReason, UiSupportSnapshot};
