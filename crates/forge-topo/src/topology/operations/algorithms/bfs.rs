//! Breadth-first traversal helpers for topology algorithms.
//!
//! DOMAIN: Reusable BFS traversal over topology face adjacency.
//!
//! INVARIANTS:
//! - Deterministic traversal order via deterministic adjacency queries
//! - Corruption-safe traversal through query APIs
//! - No topology mutation

use std::collections::VecDeque;

use forge_core::KernelError;

use crate::arena::TopologyArena;
use crate::handles::FaceId;
use crate::topology::bitset::EntityBitset;
use crate::topology::queries::classification::face_adjacent_faces;

/// Collect one connected face component using a caller-supplied edge relation.
///
/// Starts from `seed`, marks visited faces in `visited`, and traverses adjacency
/// when `include_neighbor(current, neighbor)` returns `true`.
pub fn collect_connected_face_component<F>(
    arena: &TopologyArena,
    seed: FaceId,
    visited: &mut EntityBitset,
    mut include_neighbor: F,
) -> Result<EntityBitset, KernelError>
where
    F: FnMut(FaceId, FaceId) -> Result<bool, KernelError>,
{
    let mut group = EntityBitset::for_faces(arena);
    let mut queue = VecDeque::new();

    let _ = visited.insert(seed.index());
    let _ = group.insert(seed.index());
    queue.push_back(seed);

    while let Some(current) = queue.pop_front() {
        let neighbors = face_adjacent_faces(arena, current)?;
        for neighbor in neighbors {
            if visited.contains(neighbor.index()).unwrap_or(false) {
                continue;
            }

            let _ = visited.insert(neighbor.index());
            if include_neighbor(current, neighbor)? {
                let _ = group.insert(neighbor.index());
                queue.push_back(neighbor);
            }
        }
    }

    Ok(group)
}
