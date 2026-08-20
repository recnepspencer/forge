mod derived_state_reconstruction;
mod event_loop;
mod graphics;
mod host_state;
#[cfg(test)]
mod host_state_lifecycle_tests;
mod mechanics_adapter;
mod observation;
mod physical_work_signal;
mod presentation;
mod readiness;
mod resource_census;
mod resource_ownership;
mod resource_registry;
mod text_atlas;

pub use derived_state_reconstruction::{
    UiNativeDerivedStateLossClass, UiNativeDerivedStateReconstructionObservation,
};
pub use event_loop::{
    UiNativeClientAuthoredMountedInstanceObservation, UiNativeClientConditionalOutcome,
    UiNativeClientDerivedStateLossClass, UiNativeClientDerivedStateReconstructionObservation,
    UiNativeClientPresentationAttribution, UiNativeClientPresentationMechanicIdentityObservation,
    UiNativeClientPresentationSemanticChange,
    UiNativeClientPresentationSemanticFrontierObservation,
    UiNativeClientPresentationSemanticSubscriberObservation,
    UiNativeClientPresentationTransitionKind, UiNativeClientPresentationTransitionObservation,
    UiNativeClientResourceObservation, UiNativeClientShutdownObservation,
    UiNativeClientTextPresentationWorkObservation, UiNativeEventLoopCleanup,
    UiNativeEventLoopClient, UiNativeEventLoopClientCleanup, UiNativeEventLoopClientClose,
    UiNativeEventLoopDirective, UiNativeEventLoopRunDenial, UiNativeEventLoopRunReport,
    UiNativeEventLoopStopReport, UiNativePhysicalPresentationCorrelation,
    UiNativePhysicalProgressClass, UiNativePhysicalProgressGrant, UiNativeReadinessGrant,
    WorthUiNativeEventLoop,
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
    UiNativeGlyphObservation, UiNativeGraphicsObservation, UiNativePresentationObservation,
    UiNativePresentationWorkKind, UiNativeRetainedFrameObservation,
};
pub use physical_work_signal::{
    UiNativePhysicalSignalExternalStatusClass, UiNativePhysicalSignalLifecycleObservation,
    UiNativePhysicalSignalObservationOriginClass, UiNativePhysicalSignalSettlementClass,
    UiNativePhysicalSignalTransitionObservation, UiNativePhysicalSignalWorkClass,
};
#[cfg(test)]
pub(crate) use presentation::GPU_WAIT_DEADLINE;
pub(crate) use presentation::{UiNativePendingPresentation, UiNativeRetainedDrawList};
pub(crate) use readiness::{UiNativeReadinessRegistry, UiNativeReadyOwner};
pub use resource_census::UiNativeResourceCensus;
pub(crate) use resource_census::UiNativeResourceClass;
pub(crate) use resource_ownership::UiNativeOwnedResource;
pub(crate) use resource_registry::{UiNativeResourceOwner, UiNativeResourceRegistry};
pub use text_atlas::UiNativeTextAtlasPlanObservation;
pub use text_atlas::UiNativeTextPinObservation;
