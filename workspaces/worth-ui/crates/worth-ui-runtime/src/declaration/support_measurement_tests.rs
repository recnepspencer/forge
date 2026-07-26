use crate::facade::WorthUiRustAuthoredDeclarationFixture;
use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken,
};
use worth_ui_host_contract::WorthUiHostCapability;

use crate::declaration::artifact::ui_declaration_lowering::UiDeclarationLowering;
use crate::declaration::{
    UiDeclarationSupportRowSchemaKind, UiDeclaredMeasurementBasisSource,
    UiDeclaredMeasurementConstraintModifier, UiDeclaredMeasurementEvidenceRequirement,
    UiDeclaredMeasurementMode, UiDeclaredMeasurementOwnershipPosture,
    UiDeclaredMeasurementPolicyPosture,
};

fn lower(spec: UiDslSemanticArtifactSpec) -> crate::declaration::UiDeclarationArtifact {
    let package = WorthUiRustAuthoredDeclarationFixture::named(
        "worth-ui.runtime.declaration-support-measurement",
    );
    UiDeclarationLowering::lower(package.admit_semantic_artifact(spec))
}

fn control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.save"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/declaration_support.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:save"))
    .with_structural_token(UiDslStructuralToken::new("operator:stack"))
    .with_posture_token(UiDslPostureToken::new("query-binding:attached:view"))
    .with_posture_token(UiDslPostureToken::new("service:portal"))
    .with_posture_token(UiDslPostureToken::new("touch:press"))
    .with_posture_token(UiDslPostureToken::new("measurement:mode:hug-height"))
    .with_posture_token(UiDslPostureToken::new("measurement:constraint:bounded"))
    .with_posture_token(UiDslPostureToken::new("measurement:scroll-owned"))
    .with_posture_token(UiDslPostureToken::new(
        "measurement:evidence:font-metrics-required",
    ))
    .with_posture_token(UiDslPostureToken::new("host-capability:text-input"))
    .with_posture_token(UiDslPostureToken::new("host-capability:ime"))
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

fn expected_portal_measurement_policy() -> UiDeclaredMeasurementPolicyPosture {
    UiDeclaredMeasurementPolicyPosture::new(
        None,
        None,
        Some(UiDeclaredMeasurementBasisSource::PortalAnchor),
        Some(UiDeclaredMeasurementOwnershipPosture::PortalAnchorBasisRequired),
        vec![UiDeclaredMeasurementEvidenceRequirement::PortalAnchorMetrics],
    )
    .expect("test posture should contain portal measurement meaning")
}

#[test]
fn support_snapshot_projects_schema_limited_control_rows() {
    let artifact = lower(control_spec());
    let snapshot = artifact
        .support_snapshot()
        .expect("control declaration should derive support snapshot");

    assert_eq!(snapshot.rows().len(), 5);
    assert_eq!(
        snapshot
            .row(UiDeclarationSupportRowSchemaKind::QueryBinding)
            .expect("query-binding row should exist")
            .declared_query_binding_posture(),
        Some(&crate::declaration::UiDeclaredQueryBindingPosture::AttachedViewBinding),
    );
    assert_eq!(
        snapshot
            .row(UiDeclarationSupportRowSchemaKind::ServiceUsage)
            .expect("service row should exist")
            .declared_service_usage_posture(),
        Some(&crate::declaration::UiDeclaredServiceUsagePosture::Portal),
    );
    assert_eq!(
        snapshot
            .row(UiDeclarationSupportRowSchemaKind::TouchMeaning)
            .expect("touch row should exist")
            .declared_touch_meaning_posture(),
        Some(&crate::declaration::UiDeclaredTouchMeaningPosture::Press),
    );
    assert_eq!(
        snapshot
            .row(UiDeclarationSupportRowSchemaKind::MeasurementPolicy)
            .expect("measurement row should exist")
            .declared_measurement_policy_posture(),
        Some(&expected_measurement_policy()),
    );
    assert_eq!(
        snapshot
            .row(UiDeclarationSupportRowSchemaKind::HostCapability)
            .expect("host row should exist")
            .declared_host_capability_posture()
            .map(|posture| posture.required_capabilities()),
        Some(&[WorthUiHostCapability::Ime, WorthUiHostCapability::TextInput,][..]),
    );
}

#[test]
fn support_snapshot_preserves_portal_measurement_policy_projection() {
    let artifact = lower(
        UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.inspector.portal"),
            UiDslSemanticFamily::Control,
            UiDslSourceProvenance::file_authored("app/declaration_support_portal.wui", 0),
        )
        .with_structural_token(UiDslStructuralToken::new("control:portal"))
        .with_structural_token(UiDslStructuralToken::new("operator:portal-anchor"))
        .with_posture_token(UiDslPostureToken::new("measurement:portal-anchored")),
    );
    let snapshot = artifact
        .support_snapshot()
        .expect("portal declaration should derive support snapshot");

    assert_eq!(
        snapshot
            .row(UiDeclarationSupportRowSchemaKind::MeasurementPolicy)
            .expect("measurement row should exist")
            .declared_measurement_policy_posture(),
        Some(&expected_portal_measurement_policy()),
    );
}
