use super::{
    HeadlessHost, UiDslSemanticArtifactSpec, WorthUi, WorthUiApplicationPreparationDenial,
    WorthUiRustAuthoredDeclarationFixture,
};
use worth_ui_certification::WorthUiCertificationBuilderExt;

pub(super) fn freeze_denial(
    package_name: &'static str,
    spec: UiDslSemanticArtifactSpec,
) -> WorthUiApplicationPreparationDenial {
    match WorthUi::app()
        .bind_certification_host_adapter(
            worth_ui_host_contract::UiCertificationHostBindingGrant::for_certification(),
            HeadlessHost,
        )
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(package_name)
                .with_semantic_artifact_spec(spec),
        )
        .freeze()
    {
        Ok(_) => panic!("invalid declaration authority must deny application preparation"),
        Err(denial) => denial,
    }
}

pub(super) fn artifact_from_compiler_provenance<'a>(
    app: &'a worth_ui::facade::app::WorthUiApp,
    provenance: &super::UiDslSourceProvenance,
) -> &'a super::UiDeclarationArtifact {
    super::artifact_from_file_provenance(
        app,
        provenance.module_path(),
        provenance.declaration_index(),
    )
}
