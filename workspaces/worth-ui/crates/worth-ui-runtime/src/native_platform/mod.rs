//! Native application preparation and the single enforceable runtime binding gate.

mod application;
mod application_driver;
pub(crate) mod authorized_native_host;
mod native_platform_binding;
mod outcome;
mod platform;
mod profile;
pub(crate) mod text_presentation;

pub use crate::facade::entry::{
    UiNativeApplicationFrame, UiNativeApplicationProgram, UiNativeApplicationProgramDenial,
    UiNativeComponentPresenceChange, UiNativeComponentSemanticTextChange,
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
