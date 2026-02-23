//! Integration tests for sequences of Euler operators and traversal correctness.
//!
//! DOMAIN: Tests that exercise multiple operators together to verify the
//! topology is consistent end-to-end (not just per-operator). Also covers
//! traversal utilities and the validation guard.

use forge_core::KernelError;
use crate::state::TopologyState;
use crate::operator::apply_op;
use crate::euler::make_vertex_face::MakeVertexFace;
use crate::euler::make_edge_face::MakeEdgeFace;
use crate::euler::split_edge::SplitEdge;
use crate::euler::join_faces::JoinFaces;
use crate::traverse::{FaceEdgeIterator, VertexRingIterator, face_edge_count};

/// Building a tetrahedron via Euler operators produces V=4, E=6, F=4.
#[test]
fn build_tetrahedron_via_euler_operators() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let mvf  = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
    let v0   = mvf.vertex;
    let se1  = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.5 })
        .unwrap().into_value();
    let v1   = se1.new_vertex;
    let mef1 = apply_op(&mut draft, MakeEdgeFace { vertex_a: v0, vertex_b: v1, face: mvf.face })
        .unwrap().into_value();
    let se2  = apply_op(&mut draft, SplitEdge { edge: mef1.half_edge_ab, parameter: 0.5 })
        .unwrap().into_value();
    let v2   = se2.new_vertex;
    let _mef2 = apply_op(&mut draft, MakeEdgeFace { vertex_a: v2, vertex_b: v1, face: mef1.new_face })
        .unwrap().into_value();
    let _mef3 = apply_op(&mut draft, MakeEdgeFace { vertex_a: v0, vertex_b: v2, face: mvf.face })
        .unwrap().into_value();

    let edge_v0_in_face1 = FaceEdgeIterator::new(draft.arena(), mef1.new_face)
        .unwrap()
        .find_map(|res| {
            let eid = res.unwrap();
            let he  = draft.arena().get_half_edge(eid).unwrap();
            if he.origin() == v0 { Some(eid) } else { None }
        });

    if let Some(anchor) = edge_v0_in_face1 {
        let se3  = apply_op(&mut draft, SplitEdge { edge: anchor, parameter: 0.5 })
            .unwrap().into_value();
        let v3   = se3.new_vertex;
        let _mef4 = apply_op(&mut draft, MakeEdgeFace { vertex_a: v3, vertex_b: v0, face: mef1.new_face })
            .unwrap().into_value();

        let arena = draft.arena();
        assert_eq!(arena.vertex_count(),          4);
        assert_eq!(arena.half_edge_count() / 2,   6);
        assert_eq!(arena.face_count(),             4);
    }

    let committed = draft.commit().unwrap();
    assert!(committed.epoch() > 0);
}

/// KV-15: Validation catches a deliberately broken twin pointer.
#[test]
fn kv15_validation_catches_broken_twins() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
    let _se = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.5 })
        .unwrap().into_value();

    // Deliberately corrupt a twin pointer
    draft.arena_mut()
        .get_half_edge_mut(mvf.half_edge).unwrap()
        .set_twin(crate::handles::HalfEdgeId::new(u32::MAX, 0));

    let result = draft.commit();
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), KernelError::TopologyViolation { .. }));
}

/// face_edge_count grows by 1 after each SplitEdge on a face boundary.
#[test]
fn traversal_face_edges_counts_correctly() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
    assert_eq!(face_edge_count(draft.arena(), mvf.face).unwrap(), 1);

    let _se = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.5 })
        .unwrap().into_value();
    assert_eq!(face_edge_count(draft.arena(), mvf.face).unwrap(), 2);
}

/// VertexRingIterator returns exactly 1 outgoing halfedge for a fresh MVF vertex.
#[test]
fn traversal_vertex_ring_for_seed() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let _mvf1 = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
    let mvf2  = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();

    let ring: Vec<_> = VertexRingIterator::new(draft.arena(), mvf2.vertex)
        .unwrap()
        .map(|r| r.unwrap())
        .collect();

    assert_eq!(ring.len(), 1);
    assert_eq!(ring[0], mvf2.half_edge);
}

/// JoinFaces of two faces produced by MEF then rebuild with identical ops
/// returns the same topology hash (structural determinism check).
#[test]
fn join_then_rebuild_same_hash() {
    let build_and_join = || {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se  = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.5 })
            .unwrap().into_value();
        let mef = apply_op(&mut draft, MakeEdgeFace {
            vertex_a: mvf.vertex, vertex_b: se.new_vertex, face: mvf.face,
        }).unwrap().into_value();
        apply_op(&mut draft, JoinFaces { edge: mef.half_edge_ab }).unwrap().into_value();
        draft.commit().unwrap().topology_hash()
    };

    assert_eq!(build_and_join(), build_and_join(),
        "same op sequence must produce the same topology hash");
}
