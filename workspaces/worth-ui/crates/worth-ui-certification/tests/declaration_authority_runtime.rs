use worth_ui::facade::app::WorthUi;
use worth_ui::facade::declaration::{UiDeclarationArtifact, UiDeclarationEquivalenceContract};
use worth_ui_dsl::{
    UiDslAspectName, UiDslLoweringReceipt, UiDslPostureToken, UiDslSemanticArtifactSpec,
    UiDslSemanticFamily, UiDslSemanticKey, UiDslSourceProvenance, UiDslStructuralToken,
    UiDslSupportToken, WorthUiDslPackage,
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

fn freeze_artifacts(package: WorthUiDslPackage) -> Vec<UiDeclarationArtifact> {
    WorthUi::app()
        .with_dsl_package(package)
        .freeze()
        .declaration_artifacts()
        .to_vec()
}

fn bootstrap_receipts(package: &WorthUiDslPackage) -> Vec<UiDslLoweringReceipt> {
    package.runtime_lowering_receipts()
}

#[test]
fn public_runtime_lowering_receipts_include_caller_authored_semantics() {
    let package = WorthUiDslPackage::named("worth-ui.certification.declaration")
        .with_semantic_artifact_spec(declaration_input());
    let caller_receipt = package.admitted_declarations()[0].clone();
    let receipts = bootstrap_receipts(&package);

    assert_eq!(receipts.len(), 2);
    assert_eq!(
        receipts[0].source_provenance().module_path(),
        "worth-ui.runtime.bootstrap"
    );
    assert_eq!(receipts[0].source_provenance().declaration_index(), 0);
    assert_ne!(receipts[0].semantic_input_digest(), 0);
    assert_eq!(
        receipts[1].source_provenance(),
        caller_receipt.source_provenance()
    );
    assert_eq!(
        receipts[1].semantic_input_digest(),
        caller_receipt.semantic_input_digest()
    );
}

#[test]
fn public_app_freeze_is_stable_under_non_semantic_caller_receipt_noise() {
    let baseline_package = WorthUiDslPackage::named("worth-ui.certification.declaration")
        .with_semantic_artifact_spec(declaration_input());
    let noisy_package = WorthUiDslPackage::named("worth-ui.certification.declaration")
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
    let package = WorthUiDslPackage::empty();
    let bootstrap_receipt = bootstrap_receipts(&package).remove(0);

    let first_artifacts = freeze_artifacts(WorthUiDslPackage::empty());
    let second_artifacts = freeze_artifacts(WorthUiDslPackage::empty());
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
    assert_eq!(
        artifact.provenance().semantic_input_digest(),
        bootstrap_receipt.semantic_input_digest()
    );
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
