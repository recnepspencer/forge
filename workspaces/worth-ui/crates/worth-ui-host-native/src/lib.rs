//! Qualified native mechanics profiles and the Worth-owned native host.

mod native;
mod native_profile;
mod prepared_host;
#[cfg(feature = "certification-support")]
mod qualification;
mod text_profile;

pub use native::{
    UiNativeClientAuthoredMountedInstanceObservation, UiNativeClientConditionalOutcome,
    UiNativeClientDerivedStateLossClass, UiNativeClientDerivedStateReconstructionObservation,
    UiNativeClientPresentationAttribution, UiNativeClientPresentationMechanicIdentityObservation,
    UiNativeClientPresentationSemanticChange,
    UiNativeClientPresentationSemanticFrontierObservation,
    UiNativeClientPresentationSemanticSubscriberObservation,
    UiNativeClientPresentationTransitionKind, UiNativeClientPresentationTransitionObservation,
    UiNativeClientResourceObservation, UiNativeClientShutdownObservation,
    UiNativeClientTextPresentationWorkObservation, UiNativeDerivedStateLossClass,
    UiNativeDerivedStateReconstructionObservation, UiNativeEffectPosture, UiNativeEventLoopCleanup,
    UiNativeEventLoopClient, UiNativeEventLoopClientCleanup, UiNativeEventLoopClientClose,
    UiNativeEventLoopDirective, UiNativeEventLoopRunDenial, UiNativeEventLoopRunReport,
    UiNativeEventLoopStopReport, UiNativeGlyphObservation, UiNativeGraphicsObservation,
    UiNativePhysicalPresentationCorrelation, UiNativePhysicalProgressClass,
    UiNativePhysicalProgressGrant, UiNativePhysicalSignalExternalStatusClass,
    UiNativePhysicalSignalLifecycleObservation, UiNativePhysicalSignalObservationOriginClass,
    UiNativePhysicalSignalSettlementClass, UiNativePhysicalSignalTransitionObservation,
    UiNativePhysicalSignalWorkClass, UiNativePresentationObservation, UiNativePresentationWorkKind,
    UiNativeReadinessGrant, UiNativeResourceCensus, UiNativeRetainedFrameObservation,
    UiNativeTextAtlasPlanObservation, UiNativeTextPinObservation, WorthUiNativeEventLoop,
};
pub use native_profile::{
    UiNativeMechanicsCapacities, UiNativePlatformProfileIdentity, WORTH_UI_NATIVE_PROFILE_MANIFEST,
};
pub use prepared_host::{
    UiNativeWindowConfiguration, WorthUiPreparedNativeHost, WorthUiPreparedNativeMechanics,
};
#[cfg(feature = "certification-support")]
pub use qualification::{UiNativeQualificationPlan, UiNativeQualificationPlanDenial};
pub use text_profile::{
    UiBodyDefaultAtlasCapacities, UiBodyDefaultTextProfileIdentity,
    UiUnsupportedBodyDefaultCodePoint, WORTH_UI_BODY_DEFAULT_FONT, WORTH_UI_BODY_DEFAULT_LICENSE,
    WORTH_UI_TEXT_PROFILE_MANIFEST,
};

#[cfg(test)]
mod qualification_tests;
