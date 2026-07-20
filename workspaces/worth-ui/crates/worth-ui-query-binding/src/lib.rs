//! Query binding surfaces grouped by lifecycle: subsystem entry → prerequisite boundary.

#[cfg(feature = "certification-construction")]
pub mod certification;
mod consumption;
mod declaration;
mod domain_marker;
mod domain_package;
pub mod entry;
mod installed_domain;
mod installed_measurements;
mod installed_reference;
mod live_admission;
mod live_resource;
mod native_aspect_contracts;
pub mod prerequisites;

// Subsystem entry lane
pub use consumption::{WorthUiQueryLiveProjectionOutcome, WorthUiQuerySnapshotProjectionOutcome};
pub use declaration::{
    WorthUiInstalledLiveQueryView, WorthUiInstalledQueryView, WorthUiInstalledSnapshotQueryView,
    WorthUiQueryBindingContractIdentity, WorthUiQueryViewDeclarationDenial,
    WorthUiQueryViewDefinition, WorthUiQueryViewDefinitionDigest, WorthUiQueryViewIdentity,
    WorthUiQueryViewIdentityError, WorthUiQueryViewLifecycle, WorthUiQueryViewProjectionDenial,
    WorthUiQueryViewShape,
};
pub use domain_marker::WorthUiDomainEntry;
pub use domain_package::worth_ui_domain_package;
pub use entry::{
    WorthUiInstalledQueryBindingPlan, WorthUiPreparedQueryBindingSuccession,
    WorthUiQueryBindingPlan, WorthUiQueryBindingRegistrationDenial,
    WorthUiQueryBindingRegistrationDenialKind, WorthUiQueryBindingSuccessionChange,
    WorthUiQueryBindingSuccessionDenial, WorthUiQueryViewExecutionEvidenceDenial,
    WorthUiRuntimeQueryBinding,
};
pub use installed_domain::{
    WorthUiInstalledQueryDomain, WorthUiQueryDomainRebindDenial,
    WorthUiQueryDomainRebindDenialKind, WorthUiQueryDomainRebindReceipt,
    WorthUiQueryInstallationDenial, WorthUiQueryInstallationDenialKind, WorthUiQueryWorkspaceExt,
};
pub use installed_measurements::{WorthUiMeasurementContribution, WorthUiQueryExt};
pub use installed_reference::WorthUiInstalledQueryBindingReference;
pub use live_admission::{WorthUiQueryLiveAdmissionDenial, WorthUiQueryLiveAdmissionStop};
pub use live_resource::{
    WorthUiQueryLiveAuthorityCloseStop, WorthUiQueryLiveCloseOutcome, WorthUiQueryLiveOpenError,
    WorthUiQueryLiveOpenOutcome, WorthUiQueryLiveRead, WorthUiQueryLiveResource,
    WorthUiQueryLiveRetirement, WorthUiQueryLiveRetirementAuthorityStop,
    WorthUiQueryLiveRetirementCloseOutcome, WorthUiQueryLiveRetirementCloseReceipt,
    WorthUiQueryLiveRetirementRuntimeStop, WorthUiQueryLiveRuntimeCloseStop,
};
pub use native_aspect_contracts::worth_ui_native_aspect_contracts;
// Prerequisite boundary lane
pub use prerequisites::{
    WorthUiQueryAllocationConsumptionIdentity, WorthUiQueryAllocationInvalidationBasis,
    WorthUiQueryAllocationSourceGeneration, WorthUiQueryAllocationSourceIdentity,
    WorthUiQueryAllocationSourceOrder, WorthUiQueryAuthorityHandle, WorthUiQueryAuthorityIndexKey,
    WorthUiQueryBasisAuthority, WorthUiQueryBasisIdentity, WorthUiQueryBasisPosture,
    WorthUiQueryCausalExplanationLane, WorthUiQueryInspectionLane,
    WorthUiQueryMeasurementFactEligibility, WorthUiQueryMeasurementFactEligibilityError,
    WorthUiQueryMeasurementFactFamily, WorthUiQueryMeasurementFactObservation,
    WorthUiQueryMeasurementFactObservationError, WorthUiQueryMeasurementFactReceipt,
    WorthUiQueryMeasurementFactReceiptError, WorthUiQueryMeasurementFactSettlement,
    WorthUiQueryMeasurementFactSettlementDenial, WorthUiQueryMeasurementRefinementCounters,
    WorthUiQueryPrerequisiteBoundary, WorthUiQueryPrerequisiteEvidence,
    WorthUiQueryPrerequisiteEvidenceError, WorthUiQueryProjectionConsumptionLane,
    WorthUiQueryProjectionContractIdentity, WorthUiQueryProjectionWarningKind,
    WorthUiQueryResolutionMode, WorthUiQueryViewExecutionEvidenceReference,
};

#[cfg(test)]
mod installed_measurements_tests;
#[cfg(test)]
mod live_resource_tests;
#[cfg(test)]
mod succession_tests;
