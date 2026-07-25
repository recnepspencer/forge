use worth_ui::facade::app::WorthUi;
use worth_ui::facade::declaration::{UiDeclarationArtifact, UiDeclarationStructuralRole};
use worth_ui::facade::graph::{
    UiGraphAxisParticipation, UiGraphPageParticipationMutationKind, UiGraphParticipationAxis,
    UiGraphParticipationEvidenceHandle, UiGraphParticipationMutation,
    UiGraphParticipationReasonCode, UiGraphParticipationReasonSource, UiGraphParticipationStatus,
};
use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken, WorthUiDslPackage,
};

#[test]
fn node_participation_posture_exposes_every_required_axis_with_status_reason_and_evidence() {
    let app = participation_app();
    let graph = app.graph();
    let control_node = graph
        .lookup()
        .graph_node(graph_node_identity(
            graph,
            artifact_from_file_provenance(&app, "app/graph_participation_runtime.wui", 0),
        ))
        .expect("control declaration should admit one graph node");
    let diagnostic_node = graph
        .lookup()
        .graph_node(graph_node_identity(
            graph,
            artifact_from_file_provenance(&app, "app/graph_participation_runtime.wui", 2),
        ))
        .expect("diagnostic declaration should admit one graph node");
    let control_node = control_node.value();
    let diagnostic_node = diagnostic_node.value();

    assert_axis(
        control_node.participation_posture(),
        UiGraphParticipationAxis::Exists,
        UiGraphParticipationStatus::Admitted,
        UiGraphParticipationReasonSource::GraphInstantiation,
        UiGraphParticipationReasonCode::InstantiatedNodeExists,
        UiGraphParticipationEvidenceHandle::InstantiationPlan,
    );
    assert_axis(
        control_node.participation_posture(),
        UiGraphParticipationAxis::Mounted,
        UiGraphParticipationStatus::Deferred,
        UiGraphParticipationReasonSource::MountEligibility,
        UiGraphParticipationReasonCode::MountedAxisAwaitsRuntimeMutation,
        UiGraphParticipationEvidenceHandle::MountEligibilitySeed,
    );
    for axis in [
        UiGraphParticipationAxis::Visible,
        UiGraphParticipationAxis::Layout,
        UiGraphParticipationAxis::HitTest,
        UiGraphParticipationAxis::Focus,
        UiGraphParticipationAxis::Accessibility,
        UiGraphParticipationAxis::Paint,
        UiGraphParticipationAxis::Input,
    ] {
        let posture = control_node.participation_posture().axis(axis);
        assert_eq!(posture.status(), UiGraphParticipationStatus::Deferred);
        assert_eq!(
            posture.source(),
            UiGraphParticipationReasonSource::ReservedRuntimeMutation
        );
        assert_eq!(
            posture.evidence_handle(),
            UiGraphParticipationEvidenceHandle::ReservedRuntimeMutationLane
        );
    }
    assert_axis(
        control_node.participation_posture(),
        UiGraphParticipationAxis::QueryBound,
        UiGraphParticipationStatus::Admitted,
        UiGraphParticipationReasonSource::AttachmentPosture,
        UiGraphParticipationReasonCode::QueryBindingAttached,
        UiGraphParticipationEvidenceHandle::QueryBindingAttachment,
    );
    assert_axis(
        control_node.participation_posture(),
        UiGraphParticipationAxis::ServiceBound,
        UiGraphParticipationStatus::Admitted,
        UiGraphParticipationReasonSource::AttachmentPosture,
        UiGraphParticipationReasonCode::ServiceUsageAttached,
        UiGraphParticipationEvidenceHandle::ServiceUsageAttachment,
    );
    assert_axis(
        control_node.participation_posture(),
        UiGraphParticipationAxis::Diagnostic,
        UiGraphParticipationStatus::Withheld,
        UiGraphParticipationReasonSource::ContainmentClaim,
        UiGraphParticipationReasonCode::DiagnosticSurfaceAbsent,
        UiGraphParticipationEvidenceHandle::DiagnosticContainmentClaim,
    );
    assert_axis(
        diagnostic_node.participation_posture(),
        UiGraphParticipationAxis::Diagnostic,
        UiGraphParticipationStatus::Admitted,
        UiGraphParticipationReasonSource::ContainmentClaim,
        UiGraphParticipationReasonCode::DiagnosticSurfaceOwned,
        UiGraphParticipationEvidenceHandle::DiagnosticContainmentClaim,
    );
}

#[test]
fn page_participation_index_returns_axis_specific_explanation_rows_without_walks() {
    let app = participation_app();
    let graph = app.graph();
    let root_page_id = graph_node_identity(graph, root_page_artifact(&app));
    let control_id = graph_node_identity(
        graph,
        artifact_from_file_provenance(&app, "app/graph_participation_runtime.wui", 0),
    );
    let diagnostic_id = graph_node_identity(
        graph,
        artifact_from_file_provenance(&app, "app/graph_participation_runtime.wui", 2),
    );
    let control_node = graph
        .lookup()
        .graph_node(control_id)
        .expect("control declaration should admit one graph node");
    let diagnostic_node = graph
        .lookup()
        .graph_node(diagnostic_id)
        .expect("diagnostic declaration should admit one graph node");
    let control_node = control_node.value();
    let diagnostic_node = diagnostic_node.value();

    assert_page_axis_parity(
        graph,
        root_page_id,
        control_id,
        control_node.participation_posture(),
    );
    assert_page_axis_parity(
        graph,
        root_page_id,
        diagnostic_id,
        diagnostic_node.participation_posture(),
    );
}

#[test]
fn participation_mutation_surface_preserves_axis_specific_page_effects() {
    let app = participation_app();
    let graph = app.graph();
    let root_page_id = graph_node_identity(graph, root_page_artifact(&app));
    let control_node_identity = graph_node_identity(
        graph,
        artifact_from_file_provenance(&app, "app/graph_participation_runtime.wui", 0),
    );
    let control_node = graph
        .lookup()
        .graph_node(control_node_identity)
        .expect("control declaration should admit one graph node");
    let control_node = control_node.value();
    let visible_admission =
        UiGraphAxisParticipation::runtime_mutation(UiGraphParticipationStatus::Admitted);
    let visible_mutation = UiGraphParticipationMutation::axis_transition(
        control_node_identity,
        root_page_id,
        control_node.participation_posture(),
        UiGraphParticipationAxis::Visible,
        visible_admission,
    );
    let query_withdrawal =
        UiGraphAxisParticipation::runtime_mutation(UiGraphParticipationStatus::Withheld);
    let query_mutation = UiGraphParticipationMutation::axis_transition(
        control_node_identity,
        root_page_id,
        control_node.participation_posture(),
        UiGraphParticipationAxis::QueryBound,
        query_withdrawal,
    );

    assert_eq!(
        visible_mutation
            .updated_posture()
            .axis(UiGraphParticipationAxis::Visible)
            .status(),
        UiGraphParticipationStatus::Admitted
    );
    assert_eq!(
        visible_mutation
            .page_participation_mutation()
            .expect("visible admission should add one page participation row")
            .kind(),
        UiGraphPageParticipationMutationKind::Added
    );
    assert_eq!(
        query_mutation
            .page_participation_mutation()
            .expect("query withdrawal should remove one page participation row")
            .kind(),
        UiGraphPageParticipationMutationKind::Removed
    );
    assert_eq!(
        query_mutation.next_axis_participation().reason(),
        UiGraphParticipationReasonCode::RuntimeMutationApplied
    );
    assert_eq!(
        query_mutation.next_axis_participation().source(),
        UiGraphParticipationReasonSource::ParticipationMutation
    );
}

fn assert_axis(
    posture: worth_ui::facade::graph::UiGraphParticipationPosture,
    axis: UiGraphParticipationAxis,
    status: UiGraphParticipationStatus,
    source: UiGraphParticipationReasonSource,
    reason: UiGraphParticipationReasonCode,
    evidence: UiGraphParticipationEvidenceHandle,
) {
    let participation = posture.axis(axis);

    assert_eq!(participation.status(), status);
    assert_eq!(participation.source(), source);
    assert_eq!(participation.reason(), reason);
    assert_eq!(participation.evidence_handle(), evidence);
}

fn assert_page_axis_parity(
    graph: worth_ui::facade::graph::UiGraphAuthority<'_>,
    page_node_identity: worth_ui::facade::graph::UiGraphNodeIdentity,
    member_node_identity: worth_ui::facade::graph::UiGraphNodeIdentity,
    participation_posture: worth_ui::facade::graph::UiGraphParticipationPosture,
) {
    for axis in UiGraphParticipationAxis::ALL {
        let axis_participation = participation_posture.axis(axis);
        let matching_rows = graph
            .lookup()
            .page_participation(page_node_identity, axis)
            .value()
            .iter()
            .filter(|row| row.member_node_identity() == member_node_identity)
            .collect::<Vec<_>>();

        if axis_participation.status().admitted() {
            assert_eq!(
                matching_rows.len(),
                1,
                "expected one page participation row for {axis:?} on {member_node_identity:?}"
            );
            assert_eq!(matching_rows[0].page_node_identity(), page_node_identity);
            assert_eq!(matching_rows[0].axis(), axis);
            assert_eq!(matching_rows[0].axis_participation(), axis_participation);
        } else {
            assert!(
                matching_rows.is_empty(),
                "expected no page participation row for non-admitted {axis:?} on {member_node_identity:?}"
            );
        }
    }
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

fn root_page_artifact(app: &worth_ui::facade::app::WorthUiApp) -> &UiDeclarationArtifact {
    app.declaration_artifacts()
        .iter()
        .find(|artifact| {
            artifact
                .graph_handoff()
                .map(|handoff| handoff.role() == UiDeclarationStructuralRole::Page)
                .unwrap_or(false)
        })
        .expect("bootstrap root page artifact should exist")
}

fn participation_app() -> worth_ui::facade::app::WorthUiApp {
    WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.graph-participation")
                .with_semantic_artifact_spec(slotted_control_spec())
                .with_semantic_artifact_spec(region_spec())
                .with_semantic_artifact_spec(diagnostic_surface_spec()),
        )
        .freeze()
        .expect("application preparation should succeed")
}

fn slotted_control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.save"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/graph_participation_runtime.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:save"))
    .with_structural_token(UiDslStructuralToken::new("slot:footer"))
    .with_posture_token(UiDslPostureToken::new("query-binding:attached:view"))
    .with_posture_token(UiDslPostureToken::new("service:portal"))
}

fn region_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.region.sidebar"),
        UiDslSemanticFamily::Region,
        UiDslSourceProvenance::file_authored("app/graph_participation_runtime.wui", 1),
    )
    .with_structural_token(UiDslStructuralToken::new("region:sidebar"))
}

fn diagnostic_surface_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.diagnostic_surface.lint"),
        UiDslSemanticFamily::DiagnosticSurface,
        UiDslSourceProvenance::file_authored("app/graph_participation_runtime.wui", 2),
    )
    .with_structural_token(UiDslStructuralToken::new("diagnostic-surface:lint"))
}
