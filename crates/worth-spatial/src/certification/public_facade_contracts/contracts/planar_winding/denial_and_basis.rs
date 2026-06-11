use worth_spatial::facade::planar_winding::{
    CertifiedPolygonWinding2D, CertifiedPolygonWinding2DDenialKind,
    CertifiedPolygonWinding2DFactError, CertifiedProjectedLoop2D,
};

use super::proof_fixture::{certified_frame, loop_points, winding_contracts};
use super::surface_contract::topology_basis;

#[test]
fn certified_polygon_winding_denies_self_intersection_and_ambiguous_touch() {
    let world = "winding-bowtie";
    let frame = certified_frame(world, "movement:stable");
    let bowtie = CertifiedProjectedLoop2D::from_projected_vertices(
        "loop:bowtie",
        topology_basis("loop:bowtie"),
        loop_points(
            world,
            &frame,
            "bowtie",
            &[[0.0, 0.0], [3.0, 3.0], [0.0, 3.0], [3.0, 0.0]],
        ),
    )
    .expect("bowtie loop");

    let error = CertifiedPolygonWinding2D::certify(bowtie)
        .within_planar_neighborhood("topology:bowtie")
        .compile(&winding_contracts(world))
        .expect("plan")
        .certify()
        .expect_err("figure-eight must deny");

    assert_winding_denial_kind(
        error,
        CertifiedPolygonWinding2DDenialKind::SelfIntersectionOrAmbiguousTouch,
    );
}

#[test]
fn certified_polygon_winding_denies_containment_boundary_touch_as_ambiguous() {
    let world = "winding-boundary-touch";
    let frame = certified_frame(world, "movement:stable");
    let outer = CertifiedProjectedLoop2D::from_projected_vertices(
        "loop:outer",
        topology_basis("loop:outer"),
        loop_points(
            world,
            &frame,
            "outer-touch",
            &[[0.0, 0.0], [4.0, 0.0], [4.0, 4.0], [0.0, 4.0]],
        ),
    )
    .expect("outer loop");
    let touching_hole = CertifiedProjectedLoop2D::from_projected_vertices(
        "loop:touching-hole",
        topology_basis("loop:touching-hole"),
        loop_points(
            world,
            &frame,
            "touching-hole",
            &[[0.0, 1.0], [1.0, 1.0], [1.0, 2.0], [0.0, 2.0]],
        ),
    )
    .expect("touching hole");

    let error = CertifiedPolygonWinding2D::certify(outer)
        .with_containment_candidate(touching_hole)
        .within_planar_neighborhood("topology:touching-hole")
        .compile(&winding_contracts(world))
        .expect("plan")
        .certify()
        .expect_err("boundary touch must deny");

    assert_winding_denial_kind(
        error,
        CertifiedPolygonWinding2DDenialKind::SelfIntersectionOrAmbiguousTouch,
    );
}

#[test]
fn certified_polygon_winding_denies_duplicate_vertices_before_facts() {
    let world = "winding-duplicate";
    let frame = certified_frame(world, "movement:stable");
    let duplicate = CertifiedProjectedLoop2D::from_projected_vertices(
        "loop:duplicate",
        topology_basis("loop:duplicate"),
        loop_points(
            world,
            &frame,
            "duplicate",
            &[[0.0, 0.0], [3.0, 0.0], [3.0, 0.0], [0.0, 3.0]],
        ),
    )
    .expect("duplicate loop");

    let contracts = winding_contracts(world);
    let denial = match CertifiedPolygonWinding2D::certify(duplicate)
        .within_planar_neighborhood("topology:duplicate")
        .compile(&contracts)
    {
        Ok(_) => panic!("duplicate vertex must deny before plan"),
        Err(denial) => denial,
    };

    assert_eq!(
        denial.kind(),
        CertifiedPolygonWinding2DDenialKind::DuplicateVertex
    );
}

#[test]
fn certified_polygon_winding_denies_mixed_movement_rotation_basis() {
    let world = "winding-mixed-movement";
    let stable = certified_frame(world, "movement:stable");
    let moved = certified_frame(world, "movement:moved");
    let mut points = loop_points(
        world,
        &stable,
        "mixed",
        &[[0.0, 0.0], [3.0, 0.0], [3.0, 3.0]],
    );
    points.push(loop_points(world, &moved, "mixed-moved", &[[0.0, 3.0]])[0].clone());

    let loop_ = CertifiedProjectedLoop2D::from_projected_vertices(
        "loop:mixed",
        topology_basis("loop:mixed"),
        points,
    )
    .expect("mixed loop");

    let contracts = winding_contracts(world);
    let denial = match CertifiedPolygonWinding2D::certify(loop_)
        .within_planar_neighborhood("topology:mixed")
        .compile(&contracts)
    {
        Ok(_) => panic!("mixed movement must deny"),
        Err(denial) => denial,
    };

    assert_eq!(
        denial.kind(),
        CertifiedPolygonWinding2DDenialKind::MovementRotationMismatch
    );
}

fn assert_winding_denial_kind(
    error: CertifiedPolygonWinding2DFactError,
    expected: CertifiedPolygonWinding2DDenialKind,
) {
    match error {
        CertifiedPolygonWinding2DFactError::WindingBasis { denial } => {
            assert_eq!(denial.kind(), expected);
        }
        other => panic!("expected winding basis denial, got {other:?}"),
    }
}
