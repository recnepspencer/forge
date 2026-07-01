use super::*;

#[test]
fn selected_route_packet_carries_route_and_support_identity_once() {
    let packet = current_worth_touched_graph_conflict_selected_route_packet()
        .expect("selected-route packet should build from current proof surfaces");

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
