//! Tests for the geometry store.

use forge_geom::plane::Plane;
use forge_topo::handles::{FaceId, VertexId};
use super::schema::GeometryStore;

#[test]
fn store_and_retrieve_vertex_position() {
    let mut store = GeometryStore::new();
    let vertex = VertexId::new(0, 0);
    let position = [1.0, 2.0, 3.0];

    store.set_vertex_position(vertex, position);

    let retrieved = store.get_vertex_position(vertex);
    assert_eq!(retrieved, Some(&position));
}

#[test]
fn store_and_retrieve_face_plane() {
    let mut store = GeometryStore::new();
    let face = FaceId::new(0, 0);
    let plane = Plane::try_new([0.0, 0.0, 1.0], 0.0).unwrap();

    store.set_face_plane(face, plane);

    let retrieved = store.get_face_plane(face);
    assert!(retrieved.is_some());
    let p = retrieved.unwrap();
    assert_eq!(p.normal()[2], 1.0);
}

#[test]
fn missing_vertex_returns_none() {
    let store = GeometryStore::new();
    let vertex = VertexId::new(99, 0);
    assert_eq!(store.get_vertex_position(vertex), None);
}

#[test]
fn missing_face_returns_none() {
    let store = GeometryStore::new();
    let face = FaceId::new(99, 0);
    assert!(store.get_face_plane(face).is_none());
}

#[test]
fn stale_generation_returns_none() {
    let mut store = GeometryStore::new();
    let vertex_gen0 = VertexId::new(0, 0);
    let vertex_gen1 = VertexId::new(0, 1);

    store.set_vertex_position(vertex_gen0, [1.0, 2.0, 3.0]);

    assert_eq!(store.get_vertex_position(vertex_gen0), Some(&[1.0, 2.0, 3.0]));
    assert_eq!(store.get_vertex_position(vertex_gen1), None);
}

#[test]
fn counts_reflect_insertions() {
    let mut store = GeometryStore::new();
    assert_eq!(store.face_plane_count(), 0);
    assert_eq!(store.vertex_position_count(), 0);

    store.set_face_plane(
        FaceId::new(0, 0),
        Plane::try_new([1.0, 0.0, 0.0], 0.0).unwrap(),
    );
    store.set_vertex_position(VertexId::new(0, 0), [0.0, 0.0, 0.0]);
    store.set_vertex_position(VertexId::new(1, 0), [1.0, 0.0, 0.0]);

    assert_eq!(store.face_plane_count(), 1);
    assert_eq!(store.vertex_position_count(), 2);
}

#[test]
fn geometry_source_trait_returns_plane() {
    use forge_core::GeometrySource;

    let mut store = GeometryStore::new();
    let plane = Plane::try_new([0.0, 1.0, 0.0], -5.0).unwrap();
    store.set_face_plane(FaceId::new(0, 0), plane);

    let result = store.get_plane(0);
    assert!(result.is_ok());
    let coeffs = result.unwrap();
    assert!((coeffs[1] - 1.0).abs() < 1e-10);
}

#[test]
fn geometry_source_missing_plane_returns_error() {
    use forge_core::GeometrySource;

    let store = GeometryStore::new();
    let result = store.get_plane(42);
    assert!(result.is_err());
}
