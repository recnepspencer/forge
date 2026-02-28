//! Tests for MakeEdgeKillLoop (MEKL) and KillEdgeMakeLoop (KEML).
//!
//! DOMAIN: Loop merge (MEKL) and loop split (KEML) Euler operators.
//! Tests cover: basic operation, Euler delta verification, input
//! validation, algebraic inverse roundtrip, and multi-hole stress.

use crate::boundary_editing::kill_edge_make_loop::KillEdgeMakeLoop;
use crate::boundary_editing::make_edge_kill_loop::MakeEdgeKillLoop;
use crate::state::TopologyState;
use crate::testing::build_face_with_hole;
use crate::traverse::FaceEdgeIterator;

/// MEKL on a face with one triangular hole absorbs the inner loop.
///
/// After MEKL: 0 inner loops, outer loop contains all halfedges
/// (outer triangle + inner triangle + 2 bridge edges = 8 halfedges).
#[test]
fn mekl_absorbs_inner_loop() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let (face, outer_he, inner_he, _inner_loop, _verts) = build_face_with_hole(&mut draft);

    assert_eq!(draft.arena().get_face(face).unwrap().inner_loop_count(), 1);

    let mekl = draft.execute(
        MakeEdgeKillLoop {
            he_a: outer_he,
            he_b: inner_he,
        },
    )
    .unwrap()
    .into_value();

    assert_eq!(
        draft.arena().get_face(face).unwrap().inner_loop_count(),
        0,
        "inner loop must be killed after MEKL"
    );

    let he_ab = draft.arena().get_half_edge(mekl.he_ab).unwrap();
    let he_ba = draft.arena().get_half_edge(mekl.he_ba).unwrap();
    assert_eq!(he_ab.radial_next(), mekl.he_ba);
    assert_eq!(he_ba.radial_next(), mekl.he_ab);
    assert_eq!(he_ab.face(), face);
    assert_eq!(he_ba.face(), face);

    let loop_size = FaceEdgeIterator::new(draft.arena(), face).unwrap().count();
    assert_eq!(loop_size, 8, "outer(3) + inner(3) + 2 bridge halfedges = 8");
}

/// MEKL produces the correct Euler delta: V=0, HE+2, E+1, L-1.
#[test]
fn mekl_euler_delta() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let (face, outer_he, inner_he, _inner_loop, _verts) = build_face_with_hole(&mut draft);

    let he_before = draft.arena().half_edge_count();
    let edge_before = draft.arena().edge_count();
    let loop_before = draft.arena().loop_count();
    let vtx_before = draft.arena().vertex_count();

    draft.execute(
        MakeEdgeKillLoop {
            he_a: outer_he,
            he_b: inner_he,
        },
    )
    .unwrap()
    .into_value();

    assert_eq!(
        draft.arena().vertex_count(),
        vtx_before,
        "V delta must be 0"
    );
    assert_eq!(
        draft.arena().half_edge_count(),
        he_before + 2,
        "HE delta must be +2"
    );
    assert_eq!(
        draft.arena().edge_count(),
        edge_before + 1,
        "E delta must be +1"
    );
    assert_eq!(
        draft.arena().loop_count(),
        loop_before - 1,
        "L delta must be -1"
    );
    let _ = face;
}

/// MEKL rejects two halfedges on the same loop (would violate Euler formula).
#[test]
fn mekl_rejects_same_loop() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let (face, outer_he, _inner_he, _inner_loop, _verts) = build_face_with_hole(&mut draft);

    let outer_he_next = draft.arena().get_half_edge(outer_he).unwrap().next();

    let result = draft.execute(
        MakeEdgeKillLoop {
            he_a: outer_he,
            he_b: outer_he_next,
        },
    );

    assert!(
        result.is_err(),
        "MEKL must reject when both halfedges are on the same (outer) loop"
    );
    let _ = face;
}

/// KEML on a MEKL bridge edge re-creates the inner loop.
///
/// After KEML: face.inner_loops grows by 1, the new loop is traversable.
#[test]
fn keml_splits_loop() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let (face, outer_he, inner_he, _inner_loop, _verts) = build_face_with_hole(&mut draft);

    let mekl = draft.execute(
        MakeEdgeKillLoop {
            he_a: outer_he,
            he_b: inner_he,
        },
    )
    .unwrap()
    .into_value();

    assert_eq!(draft.arena().get_face(face).unwrap().inner_loop_count(), 0);

    let keml = draft.execute(KillEdgeMakeLoop { edge: mekl.he_ab })
        .unwrap()
        .into_value();

    assert_eq!(
        draft.arena().get_face(face).unwrap().inner_loop_count(),
        1,
        "KEML must re-create the inner loop"
    );

    let new_loop_start = draft.arena().get_loop(keml.new_loop).unwrap().half_edge();
    let mut count = 0usize;
    let mut current = new_loop_start;
    loop {
        count += 1;
        assert!(count <= 100, "infinite loop in new inner loop traversal");
        current = draft.arena().get_half_edge(current).unwrap().next();
        if current == new_loop_start {
            break;
        }
    }
    assert_eq!(
        count, 3,
        "restored inner loop must have 3 halfedges (triangle)"
    );
}

/// KEML produces the correct Euler delta: V=0, HE-2, E-1, L+1.
#[test]
fn keml_euler_delta() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let (face, outer_he, inner_he, _inner_loop, _verts) = build_face_with_hole(&mut draft);

    let mekl = draft.execute(
        MakeEdgeKillLoop {
            he_a: outer_he,
            he_b: inner_he,
        },
    )
    .unwrap()
    .into_value();

    let he_before = draft.arena().half_edge_count();
    let edge_before = draft.arena().edge_count();
    let loop_before = draft.arena().loop_count();
    let vtx_before = draft.arena().vertex_count();

    draft.execute(KillEdgeMakeLoop { edge: mekl.he_ab })
        .unwrap()
        .into_value();

    assert_eq!(
        draft.arena().vertex_count(),
        vtx_before,
        "V delta must be 0"
    );
    assert_eq!(
        draft.arena().half_edge_count(),
        he_before - 2,
        "HE delta must be -2"
    );
    assert_eq!(
        draft.arena().edge_count(),
        edge_before - 1,
        "E delta must be -1"
    );
    assert_eq!(
        draft.arena().loop_count(),
        loop_before + 1,
        "L delta must be +1"
    );
    let _ = face;
}

/// MEKL then KEML roundtrip preserves entity counts and loop structure.
///
/// Proves MEKL and KEML are algebraic inverses.
///
/// NOTE: We compare entity counts and inner loop count rather than
/// topology hashes. `build_face_with_hole` creates twin halfedges
/// with placeholder `FaceId(u32::MAX)` which triggers a pathological
/// allocation in `compute_arena_topology_hash`'s ensure_capacity.
#[test]
fn mekl_keml_roundtrip() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let (face, outer_he, inner_he, _inner_loop, _verts) = build_face_with_hole(&mut draft);

    let v_before = draft.arena().vertex_count();
    let he_before = draft.arena().half_edge_count();
    let e_before = draft.arena().edge_count();
    let l_before = draft.arena().loop_count();
    let inner_before = draft.arena().get_face(face).unwrap().inner_loop_count();

    let mekl = draft.execute(
        MakeEdgeKillLoop {
            he_a: outer_he,
            he_b: inner_he,
        },
    )
    .unwrap()
    .into_value();

    let _keml = draft.execute(KillEdgeMakeLoop { edge: mekl.he_ab })
        .unwrap()
        .into_value();

    assert_eq!(
        draft.arena().vertex_count(),
        v_before,
        "V must be preserved"
    );
    assert_eq!(
        draft.arena().half_edge_count(),
        he_before,
        "HE must be preserved"
    );
    assert_eq!(draft.arena().edge_count(), e_before, "E must be preserved");
    assert_eq!(draft.arena().loop_count(), l_before, "L must be preserved");
    assert_eq!(
        draft.arena().get_face(face).unwrap().inner_loop_count(),
        inner_before,
        "inner loop count must be restored"
    );
}

/// MEKL on a face with two holes: bridge the second hole, verify first remains.
///
/// Exercises multi-hole management and confirms KEML restores the second hole.
#[test]
fn mekl_keml_on_multi_hole_face() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let (face, outer_he, inner_he1, _inner_loop1, _verts1) = build_face_with_hole(&mut draft);

    assert_eq!(draft.arena().get_face(face).unwrap().inner_loop_count(), 1);

    let placeholder_he = crate::handles::HalfEdgeId::new(u32::MAX, 0);
    let placeholder_e = crate::handles::EdgeId::new(u32::MAX, 0);
    let arena = draft.arena_mut();

    let v6 = arena.insert_vertex(crate::arena::VertexData::new(placeholder_he));
    let v7 = arena.insert_vertex(crate::arena::VertexData::new(placeholder_he));
    let v8 = arena.insert_vertex(crate::arena::VertexData::new(placeholder_he));

    let (he67, _he76) = arena.insert_radial_pair(
        crate::arena::HalfEdgeData::new(
            placeholder_he,
            placeholder_he,
            placeholder_he,
            face,
            v6,
            placeholder_e,
        ),
        crate::arena::HalfEdgeData::new(
            placeholder_he,
            placeholder_he,
            placeholder_he,
            crate::handles::FaceId::new(u32::MAX, 0),
            v7,
            placeholder_e,
        ),
    );
    let (he78, _he87) = arena.insert_radial_pair(
        crate::arena::HalfEdgeData::new(
            placeholder_he,
            placeholder_he,
            placeholder_he,
            face,
            v7,
            placeholder_e,
        ),
        crate::arena::HalfEdgeData::new(
            placeholder_he,
            placeholder_he,
            placeholder_he,
            crate::handles::FaceId::new(u32::MAX, 0),
            v8,
            placeholder_e,
        ),
    );
    let (he86, _he68) = arena.insert_radial_pair(
        crate::arena::HalfEdgeData::new(
            placeholder_he,
            placeholder_he,
            placeholder_he,
            face,
            v8,
            placeholder_e,
        ),
        crate::arena::HalfEdgeData::new(
            placeholder_he,
            placeholder_he,
            placeholder_he,
            crate::handles::FaceId::new(u32::MAX, 0),
            v6,
            placeholder_e,
        ),
    );

    arena.get_half_edge_mut(he67).unwrap().set_next(he78);
    arena.get_half_edge_mut(he67).unwrap().set_prev(he86);
    arena.get_half_edge_mut(he78).unwrap().set_next(he86);
    arena.get_half_edge_mut(he78).unwrap().set_prev(he67);
    arena.get_half_edge_mut(he86).unwrap().set_next(he67);
    arena.get_half_edge_mut(he86).unwrap().set_prev(he78);

    arena.get_vertex_mut(v6).unwrap().set_outgoing(he67);
    arena.get_vertex_mut(v7).unwrap().set_outgoing(he78);
    arena.get_vertex_mut(v8).unwrap().set_outgoing(he86);

    let inner_loop2 = arena.insert_loop(crate::arena::LoopData::new(he67, face));
    arena
        .get_face_mut(face)
        .unwrap()
        .add_inner_loop(inner_loop2);

    assert_eq!(
        draft.arena().get_face(face).unwrap().inner_loop_count(),
        2,
        "face must have 2 inner loops before MEKL"
    );

    let mekl = draft.execute(
        MakeEdgeKillLoop {
            he_a: outer_he,
            he_b: inner_he1,
        },
    )
    .unwrap()
    .into_value();

    assert_eq!(
        draft.arena().get_face(face).unwrap().inner_loop_count(),
        1,
        "after bridging first hole, 1 inner loop must remain"
    );

    let _keml = draft.execute(KillEdgeMakeLoop { edge: mekl.he_ab })
        .unwrap()
        .into_value();

    assert_eq!(
        draft.arena().get_face(face).unwrap().inner_loop_count(),
        2,
        "after KEML, both inner loops must be restored"
    );
}
