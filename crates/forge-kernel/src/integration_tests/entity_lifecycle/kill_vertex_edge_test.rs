//! Integration tests: KillVertexEdge (KVE) — edge merge.
//!
//! DOMAIN: KVE is the inverse of SplitEdge's geometric sense — it removes
//! a valence-2 vertex and merges its two edges into one. The vertex must
//! have exactly 2 edges radiating from it.

use crate::integration_tests::harness::assertions::{
    assert_all_invariants, assert_counts, assert_face_valence, EntityCounts,
};
use crate::integration_tests::harness::shapes::{
    collect_face_loop, first_halfedge_of_face, unit_cube,
};
use forge_topo::entity_lifecycle::kill_vertex_edge::KillVertexEdge;
use forge_topo::entity_lifecycle::split_edge::SplitEdge;

/// SplitEdge creates a valence-2 vertex, then KVE merges it back.
#[test]
fn split_edge_then_kve_roundtrip() {
    let (topo, handles) = unit_cube().unwrap();
    let mut draft = topo.into_mutation();

    let face = handles.faces[0];
    let start_he = first_halfedge_of_face(draft.arena(), face).unwrap();

    let se = draft.execute(SplitEdge {
        edge: start_he,
        parameter: 0.5,
    }).unwrap().into_value();

    assert_eq!(draft.arena().vertex_count(), 9);

    let kve = draft.execute(KillVertexEdge {
        vertex: se.new_vertex,
    }).unwrap().into_value();

    assert_eq!(draft.arena().vertex_count(), 8, "KVE should remove midpoint vertex");
    assert_face_valence(draft.arena(), face, 4);
    assert_all_invariants(draft.arena());

    let committed = draft.commit().unwrap();
    assert_all_invariants(committed.arena());
    assert_counts(committed.arena(), EntityCounts {
        faces: 6, vertices: 8, half_edges: 24, edges: 12, loops: 6, shells: 1, bodies: 1,
    });
}

/// Split two separate edges, KVE both back — independent roundtrips.
#[test]
fn two_independent_kve_roundtrips() {
    let (topo, handles) = unit_cube().unwrap();
    let mut draft = topo.into_mutation();

    let face = handles.faces[0];
    let start_he = first_halfedge_of_face(draft.arena(), face).unwrap();
    let loop_hes = collect_face_loop(draft.arena(), start_he).unwrap();

    let se1 = draft.execute(SplitEdge {
        edge: loop_hes[0],
        parameter: 0.5,
    }).unwrap().into_value();

    let se2 = draft.execute(SplitEdge {
        edge: loop_hes[2],
        parameter: 0.5,
    }).unwrap().into_value();

    assert_eq!(draft.arena().vertex_count(), 10);

    draft.execute(KillVertexEdge { vertex: se1.new_vertex }).unwrap();
    draft.execute(KillVertexEdge { vertex: se2.new_vertex }).unwrap();

    assert_eq!(draft.arena().vertex_count(), 8);
    assert_all_invariants(draft.arena());

    let committed = draft.commit().unwrap();
    assert_counts(committed.arena(), EntityCounts {
        faces: 6, vertices: 8, half_edges: 24, edges: 12, loops: 6, shells: 1, bodies: 1,
    });
}

/// Double split on same edge, then KVE both midpoints back.
#[test]
fn double_split_then_kve_both() {
    let (topo, handles) = unit_cube().unwrap();
    let mut draft = topo.into_mutation();

    let face = handles.faces[0];
    let start_he = first_halfedge_of_face(draft.arena(), face).unwrap();

    let se1 = draft.execute(SplitEdge {
        edge: start_he,
        parameter: 0.33,
    }).unwrap().into_value();

    let se2 = draft.execute(SplitEdge {
        edge: se1.he_mb,
        parameter: 0.5,
    }).unwrap().into_value();

    assert_eq!(draft.arena().vertex_count(), 10);
    assert_face_valence(draft.arena(), face, 6);

    draft.execute(KillVertexEdge { vertex: se2.new_vertex }).unwrap();
    draft.execute(KillVertexEdge { vertex: se1.new_vertex }).unwrap();

    assert_eq!(draft.arena().vertex_count(), 8);
    assert_face_valence(draft.arena(), face, 4);
    assert_all_invariants(draft.arena());
}
