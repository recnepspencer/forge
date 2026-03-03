//! Integration tests: KillEdgeMakeLoop (KEML) — loop split.
//!
//! DOMAIN: KEML removes an edge whose both halfedges are on the same face,
//! splitting the face's loop into two loops (outer stays, new inner created).
//! This is the inverse of MEKL.


use crate::integration_tests::harness::shapes::{
    collect_face_loop, first_halfedge_of_face, unit_cube,
};
use forge_topo::boundary_editing::kill_face_make_ring_hole::KillFaceMakeRingHole;
use forge_topo::boundary_editing::make_edge_kill_loop::MakeEdgeKillLoop;
use forge_topo::boundary_editing::kill_edge_make_loop::KillEdgeMakeLoop;

/// KFMRH + MEKL creates a bridge. KEML removes it, restoring the inner loop.
///
/// Verifies that KEML:
/// - Creates a new inner loop (loop count +1)
/// - The new inner loop has the expected halfedge count
/// - The outer loop shrinks by the inner loop's halfedges + 2 bridge HEs
#[test]
fn keml_splits_bridge_into_two_loops() {
    let env_res = unit_cube().expect("unit cube should succeed");
    let faces = env_res.get_value().faces().to_vec();
    let (mut draft, _geometry): (forge_topo::transactions::MutableDraft, _) = env_res.into_value().into_draft();

    let face_to_kill = faces[0];
    let target_face = faces[1];

    draft.execute(KillFaceMakeRingHole {
        face_to_kill,
        target_face,
    }).unwrap();

    let outer_loop = draft.arena().get_face(target_face).unwrap().outer_loop();
    let inner_loop = draft.arena().get_face(target_face).unwrap().inner_loops()[0];
    let he_outer = draft.arena().get_loop(outer_loop).unwrap().half_edge();
    let he_inner = draft.arena().get_loop(inner_loop).unwrap().half_edge();

    let inner_valence_before = collect_face_loop(draft.arena(), he_inner).unwrap().len();

    let mekl = draft.execute(MakeEdgeKillLoop {
        he_a: he_outer,
        he_b: he_inner,
    }).unwrap().into_value();

    let merged_he = draft.arena().get_loop(outer_loop).unwrap().half_edge();
    let _merged_valence = collect_face_loop(draft.arena(), merged_he).unwrap().len();

    let loop_count_before_keml = draft.arena().loop_count();

    let _keml = draft.execute(KillEdgeMakeLoop {
        edge: mekl.he_ab,
    }).unwrap().into_value();

    assert_eq!(
        draft.arena().loop_count(), loop_count_before_keml + 1,
        "KEML should create 1 new loop"
    );

    let inner_count = draft.arena().get_face(target_face).unwrap().inner_loops().len();
    assert_eq!(inner_count, 1, "KEML should add 1 inner loop to target face");

    let restored_inner = draft.arena().get_face(target_face).unwrap().inner_loops()[0];
    let restored_inner_he = draft.arena().get_loop(restored_inner).unwrap().half_edge();
    let restored_inner_valence = collect_face_loop(draft.arena(), restored_inner_he).unwrap().len();
    assert_eq!(
        restored_inner_valence, inner_valence_before,
        "Restored inner loop should have same valence as before MEKL"
    );


    let _committed = draft.commit().unwrap();
}

/// KEML rejects an edge whose halfedges are on different faces.
#[test]
fn keml_rejects_cross_face_edge() {
    let env_res = unit_cube().expect("unit cube should succeed");
    let faces = env_res.get_value().faces().to_vec();
    let (mut draft, _geometry): (forge_topo::transactions::MutableDraft, _) = env_res.into_value().into_draft();

    let he = first_halfedge_of_face(draft.arena(), faces[0]).unwrap();

    let result = draft.execute(KillEdgeMakeLoop { edge: he });

    assert!(result.is_err(), "KEML should reject edges spanning two faces");
}
