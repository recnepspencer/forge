mod event_loop;
mod graphics;
mod host_state;
mod mechanics_adapter;
mod observation;
mod presentation;
mod readiness;
mod resource_census;
mod resource_ownership;
mod resource_registry;

pub use event_loop::{
    UiNativeClientPresentationAttribution, UiNativeEventLoopCleanup, UiNativeEventLoopClient,
    UiNativeEventLoopClientCleanup, UiNativeEventLoopClientClose, UiNativeEventLoopDirective,
    UiNativeEventLoopRunDenial, UiNativeEventLoopRunReport, UiNativeEventLoopStopReport,
    UiNativeReadinessGrant, WorthUiNativeEventLoop,
};
pub use host_state::UiNativeEffectPosture;
pub use mechanics_adapter::WorthUiNativeMechanicsAdapter;
pub use observation::{UiNativeGraphicsObservation, UiNativePresentationObservation};
pub use resource_census::UiNativeResourceCensus;

#[cfg(test)]
pub(crate) use graphics::QUALIFIED_DX12_PRESENTATION_SYSTEM;
pub(crate) use graphics::{
    UiNativeGraphics, UiNativeGraphicsPort, UiNativeOwnedGraphics, UiWgpuNativeGraphicsPort,
};
pub(crate) use host_state::UiNativeHostState;
pub(crate) use observation::UiNativePresentationInput;
pub(crate) use presentation::UiNativePendingPresentation;
#[cfg(test)]
pub(crate) use presentation::GPU_WAIT_DEADLINE;
pub(crate) use readiness::{UiNativeReadinessRegistry, UiNativeReadyOwner};
pub(crate) use resource_census::UiNativeResourceClass;
pub(crate) use resource_ownership::UiNativeOwnedResource;
pub(crate) use resource_registry::{UiNativeResourceOwner, UiNativeResourceRegistry};
