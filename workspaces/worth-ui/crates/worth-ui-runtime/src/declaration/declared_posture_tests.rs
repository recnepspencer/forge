use crate::facade::WorthUiRustAuthoredDeclarationFixture;
use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken,
};
use worth_ui_host_contract::WorthUiHostCapability;

use crate::declaration::artifact::ui_declaration_lowering::UiDeclarationLowering;
use crate::declaration::{
    UiDeclarationFamilyKind, UiDeclaredMeasurementMode, UiDeclaredMeasurementPolicyPosture,
    UiDeclaredPostureAdmission, UiDeclaredPostureAdmissionDenial, UiDeclaredPostureApplicability,
    UiDeclaredPostureLaneKind, UiDeclaredQueryBindingPosture, UiDeclaredServiceUsagePosture,
    UiDeclaredTouchMeaningPosture,
};

fn lower(spec: UiDslSemanticArtifactSpec) -> crate::declaration::UiDeclarationArtifact {
    let package =
        WorthUiRustAuthoredDeclarationFixture::named("worth-ui.runtime.declared-posture.tests");
    UiDeclarationLowering::lower(package.admit_semantic_artifact(spec))
}

fn assert_lane_applicability(
    artifact: &crate::declaration::UiDeclarationArtifact,
    expected: [UiDeclaredPostureApplicability; 5],
) {
    let posture = artifact
        .declared_posture()
        .expect("declaration should admit declared posture");

    assert_eq!(posture.query_binding().applicability(), expected[0]);
    assert_eq!(posture.service_usage().applicability(), expected[1]);
    assert_eq!(posture.touch_meaning().applicability(), expected[2]);
    assert_eq!(posture.measurement_policy().applicability(), expected[3]);
    assert_eq!(posture.host_capability().applicability(), expected[4]);
}

fn family_spec(family: UiDslSemanticFamily, declaration_index: usize) -> UiDslSemanticArtifactSpec {
    match family {
        UiDslSemanticFamily::Page => UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.page.root"),
            family,
            UiDslSourceProvenance::file_authored("app/family_matrix.wui", declaration_index),
        )
        .with_structural_token(UiDslStructuralToken::new("page:product-root")),
        UiDslSemanticFamily::PageSet => UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.page_set.shell"),
            family,
            UiDslSourceProvenance::file_authored("app/family_matrix.wui", declaration_index),
        )
        .with_structural_token(UiDslStructuralToken::new("page-set:shell")),
        UiDslSemanticFamily::Region => UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.region.sidebar"),
            family,
            UiDslSourceProvenance::file_authored("app/family_matrix.wui", declaration_index),
        )
        .with_structural_token(UiDslStructuralToken::new("region:sidebar")),
        UiDslSemanticFamily::Mosaic => UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.mosaic.workspace"),
            family,
            UiDslSourceProvenance::file_authored("app/family_matrix.wui", declaration_index),
        )
        .with_structural_token(UiDslStructuralToken::new("mosaic:workspace")),
        UiDslSemanticFamily::LocalComposition => UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.local_composition.inspector"),
            family,
            UiDslSourceProvenance::file_authored("app/family_matrix.wui", declaration_index),
        )
        .with_structural_token(UiDslStructuralToken::new("local-composition:inspector")),
        UiDslSemanticFamily::Control => UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.control.save"),
            family,
            UiDslSourceProvenance::file_authored("app/family_matrix.wui", declaration_index),
        )
        .with_structural_token(UiDslStructuralToken::new("control:save")),
        UiDslSemanticFamily::QueryBinding => UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.query.selection"),
            family,
            UiDslSourceProvenance::file_authored("app/family_matrix.wui", declaration_index),
        )
        .with_posture_token(UiDslPostureToken::new("query-binding:standalone")),
        UiDslSemanticFamily::Intent => UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.intent.selection"),
            family,
            UiDslSourceProvenance::file_authored("app/family_matrix.wui", declaration_index),
        )
        .with_posture_token(UiDslPostureToken::new("intent:standalone")),
        UiDslSemanticFamily::DiagnosticSurface => UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.diagnostic_surface.lint"),
            family,
            UiDslSourceProvenance::file_authored("app/family_matrix.wui", declaration_index),
        )
        .with_structural_token(UiDslStructuralToken::new("diagnostic-surface:lint")),
        UiDslSemanticFamily::RuntimeService => {
            panic!("runtime-service declarations use service handoff admission")
        }
    }
}

fn expected_measurement_policy(
    mode: UiDeclaredMeasurementMode,
) -> UiDeclaredMeasurementPolicyPosture {
    UiDeclaredMeasurementPolicyPosture::new(Some(mode), None, None, None, Vec::new())
        .expect("test posture should contain measurement meaning")
}

#[test]
fn declared_posture_projects_typed_control_intent_lanes() {
    let artifact = lower(
        UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.inspector.save"),
            UiDslSemanticFamily::Control,
            UiDslSourceProvenance::file_authored("app/workflow_editor.wui", 0),
        )
        .with_structural_token(UiDslStructuralToken::new("control:save"))
        .with_posture_token(UiDslPostureToken::new("query-binding:attached:view"))
        .with_posture_token(UiDslPostureToken::new("service:portal"))
        .with_posture_token(UiDslPostureToken::new("touch:press"))
        .with_posture_token(UiDslPostureToken::new("measurement:hug-height"))
        .with_posture_token(UiDslPostureToken::new("host-capability:text-input")),
    );
    let posture = artifact
        .declared_posture()
        .expect("control declaration should admit declared posture");

    assert_eq!(
        posture.query_binding().applicability(),
        UiDeclaredPostureApplicability::Optional
    );
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
    assert_eq!(
        posture.measurement_policy().admitted(),
        Some(&expected_measurement_policy(
            UiDeclaredMeasurementMode::HugHeight
        ))
    );
    assert_eq!(
        posture
            .host_capability()
            .admitted()
            .map(|posture| posture.required_capabilities()),
        Some(&[WorthUiHostCapability::TextInput][..])
    );
}

#[test]
fn declared_posture_classifies_every_lane_for_admitted_families() {
    let cases = [
        (
            UiDslSemanticFamily::Page,
            [
                UiDeclaredPostureApplicability::Optional,
                UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted,
                UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted,
                UiDeclaredPostureApplicability::Optional,
                UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted,
            ],
        ),
        (
            UiDslSemanticFamily::PageSet,
            [
                UiDeclaredPostureApplicability::Optional,
                UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted,
                UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted,
                UiDeclaredPostureApplicability::Optional,
                UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted,
            ],
        ),
        (
            UiDslSemanticFamily::Region,
            [
                UiDeclaredPostureApplicability::Optional,
                UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted,
                UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted,
                UiDeclaredPostureApplicability::Optional,
                UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted,
            ],
        ),
        (
            UiDslSemanticFamily::Mosaic,
            [
                UiDeclaredPostureApplicability::Optional,
                UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted,
                UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted,
                UiDeclaredPostureApplicability::Optional,
                UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted,
            ],
        ),
        (
            UiDslSemanticFamily::LocalComposition,
            [
                UiDeclaredPostureApplicability::Optional,
                UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted,
                UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted,
                UiDeclaredPostureApplicability::Optional,
                UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted,
            ],
        ),
        (
            UiDslSemanticFamily::Control,
            [
                UiDeclaredPostureApplicability::Optional,
                UiDeclaredPostureApplicability::Optional,
                UiDeclaredPostureApplicability::Optional,
                UiDeclaredPostureApplicability::Optional,
                UiDeclaredPostureApplicability::Optional,
            ],
        ),
        (
            UiDslSemanticFamily::QueryBinding,
            [
                UiDeclaredPostureApplicability::Required,
                UiDeclaredPostureApplicability::NotApplicable,
                UiDeclaredPostureApplicability::NotApplicable,
                UiDeclaredPostureApplicability::NotApplicable,
                UiDeclaredPostureApplicability::NotApplicable,
            ],
        ),
        (
            UiDslSemanticFamily::Intent,
            [
                UiDeclaredPostureApplicability::NotApplicable,
                UiDeclaredPostureApplicability::NotApplicable,
                UiDeclaredPostureApplicability::NotApplicable,
                UiDeclaredPostureApplicability::NotApplicable,
                UiDeclaredPostureApplicability::NotApplicable,
            ],
        ),
        (
            UiDslSemanticFamily::DiagnosticSurface,
            [
                UiDeclaredPostureApplicability::NotApplicable,
                UiDeclaredPostureApplicability::DiagnosticOnly,
                UiDeclaredPostureApplicability::DiagnosticOnly,
                UiDeclaredPostureApplicability::Optional,
                UiDeclaredPostureApplicability::DiagnosticOnly,
            ],
        ),
    ];

    for (declaration_index, (family, expected)) in cases.into_iter().enumerate() {
        let artifact = lower(family_spec(family, declaration_index));
        assert_lane_applicability(&artifact, expected);
    }

    let query_binding = lower(family_spec(UiDslSemanticFamily::QueryBinding, 100));
    assert_eq!(
        query_binding
            .declared_posture()
            .expect("query-binding should admit")
            .query_binding()
            .admitted(),
        Some(&UiDeclaredQueryBindingPosture::StandaloneBinding)
    );
}

#[test]
fn contradictory_service_usage_denies_through_typed_posture_admission() {
    let artifact = lower(
        UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.inspector.save"),
            UiDslSemanticFamily::Control,
            UiDslSourceProvenance::file_authored("app/denials.wui", 0),
        )
        .with_structural_token(UiDslStructuralToken::new("control:save"))
        .with_posture_token(UiDslPostureToken::new("service:portal"))
        .with_posture_token(UiDslPostureToken::new("service:scroll")),
    );

    assert_eq!(
        artifact.declared_posture_admission(),
        &UiDeclaredPostureAdmission::Denied(
            UiDeclaredPostureAdmissionDenial::ContradictoryLaneClaims {
                family: UiDeclarationFamilyKind::Control,
                lane: UiDeclaredPostureLaneKind::ServiceUsage,
                observed: vec!["service:portal".to_owned(), "service:scroll".to_owned()],
            },
        ),
    );
}

#[test]
fn impossible_touch_meaning_denies_through_typed_posture_admission() {
    let artifact = lower(
        UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.inspector.save"),
            UiDslSemanticFamily::Control,
            UiDslSourceProvenance::file_authored("app/denials.wui", 1),
        )
        .with_structural_token(UiDslStructuralToken::new("control:save"))
        .with_posture_token(UiDslPostureToken::new("touch:swim")),
    );

    assert_eq!(
        artifact.declared_posture(),
        Err(&UiDeclaredPostureAdmissionDenial::InvalidLaneClaim {
            family: UiDeclarationFamilyKind::Control,
            lane: UiDeclaredPostureLaneKind::TouchMeaning,
            observed: vec!["touch:swim".to_owned()],
        }),
    );
}

#[test]
fn invalid_measurement_policy_denies_through_typed_posture_admission() {
    let artifact = lower(
        UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.inspector.save"),
            UiDslSemanticFamily::Control,
            UiDslSourceProvenance::file_authored("app/denials.wui", 2),
        )
        .with_structural_token(UiDslStructuralToken::new("control:save"))
        .with_posture_token(UiDslPostureToken::new("measurement:runtime-observed")),
    );

    assert_eq!(
        artifact.declared_posture(),
        Err(&UiDeclaredPostureAdmissionDenial::InvalidLaneClaim {
            family: UiDeclarationFamilyKind::Control,
            lane: UiDeclaredPostureLaneKind::MeasurementPolicy,
            observed: vec!["measurement:runtime-observed".to_owned()],
        }),
    );
}

#[test]
fn not_yet_admitted_host_capability_usage_denies_for_page_family() {
    let artifact = lower(
        UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.page.root"),
            UiDslSemanticFamily::Page,
            UiDslSourceProvenance::file_authored("app/denials.wui", 3),
        )
        .with_structural_token(UiDslStructuralToken::new("page:product-root"))
        .with_posture_token(UiDslPostureToken::new("host-capability:ime")),
    );

    assert_eq!(
        artifact.declared_posture(),
        Err(
            &UiDeclaredPostureAdmissionDenial::LaneArchitecturallyOwnedButNotYetAdmitted {
                family: UiDeclarationFamilyKind::Page,
                lane: UiDeclaredPostureLaneKind::HostCapability,
                observed: vec!["host-capability:ime".to_owned()],
            },
        ),
    );
}

#[test]
fn additive_host_capability_requirements_admit_as_one_declared_posture_lane() {
    let artifact = lower(
        UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.inspector.name"),
            UiDslSemanticFamily::Control,
            UiDslSourceProvenance::file_authored("app/host_capabilities.wui", 0),
        )
        .with_structural_token(UiDslStructuralToken::new("control:name"))
        .with_posture_token(UiDslPostureToken::new("host-capability:text-input"))
        .with_posture_token(UiDslPostureToken::new("host-capability:ime")),
    );
    let host_capability = artifact
        .declared_posture()
        .expect("control declaration should admit additive host requirements")
        .host_capability()
        .admitted()
        .expect("host capability posture should be present");

    assert_eq!(
        host_capability.required_capabilities(),
        &[WorthUiHostCapability::Ime, WorthUiHostCapability::TextInput,]
    );
    assert!(host_capability.requires(WorthUiHostCapability::TextInput));
    assert!(host_capability.requires(WorthUiHostCapability::Ime));
}
