use worth_ui::facade::app::WorthUi;
use worth_ui::facade::declaration::{UiDeclarationArtifact, UiDeclarationStructuralRole};
use worth_ui::facade::graph::{
    UiGraphAxisParticipation, UiGraphInstantiationPlan, UiGraphMountedPostureRelationship,
    UiGraphMountedReceiptMutationKind, UiGraphParticipationAxis, UiGraphParticipationStatus,
    UiGraphWorldProfile,
};
use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken, WorthUiDslPackage,
};

#[test]
fn mounted_receipt_authority_seeds_correspond_explicitly_to_graph_nodes() {
    let app = mounted_receipt_app();
    let plan = mounted_receipt_plan(&app);
    let reservations = plan.mounted_receipt_reservations(UiGraphWorldProfile::authoritative());
    let commit = plan
        .commit_initial_generation(UiGraphWorldProfile::authoritative())
        .expect("admitted plan should commit one coherent graph generation");
    let graph = commit.graph();

    assert_eq!(reservations.len(), graph.node_count());
    assert_eq!(reservations.len(), graph.mounted_receipt_slot_count());

    for reservation in reservations {
        let slot = graph
            .lookup()
            .mounted_receipt_slot_for_node(reservation.graph_node_identity())
            .expect("every admitted graph node should own one mounted receipt slot");
        let slot = slot.value();

        assert_eq!(
            slot.mounted_receipt_identity(),
            reservation.mounted_receipt_identity()
        );
        assert_eq!(slot.graph_node_identity(), reservation.graph_node_identity());
        assert_eq!(slot.authority_seed(), reservation.authority_seed());
        assert_eq!(
            slot.mounted_posture_relationship(),
            reservation.mounted_posture_relationship()
        );
        assert!(slot.authority_seed().graph_owned_slot_reserved());
        assert_eq!(
            slot.mounted_posture_relationship(),
            UiGraphMountedPostureRelationship::ReservedMountedAuthoritySlot
        );
        assert_ne!(
            slot.mounted_receipt_identity().digest(),
            reservation.graph_node_identity().digest()
        );
        assert_eq!(
            graph
                .lookup()
                .mounted_receipt_slot_for_node(reservation.graph_node_identity())
                .map(|lookup| lookup.value().mounted_receipt_identity()),
            Some(slot.mounted_receipt_identity())
        );
    }
}

#[test]
fn mounted_receipt_lookup_is_bounded_and_authoritative() {
    let app = mounted_receipt_app();
    let graph = app.graph();
    let control_id = graph_node_identity(
        graph,
        artifact_from_file_provenance(&app, "app/mounted_receipt_runtime.wui", 0),
    );
    let slot = graph
        .lookup()
        .mounted_receipt_slot_for_node(control_id)
        .expect("control node should own one mounted receipt slot");
    let slot = slot.value();
    let mounted_receipt_identity = slot.mounted_receipt_identity();

    assert_eq!(mounted_receipt_identity, slot.mounted_receipt_identity());
    assert_eq!(
        graph
            .lookup()
            .mounted_receipt_slot(mounted_receipt_identity)
            .expect("receipt identity lookup should resolve the stored slot")
            .value(),
        slot
    );
}

#[test]
fn mounted_axis_changes_and_mounted_receipt_mutation_stay_aligned() {
    let app = mounted_receipt_app();
    let graph = app.graph();
    let control_id = graph_node_identity(
        graph,
        artifact_from_file_provenance(&app, "app/mounted_receipt_runtime.wui", 0),
    );
    let control_node = graph
        .lookup()
        .graph_node(control_id)
        .expect("control declaration should admit one graph node");
    let control_node = control_node.value();
    let mounted_admission = UiGraphAxisParticipation::runtime_mutation(
        UiGraphParticipationStatus::Admitted,
    );
    let mounted_withdrawal = UiGraphAxisParticipation::runtime_mutation(
        UiGraphParticipationStatus::Withheld,
    );

    let create_transition = graph
        .mounted_receipt_transition_for_node(
            control_id,
            control_node
                .participation_posture()
                .axis(UiGraphParticipationAxis::Mounted),
            mounted_admission,
        )
        .expect("mounted admission should create one mounted receipt transition");
    let remove_transition = graph
        .mounted_receipt_transition_for_node(control_id, mounted_admission, mounted_withdrawal)
        .expect("mounted withdrawal should remove one mounted receipt transition");
    let no_mutation = graph.mounted_receipt_transition_for_node(
        control_id,
        mounted_admission,
        mounted_admission,
    );
    let no_withdrawal = graph.mounted_receipt_transition_for_node(
        control_id,
        mounted_withdrawal,
        mounted_withdrawal,
    );
    let create_mutation = create_transition.mutation();
    let remove_mutation = remove_transition.mutation();
    let create_slot = create_transition.authority_record();

    assert_eq!(
        create_transition.kind(),
        UiGraphMountedReceiptMutationKind::CreateSlot
    );
    assert_eq!(
        remove_transition.kind(),
        UiGraphMountedReceiptMutationKind::RemoveSlot
    );
    assert_eq!(create_mutation.kind(), create_transition.kind());
    assert_eq!(remove_mutation.kind(), remove_transition.kind());
    assert_eq!(create_mutation.graph_node_identity(), control_id);
    assert_eq!(
        create_mutation.mounted_receipt_identity(),
        create_slot.mounted_receipt_identity()
    );
    assert!(create_mutation.authority_seed().graph_owned_slot_reserved());
    assert_eq!(
        create_mutation.mounted_posture_relationship(),
        UiGraphMountedPostureRelationship::ReservedMountedAuthoritySlot
    );
    assert!(no_mutation.is_none());
    assert!(no_withdrawal.is_none());
}

fn mounted_receipt_plan(app: &worth_ui::facade::app::WorthUiApp) -> UiGraphInstantiationPlan {
    let root_page_handoff = root_page_artifact(app)
        .graph_handoff()
        .expect("bootstrap root page should lower to graph handoff");
    let control_handoff = artifact_from_file_provenance(app, "app/mounted_receipt_runtime.wui", 0)
        .graph_handoff()
        .expect("control declaration should lower to graph handoff");
    let region_handoff = artifact_from_file_provenance(app, "app/mounted_receipt_runtime.wui", 1)
        .graph_handoff()
        .expect("region declaration should lower to graph handoff");

    UiGraphInstantiationPlan::admit_handoffs(&[root_page_handoff, control_handoff, region_handoff], &[])
        .expect("mounted receipt certification app should admit graph instantiation")
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

fn mounted_receipt_app() -> worth_ui::facade::app::WorthUiApp {
    WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.mounted-receipt")
                .with_semantic_artifact_spec(control_spec())
                .with_semantic_artifact_spec(region_spec()),
        )
        .freeze()
}

fn control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.save"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/mounted_receipt_runtime.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:save"))
    .with_structural_token(UiDslStructuralToken::new("slot:footer"))
    .with_posture_token(UiDslPostureToken::new("query-binding:attached:view"))
}

fn region_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.region.sidebar"),
        UiDslSemanticFamily::Region,
        UiDslSourceProvenance::file_authored("app/mounted_receipt_runtime.wui", 1),
    )
    .with_structural_token(UiDslStructuralToken::new("region:sidebar"))
}
