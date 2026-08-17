use worth_ui::facade::app::{WorthUiApp, WorthUiHostNeutralApp};
use worth_ui_host_headless::{
    WorthUiHeadlessBaselineUnavailableHost, WorthUiHeadlessCapabilityProfileHost,
    WorthUiHeadlessHost, WorthUiHeadlessPortalAnchorHost, WorthUiHeadlessRecorder,
};
use worth_ui_runtime::facade::entry::{
    WorthUiCertificationApplicationTransition, WorthUiLegacyEguiApplicationTransition,
};

pub(crate) mod sealed {
    pub trait Sealed {}
}

/// Fixed certification host families admitted by the governed scenario.
///
/// Activation consumes already-frozen host-neutral application meaning. The
/// product builder never carries a host selection or session plan.
pub trait FixedCertificationHostBinding: sealed::Sealed + 'static {
    fn activate(self, application: WorthUiHostNeutralApp) -> WorthUiApp;
}

impl sealed::Sealed for WorthUiHeadlessHost {}

impl FixedCertificationHostBinding for WorthUiHeadlessHost {
    fn activate(self, application: WorthUiHostNeutralApp) -> WorthUiApp {
        WorthUiCertificationApplicationTransition::activate_headless(application)
    }
}

impl sealed::Sealed for WorthUiHeadlessRecorder {}

impl FixedCertificationHostBinding for WorthUiHeadlessRecorder {
    fn activate(self, application: WorthUiHostNeutralApp) -> WorthUiApp {
        WorthUiCertificationApplicationTransition::activate_recorder(application, self)
    }
}

impl sealed::Sealed for WorthUiHeadlessCapabilityProfileHost {}

impl FixedCertificationHostBinding for WorthUiHeadlessCapabilityProfileHost {
    fn activate(self, application: WorthUiHostNeutralApp) -> WorthUiApp {
        WorthUiCertificationApplicationTransition::activate_capability_profile(application, self)
    }
}

impl sealed::Sealed for WorthUiHeadlessPortalAnchorHost {}

impl FixedCertificationHostBinding for WorthUiHeadlessPortalAnchorHost {
    fn activate(self, application: WorthUiHostNeutralApp) -> WorthUiApp {
        WorthUiCertificationApplicationTransition::activate_portal_anchor(application, self)
    }
}

impl sealed::Sealed for WorthUiHeadlessBaselineUnavailableHost {}

impl FixedCertificationHostBinding for WorthUiHeadlessBaselineUnavailableHost {
    fn activate(self, application: WorthUiHostNeutralApp) -> WorthUiApp {
        WorthUiCertificationApplicationTransition::activate_baseline_unavailable(application, self)
    }
}

impl sealed::Sealed for worth_ui_runtime::certification_support::ScriptedPresentationHost {}

impl FixedCertificationHostBinding
    for worth_ui_runtime::certification_support::ScriptedPresentationHost
{
    fn activate(self, application: WorthUiHostNeutralApp) -> WorthUiApp {
        WorthUiCertificationApplicationTransition::activate_scripted_presentation(application, self)
    }
}

impl sealed::Sealed for worth_ui_host_egui::WorthUiHostEgui {}

impl FixedCertificationHostBinding for worth_ui_host_egui::WorthUiHostEgui {
    fn activate(self, application: WorthUiHostNeutralApp) -> WorthUiApp {
        WorthUiLegacyEguiApplicationTransition::activate(application, self)
    }
}
