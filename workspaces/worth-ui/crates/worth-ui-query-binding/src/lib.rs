//! Worth UI binding for Query-installed snapshot operations.
//!
//! The ordinary surface follows installed reference, consumer progression,
//! exact settlement, and downstream observation. The predecessor projection
//! lane is isolated under `compatibility::managed_live` until Query's public
//! operation-native live lifecycle is available.

mod application_binding;
#[cfg(feature = "certification-construction")]
pub mod certification;
pub mod compatibility;
mod consumption;
mod declaration;
mod domain_marker;
mod domain_package;
pub mod entry;
mod inspection;
mod installed_domain;
mod native_aspect_contracts;
#[cfg(test)]
mod snapshot_progression_tests;

// Subsystem entry lane
pub use application_binding::{
    WorthUiBoundSnapshotMeasurement, WorthUiConsumedSnapshotProjection,
    WorthUiDeferredSnapshotConsumer, WorthUiExactSettledSnapshotEvidence,
    WorthUiExecutedSnapshotConsumer, WorthUiInstalledQueryBindingReference,
    WorthUiPreparedSnapshotConsumer, WorthUiPublishedSnapshotConsumer,
    WorthUiQueryAllocationDetail, WorthUiQueryConsumerRequirements, WorthUiQueryDenialPresentation,
    WorthUiQueryInspectionRelevance, WorthUiQueryMeasurementFactFamily,
    WorthUiQueryMeasurementFactObservation, WorthUiQueryMeasurementFactObservationError,
    WorthUiQueryMeasurementRefinementCounters, WorthUiQueryOperationAttemptDenial,
    WorthUiSettledMeasurementFactBatch, WorthUiSettledSnapshotFact,
    WorthUiSettledSnapshotProjection, WorthUiSettledSnapshotSourceGeneration,
    WorthUiSettledSnapshotSourceOrder, WorthUiSnapshotConsumerExecutionOutcome,
    WorthUiSnapshotConsumerPreparationDenial, WorthUiSnapshotProjectionConsumptionOutcome,
    WorthUiSnapshotProjectionPublicationOutcome, WorthUiSnapshotProjectionSettlementOutcome,
};
pub(crate) use compatibility::managed_live::{
    WorthUiExactManagedLiveResourceEvidence, WorthUiQueryLiveAdmissionDenial,
    WorthUiQueryLiveAdmissionStop, WorthUiQueryLiveOpenError, WorthUiQueryLiveOpenOutcome,
    WorthUiQueryLiveProjectionOutcome, WorthUiQueryLiveResource, WorthUiQueryLiveRetirement,
    WorthUiQueryMeasurementFactSettlement, WorthUiQueryMeasurementFactSettlementDenial,
};
pub use declaration::{
    WorthUiInstalledQueryView, WorthUiInstalledSnapshotQueryView,
    WorthUiQueryViewDeclarationDenial, WorthUiQueryViewDefinition,
    WorthUiQueryViewDefinitionDigest, WorthUiQueryViewIdentity, WorthUiQueryViewIdentityError,
    WorthUiQueryViewLifecycle, WorthUiQueryViewShape,
};
pub use domain_marker::WorthUiDomainEntry;
pub use domain_package::worth_ui_domain_package;
pub use entry::{
    WorthUiInstalledQueryBindingPlan, WorthUiPreparedQueryBindingSuccession,
    WorthUiQueryAllocationSourceGeneration, WorthUiQueryAllocationSourceOrder,
    WorthUiQueryBindingPlan, WorthUiQueryBindingRegistrationDenial,
    WorthUiQueryBindingRegistrationDenialKind, WorthUiQueryBindingSuccessionChange,
    WorthUiQueryBindingSuccessionDenial, WorthUiQueryFrameEvidence,
    WorthUiQueryReferenceMembershipObservation, WorthUiQueryViewExecutionEvidenceDenial,
    WorthUiQueryViewExecutionEvidenceReference, WorthUiRuntimeQueryBinding,
    WorthUiRuntimeQueryStateObservation, WorthUiSettledSnapshotAdmissionDenial,
    WorthUiSettledSnapshotAdmissionStop,
};
pub use inspection::{
    WorthUiExactQueryArtifactInspection, WorthUiQueryInspection,
    WorthUiQueryInspectionEvidencePolicy, WorthUiQueryInspectionMaterializationCounters,
    WorthUiSettledSnapshotInspection, WorthUiSettledSnapshotRichEvidence,
};
pub use installed_domain::{
    install_worth_ui_operation_executors, WorthUiInstalledQueryDomain, WorthUiMeasurementRecording,
    WorthUiMeasurementRecordingFamily, WorthUiQueryDomainRebindDenial,
    WorthUiQueryDomainRebindDenialKind, WorthUiQueryDomainRebindNextAction,
    WorthUiQueryDomainRebindReceipt, WorthUiQueryInstallationDenial,
    WorthUiQueryInstallationDenialKind, WorthUiQueryWorkspaceExt, WorthUiSnapshotMeasurement,
    WorthUiSnapshotMeasurementFamily,
};
#[cfg(any(test, feature = "certification-construction"))]
pub use installed_domain::{
    install_worth_ui_partial_test_operation_executors, install_worth_ui_test_operation_executors,
};
pub use native_aspect_contracts::worth_ui_native_aspect_contracts;

#[cfg(test)]
mod installed_operations_tests;
#[cfg(test)]
mod live_resource_tests;
#[cfg(test)]
mod succession_tests;
