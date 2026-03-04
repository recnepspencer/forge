//! Integration tests: KillEdgeVertex (KEV) — vertex collapse.
//!
//! DOMAIN: KEV is the inverse of SplitEdge. It removes a vertex by collapsing
//! two edges into one. The surviving vertex is the origin of the input halfedge;
//! the target vertex (twin's origin) is killed.

use crate::integration_tests::harness::shapes::{
    collect_face_loop, first_halfedge_of_face, unit_cube,
};
use forge_topo::entity_lifecycle::kill_edge_vertex::KillEdgeVertex;
use forge_topo::entity_lifecycle::split_edge::SplitEdge;

/// SplitEdge then KEV is a roundtrip — counts should match original.
#[test]
fn split_then_kev_roundtrip() {
    let env_res = unit_cube().expect("unit cube should succeed");
    let faces = env_res.get_value().faces().to_vec();
    let (mut draft, _geometry): (forge_topo::transactions::MutableDraft, _) = env_res.into_value().into_draft();

    let face = faces[0];
    let start_he = first_halfedge_of_face(draft.arena(), face).unwrap();

    let se = draft.execute(SplitEdge {
        edge: start_he,
    }).unwrap().into_value();

    assert_eq!(draft.arena().vertex_count(), 9);

    let _kev = draft.execute(KillEdgeVertex {
        edge: se.he_mb,
    }).unwrap().into_value();

    assert_eq!(draft.arena().vertex_count(), 8, "KEV should kill the midpoint vertex");
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

/// Split all edges of a face, then KEV each midpoint back.
///
/// After splitting 4 edges of a cube face (creating 4 midpoints),
/// collapsing them all should restore original counts.
#[test]
fn split_all_face_edges_then_kev_all() {
    let env_res = unit_cube().expect("unit cube should succeed");
    let faces = env_res.get_value().faces().to_vec();
    let (mut draft, _geometry): (forge_topo::transactions::MutableDraft, _) = env_res.into_value().into_draft();

    let face = faces[0];
    let start_he = first_halfedge_of_face(draft.arena(), face).unwrap();
    let loop_hes = collect_face_loop(draft.arena(), start_he).unwrap();

    let mut new_hes = Vec::new();
    for &he in &loop_hes {
        let se = draft.execute(SplitEdge {
            edge: he,
        }).unwrap().into_value();
        new_hes.push(se.he_mb);
    }

    assert_eq!(draft.arena().vertex_count(), 12);
    let face_valence = collect_face_loop(draft.arena(), first_halfedge_of_face(draft.arena(), face).unwrap()).unwrap().len();
    assert_eq!(face_valence, 8);

    for he in new_hes {
        draft.execute(KillEdgeVertex { edge: he }).unwrap();
    }

    assert_eq!(draft.arena().vertex_count(), 8);
    let face_valence = collect_face_loop(draft.arena(), first_halfedge_of_face(draft.arena(), face).unwrap()).unwrap().len();
    assert_eq!(face_valence, 4);

    let _committed = draft.commit().unwrap();
}

/// KEV on a 2-split chain: split edge twice, then collapse both midpoints.
#[test]
fn double_split_then_double_kev() {
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

    draft.execute(KillEdgeVertex { edge: se2.he_mb }).unwrap();
    draft.execute(KillEdgeVertex { edge: se1.he_mb }).unwrap();

    assert_eq!(draft.arena().vertex_count(), 8);
    let face_valence = collect_face_loop(draft.arena(), first_halfedge_of_face(draft.arena(), face).unwrap()).unwrap().len();
    assert_eq!(face_valence, 4);

    let _committed = draft.commit().unwrap();
}
