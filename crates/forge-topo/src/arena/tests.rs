//! Unit tests for the arena module.

use super::*;
use crate::handles::{FaceId, HalfEdgeId, LoopId, VertexId};
use crate::testing::{dummy_face_data, dummy_halfedge_data, dummy_vertex_data};

#[test]
fn insert_and_get_vertex() {
    let mut arena = TopologyArena::new();
    let id = arena.insert_vertex(dummy_vertex_data());
    let vertex = arena.get_vertex(id);
    assert!(vertex.is_ok());
}

#[test]
fn insert_and_get_face() {
    let mut arena = TopologyArena::new();
    let id = arena.insert_face(dummy_face_data());
    let face = arena.get_face(id);
    assert!(face.is_ok());
}

#[test]
fn stale_handle_returns_error() {
    let mut arena = TopologyArena::new();
    let id = arena.insert_vertex(dummy_vertex_data());
    arena.remove_vertex(id).unwrap();
    let result = arena.get_vertex(id);
    assert!(result.is_err());
}

#[test]
fn entity_counts() {
    let mut arena = TopologyArena::new();
    assert_eq!(arena.vertex_count(), 0);
    assert_eq!(arena.face_count(), 0);

    arena.insert_vertex(dummy_vertex_data());
    arena.insert_vertex(dummy_vertex_data());
    arena.insert_face(dummy_face_data());

    assert_eq!(arena.vertex_count(), 2);
    assert_eq!(arena.face_count(), 1);
}

#[test]
fn remove_decrements_count() {
    let mut arena = TopologyArena::new();
    let id = arena.insert_vertex(dummy_vertex_data());
    assert_eq!(arena.vertex_count(), 1);
    arena.remove_vertex(id).unwrap();
    assert_eq!(arena.vertex_count(), 0);
}

#[test]
fn out_of_bounds_handle_returns_error() {
    let arena = TopologyArena::new();
    let fake_id = VertexId::new(999, 0);
    let result = arena.get_vertex(fake_id);
    assert!(result.is_err());
}

#[test]
fn clone_is_independent() {
    let mut arena = TopologyArena::new();
    let id = arena.insert_vertex(dummy_vertex_data());

    let arena_clone = arena.clone();
    arena.remove_vertex(id).unwrap();

    assert_eq!(arena.vertex_count(), 0);
    assert_eq!(arena_clone.vertex_count(), 1);
}

#[test]
fn singular_halfedge_insertion() {
    let mut arena = TopologyArena::new();
    let face = arena.insert_face(dummy_face_data());
    let vertex = arena.insert_vertex(dummy_vertex_data());

    let he_id = arena.insert_half_edge(dummy_halfedge_data(face, vertex));
    assert_eq!(he_id.index(), 0);
    assert_eq!(arena.half_edge_count(), 1);
}

#[test]
fn paired_halfedge_insertion_sets_twins() {
    let mut arena = TopologyArena::new();
    let face = arena.insert_face(dummy_face_data());
    let vertex = arena.insert_vertex(dummy_vertex_data());

    let (he0, he1) = arena.insert_radial_pair(
        dummy_halfedge_data(face, vertex),
        dummy_halfedge_data(face, vertex),
    );
    assert_eq!(arena.half_edge_count(), 2);
    assert_eq!(arena.get_half_edge(he0).unwrap().radial_next(), he1);
    assert_eq!(arena.get_half_edge(he1).unwrap().radial_next(), he0);
}

#[test]
fn loop_insert_and_get() {
    let mut arena = TopologyArena::new();
    let face = arena.insert_face(dummy_face_data());
    let loop_id = arena.insert_loop(LoopData::new(HalfEdgeId::new(0, 0), face));
    assert_eq!(arena.loop_count(), 1);
    assert!(arena.get_loop(loop_id).is_ok());
}

#[test]
fn removed_vertex_slot_is_reused() {
    let mut arena = TopologyArena::new();

    let v0 = arena.insert_vertex(dummy_vertex_data());
    arena.remove_vertex(v0).unwrap();
    let slot_count_after_remove = arena.vertex_slot_count();

    let v1 = arena.insert_vertex(dummy_vertex_data());

    assert_eq!(v1.index(), v0.index());
    assert_ne!(v1.generation(), v0.generation());
    assert_eq!(arena.vertex_slot_count(), slot_count_after_remove);
}
