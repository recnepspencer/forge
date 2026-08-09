//! Public native-platform facade over the runtime-owned binding gate.

pub use worth_ui_runtime::native_platform::{
    UiNativeApplicationBuilder, UiNativeApplicationDefinition, UiNativeApplicationPreparation,
    UiNativeApplicationPreparationDenial, UiNativeApplicationPreparationDenialCause,
    UiNativeApplicationPreparationOutcome, UiNativePlatformCloseReceipt, UiNativePlatformOutcome,
    UiNativePlatformPreparationDenial, UiNativePlatformProfile, UiNativePlatformStopReason,
    UiNativePlatformStopReport, UiNativeWindowSpec, UiPreparedNativeApplication,
    UiPreparedNativePlatform, WorthUiNativePlatform,
};
