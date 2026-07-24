mod consumption_view;
mod coordinator;
mod effect_requirements;
mod outcome;
mod preflight;
mod reconciliation;
mod shutdown;
mod state;
mod terminal;

pub(crate) use consumption_view::UiMountedHostPresentationAuthority;
pub(crate) use coordinator::UiMountedPresentationCoordinator;
pub use outcome::{
    UiMountedIndeterminateFrame, UiMountedPresentationOutcome, UiMountedPresentationReceipt,
    UiMountedPresentationWitness, UiMountedPresentedFrame, UiMountedRejectedFrame,
    UiMountedSurfacePresentationReceipt, UiMountedSurfacePresentationRejection,
    UiPresentationIndeterminateReport,
};
pub use reconciliation::{UiHostPresentationReconciliation, UiMountedSurfaceReconciliationBinding};
pub use shutdown::{
    UiMountedPresentationShutdownAttempt, UiMountedPresentationShutdownDisposition,
    UiMountedPresentationShutdownReport,
};
pub use state::{
    UiMountedPresentationAdmission, UiMountedPresentationAdmissionDenial,
    UiMountedPresentationAdmissionRejection, UiMountedPresentationAttempt,
    UiMountedPresentationCompletionDenial, UiMountedPresentationInFlight,
};
