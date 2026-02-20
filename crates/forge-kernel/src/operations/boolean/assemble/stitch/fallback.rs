//! Position-based and single-vertex-match fallback stitching (passes 3 and 4).
//!
//! DOMAIN: When index-based stitching fails (duplicate vertices at same position
//! from re-used boolean results), match by geometric endpoint positions.
//! Delegates spatial matching to `forge_geom::spatial::edge_match::EdgeMatcher`.

use std::collections::HashSet;
use forge_core::KernelError;
use forge_core::result::{TracedDecision, DecisionId, DecisionKind, DecisionTier, DecisionContext, EntityRef};
use forge_geom::spatial::edge_match::{DirectedEdge, EdgeMatcher};
use forge_topo::handles::HalfEdgeId;
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
    let stitch_tol_sq: f64 = weld_tolerance_sq * 10000.0;

    let directed_edges = build_directed_edges(draft, geom, still_unpaired);
    let id_to_he: std::collections::BTreeMap<u32, HalfEdgeId> = still_unpaired
        .iter()
        .map(|&he| (he.index(), he))
        .collect();

    let matcher = EdgeMatcher::new(directed_edges, stitch_tol_sq);
    let full_matches = matcher.find_full_matches();

    let mut position_paired: HashSet<u32> = HashSet::new();

    for m in &full_matches {
        if let (Some(&he_a), Some(&he_b)) = (id_to_he.get(&m.edge_a), id_to_he.get(&m.edge_b)) {
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

    let directed_edges = build_directed_edges_with_indices(draft, geom, &still_unpaired_4);
    let id_to_he: std::collections::BTreeMap<u32, HalfEdgeId> = still_unpaired_4
        .iter()
        .map(|&he| (he.index(), he))
        .collect();

    let matcher = EdgeMatcher::new(directed_edges, stitch_tol_sq);
    let sv_matches = matcher.find_single_vertex_matches();

    for m in &sv_matches {
        if let (Some(&he_a), Some(&he_b)) = (id_to_he.get(&m.edge_a), id_to_he.get(&m.edge_b)) {
            draft.arena_mut().get_half_edge_mut(he_a)?.set_twin(he_b);
            draft.arena_mut().get_half_edge_mut(he_b)?.set_twin(he_a);
            position_paired.insert(he_a.index());
            position_paired.insert(he_b.index());

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

    Ok(())
}

/// Build DirectedEdge entries without vertex indices (for pass 3).
fn build_directed_edges(
    draft: &MutableDraft,
    geom: &GeometryStore,
    halfedges: &[HalfEdgeId],
) -> Vec<DirectedEdge> {
    halfedges.iter()
        .filter_map(|&he_id| {
            let he_data = draft.arena().get_half_edge(he_id).ok()?;
            let origin = he_data.origin();
            let next_he = he_data.next();
            let dest = draft.arena().get_half_edge(next_he).ok()?.origin();
            let p_o = geom.get_vertex_position(origin)?;
            let p_d = geom.get_vertex_position(dest)?;
            Some(DirectedEdge {
                id: he_id.index(),
                group: Some(he_data.face().index()),
                origin_index: None,
                dest_index: None,
                origin: *p_o,
                dest: *p_d,
            })
        })
        .collect()
}

/// Build DirectedEdge entries with vertex indices (for pass 4).
fn build_directed_edges_with_indices(
    draft: &MutableDraft,
    geom: &GeometryStore,
    halfedges: &[HalfEdgeId],
) -> Vec<DirectedEdge> {
    halfedges.iter()
        .filter_map(|&he_id| {
            let he_data = draft.arena().get_half_edge(he_id).ok()?;
            let origin = he_data.origin();
            let next_he = he_data.next();
            let dest = draft.arena().get_half_edge(next_he).ok()?.origin();
            let p_o = geom.get_vertex_position(origin)?;
            let p_d = geom.get_vertex_position(dest)?;
            Some(DirectedEdge {
                id: he_id.index(),
                group: Some(he_data.face().index()),
                origin_index: Some(origin.index()),
                dest_index: Some(dest.index()),
                origin: *p_o,
                dest: *p_d,
            })
        })
        .collect()
}
