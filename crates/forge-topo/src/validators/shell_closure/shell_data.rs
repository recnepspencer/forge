//! Shared helper for collecting shell-connected face data.
//!
//! DOMAIN: Traversal utility needed by shell_consistency and euler_genus.

use crate::b_rep::TopologyArena;
use crate::handles::FaceId;
use crate::queries::traverse::FaceEdgeIterator;
use forge_core::KernelError;

/// Collect halfedge IDs for a face's loop and find neighbor faces via twins.
///
/// Returns `(neighbor_faces, edge_keys, vertex_indices)` for the face.
pub(crate) fn collect_shell_data_for_face(
    arena: &TopologyArena,
    face_id: FaceId,
) -> Result<(Vec<FaceId>, Vec<u32>, Vec<u32>), KernelError> {
    let mut neighbors = Vec::new();
    let mut edge_keys = Vec::new();
    let mut vertex_indices = Vec::new();

    for he_result in FaceEdgeIterator::new(arena, face_id)? {
        let he_id = he_result?;
        let he_data = arena.get_half_edge(he_id)?;

        vertex_indices.push(he_data.origin().index());
        edge_keys.push(he_data.edge().index());

        for neighbor_res in crate::queries::traverse::RadialEdgeIterator::new(arena, he_id)? {
            let neighbor_he = neighbor_res?;
            if neighbor_he != he_id {
                let neighbor_data = arena.get_half_edge(neighbor_he)?;
                neighbors.push(neighbor_data.face());
            }
        }
    }

    Ok((neighbors, edge_keys, vertex_indices))
}
