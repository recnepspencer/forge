use worth_ui::facade::app::WorthUi;
use worth_ui_dsl::{
    WorthUiRustAuthoredArtifactInput, WorthUiRustAuthoredArtifactInputModule,
};

fn main() {
    let input = WorthUiRustAuthoredArtifactInput::from_modules([
        WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
            .with_component("workspace.component.dashboard"),
    ]);
    let _prepared = WorthUi::app().with_rust_authored_input(input).freeze();
}
