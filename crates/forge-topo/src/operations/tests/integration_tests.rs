//! Integration tests for sequences of Euler operators and traversal correctness.
//!
//! DOMAIN: Tests that exercise multiple operators together to verify the
//! topology is consistent end-to-end (not just per-operator). Also covers
//! traversal utilities and the validation guard.

use crate::b_rep::ShellKind;
use crate::boundary_editing::join_faces::JoinFaces;
use crate::entity_lifecycle::make_edge_face::MakeEdgeFace;
use crate::entity_lifecycle::make_vertex_face::MakeVertexFace;
use crate::entity_lifecycle::split_edge::SplitEdge;
use crate::non_manifold::sew_edge::SewEdge;
use crate::transactions::TopologyState;
use crate::traverse::{face_edge_count, FaceEdgeIterator, VertexRingIterator};
use forge_core::KernelError;

/// Building a closed sphere via Euler operators produces V=2, E=1, F=1.
#[test]
fn build_sphere_via_euler_operators() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let mvf = draft
        .execute(MakeVertexFace {
            shell_kind: ShellKind::Sheet,
        })
        .unwrap()
        .into_value();
    let se1 = draft
        .execute(SplitEdge {
            edge: mvf.half_edge,
        })
        .unwrap()
        .into_value();

    draft
        .execute(SewEdge {
            he_a: mvf.half_edge,
            he_b: se1.he_mb,
        })
        .unwrap();

    // The sphere is now watertight — promote shell from Sheet to Solid.
    let face = draft.arena().get_half_edge(mvf.half_edge).unwrap().face();
    let shell = draft.arena().get_face(face).unwrap().shell();
    draft
        .arena_mut()
        .get_shell_mut(shell)
        .unwrap()
        .set_kind(crate::b_rep::ShellKind::Solid(
            crate::b_rep::ShellOrientation::Outer,
        ));

    let arena = draft.arena();
    assert_eq!(arena.vertex_count(), 2);
    assert_eq!(arena.edge_count(), 1);
    assert_eq!(arena.face_count(), 1);

    let committed = draft.commit().unwrap();
    assert!(committed.epoch() > 0);
}

/// KV-15: Validation catches a deliberately broken twin pointer.
#[test]
fn kv15_validation_catches_broken_twins() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let mvf = draft
        .execute(MakeVertexFace {
            shell_kind: ShellKind::Sheet,
        })
        .unwrap()
        .into_value();
    let _se = draft
        .execute(SplitEdge {
            edge: mvf.half_edge,
        })
        .unwrap()
        .into_value();

    // Deliberately corrupt a twin pointer
    draft
        .arena_mut()
        .get_half_edge_mut(mvf.half_edge)
        .unwrap()
        .set_radial_next(crate::handles::HalfEdgeId::DANGLING);

    let result = draft.commit();
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        KernelError::TopologyViolation { .. }
    ));
}

/// face_edge_count grows by 1 after each SplitEdge on a face boundary.
#[test]
fn traversal_face_edges_counts_correctly() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let mvf = draft
        .execute(MakeVertexFace {
            shell_kind: ShellKind::Sheet,
        })
        .unwrap()
        .into_value();
    assert_eq!(face_edge_count(draft.arena(), mvf.face).unwrap(), 1);

    let _se = draft
        .execute(SplitEdge {
            edge: mvf.half_edge,
        })
        .unwrap()
        .into_value();
    assert_eq!(face_edge_count(draft.arena(), mvf.face).unwrap(), 2);
}

/// VertexRingIterator returns exactly 1 outgoing halfedge for a fresh MVF vertex.
#[test]
fn traversal_vertex_ring_for_seed() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let _mvf1 = draft
        .execute(MakeVertexFace {
            shell_kind: ShellKind::Sheet,
        })
        .unwrap()
        .into_value();
    let mvf2 = draft
        .execute(MakeVertexFace {
            shell_kind: ShellKind::Sheet,
        })
        .unwrap()
        .into_value();

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

        let mvf = draft
            .execute(MakeVertexFace {
                shell_kind: ShellKind::Sheet,
            })
            .unwrap()
            .into_value();
        let se = draft
            .execute(SplitEdge {
                edge: mvf.half_edge,
            })
            .unwrap()
            .into_value();
        let mef = draft
            .execute(MakeEdgeFace {
                vertex_a: mvf.vertex,
                vertex_b: se.new_vertex,
                face: mvf.face,
            })
            .unwrap()
            .into_value();
        draft
            .execute(JoinFaces {
                edge: mef.half_edge_ab,
            })
            .unwrap()
            .into_value();
        draft.commit().unwrap().topology_hash()
    };

    assert_eq!(
        build_and_join(),
        build_and_join(),
        "same op sequence must produce the same topology hash"
    );
}
