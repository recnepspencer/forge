use std::ptr;

use forge_spec::facade::{MakeVertexFaceMutation, SpecState};

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
