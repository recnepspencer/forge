mod state;
#[allow(
    dead_code,
    reason = "Gate 0 freezes theme switching without making it live"
)]
mod theme;

#[cfg(test)]
mod host_completion_tests;

pub use state::UiAppearanceOwnerSnapshot;
pub(crate) use state::UiAppearanceStateAxisDemand;
pub(crate) use theme::{
    UiAppearanceThemeState, UiPreparedThemeSwitch, UiThemeCapabilityReceipt,
    UiThemeInitialBindingDenial, UiThemeSwitchDenial, UiThemeSwitchOrigin,
    UiThemeSwitchOriginAdmissionDenial, UiThemeSwitchOriginFamily, UiThemeSwitchRequest,
};
