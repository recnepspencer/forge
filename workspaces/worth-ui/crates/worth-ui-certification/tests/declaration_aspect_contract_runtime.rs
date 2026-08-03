use worth_ui::facade::app::{
    WorthUi, WorthUiApplicationPreparationDenial, WorthUiApplicationPreparationPhase,
};
use worth_ui::facade::declaration::{
    UiAspectContractAdmissionDenial, UiAspectSemanticSlice, UiDeclarationArtifact,
    UiDeclarationGraphHandoffDenial,
};
use worth_ui_certification::{
    WorthUiCertificationBuilderExt, WorthUiRustAuthoredDeclarationFixture,
};
use worth_ui_dsl::{
    UiDslAspectName, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken,
};

#[test]
fn public_freeze_exposes_typed_aspect_contract_and_coverage_report() {
    let app = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named("worth-ui.certification.aspect.coverage")
                .with_semantic_artifact_spec(aspectful_control_spec()),
        )
        .freeze()
        .expect("application preparation should succeed");
    let artifact = artifact_from_file_provenance(&app, "app/aspect_contracts.wui", 0);

    assert_eq!(
        artifact
            .aspect_contract()
            .expect("supported aspects should admit")
            .published()
            .aspects()[0]
            .semantic_slice(),
        UiAspectSemanticSlice::AppearanceBackground
    );
    assert_eq!(
        artifact
            .aspect_contract()
            .expect("supported aspects should admit")
            .published()
            .aspects()[1]
            .semantic_slice(),
        UiAspectSemanticSlice::ContentText
    );
    assert_eq!(
        artifact
            .aspect_contract()
            .expect("supported aspects should admit")
            .consumed()
            .aspects()[0]
            .semantic_slice(),
        UiAspectSemanticSlice::InteractionOperability
    );
    assert_eq!(
        artifact
            .aspect_coverage_report()
            .expect("supported aspects should admit")
            .published()[1]
            .semantic_slice(),
        UiAspectSemanticSlice::ContentText
    );
}

#[test]
fn equivalent_authored_aspect_spellings_converge_on_public_freeze_path() {
    let baseline = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.certification.aspect.equivalence.baseline",
            )
            .with_semantic_artifact_spec(aspect_equivalence_spec(
                "Content.Text",
                " Interaction.Operability ",
            )),
        )
        .freeze()
        .expect("application preparation should succeed");
    let equivalent = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.certification.aspect.equivalence.equivalent",
            )
            .with_semantic_artifact_spec(aspect_equivalence_spec(
                " content.text ",
                "interaction.operability",
            )),
        )
        .freeze()
        .expect("application preparation should succeed");
    let baseline_artifact =
        artifact_from_file_provenance(&baseline, "app/aspect_equivalence.wui", 0);
    let equivalent_artifact =
        artifact_from_file_provenance(&equivalent, "app/aspect_equivalence.wui", 0);

    assert_eq!(
        baseline_artifact.aspect_contract(),
        equivalent_artifact.aspect_contract()
    );
    assert_eq!(
        baseline_artifact.digest_projection().aspect(),
        equivalent_artifact.digest_projection().aspect()
    );
}

#[test]
fn renderer_labels_and_queryish_noise_do_not_satisfy_aspect_contract_authority() {
    let app = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named("worth-ui.certification.aspect.noise")
                .with_semantic_artifact_spec(noise_only_control_spec()),
        )
        .freeze()
        .expect("application preparation should succeed");
    let artifact = artifact_from_file_provenance(&app, "app/aspect_noise.wui", 0);

    assert!(
        artifact
            .aspect_contract()
            .expect("noise-only artifact should still admit an empty contract")
            .published()
            .aspects()
            .is_empty(),
        "renderer-local noise must not create published aspect authority"
    );
    assert!(
        artifact
            .aspect_contract()
            .expect("noise-only artifact should still admit an empty contract")
            .consumed()
            .aspects()
            .is_empty(),
        "query-ish noise labels must not create consumed aspect authority"
    );
    assert!(artifact
        .aspect_coverage_report()
        .expect("noise-only artifact should still admit an empty report")
        .published()
        .is_empty());
    assert!(artifact
        .aspect_coverage_report()
        .expect("noise-only artifact should still admit an empty report")
        .consumed()
        .is_empty());
}

#[test]
fn unsupported_authored_aspects_deny_through_public_freeze_path() {
    let denial = match WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.certification.aspect.unsupported",
            )
            .with_semantic_artifact_spec(unsupported_aspect_spec()),
        )
        .freeze()
    {
        Ok(_) => panic!("unsupported aspect slices must deny application preparation"),
        Err(denial) => denial,
    };

    assert_eq!(
        denial.phase(),
        WorthUiApplicationPreparationPhase::GraphHandoff
    );
    assert_eq!(
        denial,
        WorthUiApplicationPreparationDenial::GraphHandoff(
            UiDeclarationGraphHandoffDenial::AspectContractNotAdmitted {
                denial: UiAspectContractAdmissionDenial::UnsupportedAspectSemanticSlice {
                    family: worth_ui::facade::declaration::UiAspectFamily::Appearance,
                    canonical_label: "appearance.border".to_string(),
                },
            },
        )
    );
}

fn artifact_from_file_provenance<'a>(
    app: &'a worth_ui::facade::app::WorthUiApp,
    module_path: &str,
    declaration_index: usize,
) -> &'a UiDeclarationArtifact {
    app.declaration_artifacts()
        .iter()
        .find(|artifact| {
            let provenance = artifact.provenance().source_provenance();
            provenance.module_path() == module_path
                && provenance.declaration_index() == declaration_index
        })
        .unwrap_or_else(|| {
            panic!(
                "expected declaration artifact for {module_path}#{declaration_index} on freeze path"
            )
        })
}

fn aspectful_control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.save"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/aspect_contracts.wui", 0),
    )
    .with_published_aspect(UiDslAspectName::new("appearance.background"))
    .with_published_aspect(UiDslAspectName::new("Content.Text"))
    .with_consumed_aspect(UiDslAspectName::new("interaction.operability"))
    .with_structural_token(UiDslStructuralToken::new("control:save"))
}

fn aspect_equivalence_spec(
    published: &'static str,
    consumed: &'static str,
) -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.equivalence"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/aspect_equivalence.wui", 0),
    )
    .with_published_aspect(UiDslAspectName::new(published))
    .with_consumed_aspect(UiDslAspectName::new(consumed))
    .with_structural_token(UiDslStructuralToken::new("control:save"))
}

fn noise_only_control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.noise_only"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/aspect_noise.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:save"))
    .with_renderer_label("content.text")
    .with_diagnostic_label("query-binding:interaction.operability")
}

fn unsupported_aspect_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.unsupported"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/aspect_unsupported.wui", 0),
    )
    .with_published_aspect(UiDslAspectName::new("appearance.border"))
    .with_structural_token(UiDslStructuralToken::new("control:save"))
}
