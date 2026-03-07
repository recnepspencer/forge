use forge_spec::facade::{
    KillEdgeMakeLoopMutation, MakeEdgeKillLoopMutation, MakeFaceFromVerticesMutation,
    MakeIsolatedVertexMutation, MakeLoopInFaceFromVerticesMutation, RelationKind, SpecState,
};

use crate::operations::boundary_editing::kill_edge_make_loop::KillEdgeMakeLoop;
use crate::operations::boundary_editing::make_edge_kill_loop::MakeEdgeKillLoop;
use crate::projection::facade::{
    ProjectionBuilder, ProjectedFaceId, ProjectedTopologyQueries, compute_projected_topology_hash,
};
use crate::testing::build_face_with_hole;
use crate::transactions::facade::{TopologyState, compute_arena_topology_hash};

#[test]
fn projected_make_edge_kill_loop_matches_legacy_structural_signature() {
    let legacy = build_legacy_mekl_state();
    let projected =
        ProjectionBuilder::build(&build_spec_mekl_state()).expect("spec-state MEKL projection should succeed");

    assert_eq!(
        compute_arena_topology_hash(legacy.arena()),
        compute_projected_topology_hash(&projected)
    );
    assert_eq!(projected.face_count(), legacy.arena().face_count() as usize);
    assert_eq!(projected.loop_count(), legacy.arena().loop_count() as usize);
    assert_eq!(projected.edge_count(), legacy.arena().edge_count() as usize);
    assert_eq!(projected.face_loops(ProjectedFaceId::new(0)).len(), 1);
}

#[test]
fn projected_kill_edge_make_loop_matches_legacy_structural_signature() {
    let legacy = build_legacy_keml_state();
    let projected =
        ProjectionBuilder::build(&build_spec_keml_state()).expect("spec-state KEML projection should succeed");

    assert_eq!(
        compute_arena_topology_hash(legacy.arena()),
        compute_projected_topology_hash(&projected)
    );
    assert_eq!(projected.face_count(), legacy.arena().face_count() as usize);
    assert_eq!(projected.loop_count(), legacy.arena().loop_count() as usize);
    assert_eq!(projected.edge_count(), legacy.arena().edge_count() as usize);
    assert_eq!(projected.face_loops(ProjectedFaceId::new(0)).len(), 2);
}

fn build_spec_mekl_state() -> SpecState {
    let mut draft = SpecState::empty().into_draft();
    let v0 = draft.execute(MakeIsolatedVertexMutation).unwrap().value.vertex;
    let v1 = draft.execute(MakeIsolatedVertexMutation).unwrap().value.vertex;
    let v2 = draft.execute(MakeIsolatedVertexMutation).unwrap().value.vertex;
    let face = draft
        .execute(MakeFaceFromVerticesMutation {
            vertices: vec![v0, v1, v2],
        })
        .unwrap()
        .value
        .face;
    let h0 = draft.execute(MakeIsolatedVertexMutation).unwrap().value.vertex;
    let h1 = draft.execute(MakeIsolatedVertexMutation).unwrap().value.vertex;
    let h2 = draft.execute(MakeIsolatedVertexMutation).unwrap().value.vertex;
    let hole = draft
        .execute(MakeLoopInFaceFromVerticesMutation {
            face,
            vertices: vec![h0, h1, h2],
        })
        .unwrap()
        .value;
    let outer_half_edge = draft
        .single_outgoing_target(face, RelationKind::FaceOuterLoop)
        .and_then(|loop_id| draft.single_outgoing_target(loop_id, RelationKind::LoopEntryHalfEdge))
        .unwrap();
    draft
        .execute(MakeEdgeKillLoopMutation {
            half_edge_a: outer_half_edge,
            half_edge_b: hole.half_edges[0],
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_spec_keml_state() -> SpecState {
    let mut draft = SpecState::empty().into_draft();
    let v0 = draft.execute(MakeIsolatedVertexMutation).unwrap().value.vertex;
    let v1 = draft.execute(MakeIsolatedVertexMutation).unwrap().value.vertex;
    let v2 = draft.execute(MakeIsolatedVertexMutation).unwrap().value.vertex;
    let face = draft
        .execute(MakeFaceFromVerticesMutation {
            vertices: vec![v0, v1, v2],
        })
        .unwrap()
        .value
        .face;
    let h0 = draft.execute(MakeIsolatedVertexMutation).unwrap().value.vertex;
    let h1 = draft.execute(MakeIsolatedVertexMutation).unwrap().value.vertex;
    let h2 = draft.execute(MakeIsolatedVertexMutation).unwrap().value.vertex;
    let hole = draft
        .execute(MakeLoopInFaceFromVerticesMutation {
            face,
            vertices: vec![h0, h1, h2],
        })
        .unwrap()
        .value;
    let outer_half_edge = draft
        .single_outgoing_target(face, RelationKind::FaceOuterLoop)
        .and_then(|loop_id| draft.single_outgoing_target(loop_id, RelationKind::LoopEntryHalfEdge))
        .unwrap();
    let bridge = draft
        .execute(MakeEdgeKillLoopMutation {
            half_edge_a: outer_half_edge,
            half_edge_b: hole.half_edges[0],
        })
        .unwrap()
        .value;
    draft
        .execute(KillEdgeMakeLoopMutation {
            half_edge: bridge.half_edge_ab,
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_legacy_mekl_state() -> TopologyState {
    let mut draft = TopologyState::empty().into_mutation();
    let (_face, outer_he, inner_he, _outer_loop, _verts) = build_face_with_hole(&mut draft);
    draft
        .execute(MakeEdgeKillLoop {
            he_a: outer_he,
            he_b: inner_he,
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_legacy_keml_state() -> TopologyState {
    let mut draft = TopologyState::empty().into_mutation();
    let (_face, outer_he, inner_he, _outer_loop, _verts) = build_face_with_hole(&mut draft);
    let bridge = draft
        .execute(MakeEdgeKillLoop {
            he_a: outer_he,
            he_b: inner_he,
        })
        .unwrap()
        .into_value();
    draft
        .execute(KillEdgeMakeLoop { edge: bridge.he_ab })
        .unwrap();
    draft.commit().unwrap()
}
