//! Shell discovery and volume computation utilities.
//!
//! DOMAIN: Shared helpers for decomposing an arena into connected shells
//! and computing their signed volumes. Used by both geometric validation
//! and orientation healing.
//!
//! DEPENDENCIES: `arena` (entity data), `handles` (typed IDs),
//!               `queries/traverse` (FaceEdgeIterator)

use std::collections::{BTreeSet, VecDeque};

use forge_core::KernelError;
use crate::arena::TopologyArena;
use crate::handles::{FaceId, VertexId};
use crate::topology::queries::traverse::FaceEdgeIterator;

/// Discover all faces in a connected shell via BFS from a seed face.
///
/// Marks discovered faces in `visited` and returns the ordered list.
pub(crate) fn discover_shell_faces(
    arena: &TopologyArena,
    seed_face: FaceId,
    visited: &mut BTreeSet<u32>,
) -> Result<Vec<FaceId>, KernelError> {
    let mut shell_faces: Vec<FaceId> = Vec::new();
    let mut face_set: BTreeSet<u32> = BTreeSet::new();
    let mut queue: VecDeque<FaceId> = VecDeque::new();

    queue.push_back(seed_face);
    face_set.insert(seed_face.index());

    while let Some(face_id) = queue.pop_front() {
        shell_faces.push(face_id);

        for he_result in FaceEdgeIterator::new(arena, face_id)? {
            let he_id = he_result?;
            let he_data = arena.get_half_edge(he_id)?;

            if he_id != he_data.twin() {
                let twin_data = arena.get_half_edge(he_data.twin())?;
                let neighbor = twin_data.face();
                if face_set.insert(neighbor.index()) {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    visited.extend(face_set.iter());
    Ok(shell_faces)
}

/// Compute the signed volume of a shell using the divergence theorem.
///
/// Fan-triangulates each face and sums the signed tetrahedra volumes.
/// Positive volume = outward normals (CCW winding). Negative = inward.
pub(crate) fn compute_shell_signed_volume(
    arena: &TopologyArena,
    shell_faces: &[FaceId],
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
) -> Result<f64, KernelError> {
    let mut signed_volume = 0.0_f64;

    for &face_id in shell_faces {
        let face_verts = collect_face_positions(arena, face_id, position_fn)?;

        if face_verts.len() >= 3 {
            signed_volume += compute_fan_volume(&face_verts);
        }
    }

    Ok(signed_volume / 6.0)
}

/// Collect vertex positions for a face loop using FaceEdgeIterator.
pub(crate) fn collect_face_positions(
    arena: &TopologyArena,
    face_id: FaceId,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
) -> Result<Vec<[f64; 3]>, KernelError> {
    let mut positions: Vec<[f64; 3]> = Vec::new();

    for he_result in FaceEdgeIterator::new(arena, face_id)? {
        let he_id = he_result?;
        let he_data = arena.get_half_edge(he_id)?;
        if let Some(pos) = position_fn(he_data.origin()) {
            positions.push(pos);
        }
    }

    Ok(positions)
}

/// Compute the signed volume contribution from fan-triangulating a polygon.
///
/// Each triangle (v0, vi, vi+1) forms a tetrahedron with the origin.
pub(crate) fn compute_fan_volume(vertices: &[[f64; 3]]) -> f64 {
    let v0 = vertices[0];
    let mut volume = 0.0_f64;

    for i in 1..vertices.len() - 1 {
        let v1 = vertices[i];
        let v2 = vertices[i + 1];
        volume += v0[0] * (v1[1] * v2[2] - v1[2] * v2[1])
            + v0[1] * (v1[2] * v2[0] - v1[0] * v2[2])
            + v0[2] * (v1[0] * v2[1] - v1[1] * v2[0]);
    }

    volume
}
