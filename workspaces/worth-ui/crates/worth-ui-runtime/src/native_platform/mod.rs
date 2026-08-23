//! Native application preparation and the single enforceable runtime binding gate.

mod application;
mod application_driver;
pub(crate) mod authorized_native_host;
mod native_platform_binding;
mod outcome;
mod platform;
mod profile;
mod runtime_qualification;
pub(crate) mod text_presentation;

pub use crate::facade::entry::{
    UiNativeApplicationFrame, UiNativeApplicationProgram, UiNativeApplicationProgramDenial,
    UiNativeComponentPresenceChange, UiNativeComponentSemanticTextChange,
    UiNativeThemeTokenValueChange,
};
pub use application::{
    UiNativeApplicationBuilder, UiNativeApplicationDefinition, UiNativeApplicationPreparation,
    UiNativeApplicationPreparationDenial, UiNativeApplicationPreparationDenialCause,
    UiNativeApplicationPreparationOutcome, UiPreparedNativeApplication,
};
pub use outcome::{
    UiNativePlatformCloseReceipt, UiNativePlatformOutcome, UiNativePlatformStopReason,
    UiNativePlatformStopReport,
};
pub use platform::{UiPreparedNativePlatform, WorthUiNativePlatform};
pub use profile::{UiNativePlatformPreparationDenial, UiNativePlatformProfile, UiNativeWindowSpec};
#[cfg(feature = "certification-support")]
pub use runtime_qualification::{
    UiNativeRuntimeDerivedStateLossClass, UiNativeRuntimeQualificationPlan,
    UiNativeRuntimeQualificationPlanDenial,
};
#[cfg(feature = "certification-support")]
pub use worth_ui_host_native::{
    UiNativeClientAuthoredMountedInstanceObservation, UiNativeClientConditionalOutcome,
    UiNativeClientDerivedStateLossClass, UiNativeClientDerivedStateReconstructionObservation,
    UiNativeClientPresentationSemanticChange,
    UiNativeClientPresentationSemanticFrontierObservation,
    UiNativeClientPresentationSemanticSubscriberObservation,
    UiNativeClientPresentationTransitionKind, UiNativeClientPresentationTransitionObservation,
    UiNativeClientShutdownObservation, UiNativeClientTextPresentationWorkObservation,
    UiNativeDerivedStateLossClass, UiNativeDerivedStateReconstructionObservation,
    UiNativePhysicalSignalExternalStatusClass, UiNativePhysicalSignalObservationOriginClass,
    UiNativePhysicalSignalSettlementClass, UiNativePhysicalSignalTransitionObservation,
    UiNativePhysicalSignalWorkClass, UiNativePresentationObservation, UiNativePresentationWorkKind,
    UiNativeQualificationPlan, UiNativeQualificationPlanDenial, UiNativeRetainedFrameObservation,
};
