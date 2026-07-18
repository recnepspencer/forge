use worth_ui::facade::admission::{
    UiAdmissionFamily, UiAdmissionTarget, UiAdmissionWorld, UiSupportPosture, UiSupportReason,
};
use worth_ui::facade::app::WorthUi;
use worth_ui::facade::declaration::{
    UiDeclarationArtifact, UiDeclarationSupportMilestoneExpectation,
};
use worth_ui::facade::graph::{UiGraphSessionLabel, UiGraphWorldProfile};
use worth_ui::facade::inspection::UiInspectionSupportPosture;
use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken, WorthUiDslPackage,
};

#[test]
fn support_snapshot_keeps_supported_unsupported_deferred_and_wrong_world_separate() {
    let app = WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.admission.support")
                .with_semantic_artifact_spec(supported_control_spec())
                .with_semantic_artifact_spec(deferred_diagnostic_surface_spec())
                .with_semantic_artifact_spec(diagnostic_only_surface_spec()),
        )
        .freeze()
        .expect("application preparation should succeed");
    let foreign_app = WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.admission.support.foreign")
                .with_semantic_artifact_spec(foreign_control_spec()),
        )
        .freeze()
        .expect("application preparation should succeed");
    let boundary = app.admission();

    let supported_control = artifact_from_file_provenance(&app, "app/admission_support.wui", 0);
    let deferred_surface = artifact_from_file_provenance(&app, "app/admission_support.wui", 1);
    let diagnostic_surface = artifact_from_file_provenance(&app, "app/admission_support.wui", 2);
    let foreign_control =
        artifact_from_file_provenance(&foreign_app, "app/admission_support_foreign.wui", 0);

    let supported_snapshot = boundary.support_snapshot(&UiAdmissionTarget::graph_node(
        graph_node_identity(&app, supported_control),
        UiAdmissionWorld::authoritative(),
    ));
    let deferred_snapshot = boundary.support_snapshot(&UiAdmissionTarget::graph_node(
        graph_node_identity(&app, deferred_surface),
        UiAdmissionWorld::authoritative(),
    ));
    let unsupported_snapshot = boundary.support_snapshot(&UiAdmissionTarget::graph_node(
        graph_node_identity(&foreign_app, foreign_control),
        UiAdmissionWorld::authoritative(),
    ));
    let wrong_world_snapshot = boundary.support_snapshot(&UiAdmissionTarget::graph_node(
        graph_node_identity(&app, supported_control),
        UiAdmissionWorld::from_graph_world_profile(UiGraphWorldProfile::preview_session_label(
            UiGraphSessionLabel::new("worth-ui.preview.support")
                .expect("preview label should admit"),
        )),
    ));
    let diagnostic_only_snapshot = boundary.support_snapshot(&UiAdmissionTarget::graph_node(
        graph_node_identity(&app, diagnostic_surface),
        UiAdmissionWorld::authoritative(),
    ));

    assert_eq!(
        supported_snapshot.posture(),
        &UiSupportPosture::Supported {
            family: UiAdmissionFamily::TouchMeaning,
            world: UiAdmissionWorld::authoritative(),
        }
    );
    assert_eq!(
        supported_snapshot.inspection_posture(),
        UiInspectionSupportPosture::Supported
    );

    assert_eq!(
        deferred_snapshot.posture(),
        &UiSupportPosture::Deferred {
            family: UiAdmissionFamily::TouchMeaning,
            expected_in: UiDeclarationSupportMilestoneExpectation::Milestone32,
            world: UiAdmissionWorld::authoritative(),
        }
    );
    assert_eq!(
        deferred_snapshot.inspection_posture(),
        UiInspectionSupportPosture::Deferred
    );

    assert_eq!(
        diagnostic_only_snapshot.posture(),
        &UiSupportPosture::DiagnosticOnly {
            family: UiAdmissionFamily::TouchMeaning,
            world: UiAdmissionWorld::authoritative(),
        }
    );
    assert_eq!(
        diagnostic_only_snapshot.inspection_posture(),
        UiInspectionSupportPosture::DiagnosticOnly
    );

    assert_eq!(
        unsupported_snapshot.posture(),
        &UiSupportPosture::Unsupported {
            family: UiAdmissionFamily::TouchMeaning,
            reason: UiSupportReason::TargetOutsideAdmissionBoundary,
            world: UiAdmissionWorld::authoritative(),
        }
    );
    assert_eq!(
        unsupported_snapshot.inspection_posture(),
        UiInspectionSupportPosture::Unsupported
    );

    assert_eq!(
        wrong_world_snapshot.posture(),
        &UiSupportPosture::WrongWorld {
            family: UiAdmissionFamily::TouchMeaning,
            expected: UiAdmissionWorld::authoritative(),
            observed: UiAdmissionWorld::from_graph_world_profile(
                UiGraphWorldProfile::preview_session_label(
                    UiGraphSessionLabel::new("worth-ui.preview.support")
                        .expect("preview label should admit"),
                ),
            ),
        }
    );
    assert_eq!(
        supported_snapshot.posture().family(),
        wrong_world_snapshot.posture().family(),
    );
    assert_eq!(
        wrong_world_snapshot.inspection_posture(),
        UiInspectionSupportPosture::WrongWorld
    );
}

fn artifact_from_file_provenance<'a>(
    app: &'a worth_ui::facade::app::WorthUiApp,
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
            panic!("expected declaration artifact for {module_path}#{declaration_index}")
        })
}

fn graph_node_identity(
    app: &worth_ui::facade::app::WorthUiApp,
    artifact: &UiDeclarationArtifact,
) -> worth_ui::facade::graph::UiGraphNodeIdentity {
    app.graph()
        .lookup()
        .declaration_instances(artifact.identity())
        .value()
        .first()
        .copied()
        .expect("declaration should project one graph node")
}

fn supported_control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.save"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/admission_support.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:save"))
    .with_posture_token(UiDslPostureToken::new("touch:press"))
}

fn deferred_diagnostic_surface_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.region.deferred"),
        UiDslSemanticFamily::Region,
        UiDslSourceProvenance::file_authored("app/admission_support.wui", 1),
    )
    .with_structural_token(UiDslStructuralToken::new("region:sidebar"))
}

fn foreign_control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.foreign"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/admission_support_foreign.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:foreign"))
    .with_posture_token(UiDslPostureToken::new("touch:press"))
}

fn diagnostic_only_surface_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.diagnostics.graph"),
        UiDslSemanticFamily::DiagnosticSurface,
        UiDslSourceProvenance::file_authored("app/admission_support.wui", 2),
    )
    .with_structural_token(UiDslStructuralToken::new("diagnostic-surface:graph"))
}
