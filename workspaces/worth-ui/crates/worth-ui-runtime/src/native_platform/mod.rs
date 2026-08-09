//! Native application preparation and the single enforceable runtime binding gate.

mod application;
mod application_driver;
mod native_platform_binding;
mod outcome;
mod platform;
mod profile;

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
