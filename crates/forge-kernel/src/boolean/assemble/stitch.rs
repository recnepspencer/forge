//! Topology stitching logic.

use std::collections::{HashMap, HashSet};
use forge_core::KernelError;
use forge_topo::handles::{HalfEdgeId, VertexId};
use forge_topo::state::MutableDraft;
use forge_topo::arena::HalfEdgeData;

/// Stitch twin pointers by matching directed edges across all halfedges.
///
/// Builds a multi-map from (origin, dest) → Vec<HalfEdgeId>. For each
/// halfedge A→B, looks for an unmatched B→A halfedge. Uses greedy 1:1
/// pairing. Unpaired halfedges get boundary twins to maintain manifold.
pub fn stitch_twins(
    draft: &mut MutableDraft,
    all_he_ids: &[HalfEdgeId],
) -> Result<(), KernelError> {
    let placeholder = HalfEdgeId::new(u32::MAX, 0);

    let mut forward_map: HashMap<(u32, u32), Vec<HalfEdgeId>> = HashMap::new();

    for &he_id in all_he_ids {
        let he_data = draft.arena().get_half_edge(he_id)?;
        let origin = he_data.origin;
        let next_he = he_data.next;
        let dest = draft.arena().get_half_edge(next_he)?.origin;
        forward_map
            .entry((origin.index(), dest.index()))
            .or_default()
            .push(he_id);
    }

    let mut paired: HashSet<u32> = HashSet::new();

    for &he_id in all_he_ids {
        if paired.contains(&he_id.index()) {
            continue;
        }

        let he_data = draft.arena().get_half_edge(he_id)?;
        let origin = he_data.origin;
        let next_he = he_data.next;
        let dest = draft.arena().get_half_edge(next_he)?.origin;

        let reverse_key = (dest.index(), origin.index());

        if let Some(candidates) = forward_map.get(&reverse_key) {
            for &cand in candidates {
                if cand != he_id && !paired.contains(&cand.index()) {
                    draft.arena_mut().get_half_edge_mut(he_id)?.twin = cand;
                    draft.arena_mut().get_half_edge_mut(cand)?.twin = he_id;
                    paired.insert(he_id.index());
                    paired.insert(cand.index());
                    // eprintln!("Stitched {} <-> {}", he_id, cand);
                    break;
                }
            }
        }
    }

    let mut unpaired: Vec<(HalfEdgeId, VertexId, VertexId)> = Vec::new();
    for &he_id in all_he_ids {
        if !paired.contains(&he_id.index()) {
            let he_data = draft.arena().get_half_edge(he_id)?;
            let origin = he_data.origin;
            let next_he = he_data.next;
            let dest = draft.arena().get_half_edge(next_he)?.origin;
            unpaired.push((he_id, origin, dest));
            // eprintln!("Unpaired edge: {} ({} -> {})", he_id, origin, dest);
        }
    }

    if unpaired.is_empty() {
        return Ok(());
    }

    let mut boundary_he_by_origin: HashMap<u32, HalfEdgeId> = HashMap::new();

    for &(he_in, _origin, dest) in &unpaired {
        let boundary_face = draft.arena().get_half_edge(he_in)?.face;

        let he_out = draft.arena_mut().insert_half_edge(HalfEdgeData {
            twin: he_in,
            next: placeholder,
            prev: placeholder,
            face: boundary_face,
            origin: dest,
            lineage: None,
        });

        draft.arena_mut().get_half_edge_mut(he_in)?.twin = he_out;
        boundary_he_by_origin.insert(dest.index(), he_out);
    }

    for &(_he_in, origin, dest) in &unpaired {
        if let Some(&he_out) = boundary_he_by_origin.get(&dest.index()) {
            if let Some(&he_next) = boundary_he_by_origin.get(&origin.index()) {
                draft.arena_mut().get_half_edge_mut(he_out)?.next = he_next;
                draft.arena_mut().get_half_edge_mut(he_next)?.prev = he_out;
            }
        }
    }

    Ok(())
}
