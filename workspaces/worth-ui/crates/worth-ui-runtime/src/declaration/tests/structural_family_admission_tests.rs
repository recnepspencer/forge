use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken,
};

use crate::declaration::{
    UiDeclarationContainmentIntent, UiDeclarationFamily, UiDeclarationFamilyAdmission,
    UiDeclarationFamilyAdmissionDenial, UiDeclarationFamilyKind, UiDeclarationOrderingGuarantee,
    UiDeclarationPlanningOperatorKind, UiDeclarationRepetitionPosture,
    UiDeclarationSlotParticipationIntent, UiDeclarationStructuralRole,
};

use super::{lower, semantic_spec};

#[test]
fn contradictory_structural_claims_deny_through_family_admission() {
    let artifact = lower(
        semantic_spec().with_structural_token(UiDslStructuralToken::new("control:alternate")),
    );

    assert_eq!(
        artifact.family_admission(),
        &UiDeclarationFamilyAdmission::Denied(
            UiDeclarationFamilyAdmissionDenial::ContradictoryStructuralClaims {
                family: UiDeclarationFamilyKind::Control,
                observed: vec!["control:alternate".to_owned(), "control:save".to_owned()],
            }
        )
    );
}

#[test]
fn foreign_structural_family_claims_deny_through_family_admission() {
    let artifact =
        lower(semantic_spec().with_structural_token(UiDslStructuralToken::new("region:sidebar")));

    assert_eq!(
        artifact.family_admission(),
        &UiDeclarationFamilyAdmission::Denied(
            UiDeclarationFamilyAdmissionDenial::ContradictoryStructuralClaims {
                family: UiDeclarationFamilyKind::Control,
                observed: vec!["control:save".to_owned(), "region:sidebar".to_owned()],
            }
        )
    );
}

#[test]
fn partial_family_claims_deny_before_downstream_consumption() {
    let artifact = lower(UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.region.left_pane"),
        UiDslSemanticFamily::Region,
        UiDslSourceProvenance::file_authored("app/workflow_editor.wui", 2),
    ));

    assert_eq!(
        artifact.family_admission(),
        &UiDeclarationFamilyAdmission::Denied(
            UiDeclarationFamilyAdmissionDenial::MissingStructuralClaim {
                family: UiDeclarationFamilyKind::Region,
                expected_prefix: "region:",
            }
        )
    );
}

#[test]
fn invalid_attached_query_binding_role_tokens_deny_through_family_admission() {
    let artifact =
        lower(semantic_spec().with_posture_token(UiDslPostureToken::new("query-binding:attached")));

    assert_eq!(
        artifact.family_admission(),
        &UiDeclarationFamilyAdmission::Denied(
            UiDeclarationFamilyAdmissionDenial::InvalidAttachedRoleClaim {
                family: UiDeclarationFamilyKind::Control,
                expected_prefix: "query-binding:attached:",
                observed: vec!["query-binding:attached".to_owned()],
            }
        )
    );
}

#[test]
fn invalid_attached_intent_role_tokens_deny_through_family_admission() {
    let artifact =
        lower(semantic_spec().with_posture_token(UiDslPostureToken::new("intent:maybe")));

    assert_eq!(
        artifact.family_admission(),
        &UiDeclarationFamilyAdmission::Denied(
            UiDeclarationFamilyAdmissionDenial::InvalidAttachedRoleClaim {
                family: UiDeclarationFamilyKind::Control,
                expected_prefix: "intent:attached:",
                observed: vec!["intent:maybe".to_owned()],
            }
        )
    );
}

#[test]
fn standalone_intent_cannot_be_smuggled_through_control_posture() {
    let artifact =
        lower(semantic_spec().with_posture_token(UiDslPostureToken::new("intent:standalone")));

    assert_eq!(
        artifact.family_admission(),
        &UiDeclarationFamilyAdmission::Denied(
            UiDeclarationFamilyAdmissionDenial::StructuralFamilyCannotClaimStandaloneRole {
                family: UiDeclarationFamilyKind::Control,
                observed: vec!["intent:standalone".to_owned()],
            }
        )
    );
}

#[test]
fn structural_semantics_project_declared_slot_intent_without_graph_truth() {
    let artifact =
        lower(semantic_spec().with_structural_token(UiDslStructuralToken::new("slot:footer")));
    let semantics = artifact
        .structural_semantics()
        .expect("control declaration should admit structural semantics");
    let handoff = artifact
        .structural_handoff()
        .expect("control declaration should derive structural handoff");

    assert_eq!(semantics.family(), UiDeclarationFamilyKind::Control);
    assert_eq!(semantics.role(), UiDeclarationStructuralRole::Control);
    assert_eq!(
        semantics.operator_kind(),
        UiDeclarationPlanningOperatorKind::Control
    );
    assert_eq!(
        semantics.containment_intent(),
        &UiDeclarationContainmentIntent::DeclaredControlAttachment {
            control_name: "save".into()
        }
    );
    assert_eq!(
        semantics.slot_participation_intent(),
        &UiDeclarationSlotParticipationIntent::DeclaredSlotParticipant {
            slot_name: "footer".into()
        }
    );
    assert_eq!(
        semantics.ordering_guarantee(),
        UiDeclarationOrderingGuarantee::NotSemanticallyClaimed
    );
    assert_eq!(
        semantics.repetition_posture(),
        UiDeclarationRepetitionPosture::NotAdmitted
    );
    assert_eq!(handoff.identity(), artifact.identity());
    assert!(matches!(handoff.family(), UiDeclarationFamily::Control(_)));
    assert_eq!(handoff.role(), UiDeclarationStructuralRole::Control);
}
