//! Integration tests: KillVertexEdge (KVE) — edge merge.
//!
//! DOMAIN: KVE is the inverse of SplitEdge's geometric sense — it removes
//! a valence-2 vertex and merges its two edges into one. The vertex must
//! have exactly 2 edges radiating from it.

use crate::integration_tests::harness::shapes::{
    collect_face_loop, first_halfedge_of_face, unit_cube,
};
use forge_topo::entity_lifecycle::kill_vertex_edge::KillVertexEdge;
use forge_topo::entity_lifecycle::split_edge::SplitEdge;

/// SplitEdge creates a valence-2 vertex, then KVE merges it back.
#[test]
fn split_edge_then_kve_roundtrip() {
    let env_res = unit_cube().expect("unit cube should succeed");
    let faces = env_res.get_value().faces().to_vec();
    let (mut draft, _geometry): (forge_topo::transactions::MutableDraft, _) = env_res.into_value().into_draft();

    let face = faces[0];
    let start_he = first_halfedge_of_face(draft.arena(), face).unwrap();

    let se = draft.execute(SplitEdge {
        edge: start_he,
    }).unwrap().into_value();

    assert_eq!(draft.arena().vertex_count(), 9);

    let _kve = draft.execute(KillVertexEdge {
        vertex: se.new_vertex,
    }).unwrap().into_value();

    assert_eq!(draft.arena().vertex_count(), 8, "KVE should remove midpoint vertex");
    let face_valence = collect_face_loop(draft.arena(), first_halfedge_of_face(draft.arena(), face).unwrap()).unwrap().len();
    assert_eq!(face_valence, 4);

    let committed = draft.commit().unwrap();
    assert_eq!(committed.arena().face_count(), 6);
    assert_eq!(committed.arena().vertex_count(), 8);
    assert_eq!(committed.arena().half_edge_count(), 24);
    assert_eq!(committed.arena().edge_count(), 12);
    assert_eq!(committed.arena().loop_count(), 6);
    assert_eq!(committed.arena().shell_count(), 1);
    assert_eq!(committed.arena().body_count(), 1);
}

/// Split two separate edges, KVE both back — independent roundtrips.
#[test]
fn two_independent_kve_roundtrips() {
    let env_res = unit_cube().expect("unit cube should succeed");
    let faces = env_res.get_value().faces().to_vec();
    let (mut draft, _geometry): (forge_topo::transactions::MutableDraft, _) = env_res.into_value().into_draft();

    let face = faces[0];
    let start_he = first_halfedge_of_face(draft.arena(), face).unwrap();
    let loop_hes = collect_face_loop(draft.arena(), start_he).unwrap();

    let se1 = draft.execute(SplitEdge {
        edge: loop_hes[0],
    }).unwrap().into_value();

    let se2 = draft.execute(SplitEdge {
        edge: loop_hes[2],
    }).unwrap().into_value();

    assert_eq!(draft.arena().vertex_count(), 10);

    draft.execute(KillVertexEdge { vertex: se1.new_vertex }).unwrap();
    draft.execute(KillVertexEdge { vertex: se2.new_vertex }).unwrap();

    assert_eq!(draft.arena().vertex_count(), 8);

    let committed = draft.commit().unwrap();
    assert_eq!(committed.arena().face_count(), 6);
    assert_eq!(committed.arena().vertex_count(), 8);
    assert_eq!(committed.arena().half_edge_count(), 24);
    assert_eq!(committed.arena().edge_count(), 12);
    assert_eq!(committed.arena().loop_count(), 6);
    assert_eq!(committed.arena().shell_count(), 1);
    assert_eq!(committed.arena().body_count(), 1);
}

/// Double split on same edge, then KVE both midpoints back.
#[test]
fn double_split_then_kve_both() {
    let env_res = unit_cube().expect("unit cube should succeed");
    let faces = env_res.get_value().faces().to_vec();
    let (mut draft, _geometry): (forge_topo::transactions::MutableDraft, _) = env_res.into_value().into_draft();

    let face = faces[0];
    let start_he = first_halfedge_of_face(draft.arena(), face).unwrap();

    let se1 = draft.execute(SplitEdge {
        edge: start_he,
    }).unwrap().into_value();

    let se2 = draft.execute(SplitEdge {
        edge: se1.he_mb,
    }).unwrap().into_value();

    assert_eq!(draft.arena().vertex_count(), 10);
    let face_valence = collect_face_loop(draft.arena(), first_halfedge_of_face(draft.arena(), face).unwrap()).unwrap().len();
    assert_eq!(face_valence, 6);

    draft.execute(KillVertexEdge { vertex: se2.new_vertex }).unwrap();
    draft.execute(KillVertexEdge { vertex: se1.new_vertex }).unwrap();

    assert_eq!(draft.arena().vertex_count(), 8);
    let face_valence = collect_face_loop(draft.arena(), first_halfedge_of_face(draft.arena(), face).unwrap()).unwrap().len();
    assert_eq!(face_valence, 4);

    let _committed = draft.commit().unwrap();
}
