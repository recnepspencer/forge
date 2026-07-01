use worth_ui_dsl::{
    UiDslAspectName, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, WorthUiDslPackage,
};

fn main() {
    let spec = UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.save"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/workflow_editor.wui", 0),
    )
    .with_published_aspect(UiDslAspectName::new("content.text"));

    let _package = WorthUiDslPackage::named("worth-ui.certification.declaration")
        .with_admitted_semantic_artifact(spec);
}
