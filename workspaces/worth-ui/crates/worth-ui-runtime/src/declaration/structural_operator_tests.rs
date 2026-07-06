use worth_ui_dsl::{
    UiDslAspectName, UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily,
    UiDslSemanticKey, UiDslSourceProvenance, UiDslStructuralToken, UiDslSupportToken,
    WorthUiDslPackage,
};

use crate::capability::MosaicSizingContractId;
use crate::declaration::artifact::ui_declaration_lowering::UiDeclarationLowering;
use crate::declaration::{
    UiDeclarationContainmentIntent, UiDeclarationFamilyKind, UiDeclarationPlanningOperatorKind,
    UiDeclarationStructuralSemanticsAdmission, UiDeclarationStructuralSemanticsAdmissionDenial,
};

fn semantic_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.save"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/workflow_editor.wui", 0),
    )
    .with_published_aspect(UiDslAspectName::new("content.text"))
    .with_published_aspect(UiDslAspectName::new("appearance.background"))
    .with_structural_token(UiDslStructuralToken::new("control:save"))
    .with_posture_token(UiDslPostureToken::new("measurement:hug-height"))
    .with_support_token(UiDslSupportToken::new("support:admitted"))
}

fn lower(spec: UiDslSemanticArtifactSpec) -> crate::declaration::UiDeclarationArtifact {
    let package = WorthUiDslPackage::named("worth-ui.runtime.declaration.operator-tests");
    let receipt = package.admit_semantic_artifact(spec);

    UiDeclarationLowering::lower(receipt)
}

#[test]
fn explicit_operator_claims_admit_operator_planning_witness_independently_of_control_name() {
    let artifact = lower(
        UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.inspector.primary"),
            UiDslSemanticFamily::Control,
            UiDslSourceProvenance::file_authored("app/workflow_editor.wui", 0),
        )
        .with_published_aspect(UiDslAspectName::new("content.text"))
        .with_published_aspect(UiDslAspectName::new("appearance.background"))
        .with_structural_token(UiDslStructuralToken::new("control:primary"))
        .with_structural_token(UiDslStructuralToken::new("operator:stack"))
        .with_posture_token(UiDslPostureToken::new("measurement:hug-height"))
        .with_support_token(UiDslSupportToken::new("support:admitted")),
    );
    let semantics = artifact
        .structural_semantics()
        .expect("control declaration should admit structural semantics");

    assert_eq!(semantics.family(), UiDeclarationFamilyKind::Control);
    assert_eq!(
        semantics.operator_kind(),
        UiDeclarationPlanningOperatorKind::Stack
    );
    assert_eq!(
        semantics.containment_intent(),
        &UiDeclarationContainmentIntent::DeclaredControlAttachment {
            control_name: "primary".into()
        }
    );
}

#[test]
fn invalid_explicit_operator_claims_deny_before_handoff_derivation() {
    let artifact =
        lower(semantic_spec().with_structural_token(UiDslStructuralToken::new("operator:bogus")));
    let expected_denial =
        UiDeclarationStructuralSemanticsAdmissionDenial::InvalidPlanningOperatorClaim {
            family: UiDeclarationFamilyKind::Control,
            observed: vec!["operator:bogus".to_owned()],
        };

    assert_eq!(
        artifact.structural_semantics_admission(),
        &UiDeclarationStructuralSemanticsAdmission::Denied(expected_denial.clone())
    );
    assert_eq!(artifact.structural_semantics(), Err(&expected_denial));
}

#[test]
fn contradictory_explicit_operator_claims_deny_before_handoff_derivation() {
    let artifact = lower(
        semantic_spec()
            .with_structural_token(UiDslStructuralToken::new("operator:stack"))
            .with_structural_token(UiDslStructuralToken::new("operator:row")),
    );
    let expected_denial =
        UiDeclarationStructuralSemanticsAdmissionDenial::ContradictoryPlanningOperatorClaims {
            family: UiDeclarationFamilyKind::Control,
            observed: vec!["operator:stack".to_owned(), "operator:row".to_owned()],
        };

    assert_eq!(
        artifact.structural_semantics_admission(),
        &UiDeclarationStructuralSemanticsAdmission::Denied(expected_denial.clone())
    );
    assert_eq!(artifact.structural_semantics(), Err(&expected_denial));
}

#[test]
fn explicit_operator_claims_are_not_admitted_for_non_control_families() {
    let artifact = lower(
        UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.page.root"),
            UiDslSemanticFamily::Page,
            UiDslSourceProvenance::file_authored("app/workflow_editor.wui", 4),
        )
        .with_structural_token(UiDslStructuralToken::new("page:product-root"))
        .with_structural_token(UiDslStructuralToken::new("operator:stack")),
    );
    let expected_denial =
        UiDeclarationStructuralSemanticsAdmissionDenial::PlanningOperatorNotAdmittedForFamily {
            family: UiDeclarationFamilyKind::Page,
            observed: vec!["operator:stack".to_owned()],
        };

    assert_eq!(
        artifact.structural_semantics_admission(),
        &UiDeclarationStructuralSemanticsAdmission::Denied(expected_denial.clone())
    );
    assert_eq!(artifact.structural_semantics(), Err(&expected_denial));
}

#[test]
fn unsupported_structural_tokens_deny_before_handoff_derivation() {
    let artifact =
        lower(semantic_spec().with_structural_token(UiDslStructuralToken::new("repeat:many")));
    let expected_denial =
        UiDeclarationStructuralSemanticsAdmissionDenial::UnsupportedStructuralTokens {
            family: UiDeclarationFamilyKind::Control,
            observed: vec!["repeat:many".to_owned()],
        };

    assert_eq!(
        artifact.structural_semantics_admission(),
        &UiDeclarationStructuralSemanticsAdmission::Denied(expected_denial.clone())
    );
    assert_eq!(artifact.structural_semantics(), Err(&expected_denial));
    assert_eq!(
        artifact.graph_handoff(),
        Err(
            crate::declaration::UiDeclarationGraphHandoffDenial::StructuralSemanticsNotAdmitted {
                denial: expected_denial.clone(),
            },
        ),
    );
}

#[test]
fn slot_participation_not_admitted_for_page_structures() {
    let artifact = lower(
        UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.page.root"),
            UiDslSemanticFamily::Page,
            UiDslSourceProvenance::file_authored("app/workflow_editor.wui", 3),
        )
        .with_structural_token(UiDslStructuralToken::new("page:product-root"))
        .with_structural_token(UiDslStructuralToken::new("slot:footer")),
    );
    let expected_denial =
        UiDeclarationStructuralSemanticsAdmissionDenial::SlotParticipationNotAdmittedForFamily {
            family: UiDeclarationFamilyKind::Page,
            observed: vec!["slot:footer".to_owned()],
        };

    assert_eq!(
        artifact.structural_semantics_admission(),
        &UiDeclarationStructuralSemanticsAdmission::Denied(expected_denial.clone())
    );
    assert_eq!(artifact.structural_semantics(), Err(&expected_denial));
    assert_eq!(
        artifact.graph_handoff(),
        Err(
            crate::declaration::UiDeclarationGraphHandoffDenial::StructuralSemanticsNotAdmitted {
                denial: expected_denial.clone(),
            },
        ),
    );
}

#[test]
fn mosaic_sizing_contract_claim_is_admitted_into_the_graph_handoff_boundary() {
    let artifact = lower(
        UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.mosaic.workspace"),
            UiDslSemanticFamily::Mosaic,
            UiDslSourceProvenance::file_authored("app/workflow_editor.wui", 5),
        )
        .with_structural_token(UiDslStructuralToken::new("mosaic:workspace"))
        .with_structural_token(UiDslStructuralToken::new(
            "mosaic-sizing:workspace.sizing.workspace",
        )),
    );
    let handoff = artifact
        .graph_handoff()
        .expect("mosaic declaration should lower through graph handoff");

    assert_eq!(
        handoff.mosaic_sizing_contract_id(),
        Some(
            &MosaicSizingContractId::new("workspace.sizing.workspace")
                .expect("test sizing id should admit"),
        )
    );
}
