mod delta;
mod identity;
mod inspection;
mod model;
mod proposal;
mod reducer;
mod request;
#[cfg(feature = "certification-support")]
mod scale_certification;
mod staged_transition;
mod state;

pub(crate) use delta::{UiSelectionDelta, UiSelectionReconciliationReceipt};
pub(crate) use identity::{
    UiSelectionOwnerIdentity, UiSelectionOwnerIncarnation, UiSelectionStableKey,
};
pub(crate) use inspection::{UiSelectionDropInspectionReason, UiSelectionDropInspectionRecord};
pub(crate) use model::UiDeclaredSelectionBinding;
pub(crate) use model::{UiSelectionCatalogPosture, UiSelectionPolicy, UiSelectionRegistration};
pub(in crate::runtime) use proposal::UiStagedSelectionServiceProposal;
pub(crate) use request::{UiSelectionRequest, UiSelectionRequestDenial};
#[cfg(feature = "certification-support")]
pub(crate) use scale_certification::selection_scale_evidence;
pub(in crate::runtime) use staged_transition::{
    UiDeclaredSelectionStagingDenial, UiStagedDeclaredSelectionTransition,
};
pub(crate) use state::UiSelectionRuntimeState;

#[cfg(test)]
mod state_inspection_tests;
#[cfg(test)]
mod state_lifecycle_tests;
#[cfg(test)]
mod state_test_fixture;
#[cfg(test)]
mod state_tests;
