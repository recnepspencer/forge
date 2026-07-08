//! Launch lane — seal artifact, derive plan, build active state, host shell.

pub(crate) mod host;
mod launch_request;
mod lifecycle_state;
mod preservation;
mod seal_artifact;
mod derive_plan;
mod build_active_state;
mod launch_transition;
mod staging_transition;
mod planning_transition;
mod host_accessors;
#[cfg(test)]
mod host_test_support;

pub use host::WorthUiRuntimeHost;
pub use launch_request::{WorthUiRuntimeLaunch, WorthUiRuntimeLaunchDenial};
pub use lifecycle_state::{
    WorthUiPendingActivation, WorthUiRuntimeFrameEpoch, WorthUiRuntimeLifecycle,
    WorthUiRuntimeShutdownReceipt,
};
pub use preservation::WorthUiLastValidObservation;