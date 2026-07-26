use worth_ui::facade::app::WorthUi;
use worth_ui_dsl::{
    WorthUiArtifactInput, WorthUiSealedSemanticPackage, WorthUiSemanticDeclaration,
};

fn forge_sealed_package() {
    let _package = WorthUiSealedSemanticPackage {};
}

fn prepare_from_loose_artifact_input(input: WorthUiArtifactInput) {
    let _app = WorthUi::app().with_artifact_input(input);
}

fn prepare_from_loose_declarations(declarations: Vec<WorthUiSemanticDeclaration>) {
    let _app = WorthUi::app().with_declarations(declarations);
}

fn main() {}
