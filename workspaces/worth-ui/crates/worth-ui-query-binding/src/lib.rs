//! Worth UI binding for Query-installed operations.
//!
//! The ordinary surface follows installed reference, consumer progression,
//! exact settlement, operation-native live delivery, and downstream
//! observation.

mod application_binding;
#[cfg(any(test, feature = "certification-construction"))]
pub mod certification;
mod collection_delivery;
#[cfg(test)]
mod collection_projection_binding_tests;
#[cfg(test)]
mod collection_projection_refresh_tests;
#[cfg(test)]
mod collection_text_projection_tests;
mod declaration;
mod domain_marker;
mod domain_package;
pub mod entry;
mod inspection;
mod installed_domain;
mod native_aspect_contracts;
mod operation_live;
mod product_projection;
#[cfg(test)]
mod product_projection_tests;
mod projection_binding;
mod projection_consumption;
mod projection_invalidation;
mod projection_observation;
mod query_binding_reference;
mod query_evidence_reference;
#[cfg(test)]
mod scalar_projection_async_fixture;
#[cfg(test)]
mod scalar_projection_drift_tests;
#[cfg(test)]
mod scalar_projection_lifecycle_tests;
#[cfg(test)]
mod scalar_text_progression_tests;
#[cfg(any(test, feature = "certification-construction"))]
mod scalar_text_projection_fixture;
#[cfg(test)]
mod scalar_text_projection_tests;
#[cfg(test)]
mod snapshot_derivation_denial_tests;
#[cfg(test)]
mod snapshot_progression_tests;
#[cfg(test)]
mod snapshot_refresh_isolation_tests;
#[cfg(test)]
mod succession_tests;

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
    UiCollectionProjectionRegistration, UiCollectionSchemaRequirement,
    UiCollectionSchemaRequirementError, UiInstalledProjectionView, UiProjectionFieldRequirement,
    UiProjectionFieldRequirementError, UiProjectionLifecycleRequirement, UiProjectionNativeFamily,
    UiProjectionShape, UiScalarProjectionRegistration, UiScalarSchemaRequirement,
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
    WorthUiQueryInstallationDenialKind, WorthUiQueryWorkspaceExt, WorthUiScalarTextProjection,
    WorthUiScalarTextProjectionFamily, WorthUiSnapshotMeasurement,
    WorthUiSnapshotMeasurementFamily,
};
#[cfg(any(test, feature = "certification-construction"))]
pub use installed_domain::{
    install_worth_ui_partial_collection_test_operation_executors,
    install_worth_ui_partial_test_operation_executors, install_worth_ui_test_operation_executors,
};
pub use native_aspect_contracts::worth_ui_native_aspect_contracts;
pub use operation_live::{
    WorthUiAdmittedCollectionChangePublication, WorthUiCollectionChangeAdmissionDenial,
    WorthUiCollectionChangeAdmissionStop, WorthUiCollectionChangeHandoffRetryDenial,
    WorthUiCollectionChangePublicationDenial, WorthUiCollectionChangePublicationReceipt,
    WorthUiCollectionChangePublicationStop, WorthUiCollectionChangeStagingReceipt,
    WorthUiExactOperationLiveResourceEvidence, WorthUiOperationLiveAdmissionDenial,
    WorthUiOperationLiveAdmissionStop, WorthUiOperationLiveChangeObservation,
    WorthUiOperationLiveCloseOutcome, WorthUiOperationLiveCloseReceipt,
    WorthUiOperationLiveCloseStop, WorthUiOperationLiveObservation, WorthUiOperationLiveOpenError,
    WorthUiOperationLiveOpenRequest, WorthUiOperationLiveRefreshDenial,
    WorthUiOperationLiveRefreshError, WorthUiOperationLiveRefreshOutcome,
    WorthUiOperationLiveRefreshRequest, WorthUiOperationLiveResource,
    WorthUiOperationLiveRetirement, WorthUiOperationLiveRetirementCloseOutcome,
    WorthUiOperationLiveRetirementCloseReceipt, WorthUiOperationLiveRetirementStop,
    WorthUiOperationLiveSourceRefreshOutcome, WorthUiOperationLiveSourceRefreshStop,
    WorthUiValidatedCollectionChangeObservation,
};
pub use product_projection::{
    WorthUiQueryHostInstallationRequest, WorthUiScalarProjectionActionAdvance,
    WorthUiScalarProjectionActionDenied, WorthUiScalarProjectionActionEvidence,
    WorthUiScalarProjectionActionExecution, WorthUiScalarProjectionActionIndeterminate,
    WorthUiScalarProjectionActionInstallation, WorthUiScalarProjectionActionLiveOwner,
    WorthUiScalarProjectionActionOutcome, WorthUiScalarProjectionActionPublicationCompletion,
    WorthUiScalarProjectionActionRequest, WorthUiScalarProjectionAdvance,
    WorthUiScalarProjectionAdvanceError, WorthUiScalarProjectionHostCompletion,
    WorthUiScalarProjectionHostPlan, WorthUiScalarProjectionInstallation,
    WorthUiScalarProjectionInstallationError, WorthUiScalarProjectionLiveOwner,
    WorthUiScalarProjectionPublicationCompletion, WorthUiScalarProjectionSourceCloseError,
    WorthUiScalarProjectionSourceCloseReceipt, WorthUiScalarProjectionSourceRecord,
};
pub use projection_binding::{
    UiCollectionProjectionBinding, UiCollectionProjectionBindingAdmission,
    UiCollectionProjectionOpenOutcome, UiCollectionProjectionOpenReceipt,
    UiCollectionProjectionOpenStop, UiCollectionProjectionOpenStopKind,
    UiCollectionProjectionRefreshError, UiCollectionProjectionRefreshOutcome,
    UiCollectionProjectionRefreshReceipt, UiCollectionProjectionReplacementOutcome,
    UiCollectionProjectionReplacementReceipt, UiCollectionProjectionReplacementStop,
    UiLiveCollectionProjection, UiLiveCollectionProjectionCloseOutcome,
    UiLiveCollectionProjectionCloseReceipt, UiLiveCollectionProjectionCloseStop,
    UiProjectionBinding, UiProjectionBindingCompatibilityProof, UiProjectionBindingStopKind,
    UiProjectionBindingStopReceipt, UiScalarProjectionBinding, UiScalarProjectionBindingAdmission,
    UiScalarProjectionReplacementOutcome, UiScalarProjectionReplacementReceipt,
    UiScalarProjectionReplacementStop,
};
pub use projection_consumption::{
    UiCollectionCompleteness, UiCollectionContinuation, UiCollectionProjectionBudget,
    UiCollectionProjectionBudgetError, UiCollectionProjectionChange,
    UiCollectionProjectionDelivery, UiCollectionProjectionFactReceipt,
    UiCollectionProjectionInputFact, UiCollectionProjectionRowReference,
    UiCollectionProjectionTextRow, UiCollectionProjectionValue, UiCollectionProjectionWorkCounters,
    UiNativeTextValue, UiPresentProjection, UiProjectionAvailability,
    UiProjectionConsumptionBudget, UiProjectionConsumptionBudgetError,
    UiProjectionConsumptionLimits, UiProjectionFactReceipt, UiProjectionFactReportingProjection,
    UiProjectionFactStopKind, UiProjectionFactStopReceipt, UiProjectionInputCollectionRow,
    UiProjectionInputFactReference, UiProjectionInputFactTransition, UiProjectionInputPosture,
    UiProjectionInputRevision, UiProjectionInputSlot, UiProjectionInputTransitionStopKind,
    UiProjectionInputTransitionWork, UiProjectionOptionReference, UiProjectionPostureTrace,
    UiProjectionRetainedActivityKind, UiProjectionRetainedActivityReceipt,
    UiProjectionTransitionPosture, UiProjectionUnavailableKind, UiProjectionUnavailableReceipt,
    UiScalarProjectionFactReceipt, UiScalarProjectionInputFact, UiScalarProjectionWorkCounters,
};
pub use projection_invalidation::{
    UiScalarProjectionBatchOutcome, UiScalarProjectionInitialError,
    UiScalarProjectionTransitionReceipt, UiScalarProjectionUnchangedReceipt,
};
pub use projection_observation::{
    UiCollectionProjectionObservation, UiProjectionObservation, UiScalarProjectionObservation,
};
pub use query_binding_reference::UiQueryBindingReference;
pub use query_evidence_reference::UiQueryEvidenceReference;

#[cfg(test)]
mod installed_operations_tests;
#[cfg(test)]
mod operation_live_tests;
#[cfg(test)]
mod projection_compatibility_tests;
#[cfg(test)]
mod projection_contract_tests;
