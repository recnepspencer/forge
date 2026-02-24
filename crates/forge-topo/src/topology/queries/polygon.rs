//! Hole-aware polygon/loop extraction helpers.
//!
//! DOMAIN: Read-only extraction of ordered loop vertex IDs for a face, preserving
//! outer/inner loop separation for downstream geometric algorithms.

use forge_core::KernelError;

use crate::arena::TopologyArena;
use crate::handles::{FaceId, VertexId};
use crate::topology::queries::traverse::LoopEdgeIterator;
use crate::topology::queries::traverse::FaceLoopsIterator;

/// Return ordered vertex IDs for all loops on a face.
///
/// The first entry is the outer loop, followed by inner loops in face storage
/// order. Empty/corrupt loops propagate `KernelError`.
pub fn face_loop_vertices(
    arena: &TopologyArena,
    face: FaceId,
) -> Result<Vec<Vec<VertexId>>, KernelError> {
    let mut loops = Vec::new();
    for loop_id in FaceLoopsIterator::new(arena, face)? {
        let mut vertices = Vec::new();
        for he_res in LoopEdgeIterator::new(arena, loop_id)? {
            let he_id = he_res?;
            let he = arena.get_half_edge(he_id)?;
            vertices.push(he.origin());
        }
        loops.push(vertices);
    }
    Ok(loops)
}
