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

/// Walk a G1-continuous (tangent-continuous) edge chain starting at `start_edge`.
///
/// Fillet operations act on edge *chains* — sequences of edges that share vertices
/// and whose adjacent faces are tangent-continuous (G1) at those vertices.
/// This function walks the chain by following `twin → next` at each shared vertex
/// (the standard manifold-edge successor) and extends the chain while the dihedral
/// angle between the adjacent face normals is below `angle_threshold` (in radians).
///
/// Non-bridge edges are followed; bridge halfedges (from `BridgeEdge`) are detected
/// and skipped so that synthetic boundaries do not splice into organic fillet chains.
///
/// # Arguments
/// - `arena`: topology arena
/// - `start_edge`: the first halfedge of the chain
/// - `position_fn`: maps vertex index → 3D world position (needed for normals)
/// - `angle_threshold`: maximum dihedral angle (radians) to consider G1-continuous
///
/// Returns the ordered list of halfedge IDs forming the G1 chain.
pub fn find_g1_chain(
    arena: &TopologyArena,
    start_edge: HalfEdgeId,
    position_fn: &dyn Fn(crate::handles::VertexId) -> Option<[f64; 3]>,
    angle_threshold: f64,
) -> Result<Vec<HalfEdgeId>, KernelError> {
    let cos_threshold = angle_threshold.cos();
    let max_iter = arena.half_edge_count().max(1);
    let mut chain = vec![start_edge];
    let mut current = start_edge;

    for _ in 0..max_iter {
        let he_data = arena.get_half_edge(current)?;

        // Advance: twin of current → next of that twin → candidate next edge.
        let twin_id = he_data.twin();
        let twin_data = arena.get_half_edge(twin_id)?;
        let candidate = twin_data.next();

        // Stop if we've looped back to the start.
        if candidate == start_edge { break; }

        // Skip bridge halfedges — they are synthetic and should not be filleted.
        let candidate_data = arena.get_half_edge(candidate)?;
        if candidate_data.is_bridge() { break; }

        // Compute dihedral angle between the face of `current` and the face of `candidate`.
        let face_a = he_data.face();
        let face_b = candidate_data.face();

        if face_a == face_b {
            // Same face — the chain has folded back; stop.
            break;
        }

        let normal_a = face_normal_from_loop(arena, face_a, position_fn)?;
        let normal_b = face_normal_from_loop(arena, face_b, position_fn)?;

        if let (Some(na), Some(nb)) = (normal_a, normal_b) {
            let dot = na[0]*nb[0] + na[1]*nb[1] + na[2]*nb[2];
            if dot < cos_threshold {
                // Dihedral angle exceeds threshold — chain ends here.
                break;
            }
        } else {
            // Degenerate face — cannot compute normal, stop cascading.
            break;
        }

        chain.push(candidate);
        current = candidate;
    }

    Ok(chain)
}

/// Compute the face normal from the first three non-collinear vertices in the loop.
///
/// Returns `None` for degenerate (collinear) faces.
fn face_normal_from_loop(
    arena: &TopologyArena,
    face: FaceId,
    position_fn: &dyn Fn(crate::handles::VertexId) -> Option<[f64; 3]>,
) -> Result<Option<[f64; 3]>, KernelError> {
    let mut positions: Vec<[f64; 3]> = Vec::new();
    for he_res in FaceEdgeIterator::new(arena, face)? {
        let he_id = he_res?;
        let v = arena.get_half_edge(he_id)?.origin();
        if let Some(pos) = position_fn(v) {
            positions.push(pos);
            if positions.len() >= 3 { break; }
        }
    }

    if positions.len() < 3 {
        return Ok(None);
    }

    let a = positions[0];
    let b = positions[1];
    let c = positions[2];
    let ab = [b[0]-a[0], b[1]-a[1], b[2]-a[2]];
    let ac = [c[0]-a[0], c[1]-a[1], c[2]-a[2]];
    let nx = ab[1]*ac[2] - ab[2]*ac[1];
    let ny = ab[2]*ac[0] - ab[0]*ac[2];
    let nz = ab[0]*ac[1] - ab[1]*ac[0];
    let len = (nx*nx + ny*ny + nz*nz).sqrt();
    if len < 1e-30 {
        return Ok(None);
    }
    Ok(Some([nx/len, ny/len, nz/len]))
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
