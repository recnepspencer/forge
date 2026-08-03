use crate::facade::WorthUiRustAuthoredDeclarationFixture;
use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken,
};
use worth_ui_inspection::{
    UiInspectionQuery, UiInspectionScope, UiInspectionSupportStatus, UiInspectionTarget,
};

use crate::declaration::{
    UiDeclarationSupportRowSchemaKind, UiDeclaredMeasurementBasisSource,
    UiDeclaredMeasurementConstraintModifier, UiDeclaredMeasurementEvidenceRequirement,
    UiDeclaredMeasurementMode, UiDeclaredMeasurementOwnershipPosture,
    UiDeclaredMeasurementPolicyPosture,
};

#[test]
fn app_freeze_preserves_declared_measurement_posture_through_public_surfaces() {
    assert_app_freeze_measurement_posture(
        control_measurement_spec(
            "workflow_editor.control.save",
            "app/declaration_support_matrix.wui",
            0,
        )
        .with_posture_token(UiDslPostureToken::new("measurement:mode:hug-height"))
        .with_posture_token(UiDslPostureToken::new("measurement:constraint:bounded"))
        .with_posture_token(UiDslPostureToken::new("measurement:scroll-owned"))
        .with_posture_token(UiDslPostureToken::new(
            "measurement:evidence:font-metrics-required",
        )),
        "app/declaration_support_matrix.wui",
        0,
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
        .expect("test posture should contain scroll measurement meaning"),
    );
}

#[test]
fn app_freeze_preserves_portal_measurement_posture_through_public_surfaces() {
    assert_app_freeze_measurement_posture(
        control_measurement_spec(
            "workflow_editor.control.portal",
            "app/declaration_support_portal_matrix.wui",
            0,
        )
        .with_posture_token(UiDslPostureToken::new("measurement:portal-anchored")),
        "app/declaration_support_portal_matrix.wui",
        0,
        UiDeclaredMeasurementPolicyPosture::new(
            None,
            None,
            Some(UiDeclaredMeasurementBasisSource::PortalAnchor),
            Some(UiDeclaredMeasurementOwnershipPosture::PortalAnchorBasisRequired),
            vec![UiDeclaredMeasurementEvidenceRequirement::PortalAnchorMetrics],
        )
        .expect("test posture should contain portal measurement meaning"),
    );
}

fn assert_app_freeze_measurement_posture(
    spec: UiDslSemanticArtifactSpec,
    module_path: &str,
    declaration_index: usize,
    expected: UiDeclaredMeasurementPolicyPosture,
) {
    let app = crate::facade::WorthUi::app()
        .with_change_profile(crate::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.runtime.declaration-support.app",
            )
            .with_semantic_artifact_spec(spec),
        )
        .freeze()
        .expect("application preparation should succeed");

    let query = UiInspectionQuery::new(
        UiInspectionTarget::declared_surface(module_path, declaration_index),
        UiInspectionScope::Measurement,
    );
    let measurement_report = app.inspection_support_report_for(&query);
    assert_eq!(
        measurement_report.status(),
        UiInspectionSupportStatus::Supported
    );
    let receipt = app.inspect(query);
    assert_eq!(
        receipt.support_report().map(|report| report.status()),
        Some(UiInspectionSupportStatus::Supported)
    );
    assert_eq!(
        app.declaration_artifacts()
            .iter()
            .find(|artifact| {
                let provenance = artifact.provenance().source_provenance();
                provenance.module_path() == module_path
                    && provenance.declaration_index() == declaration_index
            })
            .and_then(|artifact| artifact.support_snapshot().ok())
            .and_then(|snapshot| snapshot.row(UiDeclarationSupportRowSchemaKind::MeasurementPolicy))
            .and_then(|row| row.declared_measurement_policy_posture()),
        Some(&expected),
    );
}

fn control_measurement_spec(
    semantic_key: &str,
    module_path: &str,
    declaration_index: usize,
) -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new(semantic_key),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored(module_path, declaration_index),
    )
    .with_structural_token(UiDslStructuralToken::new("control:save"))
}
