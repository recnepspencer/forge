use worth_ui::facade::app::WorthUi;
use worth_ui::facade::declaration::{UiDeclarationArtifact, UiDeclarationEquivalenceContract};
use worth_ui_certification::{
    WorthUiCertificationBuilderExt, WorthUiRustAuthoredDeclarationFixture,
};
use worth_ui_dsl::{
    UiDslAspectName, UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily,
    UiDslSemanticKey, UiDslSourceProvenance, UiDslStructuralToken, UiDslSupportToken,
};

fn declaration_input() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.save"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/workflow_editor.wui", 0),
    )
    .with_published_aspect(UiDslAspectName::new("content.text"))
    .with_consumed_aspect(UiDslAspectName::new("interaction.operability"))
    .with_structural_token(UiDslStructuralToken::new("control:save"))
    .with_posture_token(UiDslPostureToken::new("measurement:hug-height"))
    .with_support_token(UiDslSupportToken::new("support:admitted"))
}

fn noisy_declaration_input() -> UiDslSemanticArtifactSpec {
    declaration_input()
        .with_comment("formatted for readability")
        .with_comment("moved comment block")
        .with_formatting_profile("two-space-indent")
        .with_parser_local_id("parser-node-17")
        .with_diagnostic_label("save button readiness failed")
        .with_renderer_label("primary-action-button")
}

fn freeze_artifacts(package: WorthUiRustAuthoredDeclarationFixture) -> Vec<UiDeclarationArtifact> {
    WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(package)
        .freeze()
        .map(
            worth_ui_runtime::facade::entry::WorthUiCertificationApplicationTransition::activate_headless,
        )
        .expect("application preparation should succeed")
        .declaration_artifacts()
        .to_vec()
}

#[test]
fn dsl_declaration_receipts_exclude_runtime_owned_bootstrap_semantics() {
    let package =
        WorthUiRustAuthoredDeclarationFixture::named("worth-ui.certification.declaration")
            .with_semantic_artifact_spec(declaration_input());
    let caller_receipt = package.admitted_declarations()[0].clone();
    let receipts = package.admitted_declarations();
    let artifacts = freeze_artifacts(package);

    assert_eq!(receipts.len(), 1);
    assert_eq!(
        receipts[0].source_provenance().module_path(),
        caller_receipt.source_provenance().module_path()
    );
    assert_eq!(
        receipts[0].source_provenance(),
        caller_receipt.source_provenance()
    );
    assert_eq!(
        receipts[0].semantic_input_digest(),
        caller_receipt.semantic_input_digest()
    );
    assert_eq!(artifacts.len(), 2);
    assert_eq!(
        artifacts[0].provenance().source_provenance().module_path(),
        "worth-ui.runtime.bootstrap"
    );
}

#[test]
fn public_app_freeze_is_stable_under_non_semantic_caller_receipt_noise() {
    let baseline_package =
        WorthUiRustAuthoredDeclarationFixture::named("worth-ui.certification.declaration")
            .with_semantic_artifact_spec(declaration_input());
    let noisy_package =
        WorthUiRustAuthoredDeclarationFixture::named("worth-ui.certification.declaration")
            .with_semantic_artifact_spec(noisy_declaration_input());

    let baseline_receipt = baseline_package.admitted_declarations()[0].clone();
    let noisy_receipt = noisy_package.admitted_declarations()[0].clone();
    let baseline_artifacts = freeze_artifacts(baseline_package);
    let noisy_artifacts = freeze_artifacts(noisy_package);

    assert_eq!(baseline_artifacts, noisy_artifacts);
    assert_eq!(baseline_artifacts.len(), 2);
    assert_eq!(
        baseline_artifacts[1].provenance().semantic_input_digest(),
        baseline_receipt.semantic_input_digest()
    );
    assert_eq!(
        baseline_artifacts[1].provenance().semantic_input_digest(),
        noisy_receipt.semantic_input_digest()
    );
}

#[test]
fn public_app_freeze_returns_the_expected_bootstrap_declaration_contract() {
    let first_artifacts = freeze_artifacts(WorthUiRustAuthoredDeclarationFixture::empty());
    let second_artifacts = freeze_artifacts(WorthUiRustAuthoredDeclarationFixture::empty());
    let artifact = &first_artifacts[0];

    assert_eq!(first_artifacts, second_artifacts);
    assert_eq!(first_artifacts.len(), 1);
    assert_eq!(
        artifact.identity().equivalence_contract(),
        UiDeclarationEquivalenceContract::AuthoredSemanticMeaning
    );
    assert_eq!(
        artifact.provenance().source_provenance().module_path(),
        "worth-ui.runtime.bootstrap"
    );
    assert_eq!(
        artifact
            .provenance()
            .source_provenance()
            .declaration_index(),
        0
    );
    assert_ne!(artifact.provenance().semantic_input_digest(), 0);
    assert_eq!(
        artifact.digest_projection().family().raw(),
        stable_text_digest(UiDslSemanticFamily::Page.as_str())
    );
    assert_eq!(
        artifact.digest_projection().structural().raw(),
        digest_string_slice(["page:product-root"])
    );
    assert_eq!(
        artifact.digest_projection().posture().raw(),
        digest_string_slice(["world:authoritative"])
    );
    assert_eq!(
        artifact.digest_projection().support().raw(),
        digest_string_slice(["support:runtime-bootstrap"])
    );
    assert_ne!(artifact.digest_projection().structural().raw(), 0);
    assert_ne!(artifact.digest_projection().posture().raw(), 0);
    assert_ne!(artifact.digest_projection().support().raw(), 0);
}

fn digest_string_slice<const N: usize>(values: [&str; N]) -> u64 {
    let mut canonical = values.into_iter().collect::<Vec<_>>();
    canonical.sort();
    canonical.dedup();

    canonical
        .into_iter()
        .fold(0x9E37_79B9_7F4A_7C15, |digest, value| {
            digest.rotate_left(5) ^ stable_text_digest(value)
        })
}

fn stable_text_digest(text: &str) -> u64 {
    text.as_bytes()
        .iter()
        .fold(0xCBF2_9CE4_8422_2325, |digest, byte| {
            digest.wrapping_mul(0x0000_0100_0000_01B3) ^ u64::from(*byte)
        })
}
