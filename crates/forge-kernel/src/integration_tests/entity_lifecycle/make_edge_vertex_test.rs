//! Integration tests: MakeEdgeVertex (MEV) — vertex extension.
//!
//! DOMAIN: MEV sprouts a new wire edge from an existing vertex, creating
//! a new vertex and edge. The anchor halfedge determines the splice point.

use crate::integration_tests::harness::assertions::{
    assert_all_invariants, assert_counts, assert_face_valence, EntityCounts,
};
use crate::integration_tests::harness::shapes::{
    collect_face_loop, first_halfedge_of_face, unit_cube,
};
use forge_topo::entity_lifecycle::kill_edge_vertex::KillEdgeVertex;
use forge_topo::entity_lifecycle::make_edge_vertex::MakeEdgeVertex;

/// MEV on a cube face sprouts a wire edge, adding 1 vertex, 1 edge, 2 halfedges.
#[test]
fn mev_sprouts_wire_from_cube_vertex() {
    let (topo, handles) = unit_cube().unwrap();
    let mut draft = topo.into_mutation();

    let face = handles.faces[0];
    let anchor = first_halfedge_of_face(draft.arena(), face).unwrap();

    let mev = draft.execute(MakeEdgeVertex {
        anchor,
    }).unwrap().into_value();

    assert_eq!(draft.arena().vertex_count(), 9, "MEV should add 1 vertex");
    assert_eq!(draft.arena().edge_count(), 13, "MEV should add 1 edge");
    assert_eq!(draft.arena().half_edge_count(), 26, "MEV should add 2 halfedges");

    assert_face_valence(draft.arena(), face, 6);

    assert_all_invariants(draft.arena());

    let committed = draft.commit().unwrap();
    assert_all_invariants(committed.arena());
}

/// MEV then KEV roundtrip — should restore original counts.
#[test]
fn mev_then_kev_roundtrip() {
    let (topo, handles) = unit_cube().unwrap();
    let mut draft = topo.into_mutation();

    let face = handles.faces[0];
    let anchor = first_halfedge_of_face(draft.arena(), face).unwrap();

    let mev = draft.execute(MakeEdgeVertex {
        anchor,
    }).unwrap().into_value();

    draft.execute(KillEdgeVertex {
        edge: mev.he_out,
    }).unwrap();

    assert_eq!(draft.arena().vertex_count(), 8);
    assert_face_valence(draft.arena(), face, 4);
    assert_all_invariants(draft.arena());

    let committed = draft.commit().unwrap();
    assert_counts(committed.arena(), EntityCounts {
        faces: 6, vertices: 8, half_edges: 24, edges: 12, loops: 6, shells: 1, bodies: 1,
    });
}

/// Chain MEV: sprout two wire edges from the same vertex.
#[test]
fn chain_mev_two_wires_from_same_vertex() {
    let (topo, handles) = unit_cube().unwrap();
    let mut draft = topo.into_mutation();

    let face = handles.faces[0];
    let anchor = first_halfedge_of_face(draft.arena(), face).unwrap();

    let mev1 = draft.execute(MakeEdgeVertex {
        anchor,
    }).unwrap().into_value();

    let mev2 = draft.execute(MakeEdgeVertex {
        anchor: mev1.he_back,
    }).unwrap().into_value();

    assert_eq!(draft.arena().vertex_count(), 10, "Two MEVs add 2 vertices");
    assert_eq!(draft.arena().edge_count(), 14, "Two MEVs add 2 edges");

    assert_all_invariants(draft.arena());

    let committed = draft.commit().unwrap();
    assert_all_invariants(committed.arena());
}
