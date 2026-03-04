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

use crate::operations::volume::compute_shell_signed_volume;

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
        let signed_volume = compute_shell_signed_volume(arena, &shell, position_fn)?;

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
