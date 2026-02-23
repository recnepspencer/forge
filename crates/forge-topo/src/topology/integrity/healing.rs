//! Orientation healing for inverted shells.
//!
//! DOMAIN: Detects and repairs shells with inward-pointing normals
//! (negative signed volume) by reversing face winding order.
//!
//! DEPENDENCIES: `arena` (entity data), `handles` (typed IDs),
//!               `shell` (shared discovery/volume), `forge-core` (errors)

use std::collections::BTreeSet;

use forge_core::KernelError;
use crate::arena::TopologyArena;
use crate::handles::{FaceId, HalfEdgeId, VertexId};
use crate::topology::queries::traverse::FaceEdgeIterator;
use super::shell::{discover_shell_faces, compute_shell_signed_volume};

/// Result of an orientation healing pass.
#[derive(Debug, Clone)]
pub struct HealingResult {
    shells_checked: usize,
    shells_healed: usize,
}

impl HealingResult {
    /// Number of connected shells examined.
    pub fn shells_checked(&self) -> usize { self.shells_checked }

    /// Number of shells whose winding was reversed.
    pub fn shells_healed(&self) -> usize { self.shells_healed }
}

/// Detect and heal inverted shell orientations in an arena.
///
/// For each connected shell, computes signed volume via the divergence
/// theorem. Shells with negative volume have inward-pointing normals;
/// healing reverses the winding of every face in such shells by swapping
/// next/prev pointers on all halfedges.
///
/// Requires mutable access to the arena since it mutates halfedge
/// connectivity to flip winding direction.
pub fn heal_shell_orientation(
    arena: &mut TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
) -> Result<HealingResult, KernelError> {
    let f_total = arena.face_count();
    if f_total == 0 {
        return Ok(HealingResult { shells_checked: 0, shells_healed: 0 });
    }

    let all_faces: Vec<FaceId> = arena.iter_faces().map(|(fid, _)| fid).collect();
    let mut visited_faces: BTreeSet<u32> = BTreeSet::new();
    let mut shells_to_flip: Vec<Vec<FaceId>> = Vec::new();
    let mut shells_checked: usize = 0;

    for &seed_face in &all_faces {
        if visited_faces.contains(&seed_face.index()) {
            continue;
        }

        let shell_faces = discover_shell_faces(arena, seed_face, &mut visited_faces)?;
        let signed_volume = compute_shell_signed_volume(arena, &shell_faces, position_fn)?;
        shells_checked += 1;

        if signed_volume < 0.0 {
            shells_to_flip.push(shell_faces);
        }
    }

    let shells_healed = shells_to_flip.len();

    for shell_faces in &shells_to_flip {
        flip_shell_winding(arena, shell_faces)?;
    }

    Ok(HealingResult { shells_checked, shells_healed })
}

/// Reverse winding direction of every face in a shell.
///
/// For each face: collect all halfedges, then swap next/prev on each.
/// This reverses the traversal direction, which flips the face normal.
fn flip_shell_winding(
    arena: &mut TopologyArena,
    shell_faces: &[FaceId],
) -> Result<(), KernelError> {
    for &face_id in shell_faces {
        flip_face_winding(arena, face_id)?;
    }
    Ok(())
}

/// Reverse winding of a single face.
///
/// For each halfedge in the face loop, three mutations are required:
/// 1. `origin` ← what was previously the "target" (twin's origin)
/// 2. `next` ← old `prev`
/// 3. `prev` ← old `next`
///
/// This preserves the vertex continuity invariant:
///   `next(he).origin == twin(he).origin`
///
/// Uses a two-phase approach (read all, then mutate all) to avoid
/// data races where mutating one halfedge's pointers corrupts reads
/// of the next halfedge in the loop.
fn flip_face_winding(
    arena: &mut TopologyArena,
    face_id: FaceId,
) -> Result<(), KernelError> {
    let halfedge_ids: Vec<_> = FaceEdgeIterator::new(arena, face_id)?
        .collect::<Result<Vec<_>, _>>()?;

    let mut rewire_data: Vec<(HalfEdgeId, HalfEdgeId, HalfEdgeId, VertexId)> = Vec::new();
    for &he_id in &halfedge_ids {
        let he_data = arena.get_half_edge(he_id)?;
        let old_next = he_data.next();
        let old_prev = he_data.prev();
        let twin_id = he_data.radial_next();
        let new_origin = if he_id != twin_id {
            arena.get_half_edge(twin_id)?.origin()
        } else {
            he_data.origin()
        };
        rewire_data.push((he_id, old_next, old_prev, new_origin));
    }

    for (he_id, old_next, old_prev, new_origin) in rewire_data {
        let he_mut = arena.get_half_edge_mut(he_id)?;
        he_mut.set_next(old_prev);
        he_mut.set_prev(old_next);
        he_mut.set_origin(new_origin);
    }

    Ok(())
}
