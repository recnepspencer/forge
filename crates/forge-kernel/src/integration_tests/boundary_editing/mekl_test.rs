//! Integration tests: MakeEdgeKillLoop (MEKL) — loop bridge.
//!
//! DOMAIN: MEKL creates a bridge edge between two loops on the same face,
//! killing the inner loop by merging it into the outer loop.
//!
//! To test MEKL, we need a face with TWO loops. Strategy:
//! 1. Use KFMRH to demote a face into a hole of another face
//! 2. Then MEKL to bridge the hole back to the outer loop.

use crate::integration_tests::harness::shapes::{
    collect_face_loop, first_halfedge_of_face, unit_cube,
};
use forge_topo::boundary_editing::kill_edge_make_loop::KillEdgeMakeLoop;
use forge_topo::boundary_editing::kill_face_make_ring_hole::KillFaceMakeRingHole;
use forge_topo::boundary_editing::make_edge_kill_loop::MakeEdgeKillLoop;

/// KFMRH creates a face with an inner loop (hole), then MEKL bridges it.
///
/// After KFMRH: target_face has 2 loops (outer + inner hole).
/// After MEKL: target_face has 1 loop (bridge merged them).
/// The face valence should increase by 2 (the bridge edge adds 2 halfedges).
#[test]
fn kfmrh_then_mekl_bridges_hole() {
    let env_res = unit_cube().expect("unit cube should succeed");
    let faces = env_res.get_value().faces().to_vec();
    let (mut draft, _geometry): (forge_topo::transactions::MutableDraft, _) =
        env_res.into_value().into_draft();

    let face_to_kill = faces[0];
    let target_face = faces[1];

    let target_shell = draft.arena().get_face(target_face).unwrap().shell();
    let kill_shell = draft.arena().get_face(face_to_kill).unwrap().shell();
    assert_eq!(target_shell, kill_shell, "Both faces must be in same shell");

    let inner_loops_before = draft
        .arena()
        .get_face(target_face)
        .unwrap()
        .inner_loops()
        .len();
    assert_eq!(inner_loops_before, 0);

    let face_count_before = draft.arena().face_count();
    let _loop_count_before = draft.arena().loop_count();

    draft
        .execute(KillFaceMakeRingHole {
            face_to_kill,
            target_face,
        })
        .unwrap()
        .into_value();

    assert_eq!(
        draft.arena().face_count(),
        face_count_before - 1,
        "KFMRH removes 1 face"
    );
    let inner_loops_after = draft
        .arena()
        .get_face(target_face)
        .unwrap()
        .inner_loops()
        .len();
    assert_eq!(inner_loops_after, 1, "KFMRH should create 1 inner loop");

    let outer_loop = draft.arena().get_face(target_face).unwrap().outer_loop();
    let inner_loop = draft.arena().get_face(target_face).unwrap().inner_loops()[0];
    assert_ne!(outer_loop, inner_loop);

    let he_outer = draft.arena().get_loop(outer_loop).unwrap().half_edge();
    let he_inner = draft.arena().get_loop(inner_loop).unwrap().half_edge();

    let outer_valence = collect_face_loop(draft.arena(), he_outer).unwrap().len();
    let inner_valence = collect_face_loop(draft.arena(), he_inner).unwrap().len();

    let _mekl = draft
        .execute(MakeEdgeKillLoop {
            he_a: he_outer,
            he_b: he_inner,
        })
        .unwrap()
        .into_value();

    let inner_loops_after_mekl = draft
        .arena()
        .get_face(target_face)
        .unwrap()
        .inner_loops()
        .len();
    assert_eq!(inner_loops_after_mekl, 0, "MEKL should kill the inner loop");

    let merged_he = draft.arena().get_loop(outer_loop).unwrap().half_edge();
    let merged_valence = collect_face_loop(draft.arena(), merged_he).unwrap().len();
    assert_eq!(
        merged_valence,
        outer_valence + inner_valence + 2,
        "Merged loop should have outer + inner + 2 bridge halfedges"
    );

    let _committed = draft.commit().unwrap();
}

/// MEKL rejects halfedges that are on different faces.
#[test]
fn mekl_rejects_cross_face_halfedges() {
    let env_res = unit_cube().expect("unit cube should succeed");
    let faces = env_res.get_value().faces().to_vec();
    let (mut draft, _geometry): (forge_topo::transactions::MutableDraft, _) =
        env_res.into_value().into_draft();

    let ha = first_halfedge_of_face(draft.arena(), faces[0]).unwrap();
    let hb = first_halfedge_of_face(draft.arena(), faces[1]).unwrap();

    let result = draft.execute(MakeEdgeKillLoop { he_a: ha, he_b: hb });

    assert!(
        result.is_err(),
        "MEKL should reject halfedges on different faces"
    );
}

/// KFMRH → MEKL → KEML roundtrip restores inner loop.
#[test]
fn mekl_keml_roundtrip() {
    let env_res = unit_cube().expect("unit cube should succeed");
    let faces = env_res.get_value().faces().to_vec();
    let (mut draft, _geometry): (forge_topo::transactions::MutableDraft, _) =
        env_res.into_value().into_draft();

    let face_to_kill = faces[0];
    let target_face = faces[1];

    draft
        .execute(KillFaceMakeRingHole {
            face_to_kill,
            target_face,
        })
        .unwrap();

    let outer_loop = draft.arena().get_face(target_face).unwrap().outer_loop();
    let inner_loop = draft.arena().get_face(target_face).unwrap().inner_loops()[0];

    let he_outer = draft.arena().get_loop(outer_loop).unwrap().half_edge();
    let he_inner = draft.arena().get_loop(inner_loop).unwrap().half_edge();

    let mekl = draft
        .execute(MakeEdgeKillLoop {
            he_a: he_outer,
            he_b: he_inner,
        })
        .unwrap()
        .into_value();

    assert_eq!(
        draft
            .arena()
            .get_face(target_face)
            .unwrap()
            .inner_loops()
            .len(),
        0
    );

    let _keml = draft
        .execute(KillEdgeMakeLoop { edge: mekl.he_ab })
        .unwrap()
        .into_value();

    assert_eq!(
        draft
            .arena()
            .get_face(target_face)
            .unwrap()
            .inner_loops()
            .len(),
        1,
        "KEML should restore the inner loop"
    );
}
