use worth_ui::facade::admission::{
    UiAdmissionFamily, UiAdmissionWorld, UiSupportPosture, UiSupportReason,
};
use worth_ui::facade::app::WorthUi;
use worth_ui::facade::declaration::{
    UiDeclarationArtifact, UiDeclarationSupportMilestoneExpectation,
};
use worth_ui::facade::graph::{
    UiGraphTouchAspectPosture, UiGraphTouchAspects, UiGraphTouchTiming, UiGraphWorldProfile,
    WorthQuerySessionLabel,
};
use worth_ui::facade::obligations::{
    UiObligationCheckKind, UiObligationDispatchStopPosture, UiObligationFamily,
    UiObligationSelectionReason, UiObligationSupportBasis, UiObligationVerdictClass,
    UiObligationWorldProfileClass,
};
use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken, WorthUiDslPackage,
};

#[test]
fn verdict_stop_posture_keeps_support_and_world_denials_structurally_distinct() {
    let app = WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.obligation-verdict")
                .with_semantic_artifact_spec(supported_control_spec())
                .with_semantic_artifact_spec(deferred_surface_spec())
                .with_semantic_artifact_spec(diagnostic_only_surface_spec()),
        )
        .freeze();
    let foreign_app = WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.obligation-verdict.foreign")
                .with_semantic_artifact_spec(foreign_control_spec()),
        )
        .freeze();
    let preview_app = WorthUi::app()
        .with_graph_world_profile(UiGraphWorldProfile::preview_session_label(
            WorthQuerySessionLabel::scoped_strs("worth-ui", ["phase5", "preview"])
                .expect("preview label should admit"),
        ))
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.obligation-verdict.preview")
                .with_semantic_artifact_spec(supported_control_spec()),
        )
        .freeze();

    let supported = artifact_from_file_provenance(&app, "app/obligation_verdict_runtime.wui", 0);
    let deferred = artifact_from_file_provenance(&app, "app/obligation_verdict_runtime.wui", 1);
    let diagnostic = artifact_from_file_provenance(&app, "app/obligation_verdict_runtime.wui", 2);
    let unsupported = artifact_from_file_provenance(
        &foreign_app,
        "app/obligation_verdict_runtime_foreign.wui",
        0,
    );
    let preview =
        artifact_from_file_provenance(&preview_app, "app/obligation_verdict_runtime.wui", 0);

    let deferred_verdict = single_verdict(&app, deferred);
    let unsupported_verdict = single_verdict_for_touch(
        &app,
        declaration_structural_touch(&foreign_app, unsupported),
    );
    let wrong_world_verdict =
        single_verdict_for_touch(&app, declaration_structural_touch(&preview_app, preview));
    let diagnostic_verdicts = verdicts_for(&app, diagnostic);
    let supported_verdicts = verdicts_for(&app, supported);

    assert_eq!(
        deferred_verdict.class(),
        UiObligationVerdictClass::Violation
    );
    assert_eq!(
        deferred_verdict.stop_posture(),
        UiObligationDispatchStopPosture::Deferred
    );
    assert_eq!(
        unsupported_verdict.class(),
        UiObligationVerdictClass::Violation
    );
    assert_eq!(
        unsupported_verdict.stop_posture(),
        UiObligationDispatchStopPosture::Unsupported
    );
    assert_eq!(
        wrong_world_verdict.class(),
        UiObligationVerdictClass::Violation
    );
    assert_eq!(
        wrong_world_verdict.stop_posture(),
        UiObligationDispatchStopPosture::WrongWorld
    );
    assert!(diagnostic_verdicts
        .iter()
        .all(|verdict| verdict.class() == UiObligationVerdictClass::Advisory));
    assert!(diagnostic_verdicts.iter().all(|verdict| {
        verdict.stop_posture() == UiObligationDispatchStopPosture::DiagnosticOnly
    }));
    assert!(supported_verdicts
        .iter()
        .all(|verdict| verdict.stop_posture() == UiObligationDispatchStopPosture::None));

    assert_eq!(
        app.admission()
            .select_obligations(&declaration_structural_touch(&app, deferred))
            .support_snapshot()
            .posture(),
        &UiSupportPosture::Deferred {
            family: UiAdmissionFamily::TouchMeaning,
            expected_in: UiDeclarationSupportMilestoneExpectation::Milestone32,
            world: UiAdmissionWorld::authoritative(),
        }
    );
    assert_eq!(
        app.admission()
            .select_obligations(&declaration_structural_touch(&foreign_app, unsupported))
            .support_snapshot()
            .posture(),
        &UiSupportPosture::Unsupported {
            family: UiAdmissionFamily::TouchMeaning,
            reason: UiSupportReason::TargetOutsideAdmissionBoundary,
            world: UiAdmissionWorld::authoritative(),
        }
    );
    assert_eq!(
        app.admission()
            .select_obligations(&declaration_structural_touch(&preview_app, preview))
            .support_snapshot()
            .posture(),
        &UiSupportPosture::WrongWorld {
            family: UiAdmissionFamily::TouchMeaning,
            expected: UiAdmissionWorld::authoritative(),
            observed: UiAdmissionWorld::from_graph_world_profile(
                UiGraphWorldProfile::preview_session_label(
                    WorthQuerySessionLabel::scoped_strs("worth-ui", ["phase5", "preview"])
                        .expect("preview label should admit"),
                ),
            ),
        }
    );
}

#[path = "fixtures/obligation_dispatch_prerequisite_support/mod.rs"]
pub mod obligation_dispatch_prerequisite_support;

#[test]
fn blocked_selected_entry_verdicts_keep_identity_check_kind_and_reasons() {
    let app = obligation_dispatch_prerequisite_support::apps::query_touch_app();
    let touch = obligation_dispatch_prerequisite_support::touches::query_touch(&app);
    let bundle = obligation_dispatch_prerequisite_support::targets::execute_for_target(
        &app,
        &touch,
        obligation_dispatch_prerequisite_support::targets::wrong_query_basis_target(&touch),
    );

    assert_eq!(bundle.verdicts.len(), bundle.selected.obligations().len());
    for (verdict, selected) in bundle
        .verdicts
        .iter()
        .zip(bundle.selected.obligations().iter())
    {
        assert_eq!(verdict.family(), Some(selected.family()));
        assert_eq!(verdict.check_kind(), Some(selected.check_kind()));
        assert_eq!(verdict.selected_identity(), Some(selected.identity()));
        assert_eq!(verdict.selection_reasons(), selected.selection_reasons());
        assert_eq!(verdict.class(), UiObligationVerdictClass::Violation);
        assert_eq!(
            verdict.stop_posture(),
            UiObligationDispatchStopPosture::WrongQueryBasis {
                required: worth_ui::facade::admission::UiAdmissionQueryBasis::GraphAligned,
                observed: worth_ui::facade::admission::UiAdmissionQueryBasis::WrongWorldProjection,
            }
        );
    }

    let participation = bundle
        .verdicts
        .iter()
        .find(|verdict| verdict.family() == Some(UiObligationFamily::ParticipationLegality))
        .expect("blocked query touch should keep the participation verdict");
    assert_eq!(
        participation.check_kind(),
        Some(UiObligationCheckKind::BlockingInvariant)
    );
    assert_eq!(
        participation
            .selected_identity()
            .expect("blocked selected-entry verdict should retain identity")
            .support_basis(),
        UiObligationSupportBasis::TouchMeaning
    );
    assert!(participation.selection_reasons().contains(
        &UiObligationSelectionReason::WorldProfile(
            UiObligationWorldProfileClass::QuerySnapshotBasis,
        )
    ));
}

fn verdicts_for(
    app: &worth_ui::facade::app::WorthUiApp,
    artifact: &UiDeclarationArtifact,
) -> Box<[worth_ui_runtime::facade::obligations::UiObligationVerdict]> {
    let selected = app
        .admission()
        .select_obligations(&declaration_structural_touch(app, artifact));
    app.admission()
        .lower_obligation_dispatch(&selected)
        .execute()
}

fn single_verdict(
    app: &worth_ui::facade::app::WorthUiApp,
    artifact: &UiDeclarationArtifact,
) -> worth_ui_runtime::facade::obligations::UiObligationVerdict {
    verdicts_for(app, artifact)
        .into_vec()
        .into_iter()
        .next()
        .expect("expected one verdict")
}

fn single_verdict_for_touch(
    app: &worth_ui::facade::app::WorthUiApp,
    touch: worth_ui::facade::graph::UiGraphTouchDescriptor,
) -> worth_ui_runtime::facade::obligations::UiObligationVerdict {
    let selected = app.admission().select_obligations(&touch);
    app.admission()
        .lower_obligation_dispatch(&selected)
        .execute()
        .into_vec()
        .into_iter()
        .next()
        .expect("expected one verdict")
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
        UiDslSourceProvenance::file_authored("app/obligation_verdict_runtime.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:save"))
    .with_posture_token(UiDslPostureToken::new("touch:press"))
}

fn deferred_surface_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.region.deferred"),
        UiDslSemanticFamily::Region,
        UiDslSourceProvenance::file_authored("app/obligation_verdict_runtime.wui", 1),
    )
    .with_structural_token(UiDslStructuralToken::new("region:sidebar"))
}

fn diagnostic_only_surface_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.diagnostics.graph"),
        UiDslSemanticFamily::DiagnosticSurface,
        UiDslSourceProvenance::file_authored("app/obligation_verdict_runtime.wui", 2),
    )
    .with_structural_token(UiDslStructuralToken::new("diagnostic-surface:graph"))
}

fn foreign_control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.foreign"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/obligation_verdict_runtime_foreign.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:foreign"))
    .with_posture_token(UiDslPostureToken::new("touch:press"))
}
