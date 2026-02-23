//! Shared unit test utilities for `forge-topo`.
//!
//! Provides placeholder generation and manual structural wiring helpers.
//! Since Euler operators don't natively create disconnected loops, these
//! manual constructions are necessary to test topological edge cases.

use crate::arena::{FaceData, HalfEdgeData, LoopData, VertexData};
use crate::handles::{EdgeId, FaceId, HalfEdgeId, LoopId, ShellId, VertexId};
use crate::state::MutableDraft;

/// Returns a VertexData with a placeholder outgoing halfedge.
pub fn dummy_vertex_data() -> VertexData {
    VertexData::new(HalfEdgeId::new(u32::MAX, 0))
}

/// Returns a FaceData with a placeholder outer loop and shell.
pub fn dummy_face_data() -> FaceData {
    FaceData::new(LoopId::new(u32::MAX, 0), ShellId::new(u32::MAX, 0))
}

/// Returns a HalfEdgeData pointing to the given face and origin,
/// with placeholders for all other links.
pub fn dummy_halfedge_data(face: FaceId, origin: VertexId) -> HalfEdgeData {
    HalfEdgeData::new(
        HalfEdgeId::new(u32::MAX, 0),
        HalfEdgeId::new(u32::MAX, 0),
        HalfEdgeId::new(u32::MAX, 0),
        face,
        origin,
        EdgeId::new(u32::MAX, 0),
    )
}

/// Build a face with an outer triangle (v0→v1→v2) and an inner triangle hole (v3→v4→v5).
///
/// Returns: `(face_id, outer_he_01, inner_he_34, outer_loop_id, [v0..v5])`.
pub fn build_face_with_hole(
    draft: &mut MutableDraft,
) -> (FaceId, HalfEdgeId, HalfEdgeId, LoopId, [VertexId; 6]) {
    let placeholder_he    = HalfEdgeId::new(u32::MAX, 0);
    let placeholder_loop  = LoopId::new(u32::MAX, 0);
    let placeholder_face  = FaceId::new(u32::MAX, 0);
    let placeholder_shell = ShellId::new(u32::MAX, 0);
    let placeholder_e     = EdgeId::new(u32::MAX, 0);

    let arena = draft.arena_mut();

    let face = arena.insert_face(FaceData::new(placeholder_loop, placeholder_shell), None);
    let outer_loop = arena.insert_loop(LoopData::new(placeholder_he, face), None);
    arena.get_face_mut(face).unwrap().set_outer_loop(outer_loop);

    // Outer loop vertices
    let v0 = arena.insert_vertex(VertexData::new(placeholder_he), None);
    let v1 = arena.insert_vertex(VertexData::new(placeholder_he), None);
    let v2 = arena.insert_vertex(VertexData::new(placeholder_he), None);

    // Outer halfedges (counter-clockwise)
    let (he01, _he10) = arena.insert_half_edge_pair(
        HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face, v0, placeholder_e),
        HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, placeholder_face, v1, placeholder_e), None);
    let (he12, _he21) = arena.insert_half_edge_pair(
        HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face, v1, placeholder_e),
        HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, placeholder_face, v2, placeholder_e), None);
    let (he20, _he02) = arena.insert_half_edge_pair(
        HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face, v2, placeholder_e),
        HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, placeholder_face, v0, placeholder_e), None);

    let arena = draft.arena_mut();
    arena.get_half_edge_mut(he01).unwrap().set_next(he12);
    arena.get_half_edge_mut(he01).unwrap().set_prev(he20);
    arena.get_half_edge_mut(he12).unwrap().set_next(he20);
    arena.get_half_edge_mut(he12).unwrap().set_prev(he01);
    arena.get_half_edge_mut(he20).unwrap().set_next(he01);
    arena.get_half_edge_mut(he20).unwrap().set_prev(he12);

    arena.get_loop_mut(outer_loop).unwrap().set_half_edge(he01);
    arena.get_vertex_mut(v0).unwrap().set_outgoing(he01);
    arena.get_vertex_mut(v1).unwrap().set_outgoing(he12);
    arena.get_vertex_mut(v2).unwrap().set_outgoing(he20);

    // Inner loop vertices
    let v3 = arena.insert_vertex(VertexData::new(placeholder_he), None);
    let v4 = arena.insert_vertex(VertexData::new(placeholder_he), None);
    let v5 = arena.insert_vertex(VertexData::new(placeholder_he), None);

    // Inner halfedges (clockwise around the hole)
    let (he34, _he43) = arena.insert_half_edge_pair(
        HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face, v3, placeholder_e),
        HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, placeholder_face, v4, placeholder_e), None);
    let (he45, _he54) = arena.insert_half_edge_pair(
        HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face, v4, placeholder_e),
        HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, placeholder_face, v5, placeholder_e), None);
    let (he53, _he35) = arena.insert_half_edge_pair(
        HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face, v5, placeholder_e),
        HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, placeholder_face, v3, placeholder_e), None);

    let arena = draft.arena_mut();
    arena.get_half_edge_mut(he34).unwrap().set_next(he45);
    arena.get_half_edge_mut(he34).unwrap().set_prev(he53);
    arena.get_half_edge_mut(he45).unwrap().set_next(he53);
    arena.get_half_edge_mut(he45).unwrap().set_prev(he34);
    arena.get_half_edge_mut(he53).unwrap().set_next(he34);
    arena.get_half_edge_mut(he53).unwrap().set_prev(he45);

    arena.get_vertex_mut(v3).unwrap().set_outgoing(he34);
    arena.get_vertex_mut(v4).unwrap().set_outgoing(he45);
    arena.get_vertex_mut(v5).unwrap().set_outgoing(he53);

    let inner_loop = arena.insert_loop(LoopData::new(he34, face), None);
    arena.get_face_mut(face).unwrap().add_inner_loop(inner_loop);

    (face, he01, he34, outer_loop, [v0, v1, v2, v3, v4, v5])
}
