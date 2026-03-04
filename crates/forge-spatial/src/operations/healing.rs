//! Orientation healing for inverted shells.
//!
//! DOMAIN: Detects and repairs shells with inward-pointing normals
//! (negative signed volume) by reversing face winding order.

use crate::operations::volume::{compute_shell_signed_volume};
use forge_topo::b_rep::{TopologyArena, EntityBitset};
use forge_topo::handles::{FaceId, HalfEdgeId, VertexId};
use forge_topo::queries::shell::discover_shell_faces;
use forge_topo::traverse::FaceEdgeIterator;
use forge_core::KernelError;

/// Result of an orientation healing pass.
#[derive(Debug, Clone)]
pub struct HealingResult {
    shells_checked: usize,
    shells_healed: usize,
}

impl HealingResult {
    pub fn shells_checked(&self) -> usize { self.shells_checked }
    pub fn shells_healed(&self) -> usize { self.shells_healed }
}

/// Detect and heal inverted shell orientations in an arena.
pub fn heal_shell_orientation(
    arena: &mut TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
) -> Result<HealingResult, KernelError> {
    let f_total = arena.face_count();
    if f_total == 0 {
        return Ok(HealingResult { shells_checked: 0, shells_healed: 0 });
    }

    let all_faces: Vec<FaceId> = arena.iter_faces().map(|(fid, _)| fid).collect();
    let mut visited_faces = EntityBitset::for_faces(arena);
    let mut shells_to_flip: Vec<Vec<FaceId>> = Vec::new();
    let mut shells_checked: usize = 0;

    for &seed_face in &all_faces {
        if visited_faces.contains(seed_face.index()).unwrap_or(false) {
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
        for &face_id in shell_faces {
            flip_face_winding(arena, face_id)?;
        }
    }

    Ok(HealingResult { shells_checked, shells_healed })
}

fn flip_face_winding(arena: &mut TopologyArena, face_id: FaceId) -> Result<(), KernelError> {
    let halfedge_ids: Vec<_> = FaceEdgeIterator::new(arena, face_id)?.collect::<Result<Vec<_>, _>>()?;

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
