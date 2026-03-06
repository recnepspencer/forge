//! Shell volume computation.
//!
//! DOMAIN: Compute the signed volume of a topological shell.
//!
//! INVARIANTS:
//! - Uses the divergence theorem (sum of signed tetrahedra).
//! - Deterministic origin selection to avoid floating point precision loss.

use forge_core::KernelError;
use forge_topo::b_rep::TopologyArena;
use forge_topo::handles::{FaceId, VertexId};
use forge_topo::traverse::FaceEdgeIterator;

/// Compute the signed volume of a shell using the divergence theorem.
///
/// Fan-triangulates each face and sums the signed tetrahedra volumes.
/// Positive volume = outward normals (CCW winding). Negative = inward.
pub fn compute_shell_signed_volume(
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

/// Collect vertex positions for a face loop.
pub fn collect_face_positions(
    arena: &TopologyArena,
    face_id: FaceId,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
) -> Result<Vec<[f64; 3]>, KernelError> {
    let mut positions: Vec<[f64; 3]> = Vec::new();

    for he_result in FaceEdgeIterator::new(arena, face_id)? {
        let he_id = he_result?;
        let he_data = arena.get_half_edge(he_id)?;
        let pos = position_fn(he_data.origin()).ok_or_else(|| KernelError::TopologyViolation {
            err: forge_core::TopologyError::MissingVertexPosition {
                vertex_index: he_data.origin().index(),
                face_index: face_id.index(),
            },
            context: None,
        })?;
        positions.push(pos);
    }

    Ok(positions)
}

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

fn compute_fan_volume(vertices: &[[f64; 3]], reference: [f64; 3]) -> f64 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use forge_topo::b_rep::ShellKind;
    use forge_topo::entity_lifecycle::make_vertex_face::MakeVertexFace;
    use forge_topo::entity_lifecycle::split_edge::SplitEdge;
    use forge_topo::transactions::TopologyState;
    use std::collections::BTreeMap;

    #[test]
    fn compute_volume_of_unit_cube_approx() {
        // Build a square face, volume 0 (planar)
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = draft
            .execute(MakeVertexFace {
                shell_kind: ShellKind::Sheet,
            })
            .unwrap()
            .into_value();

        let se1 = draft
            .execute(SplitEdge {
                edge: mvf.half_edge,
            })
            .unwrap()
            .into_value();
        let se2 = draft
            .execute(SplitEdge { edge: se1.he_mb })
            .unwrap()
            .into_value();
        let se3 = draft
            .execute(SplitEdge { edge: se2.he_mb })
            .unwrap()
            .into_value();

        let mut positions = BTreeMap::new();
        positions.insert(mvf.vertex.index(), [0.0, 0.0, 0.0]);
        positions.insert(se1.new_vertex.index(), [1.0, 0.0, 0.0]);
        positions.insert(se2.new_vertex.index(), [1.0, 1.0, 0.0]);
        positions.insert(se3.new_vertex.index(), [0.0, 1.0, 0.0]);

        let state = draft.commit().unwrap();
        let position_fn = |vertex: VertexId| positions.get(&vertex.index()).copied();

        // Face volume contribution to signed volume
        let vol = compute_shell_signed_volume(state.arena(), &[mvf.face], &position_fn).unwrap();
        assert_eq!(vol, 0.0);
    }
}
