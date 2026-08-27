pub(crate) mod anchored_allocation;
mod dismissal;
mod identity;
mod lifecycle;
mod placement;
#[cfg(test)]
mod placement_tests;
mod planning;
mod proposal;
mod rebind;
mod receipt;
mod request;
mod state;
mod transition;

#[cfg(test)]
mod state_dismissal_tests;
#[cfg(test)]
mod state_tests;

pub(crate) use dismissal::{
    UiPortalDismissalIgnoreReason, UiPortalDismissalPreparation, UiPortalDismissalTrigger,
    UiPreparedPortalDismissal,
};
pub(crate) use identity::{UiPortalIdentity, UiPortalOwnerIdentity};
pub(crate) use lifecycle::{
    UiPortalDismissalCause, UiPortalInputShielding, UiPortalLifecyclePosture,
};
pub(crate) use placement::{
    UiCommittedPortalPlacement, UiPortalLayerIdentity, UiPortalPlacementDenial,
    UiPortalPlacementSide, UiPreparedPortalPlacement, UiPresentedPortalBounds,
};
pub(crate) use proposal::UiStagedPortalServiceProposal;
pub(crate) use receipt::{
    UiPortalExitRetentionReceipt, UiPortalServiceDisposition, UiPortalServiceReceipt,
};
pub(crate) use request::UiPortalServiceRequest;
pub(crate) use state::{UiPortalRuntimeState, UiPortalShutdownReport};
pub(crate) use transition::{
    UiPortalExitTerminalDenial, UiPortalServiceTransitionDenial, UiPreparedPortalServiceTransition,
};
