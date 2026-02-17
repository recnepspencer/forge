//! Traversal iterators for halfedge mesh navigation.
//!
//! DOMAIN: Read-only mesh traversal utilities.
//!
//! INVARIANTS:
//! - All traversals are bounded by loop limits to prevent infinite loops
//! - Pure functions operating on `&TopologyArena` — no mutation
//!
//! DEPENDENCIES: `arena` (entity data), `handles` (typed IDs)

use forge_core::KernelError;
use crate::arena::TopologyArena;
use crate::handles::{FaceId, HalfEdgeId, VertexId};

/// Collect all halfedge IDs around a face loop.
///
/// Starts from the face's outer loop entry halfedge and follows `next`
/// until returning to the start.
pub fn face_edges(arena: &TopologyArena, face_id: FaceId) -> Result<Vec<HalfEdgeId>, KernelError> {
    let face = arena.get_face(face_id)?;
    let loop_data = arena.get_loop(face.outer_loop)?;
    let start = loop_data.half_edge;

    let mut edges = Vec::new();
    let mut current = start;
    const MAX_TRAVERSAL_ITERATIONS: usize = 10000;

    for _ in 0..MAX_TRAVERSAL_ITERATIONS {
        edges.push(current);
        let next = arena.get_half_edge(current)?.next;
        current = next;
        if current == start {
            return Ok(edges);
        }
    }

    Err(KernelError::InternalError {
        message: "Loop limit exceeded in face_edges".to_string(),
        context: None,
    })
}

/// Collect all outgoing halfedge IDs around a vertex (the "star").
///
/// Walks: outgoing → twin → next → twin → next → ... until returning to start.
/// This visits all halfedges originating from the vertex.
pub fn vertex_ring(
    arena: &TopologyArena,
    vertex_id: VertexId,
) -> Result<Vec<HalfEdgeId>, KernelError> {
    let start = arena.get_vertex(vertex_id)?.outgoing;
    let mut ring = Vec::new();
    let mut current = start;
    const MAX_TRAVERSAL_ITERATIONS: usize = 10000;

    for _ in 0..MAX_TRAVERSAL_ITERATIONS {
        ring.push(current);
        let twin = arena.get_half_edge(current)?.twin;
        let next = arena.get_half_edge(twin)?.next;
        current = next;
        if current == start {
            return Ok(ring);
        }
    }

    Err(KernelError::InternalError {
        message: "Loop limit exceeded in vertex_ring".to_string(),
        context: None,
    })
}

/// Get the two faces sharing an edge (the face of the halfedge and its twin).
pub fn edge_faces(
    arena: &TopologyArena,
    half_edge_id: HalfEdgeId,
) -> Result<(FaceId, FaceId), KernelError> {
    let he = arena.get_half_edge(half_edge_id)?;
    let twin = arena.get_half_edge(he.twin)?;
    Ok((he.face, twin.face))
}

/// Count the number of edges around a face (the face loop length).
pub fn face_edge_count(
    arena: &TopologyArena,
    face_id: FaceId,
) -> Result<usize, KernelError> {
    let edges = face_edges(arena, face_id)?;
    Ok(edges.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::TopologyState;
    use crate::operator::apply_op;
    use crate::euler::make_vertex_face::MakeVertexFace;

    #[test]
    fn face_edges_on_mvf_returns_single_edge() {
        let state = TopologyState::empty();
        let mut draft = state.begin_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace { feature_id: 0 }).unwrap().into_value();

        let edges = face_edges(draft.arena(), mvf.face).unwrap();
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0], mvf.half_edge);
    }

    #[test]
    fn edge_faces_returns_same_face_for_degenerate() {
        let state = TopologyState::empty();
        let mut draft = state.begin_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace { feature_id: 0 }).unwrap().into_value();

        let (f1, f2) = edge_faces(draft.arena(), mvf.half_edge).unwrap();
        assert_eq!(f1, f2);
        assert_eq!(f1, mvf.face);
    }
}
