use worth_ui::facade::admission::UiAdmissionAggregation;
use worth_ui::facade::app::WorthUi;
use worth_ui::facade::declaration::UiDeclarationArtifact;
use worth_ui::facade::graph::{
    UiGraphAxisParticipation, UiGraphParticipationAxis, UiGraphParticipationStatus,
    UiGraphSessionLabel, UiGraphTouchAspectPosture, UiGraphTouchAspects, UiGraphTouchTiming,
    UiGraphWorldProfile,
};
use worth_ui::facade::obligations::{
    UiObligationDispatchStopPosture, UiObligationFamily, UiObligationVerdictClass,
};
use worth_ui_certification::scenario::installed_query_world;
use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken, WorthUiDslPackage,
};

#[test]
fn structural_selected_obligations_lower_to_stable_dispatch_and_success_verdicts() {
    let app = touch_app(UiGraphWorldProfile::hot_reload_candidate(
        UiGraphSessionLabel::new("worth-ui.phase5.hot-reload")
            .expect("hot-reload label should admit"),
    ));
    let graph = app.graph();
    let artifact = control_artifact(&app);
    let node = graph_node_identity(graph, artifact);
    let touch = graph
        .touches()
        .from_slot_occupancy(
            graph
                .touches()
                .declaration_change_receipt(artifact)
                .expect("declaration change should admit"),
            UiGraphTouchTiming::PostMutation,
            node,
            UiGraphTouchAspects::new()
                .structural(UiGraphTouchAspectPosture::Invalidated)
                .participation(UiGraphTouchAspectPosture::Written),
        )
        .expect("slot touch should admit");

    let selected = app.admission().select_obligations(&touch);
    let left = app.admission().lower_obligation_dispatch(&selected);
    let right = app.admission().lower_obligation_dispatch(&selected);
    let verdicts = left.execute();
    let report = app.admission().admit_selected_obligations(&selected);

    assert_eq!(left, right);
    assert_eq!(left.shape_digest(), right.shape_digest());
    assert_eq!(
        left.entries()
            .iter()
            .map(|entry| entry.selected().family())
            .collect::<Vec<_>>(),
        vec![
            UiObligationFamily::StructuralLegality,
            UiObligationFamily::SlotContract,
            UiObligationFamily::ParticipationLegality,
        ]
    );
    assert!(verdicts
        .iter()
        .all(|verdict| verdict.class() == UiObligationVerdictClass::Success));
    assert!(verdicts
        .iter()
        .all(|verdict| verdict.stop_posture() == UiObligationDispatchStopPosture::None));
    assert_eq!(report.aggregation(), UiAdmissionAggregation::Admitted);
    assert_eq!(report.dispatch_plan(), Some(&left));
    assert_eq!(report.verdicts(), verdicts.as_ref());
}

#[test]
fn query_selected_obligations_lower_to_narrow_dispatch_without_widening_families() {
    let app = touch_app(query_snapshot_world_profile(
        "snapshot:phase5-dispatch",
        ["worth-ui.phase5", "dispatch", "query"],
    ));
    let graph = app.graph();
    let artifact = control_artifact(&app);
    let touch = graph
        .touches()
        .from_mount_eligibility_transition(
            graph
                .touches()
                .query_binding_change_receipt()
                .expect("query world should admit query receipt"),
            UiGraphTouchTiming::PostMutation,
            mount_eligibility_transition(&app, artifact),
            UiGraphTouchAspects::new()
                .query_binding(UiGraphTouchAspectPosture::Invalidated)
                .participation(UiGraphTouchAspectPosture::Invalidated)
                .diagnostic(UiGraphTouchAspectPosture::Written),
        )
        .expect("query-backed touch should admit");

    let selected = app.admission().select_obligations(&touch);
    let dispatch = app.admission().lower_obligation_dispatch(&selected);
    let verdicts = dispatch.execute();
    let report = app.admission().admit_selected_obligations(&selected);

    assert_eq!(
        dispatch
            .entries()
            .iter()
            .map(|entry| entry.selected().family())
            .collect::<Vec<_>>(),
        selected
            .obligations()
            .iter()
            .map(|obligation| obligation.family())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        verdicts
            .iter()
            .map(|verdict| (verdict.family(), verdict.class(), verdict.stop_posture()))
            .collect::<Vec<_>>(),
        vec![
            (
                Some(UiObligationFamily::ParticipationLegality),
                UiObligationVerdictClass::Success,
                UiObligationDispatchStopPosture::None,
            ),
            (
                Some(UiObligationFamily::QueryBindingRequirement),
                UiObligationVerdictClass::Advisory,
                UiObligationDispatchStopPosture::Deferred,
            ),
            (
                Some(UiObligationFamily::DiagnosticSurfaceRequirement),
                UiObligationVerdictClass::Advisory,
                UiObligationDispatchStopPosture::DiagnosticOnly,
            ),
        ]
    );
    assert_eq!(
        report.aggregation(),
        UiAdmissionAggregation::AdmittedWithAdvisory
    );
    assert_eq!(report.dispatch_plan(), Some(&dispatch));
}

fn touch_app(world_profile: UiGraphWorldProfile) -> worth_ui::facade::app::WorthUiApp {
    WorthUi::app()
        .with_graph_world_profile(world_profile)
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.obligation-dispatch")
                .with_semantic_artifact_spec(control_spec()),
        )
        .freeze()
        .expect("application preparation should succeed")
}

fn control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.save"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/obligation_dispatch_runtime.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:save"))
    .with_structural_token(UiDslStructuralToken::new("slot:footer"))
    .with_posture_token(UiDslPostureToken::new("query-binding:attached:view"))
}

fn control_artifact(app: &worth_ui::facade::app::WorthUiApp) -> &UiDeclarationArtifact {
    app.declaration_artifacts()
        .iter()
        .find(|artifact| {
            let provenance = artifact.provenance().source_provenance();
            provenance.module_path() == "app/obligation_dispatch_runtime.wui"
                && provenance.declaration_index() == 0
        })
        .expect("control artifact should exist")
}

fn graph_node_identity(
    graph: worth_ui::facade::graph::UiGraphAuthority<'_>,
    artifact: &UiDeclarationArtifact,
) -> worth_ui::facade::graph::UiGraphNodeIdentity {
    graph
        .lookup()
        .declaration_instances(artifact.identity())
        .value()
        .first()
        .copied()
        .expect("declaration should admit one graph node")
}

fn mount_eligibility_transition(
    app: &worth_ui::facade::app::WorthUiApp,
    artifact: &UiDeclarationArtifact,
) -> worth_ui::facade::graph::UiGraphMountEligibilityTransition {
    let graph = app.graph();
    let graph_node_identity = graph_node_identity(graph, artifact);
    let control_node = graph
        .lookup()
        .graph_node(graph_node_identity)
        .expect("graph should resolve node")
        .value();

    graph
        .mount_eligibility_transition_for_node(
            graph_node_identity,
            control_node
                .participation_posture()
                .axis(UiGraphParticipationAxis::Mounted),
            UiGraphAxisParticipation::runtime_mutation(UiGraphParticipationStatus::Admitted),
        )
        .expect("mounted transition should admit")
}

fn query_snapshot_world_profile(
    snapshot_label: &str,
    schema_basis_parts: [&str; 3],
) -> UiGraphWorldProfile {
    let binding = schema_basis_parts.join(".").replace('-', "_");
    installed_query_world::settled_query_world_profile(
        worth_ui::facade::registry::ViewBindingId::new(binding.clone()).unwrap(),
        format!("{binding}.{snapshot_label}").replace('-', "_"),
    )
}
