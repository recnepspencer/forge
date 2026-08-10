use super::{WorthUiApp, WorthUiHostNeutralApp};

/// Higher migration-only transition from frozen application meaning to the
/// one concrete revision-4 egui mechanics adapter.
pub struct WorthUiLegacyEguiApplicationTransition {
    _sealed: (),
}

impl WorthUiLegacyEguiApplicationTransition {
    pub fn activate(
        application: WorthUiHostNeutralApp,
        host: worth_ui_host_egui::WorthUiHostEgui,
    ) -> WorthUiApp {
        application.bind_exact_host(host)
    }
}
