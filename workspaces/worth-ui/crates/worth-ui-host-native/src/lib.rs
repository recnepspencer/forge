//! Qualified native mechanics profiles and the Worth-owned native host.

mod native;
mod native_profile;
mod prepared_host;
mod text_profile;

pub use native::{
    UiNativeClientPresentationAttribution, UiNativeEffectPosture, UiNativeEventLoopCleanup,
    UiNativeEventLoopClient, UiNativeEventLoopClientCleanup, UiNativeEventLoopClientClose,
    UiNativeEventLoopDirective, UiNativeEventLoopRunDenial, UiNativeEventLoopRunReport,
    UiNativeEventLoopStopReport, UiNativeGraphicsObservation, UiNativePhysicalProgressGrant,
    UiNativePresentationObservation, UiNativePresentationWorkKind, UiNativeReadinessGrant,
    UiNativeResourceCensus, UiNativeRetainedFrameObservation, UiNativeTextPinObservation,
    WorthUiNativeEventLoop,
};
pub use native_profile::{
    UiNativeMechanicsCapacities, UiNativePlatformProfileIdentity, WORTH_UI_NATIVE_PROFILE_MANIFEST,
};
pub use prepared_host::{
    UiNativeWindowConfiguration, WorthUiPreparedNativeHost, WorthUiPreparedNativeMechanics,
};
pub use text_profile::{
    UiBodyDefaultAtlasCapacities, UiBodyDefaultTextProfileIdentity,
    UiUnsupportedBodyDefaultCodePoint, WORTH_UI_BODY_DEFAULT_FONT, WORTH_UI_BODY_DEFAULT_LICENSE,
    WORTH_UI_TEXT_PROFILE_MANIFEST,
};

#[cfg(test)]
mod qualification_tests;
