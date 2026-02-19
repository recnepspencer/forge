//! Topology traversal utilities.
//!
//! DOMAIN: Read-only traversal of face loops and vertex rings.
//!
//! INVARIANTS:
//! - Traversal uses explicit twin, next, prev pointers on HalfEdgeData
//! - Cycle detection guards against infinite loops (max iterations)
//! - Iterators yield `Result<HalfEdgeId, KernelError>` to handle topology corruption lazily
//!
//! DEPENDENCIES: `arena` (entity data), `handles` (typed IDs)

use forge_core::KernelError;
use crate::arena::TopologyArena;
use crate::handles::{FaceId, HalfEdgeId, VertexId};

// =========================================================================
// Face Edge Iterator (Zero Allocation)
// =========================================================================

/// Iterator over halfedges in a face loop.
///
/// Yields `HalfEdgeId`s following the `next` pointer chain.
/// Returns `Err` if the loop exceeds `MAX_ITER` (corrupted topology) or if
/// a handle is stale.
pub struct FaceEdgeIterator<'a> {
    arena: &'a TopologyArena,
    start: HalfEdgeId,
    current: Option<HalfEdgeId>,
    steps: usize,
    finished: bool,
}

impl<'a> FaceEdgeIterator<'a> {
    const MAX_ITER: usize = 100_000;

    /// Create a new iterator around a face.
    pub fn new(arena: &'a TopologyArena, face: FaceId) -> Result<Self, KernelError> {
        let face_data = arena.get_face(face)?;
        let loop_data = arena.get_loop(face_data.outer_loop())?;
        let start = loop_data.half_edge();
        
        Ok(Self {
            arena,
            start,
            current: Some(start),
            steps: 0,
            finished: false,
        })
    }
}

impl<'a> Iterator for FaceEdgeIterator<'a> {
    type Item = Result<HalfEdgeId, KernelError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let curr_id = match self.current {
            Some(id) => id,
            None => {
                self.finished = true;
                return None;
            }
        };

        // Cycle guard
        self.steps += 1;
        if self.steps >= Self::MAX_ITER {
            self.finished = true;
            return Some(Err(KernelError::InternalError {
                message: format!("Face loop exceeded {} iterations — likely corrupted", Self::MAX_ITER),
                context: None,
            }));
        }

        // Fetch data for next step
        match self.arena.get_half_edge(curr_id) {
            Ok(he_data) => {
                let next_id = he_data.next();
                if next_id == self.start {
                    self.current = None; // Finishes after this yield
                } else {
                    self.current = Some(next_id);
                }
                Some(Ok(curr_id))
            }
            Err(e) => {
                self.finished = true;
                Some(Err(e))
            }
        }
    }
}

// =========================================================================
// Vertex Ring Iterator (Zero Allocation)
// =========================================================================

/// Iterator over outgoing halfedges around a vertex.
///
/// Follows `twin -> next` to circle the vertex.
pub struct VertexRingIterator<'a> {
    arena: &'a TopologyArena,
    start: HalfEdgeId,
    current: Option<HalfEdgeId>,
    steps: usize,
    finished: bool,
}

impl<'a> VertexRingIterator<'a> {
    const MAX_ITER: usize = 100_000;

    /// Create a new iterator around a vertex.
    pub fn new(arena: &'a TopologyArena, vertex: VertexId) -> Result<Self, KernelError> {
        let vtx_data = arena.get_vertex(vertex)?;
        let start = vtx_data.outgoing();
        
        Ok(Self {
            arena,
            start,
            current: Some(start),
            steps: 0,
            finished: false,
        })
    }
}

impl<'a> Iterator for VertexRingIterator<'a> {
    type Item = Result<HalfEdgeId, KernelError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let curr_id = match self.current {
            Some(id) => id,
            None => {
                self.finished = true;
                return None;
            }
        };

        // Cycle guard
        self.steps += 1;
        if self.steps >= Self::MAX_ITER {
            self.finished = true;
            return Some(Err(KernelError::InternalError {
                message: format!("Vertex ring exceeded {} iterations — likely corrupted", Self::MAX_ITER),
                context: None,
            }));
        }

        // Logic: next_around_vertex = twin(current).next
        // 1. Get current halfedge
        // 2. Get twin
        // 3. Get twin data -> next
        
        let next_result = (|| -> Result<HalfEdgeId, KernelError> {
            let he_data = self.arena.get_half_edge(curr_id)?;
            let twin_id = he_data.twin();
            let twin_data = self.arena.get_half_edge(twin_id)?;
            Ok(twin_data.next())
        })();

        match next_result {
            Ok(next_id) => {
                if next_id == self.start {
                    self.current = None; // Finishes after this yield
                } else {
                    self.current = Some(next_id);
                }
                Some(Ok(curr_id))
            }
            Err(e) => {
                self.finished = true;
                Some(Err(e))
            }
        }
    }
}

// =========================================================================
// Helpers
// =========================================================================

/// Count the number of edges in a face loop (allocates nothing).
pub fn face_edge_count(arena: &TopologyArena, face: FaceId) -> Result<usize, KernelError> {
    let mut count = 0;
    for res in FaceEdgeIterator::new(arena, face)? {
        res?;
        count += 1;
    }
    Ok(count)
}

/// Get the faces adjacent to an edge (the faces of its two halfedges).
pub fn edge_faces(arena: &TopologyArena, he: HalfEdgeId) -> Result<(FaceId, FaceId), KernelError> {
    let he_data = arena.get_half_edge(he)?;
    let twin_data = arena.get_half_edge(he_data.twin())?;
    Ok((he_data.face(), twin_data.face()))
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
        let mut draft = state.into_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let state = draft.commit().unwrap();

        let edges: Vec<HalfEdgeId> = FaceEdgeIterator::new(state.arena(), mvf.face)
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0], mvf.half_edge);
    }

    #[test]
    fn face_edges_after_split() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let _se = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.5 }).unwrap().into_value();
        let state = draft.commit().unwrap();

        let edges: Vec<HalfEdgeId> = FaceEdgeIterator::new(state.arena(), mvf.face)
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
            
        assert_eq!(edges.len(), 2);
    }
}
