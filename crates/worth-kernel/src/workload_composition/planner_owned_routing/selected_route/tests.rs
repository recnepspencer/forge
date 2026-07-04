use super::*;
use schema::facade::platform::authority::replay_undo_semantic_graph::ReplayUndoPlannerRouteFamily;
use worth_spatial::facade::replay_undo_semantic_graph::{
    current_boolean_event_ledger_spatial_boundary, current_projection_receipt_spatial_boundary,
};

fn admitted_radial_row(
    cutover: &topology::facade::TopologyQueryBackedConsumerCutover,
) -> &topology::facade::TopologyQueryBackedConsumerFamilyRow {
    cutover
        .family_rows()
        .iter()
        .find(|row| {
            row.request_family()
                == topology::query_domain::TopologyReadRequestFamily::HalfEdgeRadialNeighborhood
        })
        .expect("packet-driven cutover should retain a radial family row")
}

#[test]
fn selected_route_packet_carries_route_and_support_identity_once() {
    let packet = current_worth_touched_graph_conflict_selected_route_packet()
        .expect("selected-route packet should build from current proof surfaces");
    let spatial_route_packet =
        worth_spatial::facade::evidence_lookup_route::current_evidence_lookup_route_packet()
            .expect("spatial planner-owned route packet should build");
    let spatial_route_projection_markers =
        super::SpatialRouteProjectionMarkers::from_route_packet(&spatial_route_packet);
    let left_boundary =
        current_boolean_event_ledger_spatial_boundary().expect("current left replay/undo boundary");
    let right_boundary =
        current_projection_receipt_spatial_boundary().expect("current right replay/undo boundary");
    let left_handoff = left_boundary.workload_handoff();
    let right_handoff = right_boundary.workload_handoff();

    assert!(!packet.packet_digest().is_empty());
    assert!(!packet.selected_route_identity_digest().is_empty());
    assert!(!packet.decision_trace_identity_digest().is_empty());
    assert!(!packet.selected_family_identity().is_empty());
    assert!(!packet.spatial_selected_family_identity().is_empty());
    assert!(!packet.selected_product_identity_digest().is_empty());
    assert!(!packet.spatial_selected_product_identity_digest().is_empty());
    assert_eq!(packet.route_authority_digests().len(), 3);
    assert_eq!(packet.route_lineage_digests().len(), 3);
    assert!(!packet.source_firewall_digest().is_empty());
    assert!(!packet.deletion_closeout_digest().is_empty());
    assert_eq!(
        packet.spatial_selected_family_identity(),
        spatial_route_packet.selected_equivalence_family_identity()
    );
    assert_eq!(
        packet.spatial_selected_product_identity_digest(),
        spatial_route_packet.compiled_product_identity_digest()
    );
    assert_eq!(
        packet.replay_undo_route_family(),
        ReplayUndoPlannerRouteFamily::Transaction
    );
    assert!(!packet.replay_undo_route_packet_identity().is_empty());
    assert!(!spatial_route_packet
        .right_route_family_identity()
        .is_empty());
    assert!(!spatial_route_packet
        .right_stage_receipt_identity()
        .is_empty());
    assert!(!spatial_route_packet
        .right_lookup_execution_receipt_digest()
        .is_empty());
    assert!(!spatial_route_packet
        .right_authority_stage_index_identity()
        .is_empty());
    assert_eq!(
        spatial_route_packet.lowering_raw_row_revisit_count(),
        left_handoff.counters().raw_row_scan_count()
            + right_handoff.counters().raw_row_scan_count()
    );
    assert_eq!(
        spatial_route_packet.lowering_right_receipt_revisit_count(),
        left_handoff.counters().broad_receipt_scan_count()
            + right_handoff.counters().broad_receipt_scan_count()
    );
    assert_eq!(
        spatial_route_packet.lowering_caller_owned_revisit_count(),
        left_handoff.counters().caller_owned_scan_count()
            + right_handoff.counters().caller_owned_scan_count()
    );
    assert_eq!(
        packet.evidence_lookup_public_closeout_digest(),
        spatial_route_projection_markers.evidence_lookup_public_closeout_digest()
    );
    assert_eq!(
        packet.evidence_lookup_family_coverage_digest(),
        spatial_route_projection_markers.evidence_lookup_family_coverage_digest()
    );
    assert_eq!(
        packet.evidence_lookup_query_surface_matrix_digest(),
        spatial_route_projection_markers.evidence_lookup_query_surface_matrix_digest()
    );
    assert_eq!(
        packet.evidence_lookup_query_consumer_kit_digest(),
        spatial_route_projection_markers.evidence_lookup_query_consumer_kit_digest()
    );
    assert_eq!(
        packet.evidence_lookup_query_boundary_support_digest(),
        spatial_route_projection_markers.evidence_lookup_query_boundary_support_digest()
    );
}

#[test]
fn selected_route_packet_rejects_mismatched_topology_support() {
    let error = current_worth_touched_graph_conflict_selected_route_packet_with_support_loaders(
        || topology::certification::current_topology_milestone_fifteen_planner_seed_support_with_hostile_selected_reuse_basis_identity_digest(
            "foreign-topology-selected-reuse-basis",
        ),
        worth_spatial::certification::current_spatial_milestone_fifteen_planner_seed_support,
    )
    .expect_err("foreign topology support should be rejected");

    assert_eq!(
        error.kind(),
        crate::workload_composition::planner_owned_routing::PlannerOwnedRoutingErrorKind::MismatchedSelectedRouteSupport
    );
}

#[test]
fn selected_route_packet_is_the_authority_for_topology_query_backed_route_admission() {
    let packet = current_worth_touched_graph_conflict_selected_route_packet()
        .expect("selected-route packet should build from current proof surfaces");
    let cutover = topology::certification::admit_current_topology_query_backed_consumer_cutover_with_selected_route_authority(&packet)
        .expect("packet-aligned topology route admission should succeed");
    let radial_row = admitted_radial_row(&cutover);

    assert_eq!(
        cutover.support_snapshot_digest(),
        packet.topology_query_support_snapshot_digest()
    );
    assert_eq!(
        cutover.handle_identity_digest(),
        packet.topology_query_handle_identity_digest()
    );
    assert_eq!(
        cutover.operating_context_identity_digest(),
        packet.topology_query_operating_context_identity_digest()
    );
    assert_eq!(
        cutover.parity_verified_count(),
        packet.topology_query_parity_verified_count()
    );
    assert_eq!(
        radial_row.reuse_posture(),
        topology::facade::TopologyReadModelReusePosture::ReuseAdmitted
    );
    assert_eq!(
        radial_row.selected_equivalence_family_identity(),
        Some(packet.selected_family_identity())
    );
    assert_eq!(
        radial_row.selected_equivalence_basis_identity_digest(),
        Some(packet.selected_equivalence_basis_identity_digest())
    );
    assert_eq!(
        radial_row.selected_compatibility_basis_identity_digest(),
        Some(packet.selected_compatibility_basis_identity_digest())
    );
    assert_eq!(
        radial_row.selected_reuse_basis_identity_digest(),
        Some(packet.selected_reuse_basis_identity_digest())
    );
    assert_eq!(
        radial_row.reuse_decision_identity_digest(),
        packet.selected_witness_identity_digest()
    );
    assert_eq!(
        radial_row.rebuild_denial_identity_digest(),
        packet.rebuild_denial_identity_digest()
    );
    assert_eq!(
        radial_row.query_execution_count(),
        packet.topology_query_execution_count()
    );
    assert_eq!(
        radial_row.row_scan_fallback_count(),
        packet.topology_row_scan_fallback_count()
    );
    assert_eq!(
        radial_row.whole_view_fallback_count(),
        packet.topology_whole_view_fallback_count()
    );
    assert_eq!(
        radial_row.repeated_rediscovery_denied_count(),
        packet.topology_repeated_rediscovery_denied_count()
    );
}

#[test]
fn packet_driven_topology_route_admission_rejects_foreign_query_posture() {
    let packet = current_worth_touched_graph_conflict_selected_route_packet()
        .expect("selected-route packet should build from current proof surfaces")
        .with_test_topology_query_support_snapshot_digest_override(
            "foreign-query-support-snapshot",
        );
    let error = topology::certification::admit_current_topology_query_backed_consumer_cutover_with_selected_route_authority(&packet)
        .expect_err("foreign packet support posture should be rejected");

    assert!(error.detail().contains("query support snapshot"));
}

#[test]
fn packet_driven_topology_route_admission_rejects_mismatched_selected_reuse_basis() {
    let packet = current_worth_touched_graph_conflict_selected_route_packet()
        .expect("selected-route packet should build from current proof surfaces")
        .with_test_selected_reuse_basis_identity_digest_override("foreign-selected-reuse-basis");
    let error = topology::certification::admit_current_topology_query_backed_consumer_cutover_with_selected_route_authority(&packet)
        .expect_err("packet with a foreign selected reuse basis should be rejected");

    assert!(error.detail().contains("selected reuse basis identity"));
}
