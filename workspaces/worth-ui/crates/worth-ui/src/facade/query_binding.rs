//! Product-facing Query view declaration and registration.
//!
//! Query execution, settlement, native access, live leases, and patch
//! translation remain owned by `worth-ui-query-binding`. The product facade
//! exposes only the sealed capabilities needed to name and register UI intent.

pub use worth_ui_runtime::facade::entry::WorthUiQueryViewRegistrationError;
pub use worth_ui_runtime::facade::query_binding::{
    UiCollectionCompleteness, UiCollectionContinuation, UiCollectionProjectionBinding,
    UiCollectionProjectionBindingAdmission, UiCollectionProjectionFactReceipt,
    UiCollectionProjectionRowReference, UiCollectionProjectionTextRow, UiCollectionProjectionValue,
    UiCollectionSchemaRequirement, UiNativeTextValue, UiPresentProjection,
    UiProjectionAvailability, UiProjectionBinding, UiProjectionBindingCompatibilityProof,
    UiProjectionBindingStopKind, UiProjectionBindingStopReceipt, UiProjectionConsumptionBudget,
    UiProjectionConsumptionBudgetError, UiProjectionConsumptionLimits, UiProjectionFactReceipt,
    UiProjectionFactStopKind, UiProjectionFactStopReceipt, UiProjectionFieldRequirement,
    UiProjectionFieldRequirementError, UiProjectionLifecycleRequirement, UiProjectionNativeFamily,
    UiProjectionRetainedActivityKind, UiProjectionRetainedActivityReceipt, UiProjectionShape,
    UiProjectionUnavailableKind, UiProjectionUnavailableReceipt, UiScalarProjectionBinding,
    UiScalarProjectionBindingAdmission, UiScalarProjectionFactReceipt, UiScalarSchemaRequirement,
    WorthUiInstalledQueryDomain, WorthUiInstalledQueryView, WorthUiInstalledSnapshotQueryView,
    WorthUiQueryBindingRegistrationDenial, WorthUiQueryBindingRegistrationDenialKind,
    WorthUiQueryViewDeclarationDenial, WorthUiQueryViewDefinition, WorthUiQueryViewIdentity,
    WorthUiQueryViewIdentityError, WorthUiQueryViewLifecycle, WorthUiQueryViewRegistration,
    WorthUiQueryViewShape,
};
