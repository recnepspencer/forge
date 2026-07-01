use super::*;

#[test]
fn admitted_public_proof_input_lowers_from_selected_route_packet() {
    let packet = crate::workload_composition::planner_owned_routing::current_worth_touched_graph_conflict_selected_route_packet()
        .expect("selected-route packet should build");
    let input = admit_worth_touched_graph_conflict_public_proof_input(&packet)
        .expect("public proof input should lower from selected-route packet");

    assert_eq!(input.selected_route_packet_digest(), packet.packet_digest());
    assert_eq!(
        input.selected_route_identity_digest(),
        packet.selected_route_identity_digest()
    );
    assert_eq!(input.selected_family_identity(), packet.selected_family_identity());
    assert_eq!(
        input.selected_product_identity_digest(),
        packet.selected_product_identity_digest()
    );
    assert_eq!(
        input.selected_witness_identity_digest(),
        packet.selected_witness_identity_digest()
    );
}

