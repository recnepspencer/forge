//! Tests for Boolean operations.

use super::test_helpers::{build_cube, execute_boolean_logged};
use super::schema::{BooleanInput, BooleanOp, FaceOrigin, FaceClassification};
use super::classify::classify_faces;
use super::split::split_all_faces;
use crate::core::ToleranceConfig;

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
        &ToleranceConfig::default(),
    );

    assert!(classified.is_ok());
    let (faces, _log) = classified.unwrap();
    assert!(!faces.is_empty());

    for face in &faces {
        assert_eq!(face.classification(), FaceClassification::Outside);
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
        &ToleranceConfig::default(),
    );

    assert!(classified.is_ok());
    let (faces, _log) = classified.unwrap();

    let inside_count = faces.iter()
        .filter(|f| f.classification() == FaceClassification::Inside)
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

    let result = execute_boolean_logged(input);
    assert!(result.is_ok(), "Union failed: {:?}", result.err());

    let bool_result = result.unwrap().into_value();
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

    let result = execute_boolean_logged(input);
    assert!(result.is_ok(), "Intersection failed: {:?}", result.err());

    let bool_result = result.unwrap().into_value();
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

    let result = execute_boolean_logged(input);
    assert!(result.is_ok(), "Subtraction failed: {:?}", result.err());

    let bool_result = result.unwrap().into_value();
    // Zero-split containment path: target outer shell (6 faces) + tool inner shell reversed (6 faces)
    assert_eq!(bool_result.target_faces_kept(), 6, "Outer cube kept as-is (zero-split containment)");
    assert_eq!(bool_result.tool_faces_kept(), 6, "Inner faces reversed to form hole (zero-split containment)");
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

    let result = execute_boolean_logged(input).unwrap().into_value();
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
    let (t_topo, t_geom, l_topo, l_geom, _target_prov, _tool_prov) = result.into_parts();

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

    let config = ToleranceConfig::default();
    let target_classified = classify_faces(
        t_topo.arena(), &t_geom,
        l_topo.arena(), &l_geom,
        FaceOrigin::Target,
        &config,
    ).unwrap().0;
    let tool_classified = classify_faces(
        l_topo.arena(), &l_geom,
        t_topo.arena(), &t_geom,
        FaceOrigin::Tool,
        &config,
    ).unwrap().0;

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
        let verts: Vec<_> = forge_topo::traverse::face_edges(t_topo.arena(), fid)
            .unwrap()
            .iter()
            .map(|he_id| {
                let he = t_topo.arena().get_half_edge(*he_id).unwrap();
                let pos = t_geom.get_vertex_position(he.origin);
                format!("{}: {:?}", he.origin, pos.cloned())
            })
            .collect();
        eprintln!("Target face {}: {} verts: {:?}", fid, verts.len(), verts);
    }
}
