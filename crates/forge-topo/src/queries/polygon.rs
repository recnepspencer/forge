//! Hole-aware polygon/loop extraction helpers.
//!
//! DOMAIN: Read-only extraction of ordered loop vertex IDs for a face, preserving
//! outer/inner loop separation for downstream geometric algorithms.

use std::collections::BTreeSet;

use forge_core::KernelError;

use crate::b_rep::TopologyArena;
use crate::handles::{FaceId, VertexId};
use crate::queries::traverse::FaceLoopsIterator;
use crate::queries::traverse::LoopEdgeIterator;

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

/// True when the face has exactly one loop (no inner loops / holes).
///
/// Several geometric algorithms (polygon overlap, CSG classification)
/// only support single-loop faces. Use this predicate to guard those
/// code paths before they attempt to process multi-ring polygons.
pub fn face_has_single_loop(
    arena: &TopologyArena,
    face: FaceId,
) -> Result<bool, KernelError> {
    let loops = face_loop_vertices(arena, face)?;
    Ok(loops.len() == 1)
}

/// Build the set of vertex-index pairs that are already adjacent on a face.
///
/// Returns a `BTreeSet<(u32, u32)>` where each pair is stored as
/// `(min_index, max_index)` — order-independent.
///
/// Any operation that inserts new edges into a face (Boolean MakeEdgeFace,
/// fillet arc insertion, shell offset edge connection) must check this set
/// to avoid creating a degenerate cut between two vertices that already
/// share a boundary edge.
pub fn face_adjacent_vertex_pairs(
    arena: &TopologyArena,
    face: FaceId,
) -> Result<BTreeSet<(u32, u32)>, KernelError> {
    use crate::traverse::FaceAllEdgesIterator;
    let mut pairs = BTreeSet::new();
    for he_result in FaceAllEdgesIterator::new(arena, face)? {
        let he = he_result?;
        let he_data = arena.get_half_edge(he)?;
        let origin = he_data.origin().index();
        let next = arena.get_half_edge(he_data.next())?.origin().index();
        let key = if origin <= next { (origin, next) } else { (next, origin) };
        pairs.insert(key);
    }
    Ok(pairs)
}
