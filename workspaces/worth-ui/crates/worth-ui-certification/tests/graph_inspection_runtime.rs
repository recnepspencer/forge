use worth_ui::facade::app::WorthUi;
use worth_ui::facade::declaration::UiDeclarationArtifact;
use worth_ui::facade::graph::{
    UiGraphEvidenceRef, UiGraphInspectionTarget, UiGraphLookupCostClass, UiGraphLookupFamily,
    UiGraphParticipationAxis,
};
use worth_ui_dsl::{
    UiDslAspectName, UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily,
    UiDslSemanticKey, UiDslSourceProvenance, UiDslStructuralToken, WorthUiDslPackage,
};

#[test]
fn graph_truth_is_inspectable_through_formal_graph_inspection_support() {
    let app = inspection_app();
    let graph = app.graph();
    let root_page = root_page_artifact(&app);
    let control = artifact_from_file_provenance(&app, "app/graph_inspection_runtime.wui", 0);
    let consumer = artifact_from_file_provenance(&app, "app/graph_inspection_runtime.wui", 1);
    let competing_control =
        artifact_from_file_provenance(&app, "app/graph_inspection_runtime.wui", 2);
    let competing_consumer =
        artifact_from_file_provenance(&app, "app/graph_inspection_runtime.wui", 3);
    let root_page_id = graph_node_identity(graph, root_page);
    let control_id = graph_node_identity(graph, control);
    let consumer_id = graph_node_identity(graph, consumer);
    let published_aspect = control
        .aspect_contract()
        .expect("control declaration should admit aspect contract")
        .published()
        .aspects()
        .first()
        .cloned()
        .expect("control declaration should publish one aspect");
    let competing_aspect = competing_control
        .aspect_contract()
        .expect("competing control should admit aspect contract")
        .published()
        .aspects()
        .first()
        .cloned()
        .expect("competing control should publish one competing aspect");
    let competing_control_id = graph_node_identity(graph, competing_control);
    let competing_consumer_id = graph_node_identity(graph, competing_consumer);
    let node_inspection = graph
        .inspection()
        .inspect_graph_node(control_id)
        .expect("graph inspection should target committed node");
    let topology_inspection = graph
        .inspection()
        .inspect_topology_node(control_id)
        .expect("topology inspection should target committed node");
    let declaration_inspection = graph
        .inspection()
        .inspect_declaration_instances(control.identity());
    let parent_child_inspection = graph.inspection().inspect_parent_child(root_page_id);
    let slot_occupants_inspection = graph
        .inspection()
        .inspect_slot_occupants(root_page_id, "footer");
    let page_inspection = graph
        .inspection()
        .inspect_page_participation(root_page_id, UiGraphParticipationAxis::QueryBound);
    let aspect_inspection = graph
        .inspection()
        .inspect_aspect_publishers(&published_aspect);
    let aspect_consumer_inspection = graph
        .inspection()
        .inspect_aspect_consumers(&published_aspect);
    let competing_aspect_inspection = graph
        .inspection()
        .inspect_aspect_publishers(&competing_aspect);
    let competing_aspect_consumer_inspection = graph
        .inspection()
        .inspect_aspect_consumers(&competing_aspect);
    let mounted_receipt_identity = graph
        .lookup()
        .mounted_receipt_slot_for_node(control_id)
        .expect("mounted receipt slot should resolve for control")
        .value()
        .mounted_receipt_identity();
    let mounted_receipt_inspection = graph
        .inspection()
        .inspect_mounted_receipt_slot(mounted_receipt_identity)
        .expect("mounted receipt inspection should target committed slot");

    assert_eq!(node_inspection.generation(), graph.generation());
    assert_eq!(
        node_inspection.target(),
        &UiGraphInspectionTarget::GraphNode(control_id)
    );
    assert_eq!(
        node_inspection.lookup_receipt().family(),
        UiGraphLookupFamily::NodeIdentity
    );
    assert_eq!(
        node_inspection.lookup_receipt().cost_class(),
        UiGraphLookupCostClass::IndexedScalar
    );
    assert!(node_inspection
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::GraphNode(control_id)));

    assert_eq!(
        topology_inspection.target(),
        &UiGraphInspectionTarget::TopologyNode(control_id)
    );
    assert_eq!(
        topology_inspection.lookup_receipt().family(),
        UiGraphLookupFamily::TopologyNode
    );
    assert_eq!(
        topology_inspection.lookup_receipt().cost_class(),
        UiGraphLookupCostClass::IndexedScalar
    );
    assert!(topology_inspection
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::GraphNode(control_id)));

    assert_eq!(
        declaration_inspection.target(),
        &UiGraphInspectionTarget::DeclarationInstances(control.identity().clone())
    );
    assert_eq!(
        declaration_inspection.lookup_receipt().family(),
        UiGraphLookupFamily::DeclarationCorrespondence
    );
    assert!(declaration_inspection
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::Declaration(control.identity().clone())));
    assert!(declaration_inspection
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::GraphNode(control_id)));

    assert_eq!(
        parent_child_inspection.target(),
        &UiGraphInspectionTarget::ParentChild(root_page_id)
    );
    assert_eq!(
        parent_child_inspection.lookup_receipt().family(),
        UiGraphLookupFamily::ParentChild
    );
    assert_eq!(
        parent_child_inspection.lookup_receipt().cost_class(),
        UiGraphLookupCostClass::IndexedSet
    );
    assert!(parent_child_inspection
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::GraphNode(root_page_id)));
    assert!(parent_child_inspection
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::GraphNode(control_id)));

    assert_eq!(
        slot_occupants_inspection.target(),
        &UiGraphInspectionTarget::SlotOccupancy {
            parent_node_identity: root_page_id,
            slot_name: "footer".into(),
        }
    );
    assert_eq!(
        slot_occupants_inspection.lookup_receipt().family(),
        UiGraphLookupFamily::SlotOccupancy
    );
    assert_eq!(
        slot_occupants_inspection.lookup_receipt().cost_class(),
        UiGraphLookupCostClass::IndexedSet
    );
    assert!(slot_occupants_inspection
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::GraphNode(control_id)));

    assert_eq!(
        page_inspection.target(),
        &UiGraphInspectionTarget::PageParticipation {
            page_node_identity: root_page_id,
            axis: UiGraphParticipationAxis::QueryBound,
        }
    );
    assert_eq!(
        page_inspection.lookup_receipt().cost_class(),
        UiGraphLookupCostClass::IndexedNeighborhood
    );
    assert!(page_inspection
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::Page(root_page_id)));

    assert_eq!(
        aspect_inspection.target(),
        &UiGraphInspectionTarget::PublishedAspect(published_aspect.clone())
    );
    assert_eq!(
        aspect_inspection.lookup_receipt().family(),
        UiGraphLookupFamily::PublishedAspect
    );
    assert!(aspect_inspection
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::Aspect(published_aspect.clone())));
    assert!(aspect_inspection
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::GraphNode(control_id)));
    assert!(!aspect_inspection
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::GraphNode(competing_control_id)));
    assert!(aspect_inspection
        .evidence_refs()
        .iter()
        .any(|evidence_ref| { matches!(evidence_ref, UiGraphEvidenceRef::MountedReceipt(_)) }));

    assert_eq!(
        aspect_consumer_inspection.target(),
        &UiGraphInspectionTarget::ConsumedAspect(published_aspect.clone())
    );
    assert_eq!(
        aspect_consumer_inspection.lookup_receipt().family(),
        UiGraphLookupFamily::ConsumedAspect
    );
    assert_eq!(
        aspect_consumer_inspection.lookup_receipt().cost_class(),
        UiGraphLookupCostClass::IndexedNeighborhood
    );
    assert!(aspect_consumer_inspection
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::Aspect(published_aspect.clone())));
    assert!(aspect_consumer_inspection
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::GraphNode(consumer_id)));
    assert!(!aspect_consumer_inspection
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::GraphNode(competing_consumer_id)));
    assert!(aspect_consumer_inspection
        .evidence_refs()
        .iter()
        .any(|evidence_ref| { matches!(evidence_ref, UiGraphEvidenceRef::MountedReceipt(_)) }));

    assert_eq!(
        competing_aspect_inspection.target(),
        &UiGraphInspectionTarget::PublishedAspect(competing_aspect.clone())
    );
    assert_eq!(
        competing_aspect_inspection.lookup_receipt().family(),
        UiGraphLookupFamily::PublishedAspect
    );
    assert_eq!(
        competing_aspect_inspection.lookup_receipt().cost_class(),
        UiGraphLookupCostClass::IndexedNeighborhood
    );
    assert!(competing_aspect_inspection
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::Aspect(competing_aspect.clone())));
    assert!(competing_aspect_inspection
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::GraphNode(competing_control_id)));
    assert!(!competing_aspect_inspection
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::GraphNode(control_id)));

    assert_eq!(
        competing_aspect_consumer_inspection.target(),
        &UiGraphInspectionTarget::ConsumedAspect(competing_aspect.clone())
    );
    assert_eq!(
        competing_aspect_consumer_inspection
            .lookup_receipt()
            .family(),
        UiGraphLookupFamily::ConsumedAspect
    );
    assert_eq!(
        competing_aspect_consumer_inspection
            .lookup_receipt()
            .cost_class(),
        UiGraphLookupCostClass::IndexedNeighborhood
    );
    assert!(competing_aspect_consumer_inspection
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::Aspect(competing_aspect.clone())));
    assert!(competing_aspect_consumer_inspection
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::GraphNode(competing_consumer_id)));
    assert!(!competing_aspect_consumer_inspection
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::GraphNode(consumer_id)));

    assert_eq!(
        mounted_receipt_inspection.target(),
        &UiGraphInspectionTarget::MountedReceipt(mounted_receipt_identity)
    );
    assert_eq!(
        mounted_receipt_inspection.lookup_receipt().family(),
        UiGraphLookupFamily::MountedReceiptSlot
    );
    assert_eq!(
        mounted_receipt_inspection.lookup_receipt().cost_class(),
        UiGraphLookupCostClass::IndexedScalar
    );
    assert!(mounted_receipt_inspection.evidence_refs().contains(
        &UiGraphEvidenceRef::MountedReceipt(mounted_receipt_identity)
    ));
    assert!(mounted_receipt_inspection
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::GraphNode(control_id)));
}

fn inspection_app() -> worth_ui::facade::app::WorthUiApp {
    WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.graph-inspection")
                .with_semantic_artifact_spec(control_spec())
                .with_semantic_artifact_spec(consumer_spec())
                .with_semantic_artifact_spec(competing_control_spec())
                .with_semantic_artifact_spec(competing_consumer_spec()),
        )
        .freeze()
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
                .map(|handoff| {
                    handoff.role()
                        == worth_ui::facade::declaration::UiDeclarationStructuralRole::Page
                })
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

fn control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.inspectable"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/graph_inspection_runtime.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:inspect"))
    .with_structural_token(UiDslStructuralToken::new("slot:footer"))
    .with_posture_token(UiDslPostureToken::new("query-binding:attached:view"))
    .with_published_aspect(UiDslAspectName::new("content.text"))
}

fn consumer_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.region.inspectable_consumer"),
        UiDslSemanticFamily::Region,
        UiDslSourceProvenance::file_authored("app/graph_inspection_runtime.wui", 1),
    )
    .with_structural_token(UiDslStructuralToken::new("region:inspect-consumer"))
    .with_consumed_aspect(UiDslAspectName::new("content.text"))
}

fn competing_control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.inspectable_icon"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/graph_inspection_runtime.wui", 2),
    )
    .with_structural_token(UiDslStructuralToken::new("control:inspect-icon"))
    .with_published_aspect(UiDslAspectName::new("appearance.background"))
}

fn competing_consumer_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.region.inspectable_icon_consumer"),
        UiDslSemanticFamily::Region,
        UiDslSourceProvenance::file_authored("app/graph_inspection_runtime.wui", 3),
    )
    .with_structural_token(UiDslStructuralToken::new("region:inspect-icon-consumer"))
    .with_consumed_aspect(UiDslAspectName::new("appearance.background"))
}
