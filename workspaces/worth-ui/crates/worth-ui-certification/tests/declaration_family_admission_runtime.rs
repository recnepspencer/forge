use std::panic::{catch_unwind, AssertUnwindSafe};

use worth_ui::facade::app::WorthUi;
use worth_ui::facade::declaration::{
    UiDeclarationArtifact, UiDeclarationFamily, UiDeclarationFamilyCatalog,
    UiDeclarationFamilyKind, UiDeclaredPostureApplicability, UiDeclaredQueryBindingPosture,
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
    let app = WorthUi::app().freeze();
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
        .freeze();
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

    let standalone_freeze = catch_unwind(AssertUnwindSafe(|| {
        let _ = WorthUi::app()
            .with_dsl_package(
                WorthUiDslPackage::named("worth-ui.certification.family.freeze-path.standalone")
                    .with_semantic_artifact_spec(standalone_query_binding_spec()),
            )
            .freeze();
    }));
    let standalone_panic = panic_message(standalone_freeze.expect_err(
        "freeze path must reject standalone query-binding declarations before graph publication",
    ));
    assert!(
        standalone_panic.contains("StructuralSemanticsNotAdmitted")
            && standalone_panic.contains("FamilyDoesNotProjectStructuralSemantics")
            && standalone_panic.contains("QueryBinding"),
        "expected standalone query-binding freeze denial to preserve structural-semantics reason, got: {standalone_panic}"
    );
}

#[test]
fn caller_authored_freeze_exposes_typed_family_denials_on_public_artifacts() {
    let contradictory_freeze = catch_unwind(AssertUnwindSafe(|| {
        let _ = WorthUi::app()
            .with_dsl_package(
                WorthUiDslPackage::named("worth-ui.certification.family.denials.contradictory")
                    .with_semantic_artifact_spec(contradictory_control_spec()),
            )
            .freeze();
    }));
    let contradictory_panic = panic_message(contradictory_freeze.expect_err(
        "freeze path must reject contradictory structural claims before graph publication",
    ));
    assert!(
        contradictory_panic.contains("FamilyNotAdmitted")
            && contradictory_panic.contains("ContradictoryStructuralClaims")
            && contradictory_panic.contains("Control")
            && contradictory_panic.contains("control:save")
            && contradictory_panic.contains("region:sidebar"),
        "expected contradictory control freeze denial to preserve typed family denial, got: {contradictory_panic}"
    );

    let invalid_attached_intent_freeze = catch_unwind(AssertUnwindSafe(|| {
        let _ = WorthUi::app()
            .with_dsl_package(
                WorthUiDslPackage::named("worth-ui.certification.family.denials.attached-intent")
                    .with_semantic_artifact_spec(invalid_attached_intent_spec()),
            )
            .freeze();
    }));
    let invalid_attached_intent_panic = panic_message(invalid_attached_intent_freeze.expect_err(
        "freeze path must reject invalid attached intent posture before graph publication",
    ));
    assert!(
        invalid_attached_intent_panic.contains("FamilyNotAdmitted")
            && invalid_attached_intent_panic.contains("InvalidAttachedRoleClaim")
            && invalid_attached_intent_panic.contains("Control")
            && invalid_attached_intent_panic.contains("intent:attached"),
        "expected invalid attached intent freeze denial to preserve typed family denial, got: {invalid_attached_intent_panic}"
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

fn panic_message(payload: Box<dyn std::any::Any + Send>) -> String {
    match payload.downcast::<String>() {
        Ok(message) => *message,
        Err(payload) => match payload.downcast::<&'static str>() {
            Ok(message) => (*message).to_string(),
            Err(_) => "<non-string panic payload>".to_string(),
        },
    }
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
