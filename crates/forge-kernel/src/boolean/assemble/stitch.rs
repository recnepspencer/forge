//! Topology stitching logic.

use std::collections::{HashMap, HashSet};
use forge_core::KernelError;
use forge_topo::handles::{HalfEdgeId, VertexId};
use forge_topo::state::MutableDraft;

/// Stitch twin pointers by matching directed edges across all halfedges.
///
/// Builds a multi-map from (origin, dest) → Vec<HalfEdgeId>. For each
/// halfedge A→B, looks for an unmatched B→A halfedge. Uses greedy 1:1
/// pairing. Unpaired halfedges get boundary twins to maintain manifold.
pub fn stitch_twins(
    draft: &mut MutableDraft,
    all_he_ids: &[HalfEdgeId],
) -> Result<(), KernelError> {
    let _placeholder = HalfEdgeId::new(u32::MAX, 0);

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
        }
    }

    if unpaired.is_empty() {
        return Ok(());
    }

    let mut unpaired_map: HashMap<(u32, u32), Vec<HalfEdgeId>> = HashMap::new();
    for &(he_id, origin, dest) in &unpaired {
        unpaired_map
            .entry((origin.index(), dest.index()))
            .or_default()
            .push(he_id);
    }

    let mut paired_unpaired: HashSet<u32> = HashSet::new();
    for &(he_id, origin, dest) in &unpaired {
        if paired_unpaired.contains(&he_id.index()) {
            continue;
        }
        let reverse_key = (dest.index(), origin.index());
        if let Some(candidates) = unpaired_map.get(&reverse_key) {
            for &cand in candidates {
                if cand != he_id && !paired_unpaired.contains(&cand.index()) {
                    draft.arena_mut().get_half_edge_mut(he_id)?.twin = cand;
                    draft.arena_mut().get_half_edge_mut(cand)?.twin = he_id;
                    paired_unpaired.insert(he_id.index());
                    paired_unpaired.insert(cand.index());
                    break;
                }
            }
        }
    }

    let still_unpaired: Vec<HalfEdgeId> = all_he_ids.iter()
        .filter(|he_id| {
            !paired.contains(&he_id.index()) && !paired_unpaired.contains(&he_id.index())
        })
        .copied()
        .collect();

    if !still_unpaired.is_empty() {
        eprintln!("=== STITCH FAILURE: {} unpaired halfedges ===", still_unpaired.len());
        for &he_id in &still_unpaired {
            let he_data = draft.arena().get_half_edge(he_id)?;
            let origin = he_data.origin;
            let next_he = he_data.next;
            let dest = draft.arena().get_half_edge(next_he)?.origin;
            eprintln!("  he={} : {} -> {} (face={})", he_id, origin, dest, he_data.face);
        }
        eprintln!("=== All directed edges in forward_map ===");
        for ((o, d), hes) in &forward_map {
            eprintln!("  ({},{}) -> {:?}", o, d, hes);
        }
        return Err(KernelError::TopologyViolation {
            err: forge_core::TopologyError::MissingTwin {
                halfedge_index: still_unpaired[0].index(),
            },
            context: Some(forge_core::ErrorContext {
                scope: forge_core::ErrorScope::Global,
                suggested_fixes: Vec::new(),
                detail: format!(
                    "{} halfedges remain unpaired after stitching (first: {})",
                    still_unpaired.len(),
                    still_unpaired[0],
                ),
            }),
        });
    }

    Ok(())
}
