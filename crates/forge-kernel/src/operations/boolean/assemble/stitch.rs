//! Topology stitching logic.
//!
//! DOMAIN: Pair halfedges via twin pointers after face assembly.
//!
//! ALGORITHM: For each directed edge (origin→dest), find the matching
//! reverse edge (dest→origin) and set twin pointers. When multiple
//! halfedges share the same directed edge (non-manifold junction from
//! boolean intersection), radially sort them by face normal around the
//! edge axis for deterministic pairing.

use std::collections::{HashMap, HashSet};
use forge_core::KernelError;
use forge_core::result::{TracedDecision, DecisionId, DecisionKind, DecisionTier, DecisionContext, EntityRef};
use forge_topo::handles::{HalfEdgeId, VertexId};
use forge_topo::state::MutableDraft;
use crate::core::ModelingContext;
use crate::geometry_store::GeometryStore;

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
    let mut forward_map: HashMap<(u32, u32), Vec<HalfEdgeId>> = HashMap::new();
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
        if paired.contains(&he_id.index()) {
            continue;
        }

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

            if unpaired_candidates.is_empty() {
                continue;
            }

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

    // ── Second pass: index-based retry on unpaired subset ──────────
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

            if unpaired_candidates.is_empty() {
                continue;
            }

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

    // ── Third pass: position-based fallback ──────────────────────────
    // When vertex indices don't match (duplicate vertices at same position
    // from re-used boolean results), match by geometric endpoint positions.
    let still_unpaired_after_retry: Vec<HalfEdgeId> = all_he_ids.iter()
        .filter(|he_id| {
            !paired.contains(&he_id.index())
                && !paired_unpaired.contains(&he_id.index())
                && !zero_length.contains(&he_id.index())
        })
        .copied()
        .collect();

    if !still_unpaired_after_retry.is_empty() {
        let mut position_paired: HashSet<u32> = HashSet::new();
        
        let edge_positions: Vec<(HalfEdgeId, [f64; 3], [f64; 3])> = still_unpaired_after_retry.iter()
            .filter_map(|&he_id| {
                let he_data = draft.arena().get_half_edge(he_id).ok()?;
                let origin = he_data.origin();
                let next_he = he_data.next();
                let dest = draft.arena().get_half_edge(next_he).ok()?.origin();
                let p_o = geom.get_vertex_position(origin)?;
                let p_d = geom.get_vertex_position(dest)?;
                Some((he_id, *p_o, *p_d))
            })
            .collect();

        let tol_sq: f64 = weld_tolerance_sq;

        for i in 0..edge_positions.len() {
            let (he_a, o_a, d_a) = edge_positions[i];
            if position_paired.contains(&he_a.index()) { continue; }

            let mut best_match: Option<(HalfEdgeId, f64)> = None;

            for j in 0..edge_positions.len() {
                if i == j { continue; }
                let (he_b, o_b, d_b) = edge_positions[j];
                if position_paired.contains(&he_b.index()) { continue; }

                let face_a = draft.arena().get_half_edge(he_a).map(|d| d.face()).ok();
                let face_b = draft.arena().get_half_edge(he_b).map(|d| d.face()).ok();
                if face_a.is_some() && face_a == face_b { continue; }

                let dx_od = o_a[0]-d_b[0]; let dy_od = o_a[1]-d_b[1]; let dz_od = o_a[2]-d_b[2];
                let dx_do = d_a[0]-o_b[0]; let dy_do = d_a[1]-o_b[1]; let dz_do = d_a[2]-o_b[2];
                let dist_sq = dx_od*dx_od + dy_od*dy_od + dz_od*dz_od
                            + dx_do*dx_do + dy_do*dy_do + dz_do*dz_do;

                if dist_sq <= tol_sq {
                    match best_match {
                        None => best_match = Some((he_b, dist_sq)),
                        Some((_, bd)) if dist_sq < bd => best_match = Some((he_b, dist_sq)),
                        _ => {}
                    }
                }
            }

            if let Some((he_b, _)) = best_match {
                draft.arena_mut().get_half_edge_mut(he_a)?.set_twin(he_b);
                draft.arena_mut().get_half_edge_mut(he_b)?.set_twin(he_a);
                position_paired.insert(he_a.index());
                position_paired.insert(he_b.index());

                let mut decision = TracedDecision::new(
                    DecisionId(he_a.index() as u64),
                    DecisionKind::PolicyApplied { policy: forge_core::PolicyKind::CoincidentGeometry, default_used: true },
                    DecisionTier::NearBoundary,
                    0.8,
                    DecisionContext::Degeneracy { 
                        description: format!("Stitched {} <-> {} (position fallback)", he_a, he_b) 
                    },
                );
                decision.set_entity_scope(EntityRef::new("HalfEdge", he_a.index()));
                ctx.get_decision_log_mut().record(decision);
            }
        }

        let final_unpaired: Vec<HalfEdgeId> = still_unpaired_after_retry.iter()
            .filter(|he_id| !position_paired.contains(&he_id.index()))
            .copied()
            .collect();

        if !final_unpaired.is_empty() {
            eprintln!("=== STITCH FAILURE: {} unpaired halfedges ===", final_unpaired.len());
            for &he_id in &final_unpaired {
                let he_data = draft.arena().get_half_edge(he_id)?;
                let origin = he_data.origin();
                let next_he = he_data.next();
                let dest = draft.arena().get_half_edge(next_he)?.origin();
                let twin_id = he_data.twin();
                let twin_status = if twin_id == he_id {
                    "self-twin".to_string()
                } else if let Ok(tw) = draft.arena().get_half_edge(twin_id) {
                    format!("twin={} face={}", twin_id, tw.face())
                } else {
                    format!("twin={} INVALID", twin_id)
                };
                let p_o = geom.get_vertex_position(origin).map(|p| format!("{:.6e},{:.6e},{:.6e}", p[0], p[1], p[2])).unwrap_or_default();
                let p_d = geom.get_vertex_position(dest).map(|p| format!("{:.6e},{:.6e},{:.6e}", p[0], p[1], p[2])).unwrap_or_default();
                eprintln!("  he={} : {} -> {} (face={}) [{}] pos=[{}]->[{}]", he_id, origin, dest, he_data.face(), twin_status, p_o, p_d);

                let face_id = he_data.face();
                eprintln!("    Face {} geometry:", face_id);
                if let Ok(edges) = forge_topo::traverse::FaceEdgeIterator::new(draft.arena(), face_id) {
                    for ehe_res in edges {
                        if let Ok(ehe) = ehe_res {
                            if let Ok(edata) = draft.arena().get_half_edge(ehe) {
                                if let Some(vpos) = geom.get_vertex_position(edata.origin()) {
                                    eprintln!("      Vertex {}: [{:e}, {:e}, {:e}]", edata.origin(), vpos[0], vpos[1], vpos[2]);
                                }
                            }
                        }
                    }
                }
            }
            return Err(KernelError::TopologyViolation {
                err: forge_core::TopologyError::MissingTwin {
                    halfedge_index: final_unpaired[0].index(),
                },
                context: Some(forge_core::ErrorContext {
                    scope: forge_core::ErrorScope::Global,
                    suggested_fixes: Vec::new(),
                    detail: format!(
                        "{} halfedges remain unpaired after stitching (first: {})",
                        final_unpaired.len(),
                        final_unpaired[0],
                    ),
                }),
            });
        }
    }

    Ok(())
}

/// Select the best twin candidate using radial angle sorting.
///
/// When multiple reverse halfedges exist for the same directed edge,
/// picks the one whose face normal is closest (smallest angle) to
/// the source halfedge's face normal when measured around the edge axis.
/// Falls back to index-based ordering for determinism if geometry is missing.
fn select_best_twin(
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
            let dot = sn[0] * cn[0] + sn[1] * cn[1] + sn[2] * cn[2];
            if dot > best_dot || (dot == best_dot && cand.index() < best.index()) {
                best_dot = dot;
                best = cand;
            }
        }
    }

    best
}
