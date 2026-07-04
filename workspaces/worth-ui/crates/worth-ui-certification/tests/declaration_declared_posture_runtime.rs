use std::panic::{catch_unwind, AssertUnwindSafe};

use worth_ui::facade::app::WorthUi;
use worth_ui::facade::declaration::{
    UiDeclarationArtifact, UiDeclaredMeasurementPolicyPosture, UiDeclaredPostureApplicability,
    UiDeclaredQueryBindingPosture, UiDeclaredServiceUsagePosture, UiDeclaredTouchMeaningPosture,
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
        posture
            .host_capability()
            .admitted()
            .map(|posture| posture.required_capabilities()),
        Some(&[WorthUiHostCapability::TextInput][..])
    );
}

#[test]
fn public_freeze_preserves_representative_family_applicability_shapes() {
    let page_app = WorthUi::app().freeze();
    let page = artifact_from_file_provenance(&page_app, "worth-ui.runtime.bootstrap", 0);

    let control_app = WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.declared-posture.classification")
                .with_semantic_artifact_spec(classification_control_spec()),
        )
        .freeze();
    let control =
        artifact_from_file_provenance(&control_app, "app/declared_posture_classification.wui", 1);

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

    let query_binding_freeze = catch_unwind(AssertUnwindSafe(|| {
        let _ = WorthUi::app()
            .with_dsl_package(
                WorthUiDslPackage::named(
                    "worth-ui.certification.declared-posture.classification.query-binding",
                )
                .with_semantic_artifact_spec(query_binding_spec()),
            )
            .freeze();
    }));
    let query_binding_panic = panic_message(query_binding_freeze.expect_err(
        "freeze path must reject standalone query-binding declarations before graph publication",
    ));
    assert!(
        query_binding_panic.contains("StructuralSemanticsNotAdmitted")
            && query_binding_panic.contains("FamilyDoesNotProjectStructuralSemantics")
            && query_binding_panic.contains("QueryBinding"),
        "expected query-binding freeze denial to preserve structural-semantics reason, got: {query_binding_panic}"
    );

    let intent_freeze = catch_unwind(AssertUnwindSafe(|| {
        let _ = WorthUi::app()
            .with_dsl_package(
                WorthUiDslPackage::named("worth-ui.certification.declared-posture.classification.intent")
                    .with_semantic_artifact_spec(intent_spec()),
            )
            .freeze();
    }));
    let intent_panic = panic_message(
        intent_freeze
            .expect_err("freeze path must reject standalone intent declarations before graph publication"),
    );
    assert!(
        intent_panic.contains("StructuralSemanticsNotAdmitted")
            && intent_panic.contains("FamilyDoesNotProjectStructuralSemantics")
            && intent_panic.contains("Intent"),
        "expected intent freeze denial to preserve structural-semantics reason, got: {intent_panic}"
    );
}

#[test]
fn invalid_declared_posture_denies_before_runtime_or_host_promotion() {
    let result = catch_unwind(AssertUnwindSafe(|| {
        let _ = WorthUi::app()
            .with_dsl_package(
                WorthUiDslPackage::named("worth-ui.certification.declared-posture.denials")
                    .with_semantic_artifact_spec(
                        control_posture_spec()
                            .with_posture_token(UiDslPostureToken::new("service:scroll")),
                    ),
            )
            .freeze();
    }));
    let panic_message = panic_message(result.expect_err(
        "freeze path must reject invalid declared posture before runtime or host promotion",
    ));
    assert!(
        panic_message.contains("DeclaredPostureNotAdmitted")
            && panic_message.contains("ContradictoryLaneClaims")
            && panic_message.contains("ServiceUsage")
            && panic_message.contains("service:portal")
            && panic_message.contains("service:scroll"),
        "expected freeze panic to preserve declared-posture denial, got: {panic_message}"
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
        Some(&[WorthUiHostCapability::Ime, WorthUiHostCapability::TextInput,][..])
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

fn panic_message(payload: Box<dyn std::any::Any + Send>) -> String {
    match payload.downcast::<String>() {
        Ok(message) => *message,
        Err(payload) => match payload.downcast::<&'static str>() {
            Ok(message) => (*message).to_string(),
            Err(_) => "<non-string panic payload>".to_string(),
        },
    }
}
