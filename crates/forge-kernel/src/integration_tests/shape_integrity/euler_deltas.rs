//! Euler operator delta assertions using `Snapshot`.
//!
//! DOMAIN: Each Euler operator has a known effect on entity counts.
//! Instead of hardcoding absolute numbers, we capture a snapshot
//! before the operation and assert the exact delta after.

use crate::integration_tests::harness::shapes::{
    collect_face_loop, first_halfedge_of_face, unit_cube,
};
use crate::integration_tests::harness::snapshot::{Snapshot, Delta};

use forge_topo::entity_lifecycle::split_edge::SplitEdge;
use forge_topo::entity_lifecycle::make_edge_face::MakeEdgeFace;
use forge_topo::entity_lifecycle::make_edge_vertex::MakeEdgeVertex;
use forge_topo::entity_lifecycle::kill_edge_vertex::KillEdgeVertex;
use forge_topo::entity_lifecycle::kill_vertex_edge::KillVertexEdge;
use forge_topo::boundary_editing::join_faces::JoinFaces;

/// SplitEdge: +1 vertex, +1 edge, +2 halfedges, 0 faces/loops/shells/bodies.
#[test]
fn split_edge_delta() {
    let env_res = unit_cube().expect("unit cube should succeed");
    let faces = env_res.get_value().faces().to_vec();
    let (mut draft, _geom): (forge_topo::transactions::MutableDraft, _) = env_res.into_value().into_draft();

    let snap = Snapshot::capture(draft.arena());

    let face = faces[0];
    let he = first_halfedge_of_face(draft.arena(), face).unwrap();
    draft.execute(SplitEdge { edge: he }).unwrap();

    snap.assert_delta(draft.arena(), Delta {
        faces: 0, vertices: 1, edges: 1, half_edges: 2,
        loops: 0, shells: 0, bodies: 0,
    });
}

/// MakeEdgeFace: +1 face, +1 edge, +2 halfedges, +1 loop, 0 vertices.
#[test]
fn make_edge_face_delta() {
    let env_res = unit_cube().expect("unit cube should succeed");
    let faces = env_res.get_value().faces().to_vec();
    let (mut draft, _geom): (forge_topo::transactions::MutableDraft, _) = env_res.into_value().into_draft();

    let face = faces[0];
    let he = first_halfedge_of_face(draft.arena(), face).unwrap();
    let loop_hes = collect_face_loop(draft.arena(), he).unwrap();

    let v_a = draft.arena().get_half_edge(loop_hes[0]).unwrap().origin();
    let v_c = draft.arena().get_half_edge(loop_hes[2]).unwrap().origin();

    let snap = Snapshot::capture(draft.arena());

    draft.execute(MakeEdgeFace {
        face,
        vertex_a: v_a,
        vertex_b: v_c,
    }).unwrap();

    snap.assert_delta(draft.arena(), Delta {
        faces: 1, vertices: 0, edges: 1, half_edges: 2,
        loops: 1, shells: 0, bodies: 0,
    });
}

/// MakeEdgeVertex: +1 vertex, +1 edge, +2 halfedges, 0 faces.
#[test]
fn make_edge_vertex_delta() {
    let env_res = unit_cube().expect("unit cube should succeed");
    let faces = env_res.get_value().faces().to_vec();
    let (mut draft, _geom): (forge_topo::transactions::MutableDraft, _) = env_res.into_value().into_draft();

    let face = faces[0];
    let anchor = first_halfedge_of_face(draft.arena(), face).unwrap();

    let snap = Snapshot::capture(draft.arena());

    draft.execute(MakeEdgeVertex { anchor }).unwrap();

    snap.assert_delta(draft.arena(), Delta {
        faces: 0, vertices: 1, edges: 1, half_edges: 2,
        loops: 0, shells: 0, bodies: 0,
    });
}

/// SplitEdge then KillEdgeVertex is a roundtrip — net delta is zero.
#[test]
fn split_then_kev_roundtrip_delta() {
    let env_res = unit_cube().expect("unit cube should succeed");
    let faces = env_res.get_value().faces().to_vec();
    let (mut draft, _geom): (forge_topo::transactions::MutableDraft, _) = env_res.into_value().into_draft();

    let snap = Snapshot::capture(draft.arena());

    let face = faces[0];
    let he = first_halfedge_of_face(draft.arena(), face).unwrap();

    let se = draft.execute(SplitEdge { edge: he }).unwrap().into_value();
    draft.execute(KillEdgeVertex { edge: se.he_mb }).unwrap();

    snap.assert_unchanged(draft.arena());
}

/// SplitEdge then KillVertexEdge is a roundtrip — net delta is zero.
#[test]
fn split_then_kve_roundtrip_delta() {
    let env_res = unit_cube().expect("unit cube should succeed");
    let faces = env_res.get_value().faces().to_vec();
    let (mut draft, _geom): (forge_topo::transactions::MutableDraft, _) = env_res.into_value().into_draft();

    let snap = Snapshot::capture(draft.arena());

    let face = faces[0];
    let he = first_halfedge_of_face(draft.arena(), face).unwrap();

    let se = draft.execute(SplitEdge { edge: he }).unwrap().into_value();
    draft.execute(KillVertexEdge { vertex: se.new_vertex }).unwrap();

    snap.assert_unchanged(draft.arena());
}

/// MEF then JoinFaces is a roundtrip — net delta is zero.
#[test]
fn mef_then_join_roundtrip_delta() {
    let env_res = unit_cube().expect("unit cube should succeed");
    let faces = env_res.get_value().faces().to_vec();
    let (mut draft, _geom): (forge_topo::transactions::MutableDraft, _) = env_res.into_value().into_draft();

    let face = faces[0];
    let he = first_halfedge_of_face(draft.arena(), face).unwrap();
    let loop_hes = collect_face_loop(draft.arena(), he).unwrap();

    let v_a = draft.arena().get_half_edge(loop_hes[0]).unwrap().origin();
    let v_c = draft.arena().get_half_edge(loop_hes[2]).unwrap().origin();

    let snap = Snapshot::capture(draft.arena());

    let mef = draft.execute(MakeEdgeFace {
        face,
        vertex_a: v_a,
        vertex_b: v_c,
    }).unwrap().into_value();

    draft.execute(JoinFaces { edge: mef.half_edge_ab }).unwrap();

    snap.assert_unchanged(draft.arena());
}
