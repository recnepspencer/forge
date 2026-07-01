use worth_ui_dsl::{
    UiDslSemanticArtifact, UiDslSemanticFamily, UiDslSemanticKey, UiDslSourceProvenance,
};

fn main() {
    let _ = UiDslSemanticArtifact::new(
        UiDslSemanticKey::new("forged.declaration"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/forged.wui", 0),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        None,
        None,
        None,
        None,
    );
}
