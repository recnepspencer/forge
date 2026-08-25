mod census;
mod orchestrator;
mod platform_ownership;
mod presentation_access;
mod presentation_retry;
#[cfg(feature = "certification-support")]
mod protocol_world;
mod recovery;
mod resource_registry;
mod shutdown;
mod surface_succession;

pub use census::UiNativeResourceCensus;
pub(crate) use census::UiNativeResourceClass;
pub(crate) use orchestrator::{
    UiNativeLifecycleDirective, UiNativeLifecycleOrchestrator, UiNativeSurfaceBasisTransition,
};
pub(crate) use platform_ownership::{close_platform_owners, register_platform_owners};
pub(crate) use presentation_access::UiNativePresentationAccess;
pub(crate) use presentation_retry::{
    UiNativePresentationRetryFinalization, UiNativePresentationRetryWake,
};
#[cfg(feature = "certification-support")]
pub use protocol_world::{
    UiNativeLifecycleProtocolReport, UiNativeLifecycleProtocolSchedule,
    UiNativeLifecycleProtocolWorld, UiNativeProtocolCloseDisposition, UiNativeProtocolClosePoint,
    UiNativeProtocolNextAction, UiNativeProtocolPredecessor, UiNativeProtocolReadback,
    UiNativeProtocolResourceCensus, UiNativeProtocolSurfaceTransition,
};
pub(crate) use recovery::{
    prepare_external_recovery, UiNativeRecoveryCause, UiNativeRecoveryLineage,
    UiNativeRecoveryRegistry, UiNativeRecoveryRequirement,
};
pub(crate) use resource_registry::{
    UiNativeOwnedResource, UiNativeResourceOwner, UiNativeResourceRegistry,
};
pub(crate) use shutdown::{progress_shutdown, UiNativeShutdownPhase};
pub(crate) use surface_succession::{
    collect_settled_device_generations, rebind_surface_scale,
    replace_retained_target_for_reconstruction, resize_surface,
};
