use forge_spec::facade::{
    JoinFacesMutation, MakeEdgeFaceMutation, MakeVertexFaceMutation, SpecState, SplitEdgeMutation,
};

use crate::b_rep::ShellKind;
use crate::operations::boundary_editing::join_faces::JoinFaces;
use crate::operations::entity_lifecycle::make_edge_face::MakeEdgeFace;
use crate::operations::entity_lifecycle::make_vertex_face::MakeVertexFace;
use crate::operations::entity_lifecycle::split_edge::SplitEdge;
use crate::projection::facade::{compute_projected_topology_hash, ProjectionBuilder};
use crate::transactions::facade::{compute_arena_topology_hash, TopologyState};

#[test]
fn projected_join_faces_matches_legacy_structural_signature() {
    let legacy = build_legacy_join_faces_state();
    let projected = ProjectionBuilder::build(&build_spec_join_faces_state())
        .expect("spec-state JoinFaces projection should succeed");

    assert_eq!(
        compute_arena_topology_hash(legacy.arena()),
        compute_projected_topology_hash(&projected)
    );
    assert_eq!(projected.face_count(), legacy.arena().face_count() as usize);
    assert_eq!(projected.loop_count(), legacy.arena().loop_count() as usize);
    assert_eq!(projected.edge_count(), legacy.arena().edge_count() as usize);
}

fn build_spec_join_faces_state() -> SpecState {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap().value;
    let split = draft
        .execute(SplitEdgeMutation {
            half_edge: seed.half_edge,
        })
        .unwrap()
        .value;
    let bridge = draft
        .execute(MakeEdgeFaceMutation {
            vertex_a: seed.vertex,
            vertex_b: split.new_vertex,
            face: seed.face,
        })
        .unwrap()
        .value;
    draft
        .execute(JoinFacesMutation {
            half_edge: bridge.half_edge_ab,
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_legacy_join_faces_state() -> TopologyState {
    let mut draft = TopologyState::empty().into_mutation();
    let mvf = draft
        .execute(MakeVertexFace {
            shell_kind: ShellKind::Sheet,
        })
        .unwrap()
        .into_value();
    let split = draft
        .execute(SplitEdge {
            edge: mvf.half_edge,
        })
        .unwrap()
        .into_value();
    let bridge = draft
        .execute(MakeEdgeFace {
            vertex_a: mvf.vertex,
            vertex_b: split.new_vertex,
            face: mvf.face,
        })
        .unwrap()
        .into_value();
    draft
        .execute(JoinFaces {
            edge: bridge.half_edge_ab,
        })
        .unwrap();
    draft.commit().unwrap()
}
