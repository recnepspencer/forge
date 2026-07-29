use worth_ui::facade::app::{WorthUi, WorthUiApplicationPreparationDenial};
use worth_ui::facade::declaration::{
    UiDeclarationArtifact, UiDeclarationFamilyKind, UiDeclarationGraphHandoffDenial,
    UiDeclarationStructuralSemanticsAdmissionDenial, UiDeclaredPostureAdmissionDenial,
    UiDeclaredPostureApplicability, UiDeclaredPostureLaneKind, UiDeclaredQueryBindingPosture,
    UiDeclaredServiceUsagePosture, UiDeclaredTouchMeaningPosture,
};
use worth_ui_certification::{
    WorthUiCertificationBuilderExt, WorthUiRustAuthoredDeclarationFixture,
};
use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken,
};
use worth_ui_host_contract::WorthUiHostCapability;
use worth_ui_test_support::UiDeclaredMeasurementMode;

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
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named("worth-ui.certification.declared-posture")
                .with_semantic_artifact_spec(control_posture_spec()),
        )
        .freeze()
        .expect("application preparation should succeed");
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
    let measurement_policy = posture
        .measurement_policy()
        .admitted()
        .expect("measurement posture should remain declaration-owned on freeze");
    assert_eq!(
        measurement_policy.mode(),
        Some(UiDeclaredMeasurementMode::HugHeight)
    );
    assert_eq!(measurement_policy.constraint_modifier(), None);
    assert_eq!(measurement_policy.basis_source(), None);
    assert_eq!(measurement_policy.ownership_posture(), None);
    assert!(measurement_policy.evidence_requirements().is_empty());
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
    let page_app = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .freeze()
        .expect("application preparation should succeed");
    let page = artifact_from_file_provenance(&page_app, "worth-ui.runtime.bootstrap", 0);

    let control_fixture = WorthUiRustAuthoredDeclarationFixture::named(
        "worth-ui.certification.declared-posture.classification",
    )
    .with_semantic_artifact_spec(classification_control_spec());
    let control_provenance =
        control_fixture.admitted_provenance_for("workflow_editor.inspector.save");
    let control_app = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(control_fixture)
        .freeze()
        .expect("application preparation should succeed");
    let control = artifact_from_compiler_provenance(&control_app, &control_provenance);

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

    let query_binding_denial = freeze_denial(
        "worth-ui.certification.declared-posture.classification.query-binding",
        query_binding_spec(),
    );
    assert_eq!(
        query_binding_denial,
        WorthUiApplicationPreparationDenial::GraphHandoff(
            UiDeclarationGraphHandoffDenial::StructuralSemanticsNotAdmitted {
                denial: UiDeclarationStructuralSemanticsAdmissionDenial::
                    FamilyDoesNotProjectStructuralSemantics {
                        family: UiDeclarationFamilyKind::QueryBinding,
                    },
            },
        )
    );

    let intent_denial = freeze_denial(
        "worth-ui.certification.declared-posture.classification.intent",
        intent_spec(),
    );
    assert_eq!(
        intent_denial,
        WorthUiApplicationPreparationDenial::GraphHandoff(
            UiDeclarationGraphHandoffDenial::StructuralSemanticsNotAdmitted {
                denial: UiDeclarationStructuralSemanticsAdmissionDenial::
                    FamilyDoesNotProjectStructuralSemantics {
                        family: UiDeclarationFamilyKind::Intent,
                    },
            },
        )
    );
}

#[test]
fn invalid_declared_posture_denies_before_runtime_or_host_promotion() {
    let denial = freeze_denial(
        "worth-ui.certification.declared-posture.denials",
        control_posture_spec().with_posture_token(UiDslPostureToken::new("service:scroll")),
    );
    assert_eq!(
        denial,
        WorthUiApplicationPreparationDenial::GraphHandoff(
            UiDeclarationGraphHandoffDenial::DeclaredPostureNotAdmitted {
                denial: UiDeclaredPostureAdmissionDenial::ContradictoryLaneClaims {
                    family: UiDeclarationFamilyKind::Control,
                    lane: UiDeclaredPostureLaneKind::ServiceUsage,
                    observed: vec!["service:portal".to_owned(), "service:scroll".to_owned()],
                },
            },
        )
    );
}

#[test]
fn host_capability_requirements_appear_as_declared_posture_before_host_inference() {
    let app = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.certification.declared-posture.host",
            )
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
        .freeze()
        .expect("application preparation should succeed");
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

fn artifact_from_compiler_provenance<'a>(
    app: &'a worth_ui::facade::app::WorthUiApp,
    provenance: &UiDslSourceProvenance,
) -> &'a UiDeclarationArtifact {
    artifact_from_file_provenance(
        app,
        provenance.module_path(),
        provenance.declaration_index(),
    )
}

fn freeze_denial(
    package_name: &'static str,
    spec: UiDslSemanticArtifactSpec,
) -> WorthUiApplicationPreparationDenial {
    match WorthUi::app()
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
