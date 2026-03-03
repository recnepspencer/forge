//! Integration tests: MakeEdgeVertex (MEV) — vertex extension.
//!
//! DOMAIN: MEV sprouts a new wire edge from an existing vertex, creating
//! a new vertex and edge. The anchor halfedge determines the splice point.

use crate::integration_tests::harness::shapes::{
    collect_face_loop, first_halfedge_of_face, unit_cube,
};
use forge_topo::entity_lifecycle::kill_edge_vertex::KillEdgeVertex;
use forge_topo::entity_lifecycle::make_edge_vertex::MakeEdgeVertex;

/// MEV on a cube face sprouts a wire edge, adding 1 vertex, 1 edge, 2 halfedges.
#[test]
fn mev_sprouts_wire_from_cube_vertex() {
    let envelope = unit_cube().unwrap();
    let faces = envelope.faces().to_vec();
    let (mut draft, _geometry) = envelope.into_draft();

    let face = faces[0];
    let anchor = first_halfedge_of_face(draft.arena(), face).unwrap();

    let _mev = draft.execute(MakeEdgeVertex {
        anchor,
    }).unwrap().into_value();

    assert_eq!(draft.arena().vertex_count(), 9, "MEV should add 1 vertex");
    assert_eq!(draft.arena().edge_count(), 13, "MEV should add 1 edge");
    assert_eq!(draft.arena().half_edge_count(), 26, "MEV should add 2 halfedges");

    let face_valence = collect_face_loop(draft.arena(), first_halfedge_of_face(draft.arena(), face).unwrap()).unwrap().len();
    assert_eq!(face_valence, 6);

    let _committed = draft.commit().unwrap();
}

/// MEV then KEV roundtrip — should restore original counts.
#[test]
fn mev_then_kev_roundtrip() {
    let envelope = unit_cube().unwrap();
    let faces = envelope.faces().to_vec();
    let (mut draft, _geometry) = envelope.into_draft();

    let face = faces[0];
    let anchor = first_halfedge_of_face(draft.arena(), face).unwrap();

    let mev = draft.execute(MakeEdgeVertex {
        anchor,
    }).unwrap().into_value();

    draft.execute(KillEdgeVertex {
        edge: mev.he_out,
    }).unwrap();

    assert_eq!(draft.arena().vertex_count(), 8);
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

/// Chain MEV: sprout two wire edges from the same vertex.
#[test]
fn chain_mev_two_wires_from_same_vertex() {
    let envelope = unit_cube().unwrap();
    let faces = envelope.faces().to_vec();
    let (mut draft, _geometry) = envelope.into_draft();

    let face = faces[0];
    let anchor = first_halfedge_of_face(draft.arena(), face).unwrap();

    let mev1 = draft.execute(MakeEdgeVertex {
        anchor,
    }).unwrap().into_value();

    let _mev2 = draft.execute(MakeEdgeVertex {
        anchor: mev1.he_back,
    }).unwrap().into_value();

    assert_eq!(draft.arena().vertex_count(), 10, "Two MEVs add 2 vertices");
    assert_eq!(draft.arena().edge_count(), 14, "Two MEVs add 2 edges");

    let _committed = draft.commit().unwrap();
}
