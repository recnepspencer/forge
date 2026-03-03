//! Integration tests: KillEdgeVertex (KEV) — vertex collapse.
//!
//! DOMAIN: KEV is the inverse of SplitEdge. It removes a vertex by collapsing
//! two edges into one. The surviving vertex is the origin of the input halfedge;
//! the target vertex (twin's origin) is killed.

use crate::integration_tests::harness::assertions::{
    assert_all_invariants, assert_counts, assert_face_valence, EntityCounts,
};
use crate::integration_tests::harness::shapes::{
    collect_face_loop, first_halfedge_of_face, unit_cube,
};
use forge_topo::entity_lifecycle::kill_edge_vertex::KillEdgeVertex;
use forge_topo::entity_lifecycle::split_edge::SplitEdge;

/// SplitEdge then KEV is a roundtrip — counts should match original.
#[test]
fn split_then_kev_roundtrip() {
    let envelope = unit_cube().unwrap();
    let faces = envelope.faces().to_vec();
    let (mut draft, _geometry) = envelope.into_draft();

    let face = faces[0];
    let start_he = first_halfedge_of_face(draft.arena(), face).unwrap();

    let se = draft.execute(SplitEdge {
        edge: start_he,
        parameter: 0.5,
    }).unwrap().into_value();

    assert_eq!(draft.arena().vertex_count(), 9);

    let _kev = draft.execute(KillEdgeVertex {
        edge: se.he_mb,
    }).unwrap().into_value();

    assert_eq!(draft.arena().vertex_count(), 8, "KEV should kill the midpoint vertex");
    assert_face_valence(draft.arena(), face, 4);

    assert_all_invariants(draft.arena());

    let committed = draft.commit().unwrap();
    assert_all_invariants(committed.arena());
    assert_counts(committed.arena(), EntityCounts {
        faces: 6, vertices: 8, half_edges: 24, edges: 12, loops: 6, shells: 1, bodies: 1,
    });
}

/// Split all edges of a face, then KEV each midpoint back.
///
/// After splitting 4 edges of a cube face (creating 4 midpoints),
/// collapsing them all should restore original counts.
#[test]
fn split_all_face_edges_then_kev_all() {
    let envelope = unit_cube().unwrap();
    let faces = envelope.faces().to_vec();
    let (mut draft, _geometry) = envelope.into_draft();

    let face = faces[0];
    let start_he = first_halfedge_of_face(draft.arena(), face).unwrap();
    let loop_hes = collect_face_loop(draft.arena(), start_he).unwrap();

    let mut new_hes = Vec::new();
    for &he in &loop_hes {
        let se = draft.execute(SplitEdge {
            edge: he,
            parameter: 0.5,
        }).unwrap().into_value();
        new_hes.push(se.he_mb);
    }

    assert_eq!(draft.arena().vertex_count(), 12);
    assert_face_valence(draft.arena(), face, 8);

    for he in new_hes {
        draft.execute(KillEdgeVertex { edge: he }).unwrap();
    }

    assert_eq!(draft.arena().vertex_count(), 8);
    assert_face_valence(draft.arena(), face, 4);
    assert_all_invariants(draft.arena());

    let committed = draft.commit().unwrap();
    assert_all_invariants(committed.arena());
}

/// KEV on a 2-split chain: split edge twice, then collapse both midpoints.
#[test]
fn double_split_then_double_kev() {
    let envelope = unit_cube().unwrap();
    let faces = envelope.faces().to_vec();
    let (mut draft, _geometry) = envelope.into_draft();

    let face = faces[0];
    let start_he = first_halfedge_of_face(draft.arena(), face).unwrap();

    let se1 = draft.execute(SplitEdge {
        edge: start_he,
        parameter: 0.5,
    }).unwrap().into_value();

    let se2 = draft.execute(SplitEdge {
        edge: se1.he_mb,
        parameter: 0.5,
    }).unwrap().into_value();

    assert_eq!(draft.arena().vertex_count(), 10);
    assert_face_valence(draft.arena(), face, 6);

    draft.execute(KillEdgeVertex { edge: se2.he_mb }).unwrap();
    draft.execute(KillEdgeVertex { edge: se1.he_mb }).unwrap();

    assert_eq!(draft.arena().vertex_count(), 8);
    assert_face_valence(draft.arena(), face, 4);
    assert_all_invariants(draft.arena());
}
