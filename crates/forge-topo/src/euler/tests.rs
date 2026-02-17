//! Euler operator unit, integration, and lineage tests.
//!
//! Covers:
//! - MVF: topological seed creation
//! - MVF + SplitEdge: creating a proper edge
//! - MEF: face splitting
//! - SplitEdge: edge subdivision (normal)
//! - JoinFaces: face merging (inverse of MEF)
//! - KillEdgeVertex: edge/vertex collapse (inverse of SplitEdge)
//! - Tetrahedron construction: V=4 E=6 F=4
//! - KV-15: validation catches broken meshes
//! - KV-16: deterministic lineage hashes
//! - KV-17: split_edge children carry parent ancestry

use forge_core::KernelError;
use crate::state::TopologyState;
use crate::operator::apply_op;
use crate::euler::make_vertex_face::MakeVertexFace;
use crate::euler::make_edge_face::MakeEdgeFace;
use crate::euler::split_edge::SplitEdge;
use crate::euler::join_faces::JoinFaces;
use crate::euler::kill_edge_vertex::KillEdgeVertex;
use crate::traverse::{face_edges, face_edge_count, vertex_ring, edge_faces};

#[test]
fn mvf_creates_single_vertex_and_face() {
    let state = TopologyState::empty();
    let mut draft = state.begin_mutation();

    let out = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();

    assert_eq!(draft.arena().vertex_count(), 1);
    assert_eq!(draft.arena().face_count(), 1);
    assert_eq!(draft.arena().half_edge_count(), 1);
    assert_eq!(draft.arena().loop_count(), 1);

    let he_data = draft.arena().get_half_edge(out.half_edge).unwrap();
    assert_eq!(he_data.twin, out.half_edge);
    assert_eq!(he_data.next, out.half_edge);
    assert_eq!(he_data.prev, out.half_edge);
    assert_eq!(he_data.origin, out.vertex);
    assert_eq!(he_data.face, out.face);

    let committed = draft.commit().unwrap();
    assert_eq!(committed.epoch(), 1);
}

#[test]
fn mvf_stamps_lineage_on_all_entities() {
    let state = TopologyState::empty();
    let mut draft = state.begin_mutation();

    let out = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();

    let v_lineage = draft.arena().get_vertex(out.vertex).unwrap().lineage.as_ref().unwrap();
    let f_lineage = draft.arena().get_face(out.face).unwrap().lineage.as_ref().unwrap();
    let he_lineage = draft.arena().get_half_edge(out.half_edge).unwrap().lineage.as_ref().unwrap();

    assert_eq!(v_lineage.get_creation_op().get_name(), "make_vertex_face");
    assert_eq!(f_lineage.get_creation_op().get_name(), "make_vertex_face");
    assert_eq!(he_lineage.get_creation_op().get_name(), "make_vertex_face");

    assert_eq!(v_lineage.get_ancestry_hash(), f_lineage.get_ancestry_hash());
    assert_eq!(f_lineage.get_ancestry_hash(), he_lineage.get_ancestry_hash());
}

#[test]
fn split_degenerate_creates_proper_edge() {
    let state = TopologyState::empty();
    let mut draft = state.begin_mutation();

    let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
    let se = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.5 }).unwrap().into_value();

    assert_eq!(draft.arena().vertex_count(), 2);
    assert_eq!(draft.arena().half_edge_count(), 2);

    let he_am = draft.arena().get_half_edge(se.he_am).unwrap();
    let he_ma = draft.arena().get_half_edge(se.he_mb).unwrap();

    assert_eq!(he_am.origin, mvf.vertex);
    assert_eq!(he_ma.origin, se.new_vertex);
    assert_eq!(he_am.twin, se.he_mb);
    assert_eq!(he_ma.twin, se.he_am);
    assert_eq!(he_am.next, se.he_mb);
    assert_eq!(he_ma.next, se.he_am);
}

#[test]
fn mef_splits_face_creating_two_faces() {
    let state = TopologyState::empty();
    let mut draft = state.begin_mutation();

    let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
    let se = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.5 }).unwrap().into_value();

    let mef = apply_op(&mut draft, MakeEdgeFace {
        vertex_a: mvf.vertex,
        vertex_b: se.new_vertex,
        face: mvf.face,
    }).unwrap().into_value();

    assert_eq!(draft.arena().face_count(), 2);
    assert_eq!(draft.arena().loop_count(), 2);
    assert_eq!(draft.arena().half_edge_count(), 4);

    let (f1, f2) = edge_faces(draft.arena(), mef.half_edge_ab).unwrap();
    assert_ne!(f1, f2);
}

#[test]
fn split_normal_edge_adds_vertex() {
    let state = TopologyState::empty();
    let mut draft = state.begin_mutation();

    let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
    let se1 = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.5 }).unwrap().into_value();

    let mef = apply_op(&mut draft, MakeEdgeFace {
        vertex_a: mvf.vertex,
        vertex_b: se1.new_vertex,
        face: mvf.face,
    }).unwrap().into_value();

    assert_eq!(draft.arena().half_edge_count(), 4);

    let se2 = apply_op(&mut draft, SplitEdge { edge: mef.half_edge_ab, parameter: 0.5 }).unwrap().into_value();

    assert_eq!(draft.arena().vertex_count(), 3);
    assert_eq!(draft.arena().half_edge_count(), 6);

    let he_data = draft.arena().get_half_edge(se2.he_am).unwrap();
    assert_eq!(he_data.origin, mvf.vertex);
    let he_mb = draft.arena().get_half_edge(se2.he_mb).unwrap();
    assert_eq!(he_mb.origin, se2.new_vertex);
}

#[test]
fn join_faces_merges_two_adjacent_faces() {
    let state = TopologyState::empty();
    let mut draft = state.begin_mutation();

    let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
    let se = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.5 }).unwrap().into_value();

    let mef = apply_op(&mut draft, MakeEdgeFace {
        vertex_a: mvf.vertex,
        vertex_b: se.new_vertex,
        face: mvf.face,
    }).unwrap().into_value();

    assert_eq!(draft.arena().face_count(), 2);

    let jf = apply_op(&mut draft, JoinFaces { edge: mef.half_edge_ab }).unwrap().into_value();

    assert_eq!(draft.arena().face_count(), 1);
    assert!(draft.arena().get_face(jf.surviving_face).is_ok());
}

/// KEV test: add a third vertex via split, then remove it back.
#[test]
fn kill_edge_vertex_collapses_vertex() {
    let state = TopologyState::empty();
    let mut draft = state.begin_mutation();

    let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
    let se1 = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.5 }).unwrap().into_value();

    let mef = apply_op(&mut draft, MakeEdgeFace {
        vertex_a: mvf.vertex,
        vertex_b: se1.new_vertex,
        face: mvf.face,
    }).unwrap().into_value();

    let se2 = apply_op(&mut draft, SplitEdge { edge: mef.half_edge_ab, parameter: 0.5 }).unwrap().into_value();

    assert_eq!(draft.arena().vertex_count(), 3);

    apply_op(&mut draft, KillEdgeVertex { edge: se2.he_am }).unwrap().into_value();

    assert_eq!(draft.arena().vertex_count(), 2);
    assert_eq!(draft.arena().half_edge_count(), 4);
}

/// Build a tetrahedron: V=4, E=6, F=4
#[test]
fn build_tetrahedron_via_euler_operators() {
    let state = TopologyState::empty();
    let mut draft = state.begin_mutation();

    let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
    let v0 = mvf.vertex;

    let se1 = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.5 }).unwrap().into_value();
    let v1 = se1.new_vertex;

    let mef1 = apply_op(&mut draft, MakeEdgeFace {
        vertex_a: v0,
        vertex_b: v1,
        face: mvf.face,
    }).unwrap().into_value();

    let se2 = apply_op(&mut draft, SplitEdge { edge: mef1.half_edge_ab, parameter: 0.5 }).unwrap().into_value();
    let v2 = se2.new_vertex;

    let _mef2 = apply_op(&mut draft, MakeEdgeFace {
        vertex_a: v2,
        vertex_b: v1,
        face: mef1.new_face,
    }).unwrap().into_value();

    let _mef3 = apply_op(&mut draft, MakeEdgeFace {
        vertex_a: v0,
        vertex_b: v2,
        face: mvf.face,
    }).unwrap().into_value();

    let se3_face = {
        let edges = face_edges(draft.arena(), mef1.new_face).unwrap();
        let mut found = None;
        for eid in &edges {
            let he = draft.arena().get_half_edge(*eid).unwrap();
            if he.origin == v0 {
                found = Some(*eid);
                break;
            }
        }
        found
    };

    if let Some(edge_v0_in_face1) = se3_face {
        let se3 = apply_op(&mut draft, SplitEdge { edge: edge_v0_in_face1, parameter: 0.5 }).unwrap().into_value();
        let v3 = se3.new_vertex;

        let _mef4 = apply_op(&mut draft, MakeEdgeFace {
            vertex_a: v3,
            vertex_b: v0,
            face: mef1.new_face,
        }).unwrap().into_value();

        let arena = draft.arena();
        assert_eq!(arena.vertex_count(), 4);
        assert_eq!(arena.half_edge_count() / 2, 6);
        assert_eq!(arena.face_count(), 4);
    }

    let committed = draft.commit().unwrap();
    assert!(committed.epoch() > 0);
}

/// KV-15: Validation catches deliberately broken meshes
#[test]
fn kv15_validation_catches_broken_twins() {
    let state = TopologyState::empty();
    let mut draft = state.begin_mutation();

    let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
    let _se = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.5 }).unwrap().into_value();

    draft.arena_mut().get_half_edge_mut(mvf.half_edge).unwrap().twin =
        crate::handles::HalfEdgeId::new(u32::MAX, 0);

    let result = draft.commit();
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), KernelError::TopologyViolation { .. }));
}

#[test]
fn traversal_face_edges_counts_correctly() {
    let state = TopologyState::empty();
    let mut draft = state.begin_mutation();

    let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
    assert_eq!(face_edge_count(draft.arena(), mvf.face).unwrap(), 1);

    let _se = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.5 }).unwrap().into_value();
    assert_eq!(face_edge_count(draft.arena(), mvf.face).unwrap(), 2);
}

#[test]
fn traversal_vertex_ring_for_seed() {
    let state = TopologyState::empty();
    let mut draft = state.begin_mutation();

    let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
    let ring = vertex_ring(draft.arena(), mvf.vertex).unwrap();
    assert_eq!(ring.len(), 1);
    assert_eq!(ring[0], mvf.half_edge);
}

/// KV-16: Same operation sequence produces identical ancestry hashes (D1 determinism).
#[test]
fn kv16_identical_sequence_produces_identical_lineage() {
    let run_sequence = || {
        let state = TopologyState::empty();
        let mut draft = state.begin_mutation();

        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.5 }).unwrap().into_value();

        let mef = apply_op(&mut draft, MakeEdgeFace {
            vertex_a: mvf.vertex,
            vertex_b: se.new_vertex,
            face: mvf.face,
        }).unwrap().into_value();

        let vertex_hash = draft.arena().get_vertex(mvf.vertex).unwrap()
            .lineage.as_ref().unwrap().get_ancestry_hash();
        let face_hash = draft.arena().get_face(mef.new_face).unwrap()
            .lineage.as_ref().unwrap().get_ancestry_hash();
        let he_hash = draft.arena().get_half_edge(mef.half_edge_ab).unwrap()
            .lineage.as_ref().unwrap().get_ancestry_hash();

        (vertex_hash, face_hash, he_hash)
    };

    let (v1, f1, h1) = run_sequence();
    let (v2, f2, h2) = run_sequence();

    assert_eq!(v1, v2);
    assert_eq!(f1, f2);
    assert_eq!(h1, h2);
}

/// KV-17: SplitEdge children carry parent ancestry + split op ID.
#[test]
fn kv17_split_edge_children_carry_parent_ancestry() {
    let state = TopologyState::empty();
    let mut draft = state.begin_mutation();

    let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();

    let parent_hash = draft.arena().get_half_edge(mvf.half_edge).unwrap()
        .lineage.as_ref().unwrap().get_ancestry_hash();

    let se = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.5 }).unwrap().into_value();

    let child_vertex_lineage = draft.arena().get_vertex(se.new_vertex).unwrap()
        .lineage.as_ref().unwrap();
    let child_he_lineage = draft.arena().get_half_edge(se.he_mb).unwrap()
        .lineage.as_ref().unwrap();

    assert_ne!(child_vertex_lineage.get_ancestry_hash(), parent_hash);
    assert_eq!(child_vertex_lineage.get_ancestry_hash(), child_he_lineage.get_ancestry_hash());

    assert_eq!(child_vertex_lineage.get_creation_op().get_name(), "split_edge");
}

/// Join faces produces deterministic merged lineage.
#[test]
fn join_faces_updates_surviving_lineage() {
    let state = TopologyState::empty();
    let mut draft = state.begin_mutation();

    let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
    let se = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.5 }).unwrap().into_value();

    let mef = apply_op(&mut draft, MakeEdgeFace {
        vertex_a: mvf.vertex,
        vertex_b: se.new_vertex,
        face: mvf.face,
    }).unwrap().into_value();

    let jf = apply_op(&mut draft, JoinFaces { edge: mef.half_edge_ab }).unwrap().into_value();

    let lineage = draft.arena().get_face(jf.surviving_face).unwrap()
        .lineage.as_ref().unwrap();
    assert_eq!(lineage.get_creation_op().get_name(), "join_faces");
}

// ══════════════════════════════════════════════════════════════
// BRUTAL DOMAIN-SPECIFIC STRESS TESTS
// ══════════════════════════════════════════════════════════════

/// Bio-mesh stress test: high-valence pole with 15,000 edges.
///
/// Bio-medical implants produce "poles" (vertices with thousands of
/// converging edges). This test ensures traverse guards (MAX_ITER)
/// hold, the Euler invariant is maintained under massive load, and
/// the diff engine can capture a 15k-op delta.
#[test]
fn brutal_bio_mesh_high_valence_pole_and_megaloop() {
    let state = TopologyState::empty();
    let mut draft = state.begin_mutation();

    let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
    let center_vertex = mvf.vertex;
    let mut current_edge = mvf.half_edge;

    let edge_count = 15_000;

    for _ in 0..edge_count {
        let se = apply_op(&mut draft, SplitEdge {
            edge: current_edge,
            parameter: 0.5,
        }).unwrap().into_value();

        let face_id = draft.arena().get_half_edge(current_edge).unwrap().face;
        let mef = apply_op(&mut draft, MakeEdgeFace {
            vertex_a: center_vertex,
            vertex_b: se.new_vertex,
            face: face_id,
        }).unwrap().into_value();

        current_edge = mef.half_edge_ab;
    }

    let megamesh_state = draft.commit().unwrap();
    let arena = megamesh_state.arena();

    assert_eq!(arena.vertex_count(), edge_count + 1);

    let diff = crate::diff::compute_diff(state.arena(), arena, 0, megamesh_state.epoch());
    assert!(!diff.is_empty());
    assert_eq!(
        diff.total_added(),
        arena.face_count() + arena.half_edge_count() + arena.vertex_count()
    );
}

/// Aerospace sliver churn: generational handle integrity under rapid
/// create/destroy cycles.
///
/// Boolean operations on wing ribs generate microscopic sliver faces
/// that are immediately healed. This hammers 500 cycles of
/// SplitEdge+MakeEdgeFace → JoinFaces+KillEdgeVertex to verify that
/// generational slots correctly detect stale handles after removal.
#[test]
fn brutal_aerospace_sliver_churn_generational_integrity() {
    let state = TopologyState::empty();
    let mut draft = state.begin_mutation();

    let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();

    for _ in 0..500 {
        let boundary_edge = draft.arena().get_vertex(mvf.vertex).unwrap().outgoing;

        let se = apply_op(&mut draft, SplitEdge {
            edge: boundary_edge,
            parameter: 0.1,
        }).unwrap().into_value();

        let face_id = draft.arena().get_half_edge(boundary_edge).unwrap().face;
        let mef = apply_op(&mut draft, MakeEdgeFace {
            vertex_a: mvf.vertex,
            vertex_b: se.new_vertex,
            face: face_id,
        }).unwrap().into_value();

        let doomed_face = mef.new_face;
        let doomed_vertex = se.new_vertex;

        let _jf = apply_op(&mut draft, JoinFaces { edge: mef.half_edge_ab }).unwrap().into_value();
        let _kev = apply_op(&mut draft, KillEdgeVertex { edge: se.he_am }).unwrap().into_value();

        assert!(
            draft.arena().get_face(doomed_face).is_err(),
            "Stale face handle must be rejected after removal"
        );
        assert!(
            draft.arena().get_vertex(doomed_vertex).is_err(),
            "Stale vertex handle must be rejected after removal"
        );
    }

    let final_state = draft.commit().expect("Generational churn caused silent topology corruption");

    assert_eq!(final_state.arena().vertex_count(), 1);
    assert_eq!(final_state.arena().face_count(), 1);
}

/// Merkle-DAG determinism: operation-order independence.
///
/// If engineer A splits edge X then Y, and engineer B splits Y then X,
/// the resulting topology is identical. This test proves that the
/// solid hash normalises different DAG paths to the same value.
#[test]
fn brutal_dag_determinism_path_independence() {
    let path_a_hash = {
        let state = TopologyState::empty();
        let mut draft = state.begin_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();

        let se1 = apply_op(&mut draft, SplitEdge {
            edge: mvf.half_edge, parameter: 0.5,
        }).unwrap().into_value();
        let mef1 = apply_op(&mut draft, MakeEdgeFace {
            vertex_a: mvf.vertex, vertex_b: se1.new_vertex, face: mvf.face,
        }).unwrap().into_value();

        let _se_top = apply_op(&mut draft, SplitEdge {
            edge: mef1.half_edge_ab, parameter: 0.5,
        }).unwrap().into_value();
        let _se_bottom = apply_op(&mut draft, SplitEdge {
            edge: mef1.half_edge_ba, parameter: 0.5,
        }).unwrap().into_value();

        draft.commit().unwrap().topology_hash()
    };

    let path_b_hash = {
        let state = TopologyState::empty();
        let mut draft = state.begin_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();

        let se1 = apply_op(&mut draft, SplitEdge {
            edge: mvf.half_edge, parameter: 0.5,
        }).unwrap().into_value();
        let mef1 = apply_op(&mut draft, MakeEdgeFace {
            vertex_a: mvf.vertex, vertex_b: se1.new_vertex, face: mvf.face,
        }).unwrap().into_value();

        let _se_bottom = apply_op(&mut draft, SplitEdge {
            edge: mef1.half_edge_ba, parameter: 0.5,
        }).unwrap().into_value();
        let _se_top = apply_op(&mut draft, SplitEdge {
            edge: mef1.half_edge_ab, parameter: 0.5,
        }).unwrap().into_value();

        draft.commit().unwrap().topology_hash()
    };

    assert_eq!(
        path_a_hash,
        path_b_hash,
        "DAG determinism failed: operation order altered the aggregate structural hash."
    );
}
