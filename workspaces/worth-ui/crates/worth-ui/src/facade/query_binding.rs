//! Product-facing Query view and projection contracts.
//!
//! Query execution, native access, and patch translation remain owned by
//! `worth-ui-query-binding`. The product facade exposes proof-carrying view and
//! projection declaration, observation, installation, and source-lifecycle
//! contracts without exposing the raw Query runtime.

pub use worth_ui_runtime::facade::entry::{
    WorthUiProjectionRegistrationError, WorthUiQueryViewRegistrationError,
};
pub use worth_ui_runtime::facade::query_binding::{
    UiCollectionCompleteness, UiCollectionContinuation, UiCollectionProjectionBinding,
    UiCollectionProjectionBindingAdmission, UiCollectionProjectionFactReceipt,
    UiCollectionProjectionObservation, UiCollectionProjectionRegistration,
    UiCollectionProjectionRowReference, UiCollectionProjectionTextRow, UiCollectionProjectionValue,
    UiCollectionSchemaRequirement, UiInstalledProjectionView, UiNativeTextValue,
    UiPresentProjection, UiProjectionAvailability, UiProjectionBinding,
    UiProjectionBindingCompatibilityProof, UiProjectionBindingStopKind,
    UiProjectionBindingStopReceipt, UiProjectionConsumptionBudget,
    UiProjectionConsumptionBudgetError, UiProjectionConsumptionLimits, UiProjectionFactReceipt,
    UiProjectionFactReportingProjection, UiProjectionFactStopKind, UiProjectionFactStopReceipt,
    UiProjectionFieldRequirement, UiProjectionFieldRequirementError,
    UiProjectionLifecycleRequirement, UiProjectionNativeFamily, UiProjectionObservation,
    UiProjectionRetainedActivityKind, UiProjectionRetainedActivityReceipt, UiProjectionShape,
    UiProjectionUnavailableKind, UiProjectionUnavailableReceipt, UiQueryBindingReference,
    UiQueryEvidenceReference, UiScalarProjectionBinding, UiScalarProjectionBindingAdmission,
    UiScalarProjectionFactReceipt, UiScalarProjectionObservation, UiScalarProjectionRegistration,
    UiScalarSchemaRequirement, WorthUiInstalledQueryDomain, WorthUiInstalledQueryView,
    WorthUiInstalledSnapshotQueryView, WorthUiQueryBindingRegistrationDenial,
    WorthUiQueryBindingRegistrationDenialKind, WorthUiQueryHostInstallationRequest,
    WorthUiQueryViewDeclarationDenial, WorthUiQueryViewDefinition, WorthUiQueryViewIdentity,
    WorthUiQueryViewIdentityError, WorthUiQueryViewLifecycle, WorthUiQueryViewRegistration,
    WorthUiQueryViewShape, WorthUiScalarProjectionActionAdvance,
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
