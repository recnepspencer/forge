//! Connected-component collection helpers for topology algorithms.
//!
//! DOMAIN: Arena-wide component discovery built on reusable BFS traversal.
//!
//! INVARIANTS:
//! - Deterministic seed iteration via `arena.iter_faces()`
//! - Corruption-safe traversal through query APIs
//! - No topology mutation

use forge_core::KernelError;

use crate::arena::TopologyArena;
use crate::handles::FaceId;
use crate::topology::bitset::EntityBitset;
use crate::topology::operations::algorithms::bfs::collect_connected_face_component;

/// Collect connected face components across the arena using caller predicates.
///
/// `init_seed(face)` decides whether a face can seed a component and may return
/// seed-specific state used by the neighbor inclusion predicate.
///
/// Faces that return `Ok(None)` from `init_seed` are marked visited and skipped.
pub fn collect_connected_face_components<S, FSeed, FInclude>(
    arena: &TopologyArena,
    mut init_seed: FSeed,
    mut include_neighbor: FInclude,
) -> Result<Vec<EntityBitset>, KernelError>
where
    FSeed: FnMut(FaceId) -> Result<Option<S>, KernelError>,
    FInclude: FnMut(&S, FaceId, FaceId) -> Result<bool, KernelError>,
{
    let mut visited = EntityBitset::for_faces(arena);
    let mut groups = Vec::new();

    for (face_id, _) in arena.iter_faces() {
        if visited.contains(face_id.index()).unwrap_or(false) {
            continue;
        }

        let Some(seed_state) = init_seed(face_id)? else {
            let _ = visited.insert(face_id.index());
            continue;
        };

        let group = collect_connected_face_component(
            arena,
            face_id,
            &mut visited,
            |current, neighbor| include_neighbor(&seed_state, current, neighbor),
        )?;
        groups.push(group);
    }

    Ok(groups)
}
