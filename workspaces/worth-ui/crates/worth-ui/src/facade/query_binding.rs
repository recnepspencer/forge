//! Product-facing Query view declaration and registration.
//!
//! Query execution, settlement, native access, live leases, and patch
//! translation remain owned by `worth-ui-query-binding`. The product facade
//! exposes only the sealed capabilities needed to name and register UI intent.

pub use worth_ui_runtime::facade::query_binding::{
    WorthUiInstalledQueryDomain, WorthUiInstalledQueryView, WorthUiInstalledSnapshotQueryView,
    WorthUiQueryBindingRegistrationDenial, WorthUiQueryBindingRegistrationDenialKind,
    WorthUiQueryViewDeclarationDenial, WorthUiQueryViewDefinition, WorthUiQueryViewIdentity,
    WorthUiQueryViewIdentityError, WorthUiQueryViewLifecycle, WorthUiQueryViewRegistration,
    WorthUiQueryViewShape,
};
