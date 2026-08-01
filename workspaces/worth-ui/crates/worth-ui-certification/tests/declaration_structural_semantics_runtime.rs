use worth_ui_certification::{
    WorthUiCertificationBuilderExt, WorthUiRustAuthoredDeclarationFixture,
};
#[path = "fixtures/declaration_structural_semantics_test_support.rs"]
mod declaration_structural_semantics_test_support;

use declaration_structural_semantics_test_support::{
    artifact_from_file_provenance, assert_structural_projection, diagnostic_surface_spec,
    local_composition_spec, mosaic_spec, page_set_spec, page_with_slot_spec, region_spec,
    slotted_control_spec, slotted_control_with_noise_spec, standalone_query_binding_spec,
    unsupported_structural_spec,
};
use worth_ui::facade::app::{WorthUi, WorthUiApplicationPreparationDenial};
use worth_ui::facade::declaration::{
    UiDeclarationContainmentIntent, UiDeclarationFamily, UiDeclarationFamilyKind,
    UiDeclarationGraphHandoffDenial, UiDeclarationOrderingGuarantee,
    UiDeclarationRepetitionPosture, UiDeclarationSlotParticipationIntent,
    UiDeclarationStructuralRole, UiDeclarationStructuralSemanticsAdmissionDenial,
};
use worth_ui_dsl::UiDslSemanticArtifactSpec;
#[test]
fn public_freeze_exposes_bootstrap_page_structural_intent_and_handoff() {
    let app = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .freeze()
        .expect("application preparation should succeed");
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
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named("worth-ui.certification.structural.slot")
                .with_semantic_artifact_spec(slotted_control_spec()),
        )
        .freeze()
        .expect("application preparation should succeed");
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
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.certification.structural.localization",
            )
            .with_semantic_artifact_spec(slotted_control_spec()),
        )
        .freeze()
        .expect("application preparation should succeed");
    let changed = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.certification.structural.localization",
            )
            .with_semantic_artifact_spec(slotted_control_with_noise_spec()),
        )
        .freeze()
        .expect("application preparation should succeed");
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
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.certification.structural.families",
            )
            .with_semantic_artifact_spec(page_set_spec())
            .with_semantic_artifact_spec(region_spec())
            .with_semantic_artifact_spec(mosaic_spec())
            .with_semantic_artifact_spec(local_composition_spec())
            .with_semantic_artifact_spec(diagnostic_surface_spec()),
        )
        .freeze()
        .expect("application preparation should succeed");

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
    let denial = freeze_denial(
        "worth-ui.certification.structural.invalid_slot",
        page_with_slot_spec(),
    );
    assert_eq!(
        denial,
        WorthUiApplicationPreparationDenial::GraphHandoff(
            UiDeclarationGraphHandoffDenial::StructuralSemanticsNotAdmitted {
                denial: UiDeclarationStructuralSemanticsAdmissionDenial::
                    SlotParticipationNotAdmittedForFamily {
                        family: UiDeclarationFamilyKind::Page,
                        observed: vec!["slot:footer".to_owned()],
                    },
            },
        )
    );
}

#[test]
fn unsupported_structural_tokens_deny_through_public_freeze_path() {
    let denial = freeze_denial(
        "worth-ui.certification.structural.unsupported",
        unsupported_structural_spec(),
    );
    assert_eq!(
        denial,
        WorthUiApplicationPreparationDenial::GraphHandoff(
            UiDeclarationGraphHandoffDenial::StructuralSemanticsNotAdmitted {
                denial:
                    UiDeclarationStructuralSemanticsAdmissionDenial::UnsupportedStructuralTokens {
                        family: UiDeclarationFamilyKind::Control,
                        observed: vec!["repeat:many".to_owned()],
                    },
            },
        )
    );
}

#[test]
fn non_structural_families_cannot_smuggle_graph_handoff_authority() {
    let app = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.certification.structural.non_structural",
            )
            .with_semantic_artifact_spec(standalone_query_binding_spec()),
        )
        .freeze()
        .expect("non-structural declarations coexist without graph authority");
    let artifact = artifact_from_file_provenance(&app, "app/structural_non_structural.wui", 0);
    assert_eq!(
        artifact.structural_semantics().unwrap_err(),
        &UiDeclarationStructuralSemanticsAdmissionDenial::FamilyDoesNotProjectStructuralSemantics {
            family: UiDeclarationFamilyKind::QueryBinding,
        }
    );
    assert!(artifact.graph_handoff().is_err());
    assert!(app
        .graph()
        .lookup()
        .declaration_instances(artifact.identity())
        .value()
        .is_empty());
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
        Ok(_) => panic!("invalid structural authority must deny application preparation"),
        Err(denial) => denial,
    }
}
