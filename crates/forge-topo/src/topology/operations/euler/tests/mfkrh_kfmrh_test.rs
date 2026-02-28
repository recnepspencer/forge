//! Tests for MakeFaceKillRingHole (MFKRH) and KillFaceMakeRingHole (KFMRH).
//!
//! DOMAIN: MFKRH promotes an inner loop to its own face. KFMRH is the
//! inverse — demotes a face back into a hole. Tests cover entity deltas,
//! inner loop bookkeeping, halfedge face pointer updates, lineage, and
//! roundtrip invariance.

use crate::euler::kill_face_make_ring_hole::KillFaceMakeRingHole;
use crate::euler::make_face_kill_ring_hole::MakeFaceKillRingHole;
use crate::operator::apply_op;
use crate::state::TopologyState;
use crate::testing::build_face_with_hole;

/// MFKRH on a face with one inner loop promotes it to a new face.
#[test]
fn mfkrh_promotes_inner_loop() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let (face, _outer_he, inner_he, _outer_loop, _verts) = build_face_with_hole(&mut draft);

    assert_eq!(draft.arena().get_face(face).unwrap().inner_loop_count(), 1);

    let inner_loop = draft.arena().get_face(face).unwrap().inner_loops()[0];

    let f_before = draft.arena().face_count();
    let l_before = draft.arena().loop_count();

    let mfkrh = apply_op(&mut draft, MakeFaceKillRingHole { loop_id: inner_loop })
        .unwrap()
        .into_value();

    assert_eq!(draft.arena().face_count(), f_before + 1, "ΔF = +1");
    assert_eq!(draft.arena().loop_count(), l_before, "ΔL = 0 (loop moved, not created)");

    assert_eq!(
        draft.arena().get_face(face).unwrap().inner_loop_count(),
        0,
        "old face must have no inner loops after promotion"
    );

    let new_face_data = draft.arena().get_face(mfkrh.new_face).unwrap();
    assert_eq!(
        new_face_data.outer_loop(),
        inner_loop,
        "promoted loop must be outer loop of new face"
    );

    let inner_he_face = draft.arena().get_half_edge(inner_he).unwrap().face();
    assert_eq!(
        inner_he_face, mfkrh.new_face,
        "halfedges on promoted loop must point to new face"
    );
}

// TODO(Phase 3): Re-enable once LineageStore lookup is wired.
// /// MFKRH stamps lineage on the new face derived from the parent face.
// #[test]
// fn mfkrh_stamps_lineage() { ... }

/// MFKRH rejects the outer loop (only inner loops can be promoted).
#[test]
fn mfkrh_rejects_outer_loop() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let (face, _outer_he, _inner_he, outer_loop, _verts) = build_face_with_hole(&mut draft);
    let _ = face;

    let result = apply_op(&mut draft, MakeFaceKillRingHole { loop_id: outer_loop });
    assert!(result.is_err(), "MFKRH must reject the outer loop");
}

/// KFMRH demotes a face's outer loop into an inner loop of a target face.
#[test]
fn kfmrh_demotes_face_to_hole() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let (face, _outer_he, inner_he, _outer_loop, _verts) = build_face_with_hole(&mut draft);

    let inner_loop = draft.arena().get_face(face).unwrap().inner_loops()[0];

    let mfkrh = apply_op(&mut draft, MakeFaceKillRingHole { loop_id: inner_loop })
        .unwrap()
        .into_value();

    assert_eq!(draft.arena().get_face(face).unwrap().inner_loop_count(), 0);

    let f_before = draft.arena().face_count();

    apply_op(
        &mut draft,
        KillFaceMakeRingHole {
            face_to_kill: mfkrh.new_face,
            target_face: face,
        },
    )
    .unwrap()
    .into_value();

    assert_eq!(draft.arena().face_count(), f_before - 1, "ΔF = -1");
    assert_eq!(
        draft.arena().get_face(face).unwrap().inner_loop_count(),
        1,
        "inner loop must be restored on target face"
    );

    let inner_he_face = draft.arena().get_half_edge(inner_he).unwrap().face();
    assert_eq!(
        inner_he_face, face,
        "halfedges on demoted loop must point back to target face"
    );
}

/// MFKRH→KFMRH roundtrip preserves all entity counts.
#[test]
fn mfkrh_kfmrh_roundtrip() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let (face, _outer_he, _inner_he, _outer_loop, _verts) = build_face_with_hole(&mut draft);

    let f_before = draft.arena().face_count();
    let l_before = draft.arena().loop_count();
    let inner_before = draft.arena().get_face(face).unwrap().inner_loop_count();

    let inner_loop = draft.arena().get_face(face).unwrap().inner_loops()[0];

    let mfkrh = apply_op(&mut draft, MakeFaceKillRingHole { loop_id: inner_loop })
        .unwrap()
        .into_value();

    apply_op(
        &mut draft,
        KillFaceMakeRingHole {
            face_to_kill: mfkrh.new_face,
            target_face: face,
        },
    )
    .unwrap()
    .into_value();

    assert_eq!(draft.arena().face_count(), f_before, "F preserved");
    assert_eq!(draft.arena().loop_count(), l_before, "L preserved");
    assert_eq!(
        draft.arena().get_face(face).unwrap().inner_loop_count(),
        inner_before,
        "inner loop count preserved"
    );
}
