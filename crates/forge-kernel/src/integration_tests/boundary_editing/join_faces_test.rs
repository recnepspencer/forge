//! Integration tests: JoinFaces on real cube geometry.
//!
//! DOMAIN: Splits cube faces with MEF, then joins them back with JoinFaces.
//! Tests round-trip identity (split + join = original topology) and
//! multi-face merges that create non-convex polygons.

use crate::integration_tests::harness::assertions::{
    assert_all_invariants, assert_counts, assert_face_valence,
    assert_reciprocity, assert_euler_formula, EntityCounts,
};
use crate::integration_tests::harness::shapes::{
    collect_face_loop, first_halfedge_of_face, unit_cube,
};
use forge_topo::boundary_editing::join_faces::JoinFaces;
use forge_topo::entity_lifecycle::make_edge_face::MakeEdgeFace;
use forge_topo::entity_lifecycle::split_edge::SplitEdge;

/// Split a cube face diagonally, then join it back.
///
/// Round-trip: MEF → JoinFaces should restore original face valence.
/// The surviving face should be a quad again.
#[test]
fn split_then_rejoin_is_identity() {
    let (topo, handles) = unit_cube().unwrap();
    let mut draft = topo.into_mutation();

    let face = handles.faces[0];
    let start_he = first_halfedge_of_face(draft.arena(), face).unwrap();
    let loop_hes = collect_face_loop(draft.arena(), start_he).unwrap();

    let v_a = draft.arena().get_half_edge(loop_hes[0]).unwrap().origin();
    let v_c = draft.arena().get_half_edge(loop_hes[2]).unwrap().origin();

    let mef = draft.execute(MakeEdgeFace {
        face,
        vertex_a: v_a,
        vertex_b: v_c,
    }).unwrap().into_value();

    assert_eq!(draft.arena().face_count(), 7);

    let jf = draft.execute(JoinFaces {
        edge: mef.half_edge_ab,
    }).unwrap().into_value();

    assert_all_invariants(draft.arena());

    assert_counts(draft.arena(), EntityCounts {
        faces: 6,
        vertices: 8,
        half_edges: 24,
        edges: 12,
        loops: 6,
        shells: 1,
        bodies: 1,
    });

    assert_face_valence(draft.arena(), jf.surviving_face, 4);

    let committed = draft.commit().unwrap();
    assert_all_invariants(committed.arena());
}

/// Join two adjacent cube faces to create a 6-sided polygon.
///
/// Find an edge shared between two cube faces and join them.
/// The surviving face should have 6 halfedges (two quads merged minus shared edge).
#[test]
fn join_two_adjacent_cube_faces() {
    let (topo, handles) = unit_cube().unwrap();
    let mut draft = topo.into_mutation();

    let face_a = handles.faces[0];
    let start_he = first_halfedge_of_face(draft.arena(), face_a).unwrap();

    let jf = draft.execute(JoinFaces {
        edge: start_he,
    }).unwrap().into_value();

    assert_all_invariants(draft.arena());

    assert_face_valence(draft.arena(), jf.surviving_face, 6);

    assert_counts(draft.arena(), EntityCounts {
        faces: 5,          // 6 - 1
        vertices: 8,
        half_edges: 22,    // 24 - 2
        edges: 11,         // 12 - 1
        loops: 5,          // 6 - 1
        shells: 1,
        bodies: 1,
    });

    let committed = draft.commit().unwrap();
    assert_all_invariants(committed.arena());
}

/// Chain: split edge → MEF → join the new face with a neighbor.
///
/// Exercises composed operator state: after SplitEdge creates a 5-gon,
/// MEF splits it, then JoinFaces merges the new triangle with an adjacent
/// cube face to create a 5-gon.
#[test]
fn split_mef_then_join_neighbor() {
    let (topo, handles) = unit_cube().unwrap();
    let mut draft = topo.into_mutation();

    let face = handles.faces[0];
    let start_he = first_halfedge_of_face(draft.arena(), face).unwrap();

    let se = draft.execute(SplitEdge {
        edge: start_he,
        parameter: 0.5,
    }).unwrap().into_value();

    let new_loop = collect_face_loop(draft.arena(), start_he).unwrap();
    let opposite_v = draft.arena().get_half_edge(new_loop[3]).unwrap().origin();

    let mef = draft.execute(MakeEdgeFace {
        face,
        vertex_a: se.new_vertex,
        vertex_b: opposite_v,
    }).unwrap().into_value();

    assert_eq!(draft.arena().face_count(), 7, "MEF adds 1 face");

    let new_face_he = first_halfedge_of_face(draft.arena(), mef.new_face).unwrap();
    let new_face_loop = collect_face_loop(draft.arena(), new_face_he).unwrap();

    let neighbor_he = new_face_loop.iter()
        .find(|&&he| {
            let radial = draft.arena().get_half_edge(he).unwrap().radial_next();
            if radial == he { return false; }
            let radial_face = draft.arena().get_half_edge(radial).unwrap().face();
            radial_face != mef.new_face && radial_face != face
        })
        .copied();

    assert!(neighbor_he.is_some(), "New face must have a neighbor other than original");
    let neighbor_he = neighbor_he.unwrap();

    let jf = draft.execute(JoinFaces {
        edge: neighbor_he,
    }).unwrap().into_value();

    assert_eq!(draft.arena().face_count(), 6, "JoinFaces removes 1 face (back to 6)");

    assert_reciprocity(draft.arena());
    assert_euler_formula(draft.arena());

    let committed = draft.commit().unwrap();
    assert_reciprocity(committed.arena());
    assert_euler_formula(committed.arena());
}

