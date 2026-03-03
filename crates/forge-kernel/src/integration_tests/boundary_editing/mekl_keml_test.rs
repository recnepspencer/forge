//! Integration tests: MEF + JoinFaces interaction.
//!
//! DOMAIN: Tests that MEF and JoinFaces compose correctly.
//! Verifies round-trip identity, multi-face operations, and
//! surviving face valence after merge.

use crate::integration_tests::harness::shapes::{
    collect_face_loop, first_halfedge_of_face, unit_cube,
};
use forge_topo::boundary_editing::join_faces::JoinFaces;
use forge_topo::entity_lifecycle::make_edge_face::MakeEdgeFace;

/// MEF splits a face, JoinFaces merges it back.
/// Verify full count restoration AND that vertex adjacency is intact.
#[test]
fn mef_then_join_roundtrip_restores_all_counts() {
    let env_res = unit_cube().expect("unit cube should succeed");
    let faces = env_res.get_value().faces().to_vec();
    let (mut draft, _geometry): (forge_topo::transactions::MutableDraft, _) = env_res.into_value().into_draft();

    let face = faces[0];
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

    let _jf = draft.execute(JoinFaces {
        edge: mef.half_edge_ab,
    }).unwrap().into_value();

    assert_eq!(draft.arena().face_count(), 6);
    assert_eq!(draft.arena().vertex_count(), 8);
    assert_eq!(draft.arena().half_edge_count(), 24);
    assert_eq!(draft.arena().edge_count(), 12);
    assert_eq!(draft.arena().loop_count(), 6);
    assert_eq!(draft.arena().shell_count(), 1);
    assert_eq!(draft.arena().body_count(), 1);

    let face_valence = collect_face_loop(draft.arena(), first_halfedge_of_face(draft.arena(), face).unwrap()).unwrap().len();
    assert_eq!(face_valence, 4);

    let _committed = draft.commit().unwrap();
}

/// MEF on two separate faces, then JoinFaces on a DIFFERENT edge (not the MEF edge).
/// This tests that MEF doesn't corrupt adjacent face topology.
#[test]
fn mef_two_faces_then_join_adjacent_pair() {
    let env_res = unit_cube().expect("unit cube should succeed");
    let faces = env_res.get_value().faces().to_vec();
    let (mut draft, _geometry): (forge_topo::transactions::MutableDraft, _) = env_res.into_value().into_draft();

    let face_a = faces[0];
    let face_b = faces[2];

    let he_a = first_halfedge_of_face(draft.arena(), face_a).unwrap();
    let loop_a = collect_face_loop(draft.arena(), he_a).unwrap();
    let va_0 = draft.arena().get_half_edge(loop_a[0]).unwrap().origin();
    let va_2 = draft.arena().get_half_edge(loop_a[2]).unwrap().origin();

    draft.execute(MakeEdgeFace {
        face: face_a,
        vertex_a: va_0,
        vertex_b: va_2,
    }).unwrap();

    let he_b = first_halfedge_of_face(draft.arena(), face_b).unwrap();
    let loop_b = collect_face_loop(draft.arena(), he_b).unwrap();
    let vb_0 = draft.arena().get_half_edge(loop_b[0]).unwrap().origin();
    let vb_2 = draft.arena().get_half_edge(loop_b[2]).unwrap().origin();

    draft.execute(MakeEdgeFace {
        face: face_b,
        vertex_a: vb_0,
        vertex_b: vb_2,
    }).unwrap();

    assert_eq!(draft.arena().face_count(), 8);

    let jf_he = first_halfedge_of_face(draft.arena(), faces[3]).unwrap();
    let jf = draft.execute(JoinFaces {
        edge: jf_he,
    }).unwrap().into_value();

    assert_eq!(draft.arena().face_count(), 7);
    let face_valence = collect_face_loop(draft.arena(), first_halfedge_of_face(draft.arena(), jf.surviving_face).unwrap()).unwrap().len();
    assert_eq!(face_valence, 6);

    let _committed = draft.commit().unwrap();
}
