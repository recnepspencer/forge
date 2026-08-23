mod derived_state_reconstruction;
mod event_loop;
mod graphics;
mod host_state;
#[cfg(test)]
mod host_state_lifecycle_tests;
mod input;
mod lifecycle_protocol;
mod mechanics_adapter;
mod observation;
mod physical_work_signal;
mod platform;
mod presentation;
mod readiness;
#[cfg(feature = "certification-support")]
mod readiness_certification;
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
    UiNativeClientObservationIngressObservation, UiNativeClientPresentationAttribution,
    UiNativeClientPresentationMechanicIdentityObservation,
    UiNativeClientPresentationSemanticChange,
    UiNativeClientPresentationSemanticFrontierObservation,
    UiNativeClientPresentationSemanticSubscriberObservation,
    UiNativeClientPresentationTransitionKind, UiNativeClientPresentationTransitionObservation,
    UiNativeClientResourceObservation, UiNativeClientShutdownObservation,
    UiNativeClientTextPresentationWorkObservation, UiNativeEventLoopCleanup,
    UiNativeEventLoopClient, UiNativeEventLoopClientCleanup, UiNativeEventLoopClientClose,
    UiNativeEventLoopDirective, UiNativeEventLoopRunDenial, UiNativeEventLoopRunReport,
    UiNativeEventLoopStopReport, UiNativeEventLoopThreadPosture, UiNativeObservationReadinessGrant,
    UiNativePhysicalPresentationCorrelation, UiNativePhysicalProgressClass,
    UiNativePhysicalProgressGrant, UiNativeReadinessGrant, WorthUiNativeEventLoop,
};
#[cfg(test)]
pub(crate) use graphics::QUALIFIED_DX12_PRESENTATION_SYSTEM;
pub(crate) use graphics::{
    UiNativeGraphics, UiNativeGraphicsPort, UiNativeOwnedGraphics, UiWgpuNativeGraphicsPort,
};
pub use host_state::UiNativeEffectPosture;
pub(crate) use host_state::UiNativeHostState;
#[cfg(feature = "certification-support")]
pub use input::{UiNativeInputObservationContract, UiNativeInputObservationContractDisposition};
pub(crate) use input::{
    UiNativeInputObservationDisposition, UiNativeInputObservationState,
    UiNativePointerPositionWitness,
};
pub use input::{
    UiNativeInputObservationEventFamily, UiNativeInputObservationReport,
    UiNativeInputObservationStop, UiNativePointerButtonObservation,
};
pub use lifecycle_protocol::{
    UiNativeLifecycleEffect, UiNativeLifecyclePhase, UiNativeLifecycleProtocol,
    UiNativeLifecycleRequiredAction, UiNativeLifecycleTransition,
};
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
pub(crate) use platform::UiNativePointerInputPort;
#[cfg(test)]
pub(crate) use presentation::GPU_WAIT_DEADLINE;
pub(crate) use presentation::{UiNativePendingPresentation, UiNativeRetainedDrawList};
pub(crate) use readiness::{UiNativeReadinessRegistry, UiNativeReadyOwner, UiNativeReadyWork};
#[cfg(feature = "certification-support")]
pub use readiness_certification::{
    UiNativeReadinessContract, UiNativeReadinessContractOutcome, UiNativeReadinessContractWork,
};
pub use resource_census::UiNativeResourceCensus;
pub(crate) use resource_census::UiNativeResourceClass;
pub(crate) use resource_ownership::UiNativeOwnedResource;
pub(crate) use resource_registry::{UiNativeResourceOwner, UiNativeResourceRegistry};
pub use text_atlas::UiNativeTextAtlasPlanObservation;
pub use text_atlas::UiNativeTextPinObservation;
