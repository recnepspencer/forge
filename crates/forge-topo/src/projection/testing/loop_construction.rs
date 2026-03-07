use forge_spec::facade::{
    MakeFaceFromVerticesMutation, MakeIsolatedVertexMutation, MakeLoopInFaceFromVerticesMutation,
    SpecState,
};

use crate::operations::boundary_editing::make_face_from_vertices::MakeFaceFromVertices;
use crate::operations::boundary_editing::make_loop_in_face_from_vertices::MakeLoopInFaceFromVertices;
use crate::operations::entity_lifecycle::make_isolated_vertex::MakeIsolatedVertex;
use crate::projection::facade::{ProjectionBuilder, ProjectedTopologyQueries, compute_projected_topology_hash};
use crate::transactions::facade::{TopologyState, compute_arena_topology_hash};

#[test]
fn projected_make_loop_in_face_from_vertices_matches_legacy_structural_signature() {
    let legacy = build_legacy_make_loop_in_face_from_vertices_state();
    let projected = ProjectionBuilder::build(&build_spec_make_loop_in_face_from_vertices_state())
        .expect("spec-state MLIFV projection should succeed");

    assert_eq!(
        compute_arena_topology_hash(legacy.arena()),
        compute_projected_topology_hash(&projected)
    );
    assert_eq!(projected.face_count(), legacy.arena().face_count() as usize);
    assert_eq!(projected.loop_count(), legacy.arena().loop_count() as usize);
    assert_eq!(projected.half_edge_count(), legacy.arena().half_edge_count() as usize);
    assert_eq!(projected.edge_count(), legacy.arena().edge_count() as usize);
    assert_eq!(projected.vertex_count(), legacy.arena().vertex_count() as usize);

    let face = crate::projection::data::ProjectedFaceId::new(0);
    assert_eq!(projected.face_loops(face).len(), 2);
    assert_eq!(projected.face(face).inner_loops.len(), 1);
}

fn build_spec_make_loop_in_face_from_vertices_state() -> SpecState {
    let mut draft = SpecState::empty().into_draft();
    let v0 = draft.execute(MakeIsolatedVertexMutation).unwrap().value.vertex;
    let v1 = draft.execute(MakeIsolatedVertexMutation).unwrap().value.vertex;
    let v2 = draft.execute(MakeIsolatedVertexMutation).unwrap().value.vertex;
    let face = draft
        .execute(MakeFaceFromVerticesMutation {
            vertices: vec![v0, v1, v2],
        })
        .unwrap()
        .value;
    let h0 = draft.execute(MakeIsolatedVertexMutation).unwrap().value.vertex;
    let h1 = draft.execute(MakeIsolatedVertexMutation).unwrap().value.vertex;
    let h2 = draft.execute(MakeIsolatedVertexMutation).unwrap().value.vertex;
    draft
        .execute(MakeLoopInFaceFromVerticesMutation {
            face: face.face,
            vertices: vec![h0, h1, h2],
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_legacy_make_loop_in_face_from_vertices_state() -> TopologyState {
    let mut draft = TopologyState::empty().into_mutation();
    let v0 = draft.execute(MakeIsolatedVertex).unwrap().into_value().vertex;
    let v1 = draft.execute(MakeIsolatedVertex).unwrap().into_value().vertex;
    let v2 = draft.execute(MakeIsolatedVertex).unwrap().into_value().vertex;
    let face = draft
        .execute(MakeFaceFromVertices {
            vertices: vec![v0, v1, v2],
        })
        .unwrap()
        .into_value();
    let h0 = draft.execute(MakeIsolatedVertex).unwrap().into_value().vertex;
    let h1 = draft.execute(MakeIsolatedVertex).unwrap().into_value().vertex;
    let h2 = draft.execute(MakeIsolatedVertex).unwrap().into_value().vertex;
    draft
        .execute(MakeLoopInFaceFromVertices {
            face: face.face,
            vertices: vec![h0, h1, h2],
        })
        .unwrap();
    draft.commit().unwrap()
}
