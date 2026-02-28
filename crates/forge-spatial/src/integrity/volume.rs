//! Shell signed-volume validation for outward-normal enforcement.
//!
//! DOMAIN: Validate that all closed shells have positive signed volume.
//! Negative volume means inward-pointing normals (winding-order defect).
//!
//! ALGORITHM: BFS-discovers each connected shell from unvisited seed faces,
//! fan-triangulates each face, sums signed tetrahedra volumes via the
//! divergence theorem. All traversal uses the public forge-topo API only.

use forge_core::KernelError;
use forge_topo::b_rep::TopologyArena;
use forge_topo::handles::{FaceId, VertexId};
use forge_topo::traverse::{FaceEdgeIterator, RadialEdgeIterator};
use std::collections::{BTreeSet, VecDeque};

/// Validate that all closed shells have positive signed volume.
pub fn validate_signed_volume(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
) -> Result<(), KernelError> {
    if arena.face_count() == 0 {
        return Ok(());
    }

    let all_face_ids: Vec<FaceId> = arena.iter_faces().map(|(id, _)| id).collect();
    let mut visited: BTreeSet<u32> = BTreeSet::new();
    let mut shell_index: u32 = 0;

    for seed_face in all_face_ids {
        if visited.contains(&seed_face.index()) {
            continue;
        }

        let shell = bfs_shell(arena, seed_face, &mut visited)?;
        let signed_volume = compute_signed_volume(arena, &shell, position_fn)?;

        if signed_volume < 0.0 {
            return Err(KernelError::TopologyViolation {
                err: forge_core::TopologyError::NegativeShellVolume {
                    shell_index,
                    signed_volume,
                },
                context: Some(forge_core::ErrorContext {
                    scope: forge_core::ErrorScope::Entity {
                        entity_kind: "Shell".to_string(),
                        index: shell_index,
                    },
                    suggested_fixes: Vec::new(),
                    detail: format!(
                        "Shell {} signed volume {:.6e} — normals point inward",
                        shell_index, signed_volume
                    ),
                }),
            });
        }

        shell_index += 1;
    }
    Ok(())
}

fn bfs_shell(
    arena: &TopologyArena,
    seed: FaceId,
    visited: &mut BTreeSet<u32>,
) -> Result<Vec<FaceId>, KernelError> {
    let mut shell = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(seed);
    visited.insert(seed.index());

    while let Some(face_id) = queue.pop_front() {
        shell.push(face_id);
        for he_res in FaceEdgeIterator::new(arena, face_id)? {
            let he_id = he_res?;
            for radial_res in RadialEdgeIterator::new(arena, he_id)? {
                let twin_id = radial_res?;
                if twin_id == he_id {
                    continue;
                }
                let neighbor = arena.get_half_edge(twin_id)?.face();
                if visited.insert(neighbor.index()) {
                    queue.push_back(neighbor);
                }
            }
        }
    }
    Ok(shell)
}

fn compute_signed_volume(
    arena: &TopologyArena,
    faces: &[FaceId],
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
) -> Result<f64, KernelError> {
    let reference = find_reference(arena, faces, position_fn);
    let mut volume = 0.0_f64;
    for &face_id in faces {
        let verts = collect_positions(arena, face_id, position_fn)?;
        if verts.len() >= 3 {
            volume += fan_volume(&verts, reference);
        }
    }
    Ok(volume / 6.0)
}

fn find_reference(
    arena: &TopologyArena,
    faces: &[FaceId],
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
) -> [f64; 3] {
    for &face_id in faces {
        if let Ok(verts) = collect_positions(arena, face_id, position_fn) {
            if !verts.is_empty() {
                return verts[0];
            }
        }
    }
    [0.0, 0.0, 0.0]
}

fn collect_positions(
    arena: &TopologyArena,
    face_id: FaceId,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
) -> Result<Vec<[f64; 3]>, KernelError> {
    let mut positions = Vec::new();
    for he_res in FaceEdgeIterator::new(arena, face_id)? {
        let he = arena.get_half_edge(he_res?)?;
        let v = he.origin();
        let pos = position_fn(v).ok_or_else(|| KernelError::TopologyViolation {
            err: forge_core::TopologyError::MissingVertexPosition {
                vertex_index: v.index(),
                face_index: face_id.index(),
            },
            context: None,
        })?;
        positions.push(pos);
    }
    Ok(positions)
}

fn fan_volume(verts: &[[f64; 3]], ref_pt: [f64; 3]) -> f64 {
    let v0 = sub(verts[0], ref_pt);
    let mut vol = 0.0_f64;
    for i in 1..verts.len() - 1 {
        let v1 = sub(verts[i], ref_pt);
        let v2 = sub(verts[i + 1], ref_pt);
        vol += v0[0] * (v1[1] * v2[2] - v1[2] * v2[1])
            + v0[1] * (v1[2] * v2[0] - v1[0] * v2[2])
            + v0[2] * (v1[0] * v2[1] - v1[1] * v2[0]);
    }
    vol
}

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
