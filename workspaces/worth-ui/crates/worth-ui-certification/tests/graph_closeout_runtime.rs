use worth_ui::facade::app::WorthUi;
use worth_ui::facade::declaration::{UiAspectName, UiAspectSemanticSlice};
use worth_ui::facade::graph::{
    UiGraphClosedSemanticLane, UiGraphCloseoutGuarantee, UiGraphCloseoutNonGoal,
    UiGraphEvidenceRefKind, UiGraphInspectionStopPoint, UiGraphInspectionTargetKind,
    UiGraphLookupFamily, UiGraphParticipationAxis, UiGraphParticipationStatus,
};
use worth_ui_dsl::{
    UiDslAspectName, UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily,
    UiDslSemanticKey, UiDslSourceProvenance, UiDslStructuralToken, WorthUiDslPackage,
};

#[test]
fn graph_closeout_report_enumerates_shipped_graph_lanes_and_explicit_non_goals() {
    let app = graph_closeout_app();
    let report = app.graph_closeout_report();
    let inspection_support = report.inspection_support();

    for lane in [
        UiGraphClosedSemanticLane::NodeIdentity,
        UiGraphClosedSemanticLane::DeclarationCorrespondence,
        UiGraphClosedSemanticLane::AttachmentPosture,
        UiGraphClosedSemanticLane::ParticipationPosture,
        UiGraphClosedSemanticLane::MountedReceiptAuthority,
        UiGraphClosedSemanticLane::AspectIndexes,
        UiGraphClosedSemanticLane::InspectionSupport,
    ] {
        assert!(report.closed_semantic_lanes().contains(&lane));
    }

    for guarantee in [
        UiGraphCloseoutGuarantee::GraphAndIndexMutationCommitAsOneGenerationTransition,
        UiGraphCloseoutGuarantee::OrdinaryLookupRemainsReceiptBackedAndBounded,
        UiGraphCloseoutGuarantee::HandoffConsumesProofBearingGraphAuthorityRatherThanRawInternals,
    ] {
        assert!(report.guarantees().contains(&guarantee));
    }

    for non_goal in [
        UiGraphCloseoutNonGoal::QueryExecution,
        UiGraphCloseoutNonGoal::TouchedObligationSelection,
        UiGraphCloseoutNonGoal::HostObservation,
        UiGraphCloseoutNonGoal::SideTopologiesOutsideGraphAuthority,
    ] {
        assert!(report.non_goals().contains(&non_goal));
    }

    for target in [
        UiGraphInspectionTargetKind::GraphNode,
        UiGraphInspectionTargetKind::TopologyNode,
        UiGraphInspectionTargetKind::PageParticipation,
        UiGraphInspectionTargetKind::MountedReceipt,
    ] {
        assert!(inspection_support.target_kinds().contains(&target));
    }

    for evidence_ref in [
        UiGraphEvidenceRefKind::GraphNode,
        UiGraphEvidenceRefKind::MountedReceipt,
        UiGraphEvidenceRefKind::Aspect,
    ] {
        assert!(inspection_support
            .evidence_ref_kinds()
            .contains(&evidence_ref));
    }

    for stop_point in [
        UiGraphInspectionStopPoint::TopologyTruth,
        UiGraphInspectionStopPoint::ParticipationTruth,
        UiGraphInspectionStopPoint::MountedReceiptAuthority,
    ] {
        assert!(inspection_support.stop_points().contains(&stop_point));
    }
}

#[test]
fn graph_handoff_surface_derives_phase_34_inputs_without_declaration_reopening() {
    let app = graph_closeout_app();
    let graph = app.graph();
    let published_aspect = UiAspectName::from_semantic_slice(UiAspectSemanticSlice::ContentText);
    let publisher_lookup = graph.lookup().published_aspect(&published_aspect);
    let publisher = publisher_lookup
        .value()
        .iter()
        .find_map(|publisher| match publisher.kind() {
            worth_ui::facade::graph::UiGraphAspectPublisherKind::GraphNode(node_identity) => {
                Some(node_identity)
            }
            worth_ui::facade::graph::UiGraphAspectPublisherKind::MountedReceiptSlot(_)
            | worth_ui::facade::graph::UiGraphAspectPublisherKind::FutureReceipt => None,
        })
        .expect("published aspect should resolve one graph-owned publisher node");
    let node = graph
        .inspection()
        .inspect_graph_node(publisher)
        .expect("graph handoff should inspect publisher node")
        .value();
    let topology = graph
        .inspection()
        .inspect_topology_node(publisher)
        .expect("graph handoff should inspect publisher topology")
        .value();
    let page_node_identity = topology
        .page_membership()
        .expect("publisher node should carry explicit page membership")
        .page_node_identity();
    let page_participation = graph
        .inspection()
        .inspect_page_participation(page_node_identity, UiGraphParticipationAxis::QueryBound);
    let declaration_lookup = graph
        .lookup()
        .declaration_instances(node.declaration_identity());
    let mounted_receipt_slot = graph
        .lookup()
        .mounted_receipt_slot_for_node(publisher)
        .expect("graph handoff should resolve mounted receipt authority seed")
        .value();
    let mounted_receipt_inspection = graph
        .inspection()
        .inspect_mounted_receipt_slot(mounted_receipt_slot.mounted_receipt_identity())
        .expect("graph handoff should inspect mounted receipt authority");

    assert_eq!(
        publisher_lookup.receipt().family(),
        UiGraphLookupFamily::PublishedAspect
    );
    assert_eq!(declaration_lookup.value(), &[publisher]);
    assert!(node.attachment_posture().query_binding_attached());
    assert_eq!(
        node.participation_posture()
            .axis(UiGraphParticipationAxis::Mounted)
            .status(),
        UiGraphParticipationStatus::Deferred
    );
    assert!(page_participation
        .value()
        .iter()
        .any(|member| member.member_node_identity() == publisher));
    assert!(mounted_receipt_slot
        .authority_seed()
        .graph_owned_slot_reserved());
    assert!(mounted_receipt_inspection
        .evidence_refs()
        .iter()
        .any(|evidence_ref| evidence_ref.kind() == UiGraphEvidenceRefKind::MountedReceipt));
}

fn graph_closeout_app() -> worth_ui::facade::app::WorthUiApp {
    WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.graph-closeout")
                .with_semantic_artifact_spec(control_spec())
                .with_semantic_artifact_spec(consumer_spec()),
        )
        .freeze()
}

fn control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.publish_text"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/graph_closeout_runtime.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:publish-text"))
    .with_structural_token(UiDslStructuralToken::new("slot:footer"))
    .with_posture_token(UiDslPostureToken::new("query-binding:attached:view"))
    .with_published_aspect(UiDslAspectName::new("content.text"))
}

fn consumer_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.region.consume_text"),
        UiDslSemanticFamily::Region,
        UiDslSourceProvenance::file_authored("app/graph_closeout_runtime.wui", 1),
    )
    .with_structural_token(UiDslStructuralToken::new("region:consume-text"))
    .with_consumed_aspect(UiDslAspectName::new("content.text"))
}
