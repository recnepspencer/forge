use worth_ui::facade::app::{WorthUi, WorthUiApp};
use worth_ui::facade::declaration::{
    UiDeclarationArtifact, UiDeclarationClosedSemanticLane, UiDeclarationCloseoutGuarantee,
    UiDeclarationCloseoutNonGoal, UiDeclarationCloseoutReport, UiDeclarationFamilyKind,
};
use worth_ui_certification::{
    WorthUiCertificationBuilderExt, WorthUiRustAuthoredDeclarationFixture,
};
use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken,
};

#[test]
fn bootstrap_app_exposes_milestone32_closeout_report() {
    let app = WorthUi::app()
        .freeze()
        .expect("application preparation should succeed");
    let report = app.declaration_closeout_report();

    assert_eq!(report, UiDeclarationCloseoutReport::milestone32());
    assert_eq!(
        report.admitted_families(),
        &[
            UiDeclarationFamilyKind::Page,
            UiDeclarationFamilyKind::PageSet,
            UiDeclarationFamilyKind::Region,
            UiDeclarationFamilyKind::Mosaic,
            UiDeclarationFamilyKind::LocalComposition,
            UiDeclarationFamilyKind::Control,
            UiDeclarationFamilyKind::QueryBinding,
            UiDeclarationFamilyKind::Intent,
            UiDeclarationFamilyKind::DiagnosticSurface,
        ],
    );
}

#[test]
fn caller_authored_app_exposes_same_closeout_contract() {
    let app = WorthUi::app()
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named("worth-ui.certification.closeout")
                .with_semantic_artifact_spec(control_closeout_spec()),
        )
        .freeze()
        .expect("application preparation should succeed");
    let _artifact = artifact_from_file_provenance(&app, "app/closeout.wui", 0);
    let report = app.declaration_closeout_report();

    assert_eq!(
        report.admitted_families(),
        &[
            UiDeclarationFamilyKind::Page,
            UiDeclarationFamilyKind::PageSet,
            UiDeclarationFamilyKind::Region,
            UiDeclarationFamilyKind::Mosaic,
            UiDeclarationFamilyKind::LocalComposition,
            UiDeclarationFamilyKind::Control,
            UiDeclarationFamilyKind::QueryBinding,
            UiDeclarationFamilyKind::Intent,
            UiDeclarationFamilyKind::DiagnosticSurface,
        ],
    );
    assert_eq!(
        report.closed_semantic_lanes(),
        &[
            UiDeclarationClosedSemanticLane::Identity,
            UiDeclarationClosedSemanticLane::FamilyAuthority,
            UiDeclarationClosedSemanticLane::AspectContracts,
            UiDeclarationClosedSemanticLane::StructuralIntent,
            UiDeclarationClosedSemanticLane::QueryBindingPosture,
            UiDeclarationClosedSemanticLane::ServiceUsagePosture,
            UiDeclarationClosedSemanticLane::TouchMeaningPosture,
            UiDeclarationClosedSemanticLane::MeasurementPolicyPosture,
            UiDeclarationClosedSemanticLane::HostCapabilityPosture,
            UiDeclarationClosedSemanticLane::SupportSnapshot,
        ],
    );
    assert_eq!(
        report.guarantees(),
        &[
            UiDeclarationCloseoutGuarantee::LowersOnceFromSemanticDslAuthority,
            UiDeclarationCloseoutGuarantee::LaneSpecificDigestLocality,
            UiDeclarationCloseoutGuarantee::NoLaterSourceReopening,
            UiDeclarationCloseoutGuarantee::GraphHandoffConsumesCanonicalDeclarationAuthorityOnly,
        ],
    );
    assert_eq!(
        report.non_goals(),
        &[
            UiDeclarationCloseoutNonGoal::GraphTruth,
            UiDeclarationCloseoutNonGoal::GraphNodeIdentity,
            UiDeclarationCloseoutNonGoal::ParticipationTruth,
            UiDeclarationCloseoutNonGoal::MountedTruth,
            UiDeclarationCloseoutNonGoal::MeasuredTruth,
            UiDeclarationCloseoutNonGoal::RuntimeParticipationExecution,
        ],
    );
}

fn artifact_from_file_provenance<'a>(
    app: &'a WorthUiApp,
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

fn control_closeout_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.closeout.save"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/closeout.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:save"))
    .with_structural_token(UiDslStructuralToken::new("slot:footer"))
    .with_posture_token(UiDslPostureToken::new("query-binding:attached:view"))
    .with_posture_token(UiDslPostureToken::new("service:portal"))
}
