//! Worth UI binding for Query-installed operations.
//!
//! The ordinary surface follows installed reference, consumer progression,
//! exact settlement, operation-native live delivery, and downstream
//! observation.

mod application_binding;
#[cfg(any(test, feature = "certification-construction"))]
pub mod certification;
mod collection_delivery;
mod declaration;
mod domain_marker;
mod domain_package;
pub mod entry;
mod inspection;
mod installed_domain;
mod native_aspect_contracts;
mod operation_live;
#[cfg(test)]
mod snapshot_derivation_denial_tests;
#[cfg(test)]
mod snapshot_progression_tests;
#[cfg(test)]
mod snapshot_refresh_isolation_tests;

// Subsystem entry lane
pub use application_binding::{
    WorthUiAdmittedQueryBindingKey, WorthUiAdmittedQueryBindingReference,
    WorthUiAdmittedQuerySettlementReference, WorthUiAdmittedQuerySettlementTouchReference,
    WorthUiBoundSnapshotMeasurement, WorthUiConsumedSnapshotProjection,
    WorthUiDeferredSnapshotConsumer, WorthUiExactSettledSnapshotEvidence,
    WorthUiExecutedSnapshotConsumer, WorthUiInstalledQueryBindingReference,
    WorthUiNativeAccessBindingCounters, WorthUiNativeKeyResolutionCounters,
    WorthUiPreparedSnapshotConsumer, WorthUiPublishedSnapshotConsumer,
    WorthUiQueryAllocationDetail, WorthUiQueryConsumerRequirements, WorthUiQueryDenialPresentation,
    WorthUiQueryInspectionRelevance, WorthUiQueryMeasurementFactFamily,
    WorthUiQueryMeasurementFactObservation, WorthUiQueryMeasurementFactObservationError,
    WorthUiQueryMeasurementRefinementCounters, WorthUiQueryOperationAttemptDenial,
    WorthUiReadmittedSettledSnapshotFact, WorthUiSettledMeasurementFactBatch,
    WorthUiSettledSnapshotDerivationStop, WorthUiSettledSnapshotFact,
    WorthUiSettledSnapshotProjection, WorthUiSettledSnapshotReadmissionDenial,
    WorthUiSettledSnapshotSourceGeneration, WorthUiSettledSnapshotSourceOrder,
    WorthUiSnapshotConsumerExecutionOutcome, WorthUiSnapshotConsumerPreparationDenial,
    WorthUiSnapshotProjectionConsumptionOutcome, WorthUiSnapshotProjectionPublicationOutcome,
    WorthUiSnapshotProjectionSettlementOutcome,
};
pub use collection_delivery::{
    WorthUiCollectionAllocationEffect, WorthUiCollectionAllocationPolicy,
    WorthUiCollectionChangeConsequence, WorthUiCollectionChangeCounters,
    WorthUiCollectionChangeInspection, WorthUiCollectionChangeKind,
    WorthUiCollectionChangeSourceReference, WorthUiCollectionContinuationPosture,
    WorthUiCollectionGraphEffect, WorthUiCollectionIncrementalConsequence,
    WorthUiCollectionMeasurementEffect, WorthUiCollectionQueryWorkInspection,
    WorthUiCollectionResetConsequence, WorthUiCollectionResetReason,
    WorthUiCollectionResultPosture, WorthUiCollectionRowReference, WorthUiCollectionWarningPosture,
};
pub use declaration::{
    WorthUiInstalledLiveQueryView, WorthUiInstalledQueryView, WorthUiInstalledSnapshotQueryView,
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
pub use operation_live::{
    WorthUiCollectionChangeAdmissionDenial, WorthUiCollectionChangeAdmissionStop,
    WorthUiCollectionChangeHandoffRetryDenial, WorthUiCollectionChangePublicationReceipt,
    WorthUiCollectionChangeStagingReceipt, WorthUiExactOperationLiveResourceEvidence,
    WorthUiOperationLiveAdmissionDenial, WorthUiOperationLiveAdmissionStop,
    WorthUiOperationLiveChangeObservation, WorthUiOperationLiveCloseOutcome,
    WorthUiOperationLiveCloseReceipt, WorthUiOperationLiveCloseStop,
    WorthUiOperationLiveObservation, WorthUiOperationLiveOpenError,
    WorthUiOperationLiveOpenRequest, WorthUiOperationLiveRefreshDenial,
    WorthUiOperationLiveRefreshError, WorthUiOperationLiveRefreshOutcome,
    WorthUiOperationLiveRefreshRequest, WorthUiOperationLiveResource,
    WorthUiOperationLiveRetirement, WorthUiOperationLiveRetirementCloseOutcome,
    WorthUiOperationLiveRetirementCloseReceipt, WorthUiOperationLiveRetirementStop,
    WorthUiOperationLiveSourceRefreshOutcome, WorthUiOperationLiveSourceRefreshStop,
};

#[cfg(test)]
mod installed_operations_tests;
#[cfg(test)]
mod operation_live_tests;
