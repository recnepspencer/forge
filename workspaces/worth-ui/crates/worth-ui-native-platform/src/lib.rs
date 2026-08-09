//! Effect-free native application preparation and lifecycle authority.

mod application;
mod native_platform_binding;
mod outcome;
mod platform;
mod profile;

pub use application::{
    UiNativeApplicationBuilder, UiNativeApplicationDefinition, UiNativeApplicationPreparation,
    UiNativeApplicationPreparationDenial, UiNativeApplicationPreparationDenialCause,
    UiNativeApplicationPreparationOutcome, UiPreparedNativeApplication,
};
pub use outcome::{UiNativePlatformOutcome, UiNativePlatformStop, UiNativePlatformStopReason};
pub use platform::{UiPreparedNativePlatform, WorthUiNativePlatform};
pub use profile::{UiNativePlatformPreparationDenial, UiNativePlatformProfile, UiNativeWindowSpec};
