use worth_ui::facade::app::WorthUi;
use worth_ui::facade::declaration::{
    UiDeclarationArtifact, UiDeclaredMeasurementPolicyPosture, UiDeclaredPostureAdmissionDenial,
    UiDeclaredPostureApplicability, UiDeclaredPostureLaneKind, UiDeclaredQueryBindingPosture,
    UiDeclaredServiceUsagePosture, UiDeclaredTouchMeaningPosture,
};
use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken, WorthUiDslPackage,
};
use worth_ui_host_contract::WorthUiHostCapability;

fn assert_applicability_vector(
    artifact: &UiDeclarationArtifact,
    expected: [UiDeclaredPostureApplicability; 5],
) {
    let posture = artifact
        .declared_posture()
        .expect("declaration posture should admit on freeze path");

    assert_eq!(posture.query_binding().applicability(), expected[0]);
    assert_eq!(posture.service_usage().applicability(), expected[1]);
    assert_eq!(posture.touch_meaning().applicability(), expected[2]);
    assert_eq!(posture.measurement_policy().applicability(), expected[3]);
    assert_eq!(posture.host_capability().applicability(), expected[4]);
}

#[test]
fn public_freeze_projects_declared_posture_contracts_from_declaration_authority() {
    let app = WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.declared-posture")
                .with_semantic_artifact_spec(control_posture_spec()),
        )
        .freeze();
    let artifact = artifact_from_file_provenance(&app, "app/declared_posture.wui", 0);
    let posture = artifact
        .declared_posture()
        .expect("control posture should admit on freeze path");

    assert_eq!(
        posture.query_binding().admitted(),
        Some(&UiDeclaredQueryBindingPosture::AttachedViewBinding)
    );
    assert_eq!(
        posture.service_usage().admitted(),
        Some(&UiDeclaredServiceUsagePosture::Portal)
    );
    assert_eq!(
        posture.touch_meaning().admitted(),
        Some(&UiDeclaredTouchMeaningPosture::Press)
    );
    assert_eq!(
        posture.measurement_policy().admitted(),
        Some(&UiDeclaredMeasurementPolicyPosture::HugHeight)
    );
    assert_eq!(
        posture.host_capability().admitted().map(|posture| posture.required_capabilities()),
        Some(&[WorthUiHostCapability::TextInput][..])
    );
}

#[test]
fn public_freeze_preserves_representative_family_applicability_shapes() {
    let app = WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.declared-posture.classification")
                .with_semantic_artifact_spec(page_spec())
                .with_semantic_artifact_spec(classification_control_spec())
                .with_semantic_artifact_spec(query_binding_spec())
                .with_semantic_artifact_spec(intent_spec()),
        )
        .freeze();
    let page = artifact_from_file_provenance(&app, "app/declared_posture_classification.wui", 0);
    let control =
        artifact_from_file_provenance(&app, "app/declared_posture_classification.wui", 1);
    let query = artifact_from_file_provenance(&app, "app/declared_posture_classification.wui", 2);
    let intent = artifact_from_file_provenance(&app, "app/declared_posture_classification.wui", 3);

    assert_applicability_vector(
        page,
        [
            UiDeclaredPostureApplicability::Optional,
            UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted,
            UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted,
            UiDeclaredPostureApplicability::Optional,
            UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted,
        ],
    );
    assert_applicability_vector(
        control,
        [
            UiDeclaredPostureApplicability::Optional,
            UiDeclaredPostureApplicability::Optional,
            UiDeclaredPostureApplicability::Optional,
            UiDeclaredPostureApplicability::Optional,
            UiDeclaredPostureApplicability::Optional,
        ],
    );
    assert_applicability_vector(
        query,
        [
            UiDeclaredPostureApplicability::Required,
            UiDeclaredPostureApplicability::NotApplicable,
            UiDeclaredPostureApplicability::NotApplicable,
            UiDeclaredPostureApplicability::NotApplicable,
            UiDeclaredPostureApplicability::NotApplicable,
        ],
    );
    assert_applicability_vector(
        intent,
        [
            UiDeclaredPostureApplicability::NotApplicable,
            UiDeclaredPostureApplicability::NotApplicable,
            UiDeclaredPostureApplicability::NotApplicable,
            UiDeclaredPostureApplicability::NotApplicable,
            UiDeclaredPostureApplicability::NotApplicable,
        ],
    );
}

#[test]
fn invalid_declared_posture_denies_before_runtime_or_host_promotion() {
    let app = WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.declared-posture.denials")
                .with_semantic_artifact_spec(
                    control_posture_spec()
                        .with_posture_token(UiDslPostureToken::new("service:scroll")),
                ),
        )
        .freeze();
    let artifact = artifact_from_file_provenance(&app, "app/declared_posture.wui", 0);
    let expected_denial = UiDeclaredPostureAdmissionDenial::ContradictoryLaneClaims {
        family: worth_ui::facade::declaration::UiDeclarationFamilyKind::Control,
        lane: UiDeclaredPostureLaneKind::ServiceUsage,
        observed: vec!["service:portal".to_owned(), "service:scroll".to_owned()],
    };

    assert_eq!(
        artifact.declared_posture(),
        Err(&expected_denial),
    );
}

#[test]
fn host_capability_requirements_appear_as_declared_posture_before_host_inference() {
    let app = WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.declared-posture.host")
                .with_semantic_artifact_spec(
                    UiDslSemanticArtifactSpec::new(
                        UiDslSemanticKey::new("workflow_editor.inspector.name"),
                        UiDslSemanticFamily::Control,
                        UiDslSourceProvenance::file_authored("app/declared_posture_host.wui", 0),
                    )
                    .with_structural_token(UiDslStructuralToken::new("control:name"))
                    .with_posture_token(UiDslPostureToken::new("touch:text-entry"))
                    .with_posture_token(UiDslPostureToken::new("host-capability:text-input"))
                    .with_posture_token(UiDslPostureToken::new("host-capability:ime")),
                ),
        )
        .freeze();
    let artifact = artifact_from_file_provenance(&app, "app/declared_posture_host.wui", 0);

    assert_eq!(
        artifact
            .declared_posture()
            .expect("control declaration should admit additive host requirements")
            .host_capability()
            .admitted()
            .map(|posture| posture.required_capabilities()),
        Some(&[
            WorthUiHostCapability::Ime,
            WorthUiHostCapability::TextInput,
        ][..])
    );
}

fn control_posture_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.save"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/declared_posture.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:save"))
    .with_posture_token(UiDslPostureToken::new("query-binding:attached:view"))
    .with_posture_token(UiDslPostureToken::new("service:portal"))
    .with_posture_token(UiDslPostureToken::new("touch:press"))
    .with_posture_token(UiDslPostureToken::new("measurement:hug-height"))
    .with_posture_token(UiDslPostureToken::new("host-capability:text-input"))
}

fn page_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.page.root"),
        UiDslSemanticFamily::Page,
        UiDslSourceProvenance::file_authored("app/declared_posture_classification.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("page:product-root"))
}

fn query_binding_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.query.selection"),
        UiDslSemanticFamily::QueryBinding,
        UiDslSourceProvenance::file_authored("app/declared_posture_classification.wui", 2),
    )
    .with_posture_token(UiDslPostureToken::new("query-binding:standalone"))
}

fn classification_control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.save"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/declared_posture_classification.wui", 1),
    )
    .with_structural_token(UiDslStructuralToken::new("control:save"))
}

fn intent_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.intent.selection"),
        UiDslSemanticFamily::Intent,
        UiDslSourceProvenance::file_authored("app/declared_posture_classification.wui", 3),
    )
    .with_posture_token(UiDslPostureToken::new("intent:standalone"))
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
