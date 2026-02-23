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
///
/// Uses the first vertex of the first face as a reference origin to
/// avoid catastrophic cancellation when the model is far from `[0,0,0]`.
pub(crate) fn compute_shell_signed_volume(
    arena: &TopologyArena,
    shell_faces: &[FaceId],
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
) -> Result<f64, KernelError> {
    let reference = find_reference_origin(arena, shell_faces, position_fn)?;
    let mut signed_volume = 0.0_f64;

    for &face_id in shell_faces {
        let face_verts = collect_face_positions(arena, face_id, position_fn)?;

        if face_verts.len() >= 3 {
            signed_volume += compute_fan_volume(&face_verts, reference);
        }
    }

    Ok(signed_volume / 6.0)
}

/// Collect vertex positions for a face loop using FaceEdgeIterator.
///
/// Returns `Err(MissingVertexPosition)` if any vertex referenced by
/// the face has no position available. Silent skipping would turn
/// quads into degenerate polygons and corrupt area/volume calculations.
pub(crate) fn collect_face_positions(
    arena: &TopologyArena,
    face_id: FaceId,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
) -> Result<Vec<[f64; 3]>, KernelError> {
    let mut positions: Vec<[f64; 3]> = Vec::new();

    for he_result in FaceEdgeIterator::new(arena, face_id)? {
        let he_id = he_result?;
        let he_data = arena.get_half_edge(he_id)?;
        let pos = position_fn(he_data.origin()).ok_or_else(|| {
            KernelError::TopologyViolation {
                err: forge_core::TopologyError::MissingVertexPosition {
                    vertex_index: he_data.origin().index(),
                    face_index: face_id.index(),
                },
                context: None,
            }
        })?;
        positions.push(pos);
    }

    Ok(positions)
}

/// Find a reference origin point from the first face of a shell.
///
/// Falls back to `[0,0,0]` if no positions are available (shouldn't
/// happen since collect_face_positions now errors on missing positions).
fn find_reference_origin(
    arena: &TopologyArena,
    shell_faces: &[FaceId],
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
) -> Result<[f64; 3], KernelError> {
    for &face_id in shell_faces {
        let positions = collect_face_positions(arena, face_id, position_fn)?;
        if !positions.is_empty() {
            return Ok(positions[0]);
        }
    }
    Ok([0.0, 0.0, 0.0])
}

/// Compute the signed volume contribution from fan-triangulating a polygon.
///
/// Each triangle (v0, vi, vi+1) forms a tetrahedron with the reference
/// origin (already subtracted from vertex positions).
pub(crate) fn compute_fan_volume(vertices: &[[f64; 3]], reference: [f64; 3]) -> f64 {
    let v0 = [
        vertices[0][0] - reference[0],
        vertices[0][1] - reference[1],
        vertices[0][2] - reference[2],
    ];
    let mut volume = 0.0_f64;

    for i in 1..vertices.len() - 1 {
        let v1 = [
            vertices[i][0] - reference[0],
            vertices[i][1] - reference[1],
            vertices[i][2] - reference[2],
        ];
        let v2 = [
            vertices[i + 1][0] - reference[0],
            vertices[i + 1][1] - reference[1],
            vertices[i + 1][2] - reference[2],
        ];
        volume += v0[0] * (v1[1] * v2[2] - v1[2] * v2[1])
            + v0[1] * (v1[2] * v2[0] - v1[0] * v2[2])
            + v0[2] * (v1[0] * v2[1] - v1[1] * v2[0]);
    }

    volume
}
