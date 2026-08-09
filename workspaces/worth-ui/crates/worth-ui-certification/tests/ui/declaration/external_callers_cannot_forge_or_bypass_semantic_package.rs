use worth_ui::facade::app::WorthUi;
use worth_ui_dsl::{
    WorthUiArtifactInput, WorthUiSealedSemanticPackage, WorthUiSemanticDeclaration,
};

fn forge_sealed_package() {
    let _package = WorthUiSealedSemanticPackage {};
}

fn prepare_from_loose_artifact_input(input: WorthUiArtifactInput) {
    let _app = WorthUi::app().bind_certification_host_adapter(worth_ui_host_contract::UiCertificationHostBindingGrant::for_certification(), worth_ui_host_headless::WorthUiHeadlessHost).with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse()).with_artifact_input(input);
}

fn prepare_from_loose_declarations(declarations: Vec<WorthUiSemanticDeclaration>) {
    let _app = WorthUi::app().bind_certification_host_adapter(worth_ui_host_contract::UiCertificationHostBindingGrant::for_certification(), worth_ui_host_headless::WorthUiHeadlessHost).with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse()).with_declarations(declarations);
}

fn main() {}
