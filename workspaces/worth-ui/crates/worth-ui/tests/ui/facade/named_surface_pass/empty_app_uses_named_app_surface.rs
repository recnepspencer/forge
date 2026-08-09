use worth_ui::facade::app::{WorthUi, WorthUiApp, WorthUiApplicationBuilder};

fn build_app() -> WorthUiApp {
    WorthUi::app()
        .bind_certification_host_adapter(worth_ui_host_contract::UiCertificationHostBindingGrant::for_certification(), worth_ui_host_headless::WorthUiHeadlessHost)
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .freeze()
        .expect("application preparation should succeed")
}

fn accepts_builder(_builder: WorthUiApplicationBuilder) {}

fn main() {
    let app = build_app();
    let builder = WorthUi::app().with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse());

    accepts_builder(builder);

    let _ = app;
}
