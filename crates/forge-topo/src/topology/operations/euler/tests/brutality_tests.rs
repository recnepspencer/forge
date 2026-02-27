//! Stress tests against domain-specific adversarial topologies.
//!
//! DOMAIN: Each test simulates a real failure mode from a production geometry
//! kernel workload. These are intentionally expensive; they are not unit tests.

use crate::euler::join_faces::JoinFaces;
use crate::euler::kill_edge_vertex::KillEdgeVertex;
use crate::euler::make_edge_face::MakeEdgeFace;
use crate::euler::make_vertex_face::MakeVertexFace;
use crate::euler::split_edge::SplitEdge;
use crate::operator::apply_op;
use crate::state::TopologyState;

use super::helpers::logged_op;

/// Bio-mesh: 15,000-edge high-valence pole.
///
/// Bio-medical implant meshes produce vertices with thousands of converging
/// edges ("poles"). This test verifies that traverse guards, Euler invariants,
/// and the diff engine all survive massive load.
#[test]
#[ignore = "Slow stress test, run with --release"]
fn brutal_bio_mesh_high_valence_pole_and_megaloop() {
    let state = TopologyState::empty();
    let mut draft = state.clone().into_mutation();

    let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
    let center_vertex = mvf.vertex;
    let mut current_edge = mvf.half_edge;

    for _ in 0..15_000 {
        let se = apply_op(
            &mut draft,
            SplitEdge {
                edge: current_edge,
                parameter: 0.5,
            },
        )
        .unwrap()
        .into_value();
        let face_id = draft.arena().get_half_edge(current_edge).unwrap().face();
        let mef = apply_op(
            &mut draft,
            MakeEdgeFace {
                vertex_a: center_vertex,
                vertex_b: se.new_vertex,
                face: face_id,
            },
        )
        .unwrap()
        .into_value();
        current_edge = mef.half_edge_ab;
    }

    let megamesh_state = draft.commit().unwrap();
    let arena = megamesh_state.arena();

    assert_eq!(arena.vertex_count(), 15_001);

    let diff = crate::diff::compute_diff(state.arena(), arena, 0, megamesh_state.epoch());
    assert!(!diff.is_empty());
    assert_eq!(
        diff.total_added(),
        arena.face_count() + arena.half_edge_count() + arena.vertex_count()
    );
}

/// Aerospace sliver churn: 500 rapid create/destroy cycles via generational handles.
///
/// Boolean operations on wing ribs generate microscopic sliver faces that are
/// immediately healed. This test hammers SplitEdge+MEF → JoinFaces+KEV cycles
/// to verify that generational slots correctly detect stale handles.
#[test]
#[ignore = "Slow stress test, run with --release"]
fn brutal_aerospace_sliver_churn_generational_integrity() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let mvf = logged_op("MVF", apply_op(&mut draft, MakeVertexFace)).unwrap();

    for i in 0..500 {
        let boundary_edge = draft.arena().get_vertex(mvf.vertex).unwrap().outgoing();
        let se = logged_op(
            &format!("SE[{i}]"),
            apply_op(
                &mut draft,
                SplitEdge {
                    edge: boundary_edge,
                    parameter: 0.1,
                },
            ),
        )
        .unwrap();

        let face_id = draft.arena().get_half_edge(boundary_edge).unwrap().face();
        let mef = logged_op(
            &format!("MEF[{i}]"),
            apply_op(
                &mut draft,
                MakeEdgeFace {
                    vertex_a: mvf.vertex,
                    vertex_b: se.new_vertex,
                    face: face_id,
                },
            ),
        )
        .unwrap();

        let doomed_face = mef.new_face;
        let doomed_vertex = se.new_vertex;

        logged_op(
            &format!("JF[{i}]"),
            apply_op(
                &mut draft,
                JoinFaces {
                    edge: mef.half_edge_ab,
                },
            ),
        )
        .unwrap();
        logged_op(
            &format!("KEV[{i}]"),
            apply_op(&mut draft, KillEdgeVertex { edge: se.he_am }),
        )
        .unwrap();

        assert!(
            draft.arena().get_face(doomed_face).is_err(),
            "stale face must be rejected"
        );
        assert!(
            draft.arena().get_vertex(doomed_vertex).is_err(),
            "stale vertex must be rejected"
        );
    }

    let final_state = draft
        .commit()
        .expect("churn caused silent topology corruption");
    assert_eq!(final_state.arena().vertex_count(), 1);
    assert_eq!(final_state.arena().face_count(), 1);
}

/// Merkle-DAG: operation-order independence for structural hashing.
///
/// If engineer A splits edge X then Y, and engineer B splits Y then X,
/// the resulting topology is structurally identical. The structural hash
/// must normalise both orderings to the same value.
#[test]
#[ignore = "Slow stress test, run with --release"]
fn brutal_dag_determinism_path_independence() {
    let path_a_hash = {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se1 = apply_op(
            &mut draft,
            SplitEdge {
                edge: mvf.half_edge,
                parameter: 0.5,
            },
        )
        .unwrap()
        .into_value();
        let mef1 = apply_op(
            &mut draft,
            MakeEdgeFace {
                vertex_a: mvf.vertex,
                vertex_b: se1.new_vertex,
                face: mvf.face,
            },
        )
        .unwrap()
        .into_value();

        let _se_top = apply_op(
            &mut draft,
            SplitEdge {
                edge: mef1.half_edge_ab,
                parameter: 0.5,
            },
        )
        .unwrap()
        .into_value();
        let _se_bottom = apply_op(
            &mut draft,
            SplitEdge {
                edge: mef1.half_edge_ba,
                parameter: 0.5,
            },
        )
        .unwrap()
        .into_value();
        draft.commit().unwrap().topology_hash()
    };

    let path_b_hash = {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se1 = apply_op(
            &mut draft,
            SplitEdge {
                edge: mvf.half_edge,
                parameter: 0.5,
            },
        )
        .unwrap()
        .into_value();
        let mef1 = apply_op(
            &mut draft,
            MakeEdgeFace {
                vertex_a: mvf.vertex,
                vertex_b: se1.new_vertex,
                face: mvf.face,
            },
        )
        .unwrap()
        .into_value();

        let _se_bottom = apply_op(
            &mut draft,
            SplitEdge {
                edge: mef1.half_edge_ba,
                parameter: 0.5,
            },
        )
        .unwrap()
        .into_value();
        let _se_top = apply_op(
            &mut draft,
            SplitEdge {
                edge: mef1.half_edge_ab,
                parameter: 0.5,
            },
        )
        .unwrap()
        .into_value();
        draft.commit().unwrap().topology_hash()
    };

    assert_eq!(
        path_a_hash, path_b_hash,
        "operation order must not alter the structural hash"
    );
}
