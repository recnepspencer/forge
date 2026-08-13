//! Public native-platform facade over the runtime-owned binding gate.

pub use worth_ui_runtime::native_platform::{
    UiNativeApplicationBuilder, UiNativeApplicationDefinition, UiNativeApplicationFrame,
    UiNativeApplicationPreparation, UiNativeApplicationPreparationDenial,
    UiNativeApplicationPreparationDenialCause, UiNativeApplicationPreparationOutcome,
    UiNativeApplicationProgram, UiNativeApplicationProgramDenial, UiNativeComponentPresenceChange,
    UiNativePlatformCloseReceipt, UiNativePlatformOutcome, UiNativePlatformPreparationDenial,
    UiNativePlatformProfile, UiNativePlatformStopReason, UiNativePlatformStopReport,
    UiNativeWindowSpec, UiPreparedNativeApplication, UiPreparedNativePlatform,
    WorthUiNativePlatform,
};
