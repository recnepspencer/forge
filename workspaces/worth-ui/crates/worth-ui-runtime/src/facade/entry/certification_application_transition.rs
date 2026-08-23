use super::{WorthUiApp, WorthUiHostNeutralApp};

/// Higher certification-only transition from frozen application meaning to
/// one exact record-only host family.
pub struct WorthUiCertificationApplicationTransition {
    _sealed: (),
}

impl WorthUiCertificationApplicationTransition {
    pub(crate) fn activate_builder_host(application: WorthUiHostNeutralApp) -> WorthUiApp {
        application.bind_exact_host(crate::certification_support::UiCertificationBuilderHost)
    }

    #[cfg(test)]
    pub(crate) fn activate_test_host<Host>(
        application: WorthUiHostNeutralApp,
        host: Host,
    ) -> WorthUiApp
    where
        Host: crate::facade::host::WorthUiHostAdapter + 'static,
    {
        application.bind_exact_host(host)
    }

    pub fn activate_headless(application: WorthUiHostNeutralApp) -> WorthUiApp {
        application.bind_exact_host(worth_ui_host_headless::WorthUiHeadlessHost)
    }

    pub fn activate_recorder(
        application: WorthUiHostNeutralApp,
        recorder: worth_ui_host_headless::WorthUiHeadlessRecorder,
    ) -> WorthUiApp {
        application.bind_exact_host(recorder)
    }

    pub fn activate_capability_profile(
        application: WorthUiHostNeutralApp,
        host: worth_ui_host_headless::WorthUiHeadlessCapabilityProfileHost,
    ) -> WorthUiApp {
        application.bind_exact_host(host)
    }

    pub fn activate_portal_anchor(
        application: WorthUiHostNeutralApp,
        host: worth_ui_host_headless::WorthUiHeadlessPortalAnchorHost,
    ) -> WorthUiApp {
        application.bind_exact_host(host)
    }

    pub fn activate_baseline_unavailable(
        application: WorthUiHostNeutralApp,
        host: worth_ui_host_headless::WorthUiHeadlessBaselineUnavailableHost,
    ) -> WorthUiApp {
        application.bind_exact_host(host)
    }

    pub fn activate_scripted_presentation(
        application: WorthUiHostNeutralApp,
        host: crate::certification_support::ScriptedPresentationHost,
    ) -> WorthUiApp {
        application.bind_exact_host(host)
    }
}
