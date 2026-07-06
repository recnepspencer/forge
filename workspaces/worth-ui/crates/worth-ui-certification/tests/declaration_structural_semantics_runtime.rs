use std::panic::{catch_unwind, AssertUnwindSafe};

mod declaration_structural_semantics_test_support;

use declaration_structural_semantics_test_support::{
    artifact_from_file_provenance, assert_structural_projection, diagnostic_surface_spec,
    local_composition_spec, mosaic_spec, page_set_spec, page_with_slot_spec, panic_message,
    region_spec, slotted_control_spec, slotted_control_with_noise_spec,
    standalone_query_binding_spec, unsupported_structural_spec,
};
use worth_ui::facade::app::WorthUi;
use worth_ui::facade::declaration::{
    UiDeclarationContainmentIntent, UiDeclarationFamily, UiDeclarationFamilyKind,
    UiDeclarationOrderingGuarantee, UiDeclarationRepetitionPosture,
    UiDeclarationSlotParticipationIntent, UiDeclarationStructuralRole,
};
use worth_ui_dsl::WorthUiDslPackage;
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
    assert_eq!(format!("{:?}", structural.operator_kind()), "Stack");
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
    assert_eq!(format!("{:?}", handoff.operator_kind()), "Stack");
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
