use worth_spatial::facade::planar_winding::{
    CertifiedLoopWinding, CertifiedPolygonWinding2D, CertifiedProjectedLoop2D,
};

use super::proof_fixture::{certified_frame, loop_points, winding_contracts};
use super::surface_contract::topology_basis;

#[test]
fn certified_polygon_winding_is_stable_under_loop_rotation_and_reversal() {
    let world = "winding-stability";
    let frame = certified_frame(world, "movement:stable");
    let vertices = loop_points(
        world,
        &frame,
        "stable",
        &[[0.0, 0.0], [3.0, 0.0], [3.0, 3.0], [0.0, 3.0]],
    );
    let canonical = receipt_for(world, vertices.clone());
    let rotated = receipt_for(
        world,
        vec![
            vertices[1].clone(),
            vertices[2].clone(),
            vertices[3].clone(),
            vertices[0].clone(),
        ],
    );
    let reversed = receipt_for(
        world,
        vec![
            vertices[0].clone(),
            vertices[3].clone(),
            vertices[2].clone(),
            vertices[1].clone(),
        ],
    );

    assert_eq!(
        canonical.primary_winding(),
        CertifiedLoopWinding::CounterClockwise
    );
    assert_eq!(
        rotated.primary_winding(),
        CertifiedLoopWinding::CounterClockwise
    );
    assert_eq!(reversed.primary_winding(), CertifiedLoopWinding::Clockwise);
    assert_eq!(canonical.declaration_digest(), rotated.declaration_digest());
    assert_eq!(canonical.fact_digest(), rotated.fact_digest());
    assert_ne!(canonical.fact_digest(), reversed.fact_digest());
    assert_eq!(canonical.counters().loop_edges_walked(), 4);
    assert_eq!(canonical.counters().segment_contact_possible_pairs(), 2);
    assert_eq!(canonical.counters().segment_contact_candidate_pairs(), 0);
    assert_eq!(canonical.counters().segment_contact_culled_pairs(), 2);
    assert_eq!(
        canonical
            .counters()
            .segment_contact_adjacent_self_pairs_skipped(),
        4
    );
    assert_eq!(canonical.counters().segment_contacts_classified(), 0);
    assert!(!canonical.counters().segment_contact_fallback_used());
    assert_eq!(canonical.counters().winding_predicates_consumed(), 2);
}

fn receipt_for(
    world: &'static str,
    vertices: Vec<worth_spatial::facade::planar_projection::ProjectPointToCertifiedPlane2DReceipt>,
) -> worth_spatial::facade::planar_winding::CertifiedPolygonWinding2DReceipt {
    let loop_ = CertifiedProjectedLoop2D::from_projected_vertices(
        "loop:stable",
        topology_basis("loop:stable"),
        vertices,
    )
    .expect("loop");
    let contracts = winding_contracts(world);
    CertifiedPolygonWinding2D::certify(loop_)
        .within_planar_neighborhood("topology:stable")
        .compile(&contracts)
        .expect("plan")
        .certify()
        .expect("receipt")
}
