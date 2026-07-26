use worth_ui::facade::graph::{
    UiGraphEvidenceRef, UiGraphInspectionTarget, UiGraphLookupCostClass, UiGraphLookupFamily,
    UiGraphParticipationAxis,
};

#[path = "graph_inspection_runtime/fixture.rs"]
mod fixture;

use fixture::{
    artifact_from_file_provenance, graph_node_identity, inspection_app, published_aspect,
    root_page_artifact,
};

#[test]
fn graph_node_inspection_reports_exact_target_cost_and_evidence() {
    let app = inspection_app();
    let graph = app.graph();
    let control = artifact_from_file_provenance(&app, "app/graph_inspection_runtime.wui", 0);
    let control_id = graph_node_identity(graph, control);
    let inspection = graph
        .inspection()
        .inspect_graph_node(control_id)
        .expect("graph inspection should target committed node");

    assert_eq!(inspection.generation(), graph.generation());
    assert_eq!(
        inspection.target(),
        &UiGraphInspectionTarget::GraphNode(control_id)
    );
    assert_eq!(
        inspection.lookup_receipt().family(),
        UiGraphLookupFamily::NodeIdentity
    );
    assert_eq!(
        inspection.lookup_receipt().cost_class(),
        UiGraphLookupCostClass::IndexedScalar
    );
    assert!(inspection
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::GraphNode(control_id)));
}

#[test]
fn topology_inspection_reports_exact_target_cost_and_evidence() {
    let app = inspection_app();
    let graph = app.graph();
    let control = artifact_from_file_provenance(&app, "app/graph_inspection_runtime.wui", 0);
    let control_id = graph_node_identity(graph, control);
    let inspection = graph
        .inspection()
        .inspect_topology_node(control_id)
        .expect("topology inspection should target committed node");

    assert_eq!(
        inspection.target(),
        &UiGraphInspectionTarget::TopologyNode(control_id)
    );
    assert_eq!(
        inspection.lookup_receipt().family(),
        UiGraphLookupFamily::TopologyNode
    );
    assert_eq!(
        inspection.lookup_receipt().cost_class(),
        UiGraphLookupCostClass::IndexedScalar
    );
    assert!(inspection
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::GraphNode(control_id)));
}

#[test]
fn declaration_inspection_reports_corresponding_graph_node_evidence() {
    let app = inspection_app();
    let graph = app.graph();
    let control = artifact_from_file_provenance(&app, "app/graph_inspection_runtime.wui", 0);
    let control_id = graph_node_identity(graph, control);
    let inspection = graph
        .inspection()
        .inspect_declaration_instances(control.identity());

    assert_eq!(
        inspection.target(),
        &UiGraphInspectionTarget::DeclarationInstances(control.identity().clone())
    );
    assert_eq!(
        inspection.lookup_receipt().family(),
        UiGraphLookupFamily::DeclarationCorrespondence
    );
    assert!(inspection
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::Declaration(control.identity().clone())));
    assert!(inspection
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::GraphNode(control_id)));
}

#[test]
fn parent_child_inspection_reports_indexed_set_evidence() {
    let app = inspection_app();
    let graph = app.graph();
    let root_page = root_page_artifact(&app);
    let control = artifact_from_file_provenance(&app, "app/graph_inspection_runtime.wui", 0);
    let root_page_id = graph_node_identity(graph, root_page);
    let control_id = graph_node_identity(graph, control);
    let inspection = graph.inspection().inspect_parent_child(root_page_id);

    assert_eq!(
        inspection.target(),
        &UiGraphInspectionTarget::ParentChild(root_page_id)
    );
    assert_eq!(
        inspection.lookup_receipt().family(),
        UiGraphLookupFamily::ParentChild
    );
    assert_eq!(
        inspection.lookup_receipt().cost_class(),
        UiGraphLookupCostClass::IndexedSet
    );
    assert!(inspection
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::GraphNode(root_page_id)));
    assert!(inspection
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::GraphNode(control_id)));
}

#[test]
fn slot_occupancy_inspection_reports_exact_slot_and_occupant() {
    let app = inspection_app();
    let graph = app.graph();
    let root_page = root_page_artifact(&app);
    let control = artifact_from_file_provenance(&app, "app/graph_inspection_runtime.wui", 0);
    let root_page_id = graph_node_identity(graph, root_page);
    let control_id = graph_node_identity(graph, control);
    let inspection = graph
        .inspection()
        .inspect_slot_occupants(root_page_id, "footer");

    assert_eq!(
        inspection.target(),
        &UiGraphInspectionTarget::SlotOccupancy {
            parent_node_identity: root_page_id,
            slot_name: "footer".into(),
        }
    );
    assert_eq!(
        inspection.lookup_receipt().family(),
        UiGraphLookupFamily::SlotOccupancy
    );
    assert_eq!(
        inspection.lookup_receipt().cost_class(),
        UiGraphLookupCostClass::IndexedSet
    );
    assert!(inspection
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::GraphNode(control_id)));
}

#[test]
fn page_participation_inspection_reports_indexed_page_neighborhood() {
    let app = inspection_app();
    let graph = app.graph();
    let root_page_id = graph_node_identity(graph, root_page_artifact(&app));
    let inspection = graph
        .inspection()
        .inspect_page_participation(root_page_id, UiGraphParticipationAxis::QueryBound);

    assert_eq!(
        inspection.target(),
        &UiGraphInspectionTarget::PageParticipation {
            page_node_identity: root_page_id,
            axis: UiGraphParticipationAxis::QueryBound,
        }
    );
    assert_eq!(
        inspection.lookup_receipt().cost_class(),
        UiGraphLookupCostClass::IndexedNeighborhood
    );
    assert!(inspection
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::Page(root_page_id)));
}

#[test]
fn published_aspect_inspection_excludes_competing_publishers() {
    let app = inspection_app();
    let graph = app.graph();
    let control = artifact_from_file_provenance(&app, "app/graph_inspection_runtime.wui", 0);
    let competing = artifact_from_file_provenance(&app, "app/graph_inspection_runtime.wui", 2);
    let control_id = graph_node_identity(graph, control);
    let competing_id = graph_node_identity(graph, competing);
    let aspect = published_aspect(control);
    let inspection = graph.inspection().inspect_aspect_publishers(&aspect);

    assert_eq!(
        inspection.target(),
        &UiGraphInspectionTarget::PublishedAspect(aspect.clone())
    );
    assert_eq!(
        inspection.lookup_receipt().family(),
        UiGraphLookupFamily::PublishedAspect
    );
    assert!(inspection
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::Aspect(aspect)));
    assert!(inspection
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::GraphNode(control_id)));
    assert!(!inspection
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::GraphNode(competing_id)));
    assert!(inspection
        .evidence_refs()
        .iter()
        .any(|reference| matches!(reference, UiGraphEvidenceRef::MountEligibility(_))));
}

#[test]
fn consumed_aspect_inspection_excludes_competing_consumers() {
    let app = inspection_app();
    let graph = app.graph();
    let control = artifact_from_file_provenance(&app, "app/graph_inspection_runtime.wui", 0);
    let consumer = artifact_from_file_provenance(&app, "app/graph_inspection_runtime.wui", 1);
    let competing = artifact_from_file_provenance(&app, "app/graph_inspection_runtime.wui", 3);
    let consumer_id = graph_node_identity(graph, consumer);
    let competing_id = graph_node_identity(graph, competing);
    let aspect = published_aspect(control);
    let inspection = graph.inspection().inspect_aspect_consumers(&aspect);

    assert_eq!(
        inspection.target(),
        &UiGraphInspectionTarget::ConsumedAspect(aspect.clone())
    );
    assert_eq!(
        inspection.lookup_receipt().family(),
        UiGraphLookupFamily::ConsumedAspect
    );
    assert_eq!(
        inspection.lookup_receipt().cost_class(),
        UiGraphLookupCostClass::IndexedNeighborhood
    );
    assert!(inspection
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::Aspect(aspect)));
    assert!(inspection
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::GraphNode(consumer_id)));
    assert!(!inspection
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::GraphNode(competing_id)));
    assert!(inspection
        .evidence_refs()
        .iter()
        .any(|reference| matches!(reference, UiGraphEvidenceRef::MountEligibility(_))));
}

#[test]
fn competing_aspect_inspections_preserve_their_own_publisher_and_consumer() {
    let app = inspection_app();
    let graph = app.graph();
    let control = artifact_from_file_provenance(&app, "app/graph_inspection_runtime.wui", 0);
    let consumer = artifact_from_file_provenance(&app, "app/graph_inspection_runtime.wui", 1);
    let competing_control =
        artifact_from_file_provenance(&app, "app/graph_inspection_runtime.wui", 2);
    let competing_consumer =
        artifact_from_file_provenance(&app, "app/graph_inspection_runtime.wui", 3);
    let control_id = graph_node_identity(graph, control);
    let consumer_id = graph_node_identity(graph, consumer);
    let competing_control_id = graph_node_identity(graph, competing_control);
    let competing_consumer_id = graph_node_identity(graph, competing_consumer);
    let aspect = published_aspect(competing_control);
    let publisher = graph.inspection().inspect_aspect_publishers(&aspect);
    let consumer = graph.inspection().inspect_aspect_consumers(&aspect);

    assert!(publisher
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::GraphNode(competing_control_id)));
    assert!(!publisher
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::GraphNode(control_id)));
    assert!(consumer
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::GraphNode(competing_consumer_id)));
    assert!(!consumer
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::GraphNode(consumer_id)));
}

#[test]
fn mount_eligibility_inspection_reports_exact_slot_and_graph_node() {
    let app = inspection_app();
    let graph = app.graph();
    let control = artifact_from_file_provenance(&app, "app/graph_inspection_runtime.wui", 0);
    let control_id = graph_node_identity(graph, control);
    let eligibility_id = graph
        .lookup()
        .mount_eligibility_slot_for_node(control_id)
        .expect("mount eligibility slot should resolve for control")
        .value()
        .mount_eligibility_identity();
    let inspection = graph
        .inspection()
        .inspect_mount_eligibility_slot(eligibility_id)
        .expect("mount eligibility inspection should target committed slot");

    assert_eq!(
        inspection.target(),
        &UiGraphInspectionTarget::MountEligibility(eligibility_id)
    );
    assert_eq!(
        inspection.lookup_receipt().family(),
        UiGraphLookupFamily::MountEligibilitySlot
    );
    assert_eq!(
        inspection.lookup_receipt().cost_class(),
        UiGraphLookupCostClass::IndexedScalar
    );
    assert!(inspection
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::MountEligibility(eligibility_id)));
    assert!(inspection
        .evidence_refs()
        .contains(&UiGraphEvidenceRef::GraphNode(control_id)));
}
