//! Product-facing Query view declaration and registration.
//!
//! Query execution, settlement, native access, live leases, and patch
//! translation remain owned by `worth-ui-query-binding`. The product facade
//! exposes only the sealed capabilities needed to name and register UI intent.

pub use worth_ui_runtime::facade::entry::{
    WorthUiProjectionRegistrationError, WorthUiQueryViewRegistrationError,
};
pub use worth_ui_runtime::facade::query_binding::{
    UiCollectionCompleteness, UiCollectionContinuation, UiCollectionProjectionBinding,
    UiCollectionProjectionBindingAdmission, UiCollectionProjectionObservation,
    UiCollectionProjectionRegistration, UiCollectionProjectionRowReference,
    UiCollectionProjectionTextRow, UiCollectionProjectionValue, UiCollectionSchemaRequirement,
    UiInstalledProjectionView, UiNativeTextValue, UiPresentProjection, UiProjectionAvailability,
    UiProjectionBinding, UiProjectionBindingCompatibilityProof, UiProjectionBindingStopKind,
    UiProjectionBindingStopReceipt, UiProjectionConsumptionBudget,
    UiProjectionConsumptionBudgetError, UiProjectionConsumptionLimits,
    UiProjectionFieldRequirement, UiProjectionFieldRequirementError,
    UiProjectionLifecycleRequirement, UiProjectionNativeFamily, UiProjectionObservation,
    UiProjectionRetainedActivityKind, UiProjectionRetainedActivityReceipt, UiProjectionShape,
    UiProjectionUnavailableKind, UiProjectionUnavailableReceipt,
    UiQueryIdentityReportingProjection, UiQueryObservationReportingProjection,
    UiScalarProjectionBinding, UiScalarProjectionBindingAdmission, UiScalarProjectionObservation,
    UiScalarProjectionRegistration, UiScalarSchemaRequirement, WorthUiInstalledQueryDomain,
    WorthUiInstalledQueryView, WorthUiInstalledSnapshotQueryView,
    WorthUiQueryBindingRegistrationDenial, WorthUiQueryBindingRegistrationDenialKind,
    WorthUiQueryHostInstallationRequest, WorthUiQueryViewDeclarationDenial,
    WorthUiQueryViewDefinition, WorthUiQueryViewIdentity, WorthUiQueryViewIdentityError,
    WorthUiQueryViewLifecycle, WorthUiQueryViewRegistration, WorthUiQueryViewShape,
    WorthUiScalarProjectionAdvance, WorthUiScalarProjectionAdvanceError,
    WorthUiScalarProjectionHostCompletion, WorthUiScalarProjectionHostPlan,
    WorthUiScalarProjectionInstallation, WorthUiScalarProjectionInstallationError,
    WorthUiScalarProjectionLiveOwner, WorthUiScalarProjectionPublicationCompletion,
    WorthUiScalarProjectionSourceCloseError, WorthUiScalarProjectionSourceCloseReceipt,
    WorthUiScalarProjectionSourceRecord,
};
