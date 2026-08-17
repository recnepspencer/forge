use worth_ui::facade::app::WorthUi;

fn main() {
    let application = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .freeze()
        .expect("host-neutral application should prepare");
    let _ = worth_ui_runtime::facade::entry::WorthUiLegacyEguiApplicationTransition::activate(
        application,
        worth_ui_host_headless::WorthUiHeadlessHost,
    );
}
