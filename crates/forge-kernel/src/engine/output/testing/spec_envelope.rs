use std::ptr;

use forge_spec::facade::{
    MakeFaceFromVerticesMutation, MakeIsolatedVertexMutation, MakeLoopInFaceFromVerticesMutation,
    MakeVertexFaceMutation, SpecState,
};

use crate::engine::output::spec_envelope::SpecEnvelope;
use crate::geometry::facade::GeometryStore;

#[test]
fn lazy_projection_materializes_from_spec_state() {
    let mut draft = SpecState::empty().into_draft();
    draft.execute(MakeVertexFaceMutation).unwrap();
    let spec = draft.commit().unwrap();

    let envelope = SpecEnvelope::new(spec, GeometryStore::default());
    let projection = envelope.projection().unwrap();

    assert_eq!(projection.body_count(), 1);
    assert_eq!(projection.face_count(), 1);
    assert_eq!(projection.half_edge_count(), 1);
    assert_eq!(projection.vertex_count(), 1);
    assert_eq!(envelope.body_count().unwrap(), 1);
    assert_eq!(envelope.face_count().unwrap(), 1);
    assert_eq!(envelope.vertex_count().unwrap(), 1);
    assert_eq!(envelope.edge_count().unwrap(), 1);
    assert_eq!(envelope.body().unwrap().raw(), 0);
    assert_eq!(envelope.shell().unwrap().raw(), 0);
}

#[test]
fn lazy_projection_is_cached() {
    let mut draft = SpecState::empty().into_draft();
    draft.execute(MakeVertexFaceMutation).unwrap();
    let spec = draft.commit().unwrap();

    let envelope = SpecEnvelope::new(spec, GeometryStore::default());
    let first = envelope.projection().unwrap();
    let second = envelope.projection().unwrap();

    assert!(ptr::eq(first, second));
}

#[test]
fn projected_handle_lists_are_cached() {
    let mut draft = SpecState::empty().into_draft();
    draft.execute(MakeVertexFaceMutation).unwrap();
    let spec = draft.commit().unwrap();

    let envelope = SpecEnvelope::new(spec, GeometryStore::default());
    let first = envelope.faces().unwrap();
    let second = envelope.faces().unwrap();

    assert!(ptr::eq(first, second));
    assert_eq!(first.len(), 1);
    assert_eq!(first[0].raw(), 0);
}

#[test]
fn projection_query_helpers_surface_face_loop_relationships() {
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
    draft
        .execute(MakeLoopInFaceFromVerticesMutation {
            face,
            vertices: vec![h0, h1, h2],
        })
        .unwrap();
    let spec = draft.commit().unwrap();

    let envelope = SpecEnvelope::new(spec, GeometryStore::default());
    let face_id = envelope.faces().unwrap()[0];
    let edge_id = envelope.edges().unwrap()[0];
    let vertex_id = envelope.vertices().unwrap()[0];

    assert_eq!(envelope.face_loops(face_id).unwrap().len(), 2);
    assert_eq!(envelope.shell_faces(envelope.shell().unwrap()).unwrap().len(), 1);
    assert_eq!(
        envelope
            .loop_half_edges(envelope.face_loops(face_id).unwrap()[0])
            .unwrap()
            .len(),
        3
    );
    assert_eq!(envelope.face_half_edges(face_id).unwrap().len(), 6);
    assert_eq!(envelope.face_edges(face_id).unwrap().len(), 6);
    assert_eq!(envelope.edge_half_edges(edge_id).unwrap().len(), 1);
    assert_eq!(envelope.edge_faces(edge_id).unwrap().len(), 1);
    assert_eq!(envelope.radial_valence(edge_id).unwrap(), 1);
    assert!(envelope.is_boundary_edge(edge_id).unwrap());
    assert_eq!(envelope.vertex_outgoing_half_edges(vertex_id).unwrap().len(), 1);
    assert_eq!(envelope.vertex_faces(vertex_id).unwrap().len(), 1);
    assert_eq!(
        envelope
            .radial_half_edges(envelope.edge_half_edges(edge_id).unwrap()[0])
            .unwrap()
            .len(),
        1
    );
}
