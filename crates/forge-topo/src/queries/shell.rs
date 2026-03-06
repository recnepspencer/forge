//! Shell discovery utilities.
//!
//! DOMAIN: Shared helpers for decomposing an arena into connected shells.
//! Pure graph traversal — no geometric computation.
//!
//! DEPENDENCIES: `arena` (entity data), `handles` (typed IDs),
//!               `queries/traverse` (FaceEdgeIterator)

use std::collections::VecDeque;

use crate::b_rep::EntityBitset;
use crate::b_rep::TopologyArena;
use crate::handles::FaceId;
use crate::queries::traverse::FaceEdgeIterator;
use forge_core::KernelError;

/// Discover all faces in a connected shell via BFS from a seed face.
///
/// Marks discovered faces in `visited` and returns the ordered list.
pub fn discover_shell_faces(
    arena: &TopologyArena,
    seed_face: FaceId,
    visited: &mut EntityBitset,
) -> Result<Vec<FaceId>, KernelError> {
    let mut shell_faces: Vec<FaceId> = Vec::new();
    let mut face_set = EntityBitset::for_faces(arena);
    let mut queue: VecDeque<FaceId> = VecDeque::new();

    queue.push_back(seed_face);
    face_set.insert(seed_face.index())?;

    while let Some(face_id) = queue.pop_front() {
        shell_faces.push(face_id);

        for he_result in FaceEdgeIterator::new(arena, face_id)? {
            let he_id = he_result?;
            for neighbor_res in crate::queries::traverse::RadialEdgeIterator::new(arena, he_id)? {
                let neighbor_id = neighbor_res?;
                if neighbor_id != he_id {
                    let neighbor_data = arena.get_half_edge(neighbor_id)?;
                    let neighbor = neighbor_data.face();
                    if face_set.insert(neighbor.index())? {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    for idx in face_set.iter_ones() {
        visited.insert(idx)?;
    }
    Ok(shell_faces)
}
