use worth_ui_dsl::{
    UiDslAspectName, UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily,
    UiDslSemanticKey, UiDslSourceProvenance, UiDslStructuralToken, UiDslSupportToken,
    WorthUiDslPackage,
};

use crate::declaration::artifact::ui_declaration_lowering::UiDeclarationLowering;
use crate::declaration::{
    UiAspectContractAdmission, UiAspectContractAdmissionDenial, UiAspectSemanticSlice,
    UiDeclarationContainmentIntent, UiDeclarationFamily, UiDeclarationFamilyAdmission,
    UiDeclarationFamilyAdmissionDenial, UiDeclarationFamilyKind, UiDeclarationOrderingGuarantee,
    UiDeclarationRepetitionPosture, UiDeclarationSlotParticipationIntent,
    UiDeclarationStructuralRole, UiDeclarationStructuralSemanticsAdmission,
    UiDeclarationStructuralSemanticsAdmissionDenial,
};

// These tests intentionally certify lane-local lowering and digest behavior.
// The ordinary public freeze-lane proof lives in worth-ui-certification.

fn semantic_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.save"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/workflow_editor.wui", 0),
    )
    .with_published_aspect(UiDslAspectName::new("content.text"))
    .with_published_aspect(UiDslAspectName::new("appearance.background"))
    .with_consumed_aspect(UiDslAspectName::new("interaction.operability"))
    .with_structural_token(UiDslStructuralToken::new("control:save"))
    .with_posture_token(UiDslPostureToken::new("measurement:hug-height"))
    .with_support_token(UiDslSupportToken::new("support:admitted"))
}

fn lower(spec: UiDslSemanticArtifactSpec) -> crate::declaration::UiDeclarationArtifact {
    let package = WorthUiDslPackage::named("worth-ui.runtime.declaration.tests");
    let receipt = package.admit_semantic_artifact(spec);

    UiDeclarationLowering::lower(receipt)
}

#[test]
fn non_semantic_noise_does_not_change_declaration_identity() {
    let baseline = lower(semantic_spec());
    let noisy = lower(
        semantic_spec()
            .with_comment("formatted for readability")
            .with_comment("moved comment block")
            .with_formatting_profile("two-space-indent")
            .with_parser_local_id("parser-node-17")
            .with_diagnostic_label("save button readiness failed")
            .with_renderer_label("primary-action-button"),
    );

    assert_eq!(baseline.identity(), noisy.identity());
    assert_eq!(baseline.digest_projection(), noisy.digest_projection());
    assert_eq!(baseline.aspect_contract(), noisy.aspect_contract());
    assert_eq!(baseline.family_admission(), noisy.family_admission());
}

#[test]
fn structural_changes_only_change_structural_identity_and_artifact_lanes() {
    let baseline = lower(semantic_spec());
    let changed =
        lower(semantic_spec().with_structural_token(UiDslStructuralToken::new("slot:footer")));

    assert_ne!(
        baseline.digest_projection().structural(),
        changed.digest_projection().structural()
    );
    assert_ne!(
        baseline.digest_projection().identity(),
        changed.digest_projection().identity()
    );
    assert_ne!(
        baseline.digest_projection().artifact(),
        changed.digest_projection().artifact()
    );
    assert_eq!(
        baseline.digest_projection().family(),
        changed.digest_projection().family()
    );
    assert_eq!(
        baseline.digest_projection().aspect(),
        changed.digest_projection().aspect()
    );
    assert_eq!(
        baseline.digest_projection().posture(),
        changed.digest_projection().posture()
    );
    assert_eq!(
        baseline.digest_projection().support(),
        changed.digest_projection().support()
    );
}

#[test]
fn support_changes_localize_to_support_and_artifact_digest_lanes() {
    let baseline = lower(semantic_spec());
    let changed =
        lower(semantic_spec().with_support_token(UiDslSupportToken::new("support:preview-only")));

    assert_ne!(
        baseline.digest_projection().support(),
        changed.digest_projection().support()
    );
    assert_ne!(
        baseline.digest_projection().artifact(),
        changed.digest_projection().artifact()
    );
    assert_eq!(
        baseline.digest_projection().identity(),
        changed.digest_projection().identity()
    );
    assert_eq!(
        baseline.digest_projection().family(),
        changed.digest_projection().family()
    );
    assert_eq!(
        baseline.digest_projection().aspect(),
        changed.digest_projection().aspect()
    );
    assert_eq!(
        baseline.digest_projection().structural(),
        changed.digest_projection().structural()
    );
    assert_eq!(
        baseline.digest_projection().posture(),
        changed.digest_projection().posture()
    );
    assert_eq!(
        baseline.structural_semantics(),
        changed.structural_semantics()
    );
    assert_eq!(baseline.structural_handoff(), changed.structural_handoff());
}

#[test]
fn equivalent_authored_aspect_spellings_converge_to_equivalent_contracts() {
    let baseline = lower(
        semantic_spec()
            .with_published_aspect(UiDslAspectName::new("Content.Text"))
            .with_consumed_aspect(UiDslAspectName::new(" Interaction.Operability ")),
    );
    let equivalent = lower(
        semantic_spec()
            .with_published_aspect(UiDslAspectName::new(" content.text "))
            .with_consumed_aspect(UiDslAspectName::new("interaction.operability")),
    );

    assert_eq!(baseline.aspect_contract(), equivalent.aspect_contract());
    assert_eq!(
        baseline.digest_projection().aspect(),
        equivalent.digest_projection().aspect()
    );
    assert_eq!(
        baseline.structural_semantics(),
        equivalent.structural_semantics()
    );
    assert_eq!(
        baseline.structural_handoff(),
        equivalent.structural_handoff()
    );
}

#[test]
fn aspect_coverage_report_explains_published_and_consumed_contracts() {
    let artifact = lower(semantic_spec());
    let coverage = artifact
        .aspect_coverage_report()
        .expect("supported aspects should admit");

    assert_eq!(
        coverage.published()[0].semantic_slice(),
        UiAspectSemanticSlice::AppearanceBackground
    );
    assert_eq!(
        coverage.published()[1].semantic_slice(),
        UiAspectSemanticSlice::ContentText
    );
    assert_eq!(
        coverage.consumed()[0].semantic_slice(),
        UiAspectSemanticSlice::InteractionOperability
    );
}

#[test]
fn unsupported_aspect_claims_deny_through_typed_aspect_admission() {
    let artifact =
        lower(semantic_spec().with_published_aspect(UiDslAspectName::new("appearance.border")));
    let expected_denial = UiAspectContractAdmissionDenial::UnsupportedAspectSemanticSlice {
        family: crate::declaration::UiAspectFamily::Appearance,
        canonical_label: "appearance.border".to_owned(),
    };

    assert_eq!(
        artifact.aspect_contract_admission(),
        &UiAspectContractAdmission::Denied(expected_denial.clone())
    );
    assert_eq!(artifact.aspect_coverage_report(), Err(&expected_denial));
}

#[test]
fn structural_families_admit_attached_query_binding_without_becoming_standalone_query_binding() {
    let artifact = lower(
        semantic_spec().with_posture_token(UiDslPostureToken::new("query-binding:attached:view")),
    );

    match artifact.family().expect("control declaration should admit") {
        UiDeclarationFamily::Control(_) => {}
        other => panic!("expected control family, got {other:?}"),
    }
    assert_eq!(
        artifact
            .declared_posture()
            .expect("control declaration should admit posture")
            .query_binding()
            .applicability(),
        crate::declaration::UiDeclaredPostureApplicability::Optional
    );
}

#[test]
fn standalone_query_binding_family_is_distinct_from_attached_query_binding_role() {
    let standalone = lower(
        UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.query.selection"),
            UiDslSemanticFamily::QueryBinding,
            UiDslSourceProvenance::file_authored("app/workflow_editor.wui", 1),
        )
        .with_posture_token(UiDslPostureToken::new("query-binding:standalone")),
    );
    let attached = lower(
        semantic_spec().with_posture_token(UiDslPostureToken::new("query-binding:attached:view")),
    );

    match standalone
        .family()
        .expect("standalone query-binding should admit")
    {
        UiDeclarationFamily::QueryBinding(binding) => {
            assert!(binding.is_standalone_family());
        }
        other => panic!("expected query-binding family, got {other:?}"),
    }

    match attached
        .family()
        .expect("attached query-binding role should admit")
    {
        UiDeclarationFamily::Control(_) => {}
        other => panic!("expected control family, got {other:?}"),
    }
    assert_eq!(
        attached
            .declared_posture()
            .expect("attached control declaration should admit posture")
            .query_binding()
            .admitted(),
        Some(&crate::declaration::UiDeclaredQueryBindingPosture::AttachedViewBinding)
    );
}

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
                observed: vec!["control:save".to_owned(), "control:alternate".to_owned()],
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
