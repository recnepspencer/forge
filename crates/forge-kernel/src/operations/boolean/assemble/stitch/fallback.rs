//! Position-based and single-vertex-match fallback stitching (passes 3 and 4).
//!
//! DOMAIN: When index-based stitching fails (duplicate vertices at same position
//! from re-used boolean results), match by geometric endpoint positions.

use std::collections::HashSet;
use forge_core::KernelError;
use forge_core::result::{TracedDecision, DecisionId, DecisionKind, DecisionTier, DecisionContext, EntityRef};
use forge_topo::handles::{HalfEdgeId, VertexId};
use forge_topo::state::MutableDraft;
use crate::core::ModelingContext;
use crate::geometry_store::GeometryStore;

/// Position-based fallback for stitching unpaired halfedges.
///
/// Uses geometric endpoint positions to match edges when vertex indices
/// diverge (passes 3 and 4). Uses a stitch-specific tolerance 100x wider
/// than vertex dedup since stitch only needs "close enough" to identify
/// the same geometric edge.
pub(super) fn stitch_position_fallback(
    draft: &mut MutableDraft,
    geom: &GeometryStore,
    still_unpaired: &[HalfEdgeId],
    weld_tolerance_sq: f64,
    ctx: &mut ModelingContext,
) -> Result<(), KernelError> {
    let mut position_paired: HashSet<u32> = HashSet::new();

    let edge_positions: Vec<(HalfEdgeId, [f64; 3], [f64; 3])> = still_unpaired.iter()
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

    let stitch_tol_sq: f64 = weld_tolerance_sq * 10000.0;

    for i in 0..edge_positions.len() {
        let (he_a, o_a, d_a) = edge_positions[i];
        if !position_paired.contains(&he_a.index()) {
            let mut best_match: Option<(HalfEdgeId, f64)> = None;

            for j in 0..edge_positions.len() {
                if i != j {
                    let (he_b, o_b, d_b) = edge_positions[j];
                    if !position_paired.contains(&he_b.index()) {
                        let face_a = draft.arena().get_half_edge(he_a).map(|d| d.face()).ok();
                        let face_b = draft.arena().get_half_edge(he_b).map(|d| d.face()).ok();
                        let same_face = face_a.is_some() && face_a == face_b;

                        if !same_face {
                            let diff_od = [o_a[0]-d_b[0], o_a[1]-d_b[1], o_a[2]-d_b[2]];
                            let diff_do = [d_a[0]-o_b[0], d_a[1]-o_b[1], d_a[2]-o_b[2]];
                            let dist_sq_od = forge_math::linalg::norm_sq(diff_od);
                            let dist_sq_do = forge_math::linalg::norm_sq(diff_do);

                            if dist_sq_od <= stitch_tol_sq && dist_sq_do <= stitch_tol_sq {
                                let total = dist_sq_od + dist_sq_do;
                                match best_match {
                                    None => best_match = Some((he_b, total)),
                                    Some((_, bd)) if total < bd => best_match = Some((he_b, total)),
                                    _ => {}
                                }
                            }
                        }
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
    }

    stitch_single_vertex_fallback(draft, geom, still_unpaired, &mut position_paired, stitch_tol_sq, ctx)?;

    let final_unpaired: Vec<HalfEdgeId> = still_unpaired.iter()
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

    Ok(())
}

/// Fourth pass: single-vertex-match fallback.
///
/// Handles edges where one vertex matched by index but the other
/// didn't (created independently in different split phases).
fn stitch_single_vertex_fallback(
    draft: &mut MutableDraft,
    geom: &GeometryStore,
    still_unpaired: &[HalfEdgeId],
    position_paired: &mut HashSet<u32>,
    stitch_tol_sq: f64,
    ctx: &mut ModelingContext,
) -> Result<(), KernelError> {
    let still_unpaired_4: Vec<HalfEdgeId> = still_unpaired.iter()
        .filter(|he_id| !position_paired.contains(&he_id.index()))
        .copied()
        .collect();

    if still_unpaired_4.is_empty() {
        return Ok(());
    }

    let mut pass4_paired: HashSet<u32> = HashSet::new();

    let edge_data_4: Vec<(HalfEdgeId, VertexId, VertexId, [f64; 3], [f64; 3])> = still_unpaired_4.iter()
        .filter_map(|&he_id| {
            let he_data = draft.arena().get_half_edge(he_id).ok()?;
            let origin = he_data.origin();
            let next_he = he_data.next();
            let dest = draft.arena().get_half_edge(next_he).ok()?.origin();
            let p_o = geom.get_vertex_position(origin)?;
            let p_d = geom.get_vertex_position(dest)?;
            Some((he_id, origin, dest, *p_o, *p_d))
        })
        .collect();

    for i in 0..edge_data_4.len() {
        let (he_a, orig_a, dest_a, o_a, d_a) = edge_data_4[i];
        if !pass4_paired.contains(&he_a.index()) {
            let mut best: Option<(HalfEdgeId, f64)> = None;

            for j in 0..edge_data_4.len() {
                if i != j {
                    let (he_b, orig_b, dest_b, o_b, d_b) = edge_data_4[j];
                    if !pass4_paired.contains(&he_b.index()) {
                        let face_a = draft.arena().get_half_edge(he_a).map(|d| d.face()).ok();
                        let face_b = draft.arena().get_half_edge(he_b).map(|d| d.face()).ok();
                        let same_face = face_a.is_some() && face_a == face_b;

                        if !same_face {
                            let origin_match = orig_a == dest_b;
                            let dest_match = dest_a == orig_b;

                            if origin_match && !dest_match {
                                let diff = [d_a[0]-o_b[0], d_a[1]-o_b[1], d_a[2]-o_b[2]];
                                let dsq = forge_math::linalg::norm_sq(diff);
                                if dsq <= stitch_tol_sq {
                                    match best {
                                        None => best = Some((he_b, dsq)),
                                        Some((_, bd)) if dsq < bd => best = Some((he_b, dsq)),
                                        _ => {}
                                    }
                                }
                            } else if !origin_match && dest_match {
                                let diff = [o_a[0]-d_b[0], o_a[1]-d_b[1], o_a[2]-d_b[2]];
                                let dsq = forge_math::linalg::norm_sq(diff);
                                if dsq <= stitch_tol_sq {
                                    match best {
                                        None => best = Some((he_b, dsq)),
                                        Some((_, bd)) if dsq < bd => best = Some((he_b, dsq)),
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if let Some((he_b, _)) = best {
                draft.arena_mut().get_half_edge_mut(he_a)?.set_twin(he_b);
                draft.arena_mut().get_half_edge_mut(he_b)?.set_twin(he_a);
                pass4_paired.insert(he_a.index());
                pass4_paired.insert(he_b.index());

                let mut decision = TracedDecision::new(
                    DecisionId(he_a.index() as u64),
                    DecisionKind::PolicyApplied { policy: forge_core::PolicyKind::CoincidentGeometry, default_used: true },
                    DecisionTier::NearBoundary,
                    0.6,
                    DecisionContext::Degeneracy { 
                        description: format!("Stitched {} <-> {} (single-vertex fallback)", he_a, he_b) 
                    },
                );
                decision.set_entity_scope(EntityRef::new("HalfEdge", he_a.index()));
                ctx.get_decision_log_mut().record(decision);
            }
        }
    }

    position_paired.extend(pass4_paired);

    Ok(())
}
