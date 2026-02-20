//! Core twin-stitching logic (passes 1 and 2).
//!
//! DOMAIN: Match halfedges by directed edge identity and set twin pointers.

use std::collections::{BTreeMap, HashSet};
use forge_core::KernelError;
use forge_core::result::{TracedDecision, DecisionId, DecisionKind, DecisionTier, DecisionContext, EntityRef};
use forge_topo::handles::{HalfEdgeId, VertexId};
use forge_topo::state::MutableDraft;
use crate::core::ModelingContext;
use crate::geometry_store::GeometryStore;

use super::fallback::stitch_position_fallback;

/// Stitch twin pointers by matching directed edges across all halfedges.
///
/// Builds a multi-map from (origin, dest) → Vec<HalfEdgeId>. For each
/// halfedge A→B, looks for an unmatched B→A halfedge. When multiple
/// candidates exist at the same (origin, dest), sorts radially by face
/// normal for deterministic pairing.
pub fn stitch_twins(
    draft: &mut MutableDraft,
    all_he_ids: &[HalfEdgeId],
    geom: &GeometryStore,
    weld_tolerance_sq: f64,
    ctx: &mut ModelingContext,
) -> Result<(), KernelError> {
    let mut forward_map: BTreeMap<(u32, u32), Vec<HalfEdgeId>> = BTreeMap::new();
    let mut zero_length: HashSet<u32> = HashSet::new();

    for &he_id in all_he_ids {
        let he_data = draft.arena().get_half_edge(he_id)?;
        let origin = he_data.origin();
        let next_he = he_data.next();
        let dest = draft.arena().get_half_edge(next_he)?.origin();
        if origin == dest {
            zero_length.insert(he_id.index());
        } else {
            forward_map
                .entry((origin.index(), dest.index()))
                .or_default()
                .push(he_id);
        }
    }

    let mut paired: HashSet<u32> = HashSet::new();

    for &he_id in all_he_ids {
        if !paired.contains(&he_id.index()) {
            let he_data = draft.arena().get_half_edge(he_id)?;
            let he_face = he_data.face();
            let origin = he_data.origin();
            let next_he = he_data.next();
            let dest = draft.arena().get_half_edge(next_he)?.origin();

            let reverse_key = (dest.index(), origin.index());

            if let Some(candidates) = forward_map.get(&reverse_key) {
                let unpaired_candidates: Vec<HalfEdgeId> = candidates.iter()
                    .filter(|&&c| {
                        c != he_id
                            && !paired.contains(&c.index())
                            && draft.arena().get_half_edge(c).map(|d| d.face() != he_face).unwrap_or(false)
                    })
                    .copied()
                    .collect();

                if !unpaired_candidates.is_empty() {
                    let best = if unpaired_candidates.len() == 1 {
                        unpaired_candidates[0]
                    } else {
                        select_best_twin(draft, geom, he_id, &unpaired_candidates)
                    };

                    draft.arena_mut().get_half_edge_mut(he_id)?.set_twin(best);
                    draft.arena_mut().get_half_edge_mut(best)?.set_twin(he_id);
                    paired.insert(he_id.index());
                    paired.insert(best.index());

                    let mut decision = TracedDecision::new(
                        DecisionId(he_id.index() as u64),
                        DecisionKind::Exact,
                        DecisionTier::Deterministic,
                        1.0,
                        DecisionContext::Degeneracy { 
                            description: format!("Stitched {} <-> {}", he_id, best) 
                        },
                    );
                    decision.set_entity_scope(EntityRef::new("HalfEdge", he_id.index()));
                    ctx.get_decision_log_mut().record(decision);
                }
            }
        }
    }

    let mut unpaired: Vec<(HalfEdgeId, VertexId, VertexId)> = Vec::new();
    for &he_id in all_he_ids {
        if paired.contains(&he_id.index()) || zero_length.contains(&he_id.index()) {
            continue;
        }
        let he_data = draft.arena().get_half_edge(he_id)?;
        let origin = he_data.origin();
        let next_he = he_data.next();
        let dest = draft.arena().get_half_edge(next_he)?.origin();
        unpaired.push((he_id, origin, dest));
    }

    if unpaired.is_empty() {
        return Ok(());
    }

    let mut unpaired_map: BTreeMap<(u32, u32), Vec<HalfEdgeId>> = BTreeMap::new();
    for &(he_id, origin, dest) in &unpaired {
        unpaired_map
            .entry((origin.index(), dest.index()))
            .or_default()
            .push(he_id);
    }

    let mut paired_unpaired: HashSet<u32> = HashSet::new();
    for &(he_id, origin, dest) in &unpaired {
        if !paired_unpaired.contains(&he_id.index()) {
            let he_face = draft.arena().get_half_edge(he_id)?.face();
            let reverse_key = (dest.index(), origin.index());
            if let Some(candidates) = unpaired_map.get(&reverse_key) {
                let unpaired_candidates: Vec<HalfEdgeId> = candidates.iter()
                    .filter(|&&c| {
                        c != he_id
                            && !paired_unpaired.contains(&c.index())
                            && draft.arena().get_half_edge(c).map(|d| d.face() != he_face).unwrap_or(false)
                    })
                    .copied()
                    .collect();

                if !unpaired_candidates.is_empty() {
                    let best = if unpaired_candidates.len() == 1 {
                        unpaired_candidates[0]
                    } else {
                        select_best_twin(draft, geom, he_id, &unpaired_candidates)
                    };

                    draft.arena_mut().get_half_edge_mut(he_id)?.set_twin(best);
                    draft.arena_mut().get_half_edge_mut(best)?.set_twin(he_id);
                    paired_unpaired.insert(he_id.index());
                    paired_unpaired.insert(best.index());

                    let mut decision = TracedDecision::new(
                        DecisionId(he_id.index() as u64),
                        DecisionKind::PolicyApplied { policy: forge_core::PolicyKind::CoincidentGeometry, default_used: true },
                        DecisionTier::Deterministic,
                        1.0,
                        DecisionContext::Degeneracy { 
                            description: format!("Stitched {} <-> {} (retry)", he_id, best) 
                        },
                    );
                    decision.set_entity_scope(EntityRef::new("HalfEdge", he_id.index()));
                    ctx.get_decision_log_mut().record(decision);
                }
            }
        }
    }

    let still_unpaired_after_retry: Vec<HalfEdgeId> = all_he_ids.iter()
        .filter(|he_id| {
            !paired.contains(&he_id.index())
                && !paired_unpaired.contains(&he_id.index())
                && !zero_length.contains(&he_id.index())
        })
        .copied()
        .collect();

    if !still_unpaired_after_retry.is_empty() {
        stitch_position_fallback(draft, geom, &still_unpaired_after_retry, weld_tolerance_sq, ctx)?;
    }

    Ok(())
}

/// Select the best twin candidate using radial angle sorting.
///
/// When multiple reverse halfedges exist for the same directed edge,
/// picks the one whose face normal is closest (smallest angle) to
/// the source halfedge's face normal when measured around the edge axis.
/// Falls back to index-based ordering for determinism if geometry is missing.
pub(super) fn select_best_twin(
    draft: &MutableDraft,
    geom: &GeometryStore,
    source_he: HalfEdgeId,
    candidates: &[HalfEdgeId],
) -> HalfEdgeId {
    let source_face = draft.arena().get_half_edge(source_he)
        .map(|d| d.face())
        .ok();

    let source_normal = source_face
        .and_then(|f| geom.get_face_plane(f))
        .map(|p| p.raw_normal());

    let Some(sn) = source_normal else {
        return *candidates.iter().min_by_key(|c| c.index()).unwrap_or(&candidates[0]);
    };

    let mut best = candidates[0];
    let mut best_dot = f64::NEG_INFINITY;

    for &cand in candidates {
        let cand_face = draft.arena().get_half_edge(cand)
            .map(|d| d.face())
            .ok();

        let cand_normal = cand_face
            .and_then(|f| geom.get_face_plane(f))
            .map(|p| p.raw_normal());

        if let Some(cn) = cand_normal {
            let dot = forge_math::linalg::dot(sn, cn);
            if dot > best_dot || (dot == best_dot && cand.index() < best.index()) {
                best_dot = dot;
                best = cand;
            }
        }
    }

    best
}
