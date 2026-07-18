use worth_ui::facade::app::{
    WorthUi, WorthUiApplicationPreparationDenial, WorthUiApplicationPreparationPhase,
};
use worth_ui::facade::declaration::{
    UiDeclarationArtifact, UiDeclarationFamily, UiDeclarationFamilyAdmissionDenial,
    UiDeclarationFamilyCatalog, UiDeclarationFamilyKind, UiDeclarationGraphHandoffDenial,
    UiDeclarationStructuralSemanticsAdmissionDenial, UiDeclaredPostureApplicability,
    UiDeclaredQueryBindingPosture,
};
use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken, WorthUiDslPackage,
};

#[test]
fn admitted_family_catalog_closes_the_initial_family_set_exactly_once() {
    assert_eq!(
        UiDeclarationFamilyCatalog::closed_initial_set(),
        &[
            UiDeclarationFamilyKind::Page,
            UiDeclarationFamilyKind::PageSet,
            UiDeclarationFamilyKind::Region,
            UiDeclarationFamilyKind::Mosaic,
            UiDeclarationFamilyKind::LocalComposition,
            UiDeclarationFamilyKind::Control,
            UiDeclarationFamilyKind::QueryBinding,
            UiDeclarationFamilyKind::Intent,
            UiDeclarationFamilyKind::DiagnosticSurface,
        ]
    );
}

#[test]
fn public_freeze_exposes_bootstrap_page_family_authority() {
    let app = WorthUi::app()
        .freeze()
        .expect("application preparation should succeed");
    let artifact = &app.declaration_artifacts()[0];

    match artifact
        .family()
        .expect("runtime bootstrap page should admit")
    {
        UiDeclarationFamily::Page(page) => {
            assert!(page.structure().is_root_page());
        }
        other => panic!("expected bootstrap page family, got {other:?}"),
    }
    let posture = artifact
        .declared_posture()
        .expect("runtime bootstrap page posture should admit");
    assert_eq!(
        posture.query_binding().applicability(),
        UiDeclaredPostureApplicability::Optional
    );
    assert_eq!(posture.query_binding().admitted(), None);
}

#[test]
fn caller_authored_freeze_distinguishes_standalone_and_attached_query_binding_roles() {
    let attached_app = WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.family.freeze-path")
                .with_semantic_artifact_spec(attached_query_binding_control_spec()),
        )
        .freeze()
        .expect("application preparation should succeed");
    let attached = artifact_from_file_provenance(&attached_app, "app/query_binding_roles.wui", 0);

    match attached
        .family()
        .expect("attached control declaration should admit")
    {
        UiDeclarationFamily::Control(_) => {}
        other => panic!("expected control family, got {other:?}"),
    }
    assert_eq!(
        attached
            .declared_posture()
            .expect("attached control declaration should admit posture")
            .query_binding()
            .applicability(),
        UiDeclaredPostureApplicability::Optional
    );
    assert_eq!(
        attached
            .declared_posture()
            .expect("attached control declaration should admit posture")
            .query_binding()
            .admitted(),
        Some(&UiDeclaredQueryBindingPosture::AttachedViewBinding)
    );

    let denial = freeze_denial(
        "worth-ui.certification.family.freeze-path.standalone",
        standalone_query_binding_spec(),
    );
    assert_eq!(
        denial.phase(),
        WorthUiApplicationPreparationPhase::GraphHandoff
    );
    assert_eq!(
        denial,
        WorthUiApplicationPreparationDenial::GraphHandoff(
            UiDeclarationGraphHandoffDenial::StructuralSemanticsNotAdmitted {
                denial: UiDeclarationStructuralSemanticsAdmissionDenial::
                    FamilyDoesNotProjectStructuralSemantics {
                        family: UiDeclarationFamilyKind::QueryBinding,
                    },
            },
        )
    );
}

#[test]
fn caller_authored_freeze_exposes_typed_family_denials_on_public_artifacts() {
    let contradictory_denial = freeze_denial(
        "worth-ui.certification.family.denials.contradictory",
        contradictory_control_spec(),
    );
    assert_eq!(
        contradictory_denial,
        WorthUiApplicationPreparationDenial::GraphHandoff(
            UiDeclarationGraphHandoffDenial::FamilyNotAdmitted {
                denial: UiDeclarationFamilyAdmissionDenial::ContradictoryStructuralClaims {
                    family: UiDeclarationFamilyKind::Control,
                    observed: vec!["control:save".to_owned(), "region:sidebar".to_owned()],
                },
            },
        )
    );

    let invalid_attached_intent_denial = freeze_denial(
        "worth-ui.certification.family.denials.attached-intent",
        invalid_attached_intent_spec(),
    );
    assert_eq!(
        invalid_attached_intent_denial,
        WorthUiApplicationPreparationDenial::GraphHandoff(
            UiDeclarationGraphHandoffDenial::FamilyNotAdmitted {
                denial: UiDeclarationFamilyAdmissionDenial::InvalidAttachedRoleClaim {
                    family: UiDeclarationFamilyKind::Control,
                    expected_prefix: "intent:attached:",
                    observed: vec!["intent:attached".to_owned()],
                },
            },
        )
    );
}

fn freeze_denial(
    package_name: &'static str,
    spec: UiDslSemanticArtifactSpec,
) -> WorthUiApplicationPreparationDenial {
    match WorthUi::app()
        .with_dsl_package(WorthUiDslPackage::named(package_name).with_semantic_artifact_spec(spec))
        .freeze()
    {
        Ok(_) => panic!("invalid declaration authority must deny application preparation"),
        Err(denial) => denial,
    }
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

fn attached_query_binding_control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.save"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/query_binding_roles.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:save"))
    .with_posture_token(UiDslPostureToken::new("query-binding:attached:view"))
}

fn standalone_query_binding_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.query.selection"),
        UiDslSemanticFamily::QueryBinding,
        UiDslSourceProvenance::file_authored("app/query_binding_roles.wui", 1),
    )
    .with_posture_token(UiDslPostureToken::new("query-binding:standalone"))
}

fn contradictory_control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.contradictory"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/family_denials.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:save"))
    .with_structural_token(UiDslStructuralToken::new("region:sidebar"))
}

fn invalid_attached_intent_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.invalid_intent"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/family_denials.wui", 1),
    )
    .with_structural_token(UiDslStructuralToken::new("control:save"))
    .with_posture_token(UiDslPostureToken::new("intent:attached"))
}
