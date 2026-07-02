use crate::workload_platform::evidence_lookup_public_closeout::current_evidence_lookup_public_closeout;
use crate::workload_platform::planner_owned_routing::evidence_lookup_route::current_evidence_lookup_route_packet;

#[test]
fn spatial_public_closeout_route_explanation_consumes_planner_route_products_without_evidence_rescan(
) {
    let packet = current_evidence_lookup_route_packet().expect("public closeout route packet");
    let closeout = current_evidence_lookup_public_closeout().expect("public closeout");
    let selected_row = closeout
        .family_stage_rows()
        .iter()
        .find(|row| {
            row.family_identity() == packet.route_family_identity() && row.stage() == packet.stage()
        })
        .expect("selected route row");
    let seed = closeout.milestone_twelve_seed();

    assert_eq!(
        seed.selected_route_family_identity(),
        packet.route_family_identity()
    );
    assert_eq!(
        seed.selected_compiled_product_identity_digest(),
        packet.compiled_product_identity_digest()
    );
    assert_eq!(
        seed.selected_equivalence_family_identity(),
        packet.selected_equivalence_family_identity()
    );
    assert_eq!(
        seed.selected_reuse_basis_identity_digest(),
        packet.selected_reuse_basis_identity_digest()
    );
    assert_eq!(
        selected_row.spatial_compiled_product_identity_digest(),
        Some(seed.selected_compiled_product_identity_digest())
    );
    assert_eq!(
        selected_row.spatial_selected_equivalence_family_identity(),
        Some(seed.selected_equivalence_family_identity())
    );
    assert_eq!(packet.lowering_raw_row_revisit_count(), 0);
    assert_eq!(packet.lowering_right_receipt_revisit_count(), 0);
    assert_eq!(packet.lowering_caller_owned_revisit_count(), 0);
}
