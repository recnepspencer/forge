//! Launch lane: commit initial allocation, lower a canonical plan, and build the owning runtime loop.

mod build_active_state;
mod launch_request;
mod launch_transition;
mod lifecycle_state;
mod planning_transition;
pub(crate) use planning_transition::UiAllocationCatalogMintAuthority;
pub(crate) use planning_transition::UiAllocationCatalogPreparationDenial;
mod preservation;
pub(crate) use preservation::WorthUiLastValidRuntimeState;
pub(crate) mod runtime_instance;
mod runtime_instance_accessors;
#[cfg(test)]
mod runtime_instance_test_support;
mod seal_artifact;
mod staging_transition;
#[cfg(any(test, feature = "certification-support"))]
pub(crate) use staging_transition::WorthUiActivationStagingPlans;

pub use launch_request::{WorthUiRuntimeLaunch, WorthUiRuntimeLaunchDenial};
pub use lifecycle_state::{
    WorthUiPendingActivation, WorthUiRuntimeFrameEpoch, WorthUiRuntimeLifecycle,
    WorthUiRuntimeShutdownReceipt,
};
pub use preservation::WorthUiLastValidObservation;
pub use runtime_instance::WorthUiRuntime;
#[cfg(test)]
pub use runtime_instance::WorthUiRuntimeFrameworkLoop;
