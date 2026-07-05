use std::panic::{catch_unwind, AssertUnwindSafe};

use worth_ui::facade::app::WorthUi;
use worth_ui::facade::declaration::{
    UiDeclarationArtifact, UiDeclarationContainmentIntent, UiDeclarationFamily,
    UiDeclarationFamilyKind, UiDeclarationOrderingGuarantee, UiDeclarationRepetitionPosture,
    UiDeclarationSlotParticipationIntent, UiDeclarationStructuralRole,
};
use worth_ui_dsl::{
    UiDslAspectName, UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily,
    UiDslSemanticKey, UiDslSourceProvenance, UiDslStructuralToken, UiDslSupportToken,
    WorthUiDslPackage,
};

#[test]
fn public_freeze_exposes_bootstrap_page_structural_intent_and_handoff() {
    let app = WorthUi::app().freeze();
    let artifact = &app.declaration_artifacts()[0];
    let structural = artifact
        .structural_semantics()
        .expect("bootstrap page should admit structural semantics");
    let handoff = artifact
        .graph_handoff()
        .expect("bootstrap page should derive structural handoff");

    assert_eq!(structural.family(), UiDeclarationFamilyKind::Page);
    assert_eq!(structural.role(), UiDeclarationStructuralRole::Page);
    assert_eq!(
        structural.containment_intent(),
        &UiDeclarationContainmentIntent::RootTopology
    );
    assert!(structural.slot_participation_intent().is_none());
    assert_eq!(
        structural.ordering_guarantee(),
        UiDeclarationOrderingGuarantee::NotSemanticallyClaimed
    );
    assert_eq!(
        structural.repetition_posture(),
        UiDeclarationRepetitionPosture::NotAdmitted
    );
    assert_eq!(handoff.identity(), artifact.identity());
    assert!(matches!(handoff.family(), UiDeclarationFamily::Page(_)));
    assert_eq!(handoff.role(), UiDeclarationStructuralRole::Page);
    assert!(handoff.containment_intent().is_root());
    assert!(handoff.slot_participation_intent().is_none());
}

#[test]
fn caller_authored_freeze_projects_structural_slot_participation_intent() {
    let app = WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.structural.slot")
                .with_semantic_artifact_spec(slotted_control_spec()),
        )
        .freeze();
    let artifact = artifact_from_file_provenance(&app, "app/structural_semantics.wui", 0);
    let structural = artifact
        .structural_semantics()
        .expect("control declaration should admit structural semantics");
    let handoff = artifact
        .graph_handoff()
        .expect("control declaration should derive structural handoff");

    assert_eq!(structural.family(), UiDeclarationFamilyKind::Control);
    assert_eq!(structural.role(), UiDeclarationStructuralRole::Control);
    assert_eq!(
        structural.containment_intent(),
        &UiDeclarationContainmentIntent::DeclaredControlAttachment {
            control_name: "save".into()
        }
    );
    assert_eq!(
        structural.slot_participation_intent(),
        &UiDeclarationSlotParticipationIntent::DeclaredSlotParticipant {
            slot_name: "footer".into()
        }
    );
    assert_eq!(
        handoff.family(),
        artifact.family().expect("control family should admit")
    );
    assert_eq!(handoff.role(), UiDeclarationStructuralRole::Control);
    assert_eq!(
        handoff.slot_participation_intent(),
        &UiDeclarationSlotParticipationIntent::DeclaredSlotParticipant {
            slot_name: "footer".into()
        }
    );
}

#[test]
fn non_structural_noise_does_not_change_structural_semantics_or_handoff() {
    let baseline = WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.structural.localization")
                .with_semantic_artifact_spec(slotted_control_spec()),
        )
        .freeze();
    let changed = WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.structural.localization")
                .with_semantic_artifact_spec(slotted_control_with_noise_spec()),
        )
        .freeze();
    let baseline_artifact =
        artifact_from_file_provenance(&baseline, "app/structural_semantics.wui", 0);
    let changed_artifact =
        artifact_from_file_provenance(&changed, "app/structural_semantics.wui", 0);
    let baseline_handoff = baseline_artifact
        .graph_handoff()
        .expect("baseline control should derive structural handoff");
    let changed_handoff = changed_artifact
        .graph_handoff()
        .expect("changed control should derive structural handoff");

    assert_eq!(
        baseline_artifact.structural_semantics(),
        changed_artifact.structural_semantics()
    );
    assert_ne!(baseline_handoff.identity(), changed_handoff.identity());
    assert_eq!(baseline_handoff.family(), changed_handoff.family());
    assert_eq!(baseline_handoff.role(), changed_handoff.role());
    assert_eq!(
        baseline_handoff.containment_intent(),
        changed_handoff.containment_intent()
    );
    assert_eq!(
        baseline_handoff.slot_participation_intent(),
        changed_handoff.slot_participation_intent()
    );
    assert_eq!(
        baseline_handoff.ordering_guarantee(),
        changed_handoff.ordering_guarantee()
    );
    assert_eq!(
        baseline_handoff.repetition_posture(),
        changed_handoff.repetition_posture()
    );
}

#[test]
fn every_admitted_structural_family_projects_declared_structural_intent() {
    let app = WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.structural.families")
                .with_semantic_artifact_spec(page_set_spec())
                .with_semantic_artifact_spec(region_spec())
                .with_semantic_artifact_spec(mosaic_spec())
                .with_semantic_artifact_spec(local_composition_spec())
                .with_semantic_artifact_spec(diagnostic_surface_spec()),
        )
        .freeze();

    assert_structural_projection(
        artifact_from_file_provenance(&app, "app/structural_families.wui", 0),
        UiDeclarationFamilyKind::PageSet,
        UiDeclarationStructuralRole::PageSet,
        &UiDeclarationContainmentIntent::DeclaredPageSetMembership {
            page_set_name: "shell".into(),
        },
        Some("shell"),
    );
    assert_structural_projection(
        artifact_from_file_provenance(&app, "app/structural_families.wui", 1),
        UiDeclarationFamilyKind::Region,
        UiDeclarationStructuralRole::Region,
        &UiDeclarationContainmentIntent::DeclaredRegionMembership {
            region_name: "sidebar".into(),
        },
        Some("sidebar"),
    );
    assert_structural_projection(
        artifact_from_file_provenance(&app, "app/structural_families.wui", 2),
        UiDeclarationFamilyKind::Mosaic,
        UiDeclarationStructuralRole::Mosaic,
        &UiDeclarationContainmentIntent::DeclaredMosaicMembership {
            mosaic_name: "workspace".into(),
        },
        Some("workspace"),
    );
    assert_structural_projection(
        artifact_from_file_provenance(&app, "app/structural_families.wui", 3),
        UiDeclarationFamilyKind::LocalComposition,
        UiDeclarationStructuralRole::LocalComposition,
        &UiDeclarationContainmentIntent::DeclaredLocalCompositionMembership {
            local_composition_name: "inspector".into(),
        },
        Some("inspector"),
    );
    assert_structural_projection(
        artifact_from_file_provenance(&app, "app/structural_families.wui", 4),
        UiDeclarationFamilyKind::DiagnosticSurface,
        UiDeclarationStructuralRole::DiagnosticSurface,
        &UiDeclarationContainmentIntent::DeclaredDiagnosticAttachment {
            diagnostic_surface_name: "lint".into(),
        },
        Some("lint"),
    );
}

#[test]
fn slot_participation_not_admitted_for_page_structures_on_freeze_path() {
    let freeze = catch_unwind(AssertUnwindSafe(|| {
        let _ = WorthUi::app()
            .with_dsl_package(
                WorthUiDslPackage::named("worth-ui.certification.structural.invalid_slot")
                    .with_semantic_artifact_spec(page_with_slot_spec()),
            )
            .freeze();
    }));
    let panic_message = panic_message(
        freeze
            .expect_err("freeze path must reject page slot participation before graph publication"),
    );
    assert!(
        panic_message.contains("StructuralSemanticsNotAdmitted")
            && panic_message.contains("SlotParticipationNotAdmittedForFamily")
            && panic_message.contains("Page")
            && panic_message.contains("slot:footer"),
        "expected page slot-participation denial to remain typed on the freeze path, got: {panic_message}"
    );
}

#[test]
fn unsupported_structural_tokens_deny_through_public_freeze_path() {
    let freeze = catch_unwind(AssertUnwindSafe(|| {
        let _ = WorthUi::app()
            .with_dsl_package(
                WorthUiDslPackage::named("worth-ui.certification.structural.unsupported")
                    .with_semantic_artifact_spec(unsupported_structural_spec()),
            )
            .freeze();
    }));
    let panic_message = panic_message(freeze.expect_err(
        "freeze path must reject unsupported structural tokens before graph publication",
    ));
    assert!(
        panic_message.contains("StructuralSemanticsNotAdmitted")
            && panic_message.contains("UnsupportedStructuralTokens")
            && panic_message.contains("Control")
            && panic_message.contains("repeat:many"),
        "expected unsupported structural token denial to remain typed on the freeze path, got: {panic_message}"
    );
}

#[test]
fn non_structural_families_cannot_smuggle_graph_handoff_authority() {
    let freeze = catch_unwind(AssertUnwindSafe(|| {
        let _ = WorthUi::app()
            .with_dsl_package(
                WorthUiDslPackage::named("worth-ui.certification.structural.non_structural")
                    .with_semantic_artifact_spec(standalone_query_binding_spec()),
            )
            .freeze();
    }));
    let panic_message = panic_message(
        freeze
            .expect_err("freeze path must reject non-structural families before graph publication"),
    );
    assert!(
        panic_message.contains("StructuralSemanticsNotAdmitted")
            && panic_message.contains("FamilyDoesNotProjectStructuralSemantics")
            && panic_message.contains("QueryBinding"),
        "expected non-structural family denial to remain typed on the freeze path, got: {panic_message}"
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

fn assert_structural_projection(
    artifact: &UiDeclarationArtifact,
    expected_family: UiDeclarationFamilyKind,
    expected_role: UiDeclarationStructuralRole,
    expected_containment: &UiDeclarationContainmentIntent,
    expected_claim_name: Option<&str>,
) {
    let structural = artifact
        .structural_semantics()
        .expect("structural family should admit structural semantics");
    let handoff = artifact
        .graph_handoff()
        .expect("structural family should derive structural handoff");

    assert_eq!(structural.family(), expected_family);
    assert_eq!(structural.role(), expected_role);
    assert_eq!(structural.containment_intent(), expected_containment);
    assert_eq!(
        structural.containment_intent().claim_name(),
        expected_claim_name
    );
    assert!(structural.slot_participation_intent().is_none());
    assert_eq!(handoff.family_kind(), expected_family);
    assert_eq!(handoff.role(), expected_role);
    assert_eq!(handoff.containment_intent(), expected_containment);
    assert_eq!(
        handoff.containment_intent().claim_name(),
        expected_claim_name
    );
    assert!(handoff.slot_participation_intent().is_none());
}

fn slotted_control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.save"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/structural_semantics.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:save"))
    .with_structural_token(UiDslStructuralToken::new("slot:footer"))
    .with_posture_token(UiDslPostureToken::new("query-binding:attached:view"))
}

fn slotted_control_with_noise_spec() -> UiDslSemanticArtifactSpec {
    slotted_control_spec()
        .with_published_aspect(UiDslAspectName::new("content.text"))
        .with_support_token(UiDslSupportToken::new("support:preview-only"))
}

fn page_set_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.page_set.shell"),
        UiDslSemanticFamily::PageSet,
        UiDslSourceProvenance::file_authored("app/structural_families.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("page-set:shell"))
}

fn region_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.region.sidebar"),
        UiDslSemanticFamily::Region,
        UiDslSourceProvenance::file_authored("app/structural_families.wui", 1),
    )
    .with_structural_token(UiDslStructuralToken::new("region:sidebar"))
}

fn mosaic_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.mosaic.workspace"),
        UiDslSemanticFamily::Mosaic,
        UiDslSourceProvenance::file_authored("app/structural_families.wui", 2),
    )
    .with_structural_token(UiDslStructuralToken::new("mosaic:workspace"))
}

fn local_composition_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.local_composition.inspector"),
        UiDslSemanticFamily::LocalComposition,
        UiDslSourceProvenance::file_authored("app/structural_families.wui", 3),
    )
    .with_structural_token(UiDslStructuralToken::new("local-composition:inspector"))
}

fn diagnostic_surface_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.diagnostics.lint"),
        UiDslSemanticFamily::DiagnosticSurface,
        UiDslSourceProvenance::file_authored("app/structural_families.wui", 4),
    )
    .with_structural_token(UiDslStructuralToken::new("diagnostic-surface:lint"))
}

fn unsupported_structural_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.unsupported"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/structural_denials.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:save"))
    .with_structural_token(UiDslStructuralToken::new("repeat:many"))
}

fn page_with_slot_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.page.invalid_slot"),
        UiDslSemanticFamily::Page,
        UiDslSourceProvenance::file_authored("app/structural_invalid_slot.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("page:product-root"))
    .with_structural_token(UiDslStructuralToken::new("slot:footer"))
}

fn standalone_query_binding_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.query.selection"),
        UiDslSemanticFamily::QueryBinding,
        UiDslSourceProvenance::file_authored("app/structural_non_structural.wui", 0),
    )
    .with_posture_token(UiDslPostureToken::new("query-binding:standalone"))
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
