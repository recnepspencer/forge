use worth_ui::facade::app::WorthUi;

fn main() {
    let app = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .freeze()
        .map(
            worth_ui_runtime::facade::entry::WorthUiCertificationApplicationTransition::activate_headless,
        )
        .expect("fixed certification host should prepare");
    let session = app.launch().expect("fixed host should launch");
    let _ = session.shutdown();
}
