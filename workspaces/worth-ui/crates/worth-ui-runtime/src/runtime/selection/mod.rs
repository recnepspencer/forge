mod delta;
mod identity;
mod model;
mod proposal;
mod reducer;
mod request;
mod staged_transition;
mod state;

pub(crate) use delta::{UiSelectionDelta, UiSelectionReconciliationReceipt};
pub(crate) use identity::{
    UiSelectionOwnerIdentity, UiSelectionOwnerIncarnation, UiSelectionStableKey,
};
pub(crate) use model::UiDeclaredSelectionBinding;
pub(crate) use model::{UiSelectionCatalogPosture, UiSelectionPolicy, UiSelectionRegistration};
pub(in crate::runtime) use proposal::UiStagedSelectionServiceProposal;
pub(crate) use request::{UiSelectionRequest, UiSelectionRequestDenial};
pub(in crate::runtime) use staged_transition::{
    UiDeclaredSelectionStagingDenial, UiStagedDeclaredSelectionTransition,
};
pub(crate) use state::UiSelectionRuntimeState;

#[cfg(test)]
mod state_lifecycle_tests;
#[cfg(test)]
mod state_test_fixture;
#[cfg(test)]
mod state_tests;
