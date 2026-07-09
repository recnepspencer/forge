//! Launch lane — seal artifact, derive plan, build active state, host shell.

mod build_active_state;
mod derive_plan;
pub(crate) mod host;
mod host_accessors;
#[cfg(test)]
mod host_test_support;
mod launch_request;
mod launch_transition;
mod lifecycle_state;
mod planning_transition;
mod preservation;
mod seal_artifact;
mod staging_transition;

pub use host::WorthUiRuntimeHost;
pub use launch_request::{WorthUiRuntimeLaunch, WorthUiRuntimeLaunchDenial};
pub use lifecycle_state::{
    WorthUiPendingActivation, WorthUiRuntimeFrameEpoch, WorthUiRuntimeLifecycle,
    WorthUiRuntimeShutdownReceipt,
};
pub use preservation::WorthUiLastValidObservation;
