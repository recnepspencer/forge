use super::*;

pub(super) fn declared_surface_app() -> WorthUiApp {
    WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.certification.inspection.relevance.declared",
            )
            .with_semantic_artifact_spec(declared_surface_region_spec()),
        )
        .freeze()
        .map(
            worth_ui_runtime::facade::entry::WorthUiCertificationApplicationTransition::activate_headless,
        )
        .expect("application preparation should succeed")
}

pub(super) fn empty_app() -> WorthUiApp {
    WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .freeze()
        .map(
            worth_ui_runtime::facade::entry::WorthUiCertificationApplicationTransition::activate_headless,
        )
        .expect("application preparation should succeed")
}

pub(super) fn declared_surface_region_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.region.root"),
        UiDslSemanticFamily::Region,
        UiDslSourceProvenance::file_authored("app/inspection_relevance.wui", 1),
    )
    .with_structural_token(UiDslStructuralToken::new("region:sidebar"))
}
