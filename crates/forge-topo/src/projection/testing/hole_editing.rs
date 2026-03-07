use forge_spec::facade::{
    KillFaceMakeRingHoleMutation, MakeFaceFromVerticesMutation, MakeFaceKillRingHoleMutation,
    MakeIsolatedVertexMutation, MakeLoopInFaceFromVerticesMutation, SpecNodeId, SpecState,
};

use crate::operations::boundary_editing::kill_face_make_ring_hole::KillFaceMakeRingHole;
use crate::operations::boundary_editing::make_face_from_vertices::MakeFaceFromVertices;
use crate::operations::boundary_editing::make_face_kill_ring_hole::MakeFaceKillRingHole;
use crate::operations::boundary_editing::make_loop_in_face_from_vertices::MakeLoopInFaceFromVertices;
use crate::operations::entity_lifecycle::make_isolated_vertex::MakeIsolatedVertex;
use crate::projection::facade::{
    ProjectionBuilder, ProjectedTopologyQueries, compute_projected_topology_hash,
};
use crate::transactions::facade::{TopologyState, compute_arena_topology_hash};

#[test]
fn projected_make_face_kill_ring_hole_matches_legacy_structural_signature() {
    let legacy = build_legacy_promoted_hole_state();
    let projected = ProjectionBuilder::build(&build_spec_promoted_hole_state())
        .expect("spec-state MFKRH projection should succeed");

    assert_eq!(
        compute_arena_topology_hash(legacy.arena()),
        compute_projected_topology_hash(&projected)
    );
    assert_eq!(projected.face_count(), legacy.arena().face_count() as usize);
    assert_eq!(projected.loop_count(), legacy.arena().loop_count() as usize);
}

#[test]
fn projected_kill_face_make_ring_hole_matches_legacy_structural_signature() {
    let legacy = build_legacy_demoted_hole_state();
    let projected = ProjectionBuilder::build(&build_spec_demoted_hole_state())
        .expect("spec-state KFMRH projection should succeed");

    assert_eq!(
        compute_arena_topology_hash(legacy.arena()),
        compute_projected_topology_hash(&projected)
    );
    assert_eq!(projected.face_count(), legacy.arena().face_count() as usize);
    assert_eq!(projected.loop_count(), legacy.arena().loop_count() as usize);
    assert_eq!(projected.face_loops(crate::projection::data::ProjectedFaceId::new(0)).len(), 2);
}

fn build_spec_promoted_hole_state() -> SpecState {
    let (mut draft, face, loop_id) = build_spec_face_with_hole_draft();
    let _ = face;
    draft
        .execute(MakeFaceKillRingHoleMutation { loop_id })
        .unwrap();
    draft.commit().unwrap()
}

fn build_spec_demoted_hole_state() -> SpecState {
    let (mut draft, face, loop_id) = build_spec_face_with_hole_draft();
    let promoted = draft
        .execute(MakeFaceKillRingHoleMutation { loop_id })
        .unwrap()
        .value;
    draft
        .execute(KillFaceMakeRingHoleMutation {
            face_to_kill: promoted.new_face,
            target_face: face,
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_spec_face_with_hole_draft() -> (forge_spec::facade::SpecDraft, SpecNodeId, SpecNodeId) {
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
    let loop_id = draft
        .execute(MakeLoopInFaceFromVerticesMutation {
            face,
            vertices: vec![h0, h1, h2],
        })
        .unwrap()
        .value
        .loop_id;
    (draft, face, loop_id)
}

fn build_legacy_promoted_hole_state() -> TopologyState {
    let mut draft = build_legacy_face_with_hole_draft();
    let face = draft.arena().iter_faces().next().unwrap().0;
    let inner_loop = draft.arena().get_face(face).unwrap().loops.inners()[0];
    draft
        .execute(MakeFaceKillRingHole { loop_id: inner_loop })
        .unwrap();
    draft.commit().unwrap()
}

fn build_legacy_demoted_hole_state() -> TopologyState {
    let mut draft = build_legacy_face_with_hole_draft();
    let face = draft.arena().iter_faces().next().unwrap().0;
    let inner_loop = draft.arena().get_face(face).unwrap().loops.inners()[0];
    let promoted = draft
        .execute(MakeFaceKillRingHole { loop_id: inner_loop })
        .unwrap()
        .into_value();
    draft
        .execute(KillFaceMakeRingHole {
            face_to_kill: promoted.new_face,
            target_face: face,
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_legacy_face_with_hole_draft() -> crate::transactions::MutableDraft {
    let mut draft = TopologyState::empty().into_mutation();
    let v0 = draft.execute(MakeIsolatedVertex).unwrap().into_value().vertex;
    let v1 = draft.execute(MakeIsolatedVertex).unwrap().into_value().vertex;
    let v2 = draft.execute(MakeIsolatedVertex).unwrap().into_value().vertex;
    let face = draft
        .execute(MakeFaceFromVertices {
            vertices: vec![v0, v1, v2],
        })
        .unwrap()
        .into_value()
        .face;
    let h0 = draft.execute(MakeIsolatedVertex).unwrap().into_value().vertex;
    let h1 = draft.execute(MakeIsolatedVertex).unwrap().into_value().vertex;
    let h2 = draft.execute(MakeIsolatedVertex).unwrap().into_value().vertex;
    draft
        .execute(MakeLoopInFaceFromVertices {
            face,
            vertices: vec![h0, h1, h2],
        })
        .unwrap();
    draft
}
