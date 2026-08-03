use worth_ui::facade::admission::WorthUiAdmissionExt;
use worth_ui::facade::admission::{UiAdmissionWorld, UiSupportPosture, UiSupportReason};
use worth_ui::facade::app::WorthUi;
use worth_ui::facade::declaration::{
    UiDeclarationArtifact, UiDeclarationSupportMilestoneExpectation,
};
use worth_ui::facade::graph::{
    UiGraphSessionLabel, UiGraphTouchAspectPosture, UiGraphTouchAspects, UiGraphTouchTiming,
    UiGraphWorldProfile,
};
use worth_ui_certification::{
    WorthUiCertificationBuilderExt, WorthUiRustAuthoredDeclarationFixture,
};
use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken,
};
use worth_ui_test_support::WorthUiApplicationBuilderCertificationExt;

#[test]
fn unsupported_deferred_and_wrong_world_posture_do_not_enter_ordinary_selection() {
    let app = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.certification.obligation-selection.rejection",
            )
            .with_semantic_artifact_spec(supported_control_spec())
            .with_semantic_artifact_spec(deferred_surface_spec()),
        )
        .freeze()
        .expect("application preparation should succeed");
    let foreign_app = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.certification.obligation-selection.foreign",
            )
            .with_semantic_artifact_spec(foreign_control_spec()),
        )
        .freeze()
        .expect("application preparation should succeed");
    let preview_app = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_graph_world_profile(UiGraphWorldProfile::preview_session_label(
            UiGraphSessionLabel::new("worth-ui.selection.preview")
                .expect("preview label should admit"),
        ))
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.certification.obligation-selection.preview",
            )
            .with_semantic_artifact_spec(supported_control_spec()),
        )
        .freeze()
        .expect("application preparation should succeed");

    let supported_artifact =
        artifact_from_file_provenance(&app, "app/obligation_selection_rejection.wui", 0);
    let deferred_artifact =
        artifact_from_file_provenance(&app, "app/obligation_selection_rejection.wui", 1);
    let foreign_artifact = artifact_from_file_provenance(
        &foreign_app,
        "app/obligation_selection_rejection_foreign.wui",
        0,
    );
    let preview_artifact =
        artifact_from_file_provenance(&preview_app, "app/obligation_selection_rejection.wui", 0);

    let supported_touch = declaration_structural_touch(&app, supported_artifact);
    let deferred_touch = declaration_structural_touch(&app, deferred_artifact);
    let foreign_touch = declaration_structural_touch(&foreign_app, foreign_artifact);
    let wrong_world_touch = declaration_structural_touch(&preview_app, preview_artifact);

    let supported_selection = app.admission().select_obligations(&supported_touch);
    let deferred_selection = app.admission().select_obligations(&deferred_touch);
    let unsupported_selection = app.admission().select_obligations(&foreign_touch);
    let wrong_world_selection = app.admission().select_obligations(&wrong_world_touch);

    assert!(!supported_selection.obligations().is_empty());

    assert_eq!(
        deferred_selection.support_snapshot().posture(),
        &UiSupportPosture::Deferred {
            family: worth_ui::facade::admission::UiAdmissionFamily::TouchMeaning,
            expected_in: UiDeclarationSupportMilestoneExpectation::Milestone32,
            world: UiAdmissionWorld::authoritative(),
        }
    );
    assert_eq!(
        deferred_selection
            .obligations()
            .iter()
            .map(|obligation| obligation.family())
            .collect::<Vec<_>>(),
        vec![worth_ui::facade::obligations::UiObligationFamily::StructuralLegality]
    );

    assert_eq!(
        unsupported_selection.support_snapshot().posture(),
        &UiSupportPosture::Unsupported {
            family: worth_ui::facade::admission::UiAdmissionFamily::TouchMeaning,
            reason: UiSupportReason::TargetOutsideAdmissionBoundary,
            world: UiAdmissionWorld::authoritative(),
        }
    );
    assert_eq!(
        unsupported_selection
            .obligations()
            .iter()
            .map(|obligation| obligation.family())
            .collect::<Vec<_>>(),
        vec![worth_ui::facade::obligations::UiObligationFamily::StructuralLegality]
    );

    assert_eq!(
        wrong_world_selection.support_snapshot().posture(),
        &UiSupportPosture::WrongWorld {
            family: worth_ui::facade::admission::UiAdmissionFamily::TouchMeaning,
            expected: UiAdmissionWorld::authoritative(),
            observed: UiAdmissionWorld::from_graph_world_profile(
                UiGraphWorldProfile::preview_session_label(
                    UiGraphSessionLabel::new("worth-ui.selection.preview")
                        .expect("preview label should admit"),
                ),
            ),
        }
    );
    assert!(wrong_world_selection.obligations().is_empty());
}

fn declaration_structural_touch(
    app: &worth_ui::facade::app::WorthUiApp,
    artifact: &UiDeclarationArtifact,
) -> worth_ui::facade::graph::UiGraphTouchDescriptor {
    let graph = app.graph();
    graph
        .touches()
        .from_node(
            graph
                .touches()
                .declaration_change_receipt(artifact)
                .expect("declaration change should admit"),
            UiGraphTouchTiming::PostMutation,
            graph_node_identity(app, artifact),
            UiGraphTouchAspects::new().structural(UiGraphTouchAspectPosture::Invalidated),
        )
        .expect("structural declaration touch should admit")
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
        UiDslSourceProvenance::file_authored("app/obligation_selection_rejection.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:save"))
    .with_posture_token(UiDslPostureToken::new("touch:press"))
}

fn deferred_surface_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.region.deferred"),
        UiDslSemanticFamily::Region,
        UiDslSourceProvenance::file_authored("app/obligation_selection_rejection.wui", 1),
    )
    .with_structural_token(UiDslStructuralToken::new("region:sidebar"))
}

fn foreign_control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.foreign"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/obligation_selection_rejection_foreign.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:foreign"))
    .with_posture_token(UiDslPostureToken::new("touch:press"))
}
