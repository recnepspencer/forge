mod event_loop;
mod graphics;
mod host_state;
mod mechanics_adapter;
mod observation;
mod physical_work_signal;
mod presentation;
mod readiness;
mod resource_census;
mod resource_ownership;
mod resource_registry;
mod text_atlas;

pub use event_loop::{
    UiNativeClientPresentationAttribution, UiNativeEventLoopCleanup, UiNativeEventLoopClient,
    UiNativeEventLoopClientCleanup, UiNativeEventLoopClientClose, UiNativeEventLoopDirective,
    UiNativeEventLoopRunDenial, UiNativeEventLoopRunReport, UiNativeEventLoopStopReport,
    UiNativePhysicalProgressGrant, UiNativeReadinessGrant, WorthUiNativeEventLoop,
};
#[cfg(test)]
pub(crate) use graphics::QUALIFIED_DX12_PRESENTATION_SYSTEM;
pub(crate) use graphics::{
    UiNativeGraphics, UiNativeGraphicsPort, UiNativeOwnedGraphics, UiWgpuNativeGraphicsPort,
};
pub use host_state::UiNativeEffectPosture;
pub(crate) use host_state::UiNativeHostState;
pub(crate) use mechanics_adapter::WorthUiNativeMechanicsAdapter;
pub(crate) use observation::UiNativePresentationInput;
pub use observation::{
    UiNativeGraphicsObservation, UiNativePresentationObservation, UiNativePresentationWorkKind,
    UiNativeRetainedFrameObservation,
};
#[cfg(test)]
pub(crate) use presentation::GPU_WAIT_DEADLINE;
pub(crate) use presentation::{UiNativePendingPresentation, UiNativeRetainedDrawList};
pub(crate) use readiness::{UiNativeReadinessRegistry, UiNativeReadyOwner};
pub use resource_census::UiNativeResourceCensus;
pub(crate) use resource_census::UiNativeResourceClass;
pub(crate) use resource_ownership::UiNativeOwnedResource;
pub(crate) use resource_registry::{UiNativeResourceOwner, UiNativeResourceRegistry};
pub use text_atlas::UiNativeTextPinObservation;
