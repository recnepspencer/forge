use worth_ui::facade::app::WorthUi;
use worth_ui_dsl::{
    WorthUiRustAuthoredArtifactInput, WorthUiRustAuthoredArtifactInputModule,
};

fn main() {
    let input = WorthUiRustAuthoredArtifactInput::from_modules([
        WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
            .with_component("workspace.component.dashboard"),
    ]);
    let _prepared = WorthUi::app()
        .bind_certification_host_adapter(worth_ui_host_contract::UiCertificationHostBindingGrant::for_certification(), worth_ui_host_headless::WorthUiHeadlessHost)
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_input(input)
        .freeze();
}
