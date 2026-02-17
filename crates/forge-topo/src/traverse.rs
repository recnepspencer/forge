//! Topology traversal utilities.
//!
//! DOMAIN: Read-only traversal of face loops and vertex rings.
//!
//! INVARIANTS:
//! - Traversal uses explicit twin, next, prev pointers on HalfEdgeData
//! - Cycle detection guards against infinite loops (max iterations)
//!
//! DEPENDENCIES: `arena` (entity data), `handles` (typed IDs)

use forge_core::KernelError;
use crate::arena::TopologyArena;
use crate::handles::{FaceId, HalfEdgeId, VertexId};

/// Collect all halfedge IDs around a face loop.
///
/// Follows the `next` pointer chain starting from the face's loop entry
/// halfedge, returning the IDs in order.
///
/// Includes a cycle guard that aborts after `MAX_ITER` steps to prevent
/// infinite loops from corrupted topology.
pub fn face_edges(arena: &TopologyArena, face: FaceId) -> Result<Vec<HalfEdgeId>, KernelError> {
    const MAX_ITER: usize = 100_000;
    let face_data = arena.get_face(face)?;
    let loop_data = arena.get_loop(face_data.outer_loop)?;
    let start = loop_data.half_edge;
    let mut edges = Vec::new();
    let mut current = start;

    loop {
        edges.push(current);
        let he_data = arena.get_half_edge(current)?;
        current = he_data.next;
        if current == start {
            break;
        }
        if edges.len() >= MAX_ITER {
            return Err(KernelError::InternalError {
                message: format!("Face loop exceeded {} iterations — likely corrupted", MAX_ITER),
                context: None,
            });
        }
    }

    Ok(edges)
}

/// Count the number of edges in a face loop.
pub fn face_edge_count(arena: &TopologyArena, face: FaceId) -> Result<usize, KernelError> {
    face_edges(arena, face).map(|e| e.len())
}

/// Collect all halfedge IDs in the vertex ring (outgoing star).
///
/// Starting from the vertex's outgoing halfedge, walks around the
/// vertex using: `twin → next` to get the next outgoing hafedge
/// from the same vertex.
pub fn vertex_ring(arena: &TopologyArena, vertex: VertexId) -> Result<Vec<HalfEdgeId>, KernelError> {
    const MAX_ITER: usize = 100_000;
    let vtx_data = arena.get_vertex(vertex)?;
    let start = vtx_data.outgoing;
    let mut ring = Vec::new();
    let mut current = start;

    loop {
        ring.push(current);
        let he_data = arena.get_half_edge(current)?;
        let twin_he = he_data.twin;
        let twin_data = arena.get_half_edge(twin_he)?;
        current = twin_data.next;
        if current == start {
            break;
        }
        if ring.len() >= MAX_ITER {
            return Err(KernelError::InternalError {
                message: format!("Vertex ring exceeded {} iterations — likely corrupted", MAX_ITER),
                context: None,
            });
        }
    }

    Ok(ring)
}

/// Get the faces adjacent to an edge (the faces of its two halfedges).
pub fn edge_faces(arena: &TopologyArena, he: HalfEdgeId) -> Result<(FaceId, FaceId), KernelError> {
    let he_data = arena.get_half_edge(he)?;
    let twin_data = arena.get_half_edge(he_data.twin)?;
    Ok((he_data.face, twin_data.face))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::TopologyState;
    use crate::operator::apply_op;
    use crate::euler::make_vertex_face::MakeVertexFace;
    use crate::euler::split_edge::SplitEdge;

    #[test]
    fn face_edges_on_seed() {
        let state = TopologyState::empty();
        let mut draft = state.begin_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let state = draft.commit().unwrap();

        let edges = face_edges(state.arena(), mvf.face).unwrap();
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0], mvf.half_edge);
    }

    #[test]
    fn face_edges_after_split() {
        let state = TopologyState::empty();
        let mut draft = state.begin_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let _se = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.5 }).unwrap().into_value();
        let state = draft.commit().unwrap();

        let edges = face_edges(state.arena(), mvf.face).unwrap();
        assert_eq!(edges.len(), 2);
    }
}
