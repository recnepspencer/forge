use forge_query::facade::ForgeQueryDeclarationFamilyMarker;
use worth_spatial::facade::planar_winding::{
    CertifiedLoopContainment, CertifiedLoopWinding, CertifiedPolygonWinding2D,
    CertifiedPolygonWinding2DDeclarationFamily, CertifiedPolygonWinding2DQueryDomain,
    CertifiedPolygonWinding2DQueryWorld, CertifiedProjectedLoop2D, CertifiedTopologyLoopBasis2D,
    WindingPolicy,
};

use super::proof_fixture::{certified_frame, loop_points, winding_contracts};

#[test]
fn spatial_public_facade_exports_readable_polygon_winding_surface() {
    let _: Option<CertifiedProjectedLoop2D> = None;
    let _: Option<CertifiedPolygonWinding2D> = None;
    let _: CertifiedPolygonWinding2DDeclarationFamily = CertifiedPolygonWinding2DDeclarationFamily;
    let _: CertifiedPolygonWinding2DQueryDomain = CertifiedPolygonWinding2DQueryDomain;
    let _: CertifiedPolygonWinding2DQueryWorld = CertifiedPolygonWinding2DQueryWorld::new("public");
    let _: WindingPolicy = WindingPolicy::DenySelfIntersectionAndAmbiguousTouch;
    let _: CertifiedLoopWinding = CertifiedLoopWinding::CounterClockwise;
    let _: CertifiedLoopContainment = CertifiedLoopContainment::ContainedHole;
}

#[test]
fn certified_polygon_winding_family_is_query_native_and_retained() {
    let aspect_contract = CertifiedPolygonWinding2DDeclarationFamily::aspect_contract();

    assert_eq!(
        CertifiedPolygonWinding2DDeclarationFamily::semantic_family_key(),
        "CertifiedPolygonWinding2D"
    );
    assert_eq!(
        CertifiedPolygonWinding2DDeclarationFamily::route_contract().reason(),
        "the declaration lowers through one relational route"
    );
    assert!(aspect_contract
        .required()
        .contains(&crate::query_contract_helpers::aspect_field_key(
            "geometry.polygon_winding_2d.vertex.projection_fact"
        )));
    assert!(aspect_contract.preserved().contains(
        &crate::query_contract_helpers::aspect_field_key(
            "geometry.polygon_winding_2d.loop.containment"
        )
    ));
}

#[test]
fn nested_hole_winding_rows_are_retained_and_replayable() {
    let world = "winding-dx";
    let frame = certified_frame(world, "movement:stable");
    let outer_points = loop_points(
        world,
        &frame,
        "outer",
        &[[0.0, 0.0], [4.0, 0.0], [4.0, 4.0], [0.0, 4.0]],
    );
    let hole_points = loop_points(
        world,
        &frame,
        "hole",
        &[[1.0, 1.0], [2.0, 1.0], [2.0, 2.0], [1.0, 2.0]],
    );
    let outer = CertifiedProjectedLoop2D::from_projected_vertices(
        "loop:outer",
        topology_basis("loop:outer"),
        outer_points.clone(),
    )
    .expect("outer loop");
    let hole = CertifiedProjectedLoop2D::from_projected_vertices(
        "loop:hole-a",
        topology_basis("loop:hole-a"),
        hole_points.clone(),
    )
    .expect("hole loop");

    let contracts = winding_contracts(world);
    let plan = CertifiedPolygonWinding2D::certify(outer)
        .with_containment_candidate(hole)
        .within_planar_neighborhood("topology:face-42:planar-contract")
        .with_policy(WindingPolicy::DenySelfIntersectionAndAmbiguousTouch)
        .compile(&contracts)
        .expect("compiled winding plan");

    assert_eq!(plan.loop_count(), 2);
    assert_eq!(plan.segment_contact_pairs_required(), 20);
    assert_eq!(plan.projected_vertex_count(), 8);

    let receipt = plan.certify().expect("winding receipt");
    assert_eq!(
        receipt.primary_winding(),
        CertifiedLoopWinding::CounterClockwise
    );
    assert_eq!(
        receipt.containment_for("loop:hole-a"),
        Some(CertifiedLoopContainment::ContainedHole)
    );
    assert_eq!(receipt.counters().projected_vertices_consumed(), 8);
    assert_eq!(receipt.counters().loop_edges_walked(), 8);
    assert_eq!(receipt.counters().winding_predicates_consumed(), 4);
    assert_eq!(receipt.counters().segment_contact_possible_pairs(), 20);
    assert_eq!(receipt.counters().segment_contact_candidate_pairs(), 0);
    assert_eq!(receipt.counters().segment_contact_culled_pairs(), 20);
    assert_eq!(
        receipt
            .counters()
            .segment_contact_adjacent_self_pairs_skipped(),
        8
    );
    assert_eq!(receipt.counters().segment_contacts_classified(), 0);
    assert!(!receipt.counters().segment_contact_fallback_used());
    assert_eq!(receipt.counters().winding_tie_breaks_used(), 0);

    let replay_outer = CertifiedProjectedLoop2D::from_projected_vertices(
        "loop:outer",
        topology_basis("loop:outer"),
        outer_points,
    )
    .expect("replay outer loop");
    let replay_hole = CertifiedProjectedLoop2D::from_projected_vertices(
        "loop:hole-a",
        topology_basis("loop:hole-a"),
        hole_points,
    )
    .expect("replay hole loop");
    let replay = CertifiedPolygonWinding2D::certify(replay_outer)
        .with_containment_candidate(replay_hole)
        .within_planar_neighborhood("topology:face-42:planar-contract")
        .with_policy(WindingPolicy::DenySelfIntersectionAndAmbiguousTouch)
        .compile(&contracts)
        .expect("compiled replay winding plan")
        .certify()
        .expect("replayed winding receipt");
    assert_eq!(receipt.declaration_digest(), replay.declaration_digest());
    assert_eq!(receipt.fact_digest(), replay.fact_digest());
}

pub(crate) fn topology_basis(identity: &'static str) -> CertifiedTopologyLoopBasis2D {
    CertifiedTopologyLoopBasis2D::from_topology_loop_fact(
        identity,
        format!("membership:{identity}"),
        "topology-spatial-contract:face-42",
    )
}
