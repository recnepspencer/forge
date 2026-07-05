use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken, WorthUiDslPackage,
};

use crate::declaration::artifact::ui_declaration_lowering::UiDeclarationLowering;
use crate::declaration::{
    UiDeclarationFamilyKind, UiDeclaredMeasurementBasisSource,
    UiDeclaredMeasurementConstraintModifier, UiDeclaredMeasurementEvidenceRequirement,
    UiDeclaredMeasurementMode, UiDeclaredMeasurementOwnershipPosture,
    UiDeclaredMeasurementPolicyPosture, UiDeclaredPostureAdmissionDenial,
    UiDeclaredPostureLaneKind,
};

fn lower(spec: UiDslSemanticArtifactSpec) -> crate::declaration::UiDeclarationArtifact {
    let package = WorthUiDslPackage::named("worth-ui.runtime.declared-measurement.tests");
    UiDeclarationLowering::lower(package.admit_semantic_artifact(spec))
}

fn expected_measurement_policy() -> UiDeclaredMeasurementPolicyPosture {
    UiDeclaredMeasurementPolicyPosture::new(
        Some(UiDeclaredMeasurementMode::HugHeight),
        Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        Some(UiDeclaredMeasurementBasisSource::ScrollViewport),
        Some(UiDeclaredMeasurementOwnershipPosture::ScrollContainerBasis),
        vec![
            UiDeclaredMeasurementEvidenceRequirement::HostFontMetrics,
            UiDeclaredMeasurementEvidenceRequirement::ScrollContentExtent,
        ],
    )
    .expect("test posture should contain measurement meaning")
}

#[test]
fn additive_measurement_claims_admit_as_one_declared_posture_lane() {
    let artifact = lower(
        UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.inspector.body"),
            UiDslSemanticFamily::Control,
            UiDslSourceProvenance::file_authored("app/measurement.wui", 0),
        )
        .with_structural_token(UiDslStructuralToken::new("control:body"))
        .with_posture_token(UiDslPostureToken::new("measurement:mode:hug-height"))
        .with_posture_token(UiDslPostureToken::new("measurement:constraint:bounded"))
        .with_posture_token(UiDslPostureToken::new("measurement:scroll-owned"))
        .with_posture_token(UiDslPostureToken::new(
            "measurement:evidence:font-metrics-required",
        )),
    );
    let measurement_policy = artifact
        .declared_posture()
        .expect("control declaration should admit additive measurement claims")
        .measurement_policy()
        .admitted();

    assert_eq!(measurement_policy, Some(&expected_measurement_policy()));
}

#[test]
fn equivalent_measurement_claim_order_converges_to_same_posture() {
    let left = lower(
        UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.inspector.left"),
            UiDslSemanticFamily::Control,
            UiDslSourceProvenance::file_authored("app/measurement_equivalence.wui", 0),
        )
        .with_structural_token(UiDslStructuralToken::new("control:left"))
        .with_posture_token(UiDslPostureToken::new("measurement:scroll-owned"))
        .with_posture_token(UiDslPostureToken::new(
            "measurement:evidence:font-metrics-required",
        ))
        .with_posture_token(UiDslPostureToken::new("measurement:constraint:bounded"))
        .with_posture_token(UiDslPostureToken::new("measurement:hug-height")),
    );
    let right = lower(
        UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.inspector.right"),
            UiDslSemanticFamily::Control,
            UiDslSourceProvenance::file_authored("app/measurement_equivalence.wui", 1),
        )
        .with_structural_token(UiDslStructuralToken::new("control:right"))
        .with_posture_token(UiDslPostureToken::new("measurement:mode:hug-height"))
        .with_posture_token(UiDslPostureToken::new("measurement:constraint:bounded"))
        .with_posture_token(UiDslPostureToken::new("measurement:basis:scroll-viewport"))
        .with_posture_token(UiDslPostureToken::new(
            "measurement:ownership:scroll-container-basis",
        ))
        .with_posture_token(UiDslPostureToken::new(
            "measurement:evidence:scroll-content-extent-required",
        ))
        .with_posture_token(UiDslPostureToken::new("measurement:font-metrics-required")),
    );

    assert_eq!(
        left.declared_posture()
            .expect("left declaration should admit")
            .measurement_policy()
            .admitted(),
        right
            .declared_posture()
            .expect("right declaration should admit")
            .measurement_policy()
            .admitted(),
    );
}

#[test]
fn unsupported_measurement_claims_deny_for_non_measurement_families() {
    let query_binding = lower(
        UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.query.measurement"),
            UiDslSemanticFamily::QueryBinding,
            UiDslSourceProvenance::file_authored("app/measurement_denials.wui", 0),
        )
        .with_posture_token(UiDslPostureToken::new("query-binding:standalone"))
        .with_posture_token(UiDslPostureToken::new("measurement:scroll-owned")),
    );
    let intent = lower(
        UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.intent.measurement"),
            UiDslSemanticFamily::Intent,
            UiDslSourceProvenance::file_authored("app/measurement_denials.wui", 1),
        )
        .with_posture_token(UiDslPostureToken::new("intent:standalone"))
        .with_posture_token(UiDslPostureToken::new("measurement:font-metrics-required")),
    );

    assert_eq!(
        query_binding.declared_posture(),
        Err(
            &UiDeclaredPostureAdmissionDenial::LaneNotApplicableForFamily {
                family: UiDeclarationFamilyKind::QueryBinding,
                lane: UiDeclaredPostureLaneKind::MeasurementPolicy,
                observed: vec!["measurement:scroll-owned".to_owned()],
            }
        ),
    );
    assert_eq!(
        intent.declared_posture(),
        Err(
            &UiDeclaredPostureAdmissionDenial::LaneNotApplicableForFamily {
                family: UiDeclarationFamilyKind::Intent,
                lane: UiDeclaredPostureLaneKind::MeasurementPolicy,
                observed: vec!["measurement:font-metrics-required".to_owned()],
            }
        ),
    );
}

#[test]
fn contradictory_same_axis_measurement_claims_deny_structurally() {
    let duplicate_mode = lower(
        UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.control.duplicate_mode"),
            UiDslSemanticFamily::Control,
            UiDslSourceProvenance::file_authored("app/measurement_denials.wui", 2),
        )
        .with_structural_token(UiDslStructuralToken::new("control:duplicate-mode"))
        .with_posture_token(UiDslPostureToken::new("measurement:mode:hug-height"))
        .with_posture_token(UiDslPostureToken::new("measurement:mode:hug-height")),
    );
    let duplicate_evidence = lower(
        UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.control.duplicate_evidence"),
            UiDslSemanticFamily::Control,
            UiDslSourceProvenance::file_authored("app/measurement_denials.wui", 3),
        )
        .with_structural_token(UiDslStructuralToken::new("control:duplicate-evidence"))
        .with_posture_token(UiDslPostureToken::new("measurement:font-metrics-required"))
        .with_posture_token(UiDslPostureToken::new("measurement:font-metrics-required")),
    );

    assert_eq!(
        duplicate_mode.declared_posture(),
        Err(&UiDeclaredPostureAdmissionDenial::ContradictoryLaneClaims {
            family: UiDeclarationFamilyKind::Control,
            lane: UiDeclaredPostureLaneKind::MeasurementPolicy,
            observed: vec![
                "measurement:mode:hug-height".to_owned(),
                "measurement:mode:hug-height".to_owned(),
            ],
        }),
    );
    assert_eq!(
        duplicate_evidence.declared_posture(),
        Err(&UiDeclaredPostureAdmissionDenial::ContradictoryLaneClaims {
            family: UiDeclarationFamilyKind::Control,
            lane: UiDeclaredPostureLaneKind::MeasurementPolicy,
            observed: vec![
                "measurement:font-metrics-required".to_owned(),
                "measurement:font-metrics-required".to_owned(),
            ],
        }),
    );
}

#[test]
fn roadmap_shorthand_lowers_to_explicit_planning_axes() {
    let shorthand = lower(
        UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.control.scroll_owned"),
            UiDslSemanticFamily::Control,
            UiDslSourceProvenance::file_authored("app/measurement_shorthand.wui", 0),
        )
        .with_structural_token(UiDslStructuralToken::new("control:scroll-owned"))
        .with_posture_token(UiDslPostureToken::new("measurement:scroll-owned")),
    );
    let explicit = lower(
        UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.control.scroll_explicit"),
            UiDslSemanticFamily::Control,
            UiDslSourceProvenance::file_authored("app/measurement_shorthand.wui", 1),
        )
        .with_structural_token(UiDslStructuralToken::new("control:scroll-explicit"))
        .with_posture_token(UiDslPostureToken::new("measurement:basis:scroll-viewport"))
        .with_posture_token(UiDslPostureToken::new(
            "measurement:ownership:scroll-container-basis",
        ))
        .with_posture_token(UiDslPostureToken::new(
            "measurement:evidence:scroll-content-extent-required",
        )),
    );

    assert_eq!(
        shorthand
            .declared_posture()
            .expect("measurement shorthand should admit")
            .measurement_policy()
            .admitted(),
        explicit
            .declared_posture()
            .expect("explicit measurement axes should admit")
            .measurement_policy()
            .admitted(),
    );
}

#[test]
fn portal_shorthand_lowers_to_explicit_planning_axes() {
    let shorthand = lower(
        UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.control.portal_anchor"),
            UiDslSemanticFamily::Control,
            UiDslSourceProvenance::file_authored("app/measurement_shorthand.wui", 2),
        )
        .with_structural_token(UiDslStructuralToken::new("control:portal-anchor"))
        .with_posture_token(UiDslPostureToken::new("measurement:portal-anchored")),
    );
    let explicit = lower(
        UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.control.portal_explicit"),
            UiDslSemanticFamily::Control,
            UiDslSourceProvenance::file_authored("app/measurement_shorthand.wui", 3),
        )
        .with_structural_token(UiDslStructuralToken::new("control:portal-explicit"))
        .with_posture_token(UiDslPostureToken::new("measurement:basis:portal-anchor"))
        .with_posture_token(UiDslPostureToken::new(
            "measurement:ownership:portal-anchor-basis-required",
        ))
        .with_posture_token(UiDslPostureToken::new(
            "measurement:evidence:portal-anchor-metrics-required",
        )),
    );

    assert_eq!(
        shorthand
            .declared_posture()
            .expect("portal shorthand should admit")
            .measurement_policy()
            .admitted(),
        explicit
            .declared_posture()
            .expect("explicit portal axes should admit")
            .measurement_policy()
            .admitted(),
    );
}

#[test]
fn impossible_measurement_axis_combinations_deny_before_planning() {
    let artifact = lower(
        UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.control.impossible_measurement"),
            UiDslSemanticFamily::Control,
            UiDslSourceProvenance::file_authored("app/measurement_denials.wui", 4),
        )
        .with_structural_token(UiDslStructuralToken::new("control:impossible-measurement"))
        .with_posture_token(UiDslPostureToken::new("measurement:basis:portal-anchor"))
        .with_posture_token(UiDslPostureToken::new(
            "measurement:ownership:scroll-container-basis",
        )),
    );

    assert_eq!(
        artifact.declared_posture(),
        Err(
            &UiDeclaredPostureAdmissionDenial::ImpossibleLaneCombination {
                family: UiDeclarationFamilyKind::Control,
                lane: UiDeclaredPostureLaneKind::MeasurementPolicy,
                observed: vec![
                    "measurement:basis:portal-anchor".to_owned(),
                    "measurement:ownership:scroll-container-basis".to_owned(),
                ],
                reason: "measurement ownership posture requires a matching basis source",
            }
        ),
    );
}

#[test]
fn portal_shorthand_with_conflicting_basis_denies_before_planning() {
    let artifact = lower(
        UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.control.portal_conflict"),
            UiDslSemanticFamily::Control,
            UiDslSourceProvenance::file_authored("app/measurement_denials.wui", 5),
        )
        .with_structural_token(UiDslStructuralToken::new("control:portal-conflict"))
        .with_posture_token(UiDslPostureToken::new("measurement:portal-anchored"))
        .with_posture_token(UiDslPostureToken::new("measurement:basis:scroll-viewport")),
    );

    assert_eq!(
        artifact.declared_posture(),
        Err(&UiDeclaredPostureAdmissionDenial::ContradictoryLaneClaims {
            family: UiDeclarationFamilyKind::Control,
            lane: UiDeclaredPostureLaneKind::MeasurementPolicy,
            observed: vec![
                "measurement:portal-anchored".to_owned(),
                "measurement:basis:scroll-viewport".to_owned(),
            ],
        }),
    );
}
