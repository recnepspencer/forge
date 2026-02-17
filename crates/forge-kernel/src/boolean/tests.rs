//! Tests for Boolean operations.

use forge_geom::bsp::{build_convex_polyhedron, BspConfig};
use forge_geom::plane::Plane;
use crate::core::ModelingContext;
use crate::mesh_builder::build_halfedge_mesh;
use super::schema::{BooleanInput, BooleanOp};
use super::classify::classify_faces;
use super::schema::FaceOrigin;
use super::assemble::execute_boolean;
use super::split::split_all_faces;

/// Build a mesh from 6 axis-aligned planes forming a cube.
fn build_cube(
    center: [f64; 3],
    half_size: f64,
) -> (forge_topo::state::TopologyState, crate::geometry_store::GeometryStore) {
    let planes = vec![
        Plane::from_point_normal(
            [center[0] + half_size, center[1], center[2]],
            [1.0, 0.0, 0.0],
        ).unwrap(),
        Plane::from_point_normal(
            [center[0] - half_size, center[1], center[2]],
            [-1.0, 0.0, 0.0],
        ).unwrap(),
        Plane::from_point_normal(
            [center[0], center[1] + half_size, center[2]],
            [0.0, 1.0, 0.0],
        ).unwrap(),
        Plane::from_point_normal(
            [center[0], center[1] - half_size, center[2]],
            [0.0, -1.0, 0.0],
        ).unwrap(),
        Plane::from_point_normal(
            [center[0], center[1], center[2] + half_size],
            [0.0, 0.0, 1.0],
        ).unwrap(),
        Plane::from_point_normal(
            [center[0], center[1], center[2] - half_size],
            [0.0, 0.0, -1.0],
        ).unwrap(),
    ];
    let cell = build_convex_polyhedron(&planes, &BspConfig::default()).unwrap();
    let mut ctx = ModelingContext::new();
    build_halfedge_mesh(&cell, &mut ctx).unwrap().into_parts()
}

#[test]
fn boolean_input_construction() {
    let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);
    let (topo_b, geom_b) = build_cube([0.5, 0.0, 0.0], 1.0);

    let input = BooleanInput::new(
        topo_a, geom_a,
        topo_b, geom_b,
        BooleanOp::Union,
    );
    assert_eq!(input.operation(), BooleanOp::Union);
}

#[test]
fn classify_disjoint_cubes() {
    let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);
    let (topo_b, geom_b) = build_cube([5.0, 5.0, 5.0], 1.0);

    let classified = classify_faces(
        topo_a.arena(),
        &geom_a,
        topo_b.arena(),
        &geom_b,
        FaceOrigin::Target,
    );

    assert!(classified.is_ok());
    let faces = classified.unwrap();
    assert!(!faces.is_empty());

    for face in &faces {
        assert_eq!(
            face.classification(),
            super::schema::FaceClassification::Outside,
        );
    }
}

#[test]
fn classify_overlapping_cubes_has_inside_faces() {
    let (topo_a, geom_a) = build_cube([0.1, 0.2, 0.3], 1.0);
    let (topo_b, geom_b) = build_cube([0.1, 0.2, 0.3], 2.0);

    let classified = classify_faces(
        topo_a.arena(),
        &geom_a,
        topo_b.arena(),
        &geom_b,
        FaceOrigin::Target,
    );

    assert!(classified.is_ok());
    let faces = classified.unwrap();

    let inside_count = faces.iter()
        .filter(|f| f.classification() == super::schema::FaceClassification::Inside)
        .count();

    assert!(inside_count > 0, "Expected some faces to be classified as Inside");
}

#[test]
fn union_of_disjoint_cubes() {
    let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);
    let (topo_b, geom_b) = build_cube([5.0, 5.0, 5.0], 1.0);

    let input = BooleanInput::new(
        topo_a, geom_a,
        topo_b, geom_b,
        BooleanOp::Union,
    );

    let result = execute_boolean(input);
    assert!(result.is_ok(), "Union failed: {:?}", result.err());

    let bool_result = result.unwrap();
    assert_eq!(bool_result.target_faces_kept(), 6);
    assert_eq!(bool_result.tool_faces_kept(), 6);
}

#[test]
fn intersection_of_concentric_cubes() {
    let (topo_a, geom_a) = build_cube([0.1, 0.2, 0.3], 2.0);
    let (topo_b, geom_b) = build_cube([0.1, 0.2, 0.3], 1.0);

    let input = BooleanInput::new(
        topo_a, geom_a,
        topo_b, geom_b,
        BooleanOp::Intersection,
    );

    let result = execute_boolean(input);
    assert!(result.is_ok(), "Intersection failed: {:?}", result.err());

    let bool_result = result.unwrap();
    assert_eq!(bool_result.target_faces_kept(), 0, "Outer faces should all be outside inner");
    assert_eq!(bool_result.tool_faces_kept(), 6, "Inner faces should all be inside outer");
}

#[test]
fn subtraction_of_concentric_cubes() {
    let (topo_a, geom_a) = build_cube([0.1, 0.2, 0.3], 2.0);
    let (topo_b, geom_b) = build_cube([0.1, 0.2, 0.3], 1.0);

    let input = BooleanInput::new(
        topo_a, geom_a,
        topo_b, geom_b,
        BooleanOp::Subtraction,
    );

    let result = execute_boolean(input);
    assert!(result.is_ok(), "Subtraction failed: {:?}", result.err());

    let bool_result = result.unwrap();
    assert_eq!(bool_result.target_faces_kept(), 54, "Outer cube splits into 54 faces by inner planes, all outside inner");
    assert_eq!(bool_result.tool_faces_kept(), 6, "Inner faces should all be inside outer");
}

#[test]
fn boolean_result_has_topology() {
    let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);
    let (topo_b, geom_b) = build_cube([5.0, 5.0, 5.0], 1.0);

    let input = BooleanInput::new(
        topo_a, geom_a,
        topo_b, geom_b,
        BooleanOp::Union,
    );

    let result = execute_boolean(input).unwrap();
    assert_eq!(result.topology().arena().vertex_count(), 16);
    assert_eq!(result.topology().arena().face_count(), 12);
    assert!(result.geometry().vertex_position_count() > 0);
}

#[test]
fn debug_split_counts_concentric() {
    let (topo_a, geom_a) = build_cube([0.1, 0.2, 0.3], 2.0);
    let (topo_b, geom_b) = build_cube([0.1, 0.2, 0.3], 1.0);

    eprintln!("Before split: target faces={}, tool faces={}",
        topo_a.arena().face_count(), topo_b.arena().face_count());

    let result = split_all_faces(topo_a, geom_a, topo_b, geom_b).unwrap();
    let (t_topo, t_geom, l_topo, l_geom) = result.into_parts();

    eprintln!("After split: target faces={}, tool faces={}",
        t_topo.arena().face_count(), l_topo.arena().face_count());
    eprintln!("Target: V={}, E={}, F={}",
        t_topo.arena().vertex_count(),
        t_topo.arena().half_edge_count() / 2,
        t_topo.arena().face_count());
    eprintln!("Tool: V={}, E={}, F={}",
        l_topo.arena().vertex_count(),
        l_topo.arena().half_edge_count() / 2,
        l_topo.arena().face_count());

    let target_classified = classify_faces(
        t_topo.arena(), &t_geom,
        l_topo.arena(), &l_geom,
        FaceOrigin::Target,
    ).unwrap();
    let tool_classified = classify_faces(
        l_topo.arena(), &l_geom,
        t_topo.arena(), &t_geom,
        FaceOrigin::Tool,
    ).unwrap();

    use super::schema::FaceClassification;
    let target_inside = target_classified.iter()
        .filter(|f| f.classification() == FaceClassification::Inside)
        .count();
    let target_outside = target_classified.iter()
        .filter(|f| f.classification() == FaceClassification::Outside)
        .count();
    let target_boundary = target_classified.iter()
        .filter(|f| f.classification() == FaceClassification::OnBoundary)
        .count();
    let tool_inside = tool_classified.iter()
        .filter(|f| f.classification() == FaceClassification::Inside)
        .count();
    let tool_outside = tool_classified.iter()
        .filter(|f| f.classification() == FaceClassification::Outside)
        .count();
    let tool_boundary = tool_classified.iter()
        .filter(|f| f.classification() == FaceClassification::OnBoundary)
        .count();

    eprintln!("Target: inside={}, outside={}, boundary={}", target_inside, target_outside, target_boundary);
    eprintln!("Tool: inside={}, outside={}, boundary={}", tool_inside, tool_outside, tool_boundary);

    for (fid, _) in t_topo.arena().iter_faces() {
        let face_data = t_topo.arena().get_face(fid).unwrap();
        let loop_data = t_topo.arena().get_loop(face_data.outer_loop).unwrap();
        let start = loop_data.half_edge;
        let mut cur = start;
        let mut verts = Vec::new();
        for _ in 0..100 {
            let he = t_topo.arena().get_half_edge(cur).unwrap();
            let pos = t_geom.get_vertex_position(he.origin);
            verts.push((he.origin, pos.cloned()));
            cur = he.next;
            if cur == start { break; }
        }
        eprintln!("Target face {}: {} verts: {:?}", fid, verts.len(),
            verts.iter().map(|(vid, p)| format!("{}: {:?}", vid, p)).collect::<Vec<_>>());
    }
}
