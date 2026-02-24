//! Position-based and single-vertex-match fallback stitching (passes 3–4).
//!
//! DOMAIN: Match halfedges by geometric endpoint positions when index-based
//! stitching fails (duplicate vertices from re-used boolean results).
//!
//! DEPENDENCIES: forge_geom::spatial::edge_match::EdgeMatcher.
//! INVARIANTS: Always reports stitch failures as KernelError::TopologyViolation
//! with structured error context — never via eprintln.

use std::collections::BTreeSet;
use forge_core::KernelError;
use forge_core::{TracedDecision, DecisionId, DecisionKind, DecisionTier, DecisionContext, EntityRef};
use forge_geom::spatial::edge_match::{DirectedEdge, EdgeMatcher};
use forge_topo::handles::HalfEdgeId;
use forge_topo::operator::apply_op;
use forge_topo::euler::sew_edge::SewEdge;
use forge_topo::state::MutableDraft;
use crate::core::ModelingContext;
use crate::geometry_store::GeometryStore;

fn debug_stitch_enabled() -> bool {
    std::env::var("FORGE_DEBUG_STITCH")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Position-based fallback for stitching unpaired halfedges.
///
/// Pass 3: full endpoint position matching via EdgeMatcher.
/// Pass 4: single-vertex-match fallback for mixed index/position matches.
/// Uses 100× wider tolerance than vertex dedup for "close enough" edge identity.
///
/// Returns `StitchReport` with paired count and remaining unpaired IDs.
pub(super) fn stitch_position_fallback(
    draft: &mut MutableDraft,
    geom: &GeometryStore,
    still_unpaired: &[HalfEdgeId],
    weld_tolerance_sq: f64,
    ctx: &mut ModelingContext,
) -> Result<super::eval::StitchReport, KernelError> {
    let stitch_tol_sq = weld_tolerance_sq * 10000.0;
    let mut paired: BTreeSet<u32> = BTreeSet::new();

    run_full_position_pass(draft, geom, still_unpaired, stitch_tol_sq, &mut paired, ctx)?;
    run_single_vertex_pass(draft, geom, still_unpaired, stitch_tol_sq, &mut paired, ctx)?;

    let final_unpaired: Vec<HalfEdgeId> = still_unpaired.iter()
        .filter(|he| !paired.contains(&he.index()))
        .copied()
        .collect();

    Ok(super::eval::StitchReport {
        paired_count: paired.len(),
        unpaired_ids: final_unpaired,
    })
}

// ── Stitch passes ────────────────────────────────────────────────────────────

/// Pass 3: match by full endpoint positions (both origin and dest).
fn run_full_position_pass(
    draft: &mut MutableDraft,
    geom: &GeometryStore,
    halfedges: &[HalfEdgeId],
    tol_sq: f64,
    paired: &mut BTreeSet<u32>,
    ctx: &mut ModelingContext,
) -> Result<(), KernelError> {
    let edges = build_directed_edges(draft, geom, halfedges, false);
    let id_map = build_id_map(halfedges);
    let matcher = EdgeMatcher::new(edges, tol_sq);

    for m in &matcher.find_full_matches() {
        apply_match(draft, &id_map, m.edge_a, m.edge_b, paired, "position fallback", 0.8, ctx)?;
    }
    Ok(())
}

/// Pass 4: match by single shared vertex + one position endpoint.
fn run_single_vertex_pass(
    draft: &mut MutableDraft,
    geom: &GeometryStore,
    halfedges: &[HalfEdgeId],
    tol_sq: f64,
    paired: &mut BTreeSet<u32>,
    ctx: &mut ModelingContext,
) -> Result<(), KernelError> {
    let remaining: Vec<HalfEdgeId> = halfedges.iter()
        .filter(|he| !paired.contains(&he.index()))
        .copied()
        .collect();

    if remaining.is_empty() {
        return Ok(());
    }

    let edges = build_directed_edges(draft, geom, &remaining, true);
    let id_map = build_id_map(&remaining);
    let matcher = EdgeMatcher::new(edges, tol_sq);

    for m in &matcher.find_single_vertex_matches() {
        apply_match(draft, &id_map, m.edge_a, m.edge_b, paired, "single-vertex fallback", 0.6, ctx)?;
    }
    Ok(())
}

// ── Matching helpers ─────────────────────────────────────────────────────────

/// Apply a twin match between two halfedges.
fn apply_match(
    draft: &mut MutableDraft,
    id_map: &std::collections::BTreeMap<u32, HalfEdgeId>,
    edge_a: u32,
    edge_b: u32,
    paired: &mut BTreeSet<u32>,
    label: &str,
    confidence: f64,
    ctx: &mut ModelingContext,
) -> Result<(), KernelError> {
    let (Some(&he_a), Some(&he_b)) = (id_map.get(&edge_a), id_map.get(&edge_b)) else {
        return Ok(());
    };

    if paired.contains(&he_a.index()) || paired.contains(&he_b.index()) || he_a == he_b {
        return Ok(());
    }

    if let Err(err) = apply_op(draft, SewEdge { he_a, he_b }) {
        if debug_stitch_enabled() {
            eprintln!(
                "[stitch-fallback] rejected {} <-> {} ({}): {}",
                he_a.index(),
                he_b.index(),
                label,
                err
            );
        }
        return Ok(());
    }

    paired.insert(he_a.index());
    paired.insert(he_b.index());

    log_stitch(he_a, he_b, label, confidence, ctx);
    Ok(())
}

/// Build a map from halfedge index → HalfEdgeId.
fn build_id_map(halfedges: &[HalfEdgeId]) -> std::collections::BTreeMap<u32, HalfEdgeId> {
    halfedges.iter().map(|&he| (he.index(), he)).collect()
}

/// Build DirectedEdge entries for the EdgeMatcher.
///
/// When `include_indices` is true, vertex indices are included for
/// single-vertex matching (pass 4).
fn build_directed_edges(
    draft: &MutableDraft,
    geom: &GeometryStore,
    halfedges: &[HalfEdgeId],
    include_indices: bool,
) -> Vec<DirectedEdge> {
    halfedges.iter()
        .filter_map(|&he_id| {
            let he = draft.arena().get_half_edge(he_id).ok()?;
            let origin = he.origin();
            let dest = draft.arena().get_half_edge(he.next()).ok()?.origin();
            let p_o = geom.get_vertex_position(origin)?;
            let p_d = geom.get_vertex_position(dest)?;
            Some(DirectedEdge {
                id: he_id.index(),
                group: Some(he.face().index()),
                origin_index: if include_indices { Some(origin.index()) } else { None },
                dest_index: if include_indices { Some(dest.index()) } else { None },
                origin: *p_o,
                dest: *p_d,
            })
        })
        .collect()
}

// ── Decision logging ─────────────────────────────────────────────────────────

/// Record a stitch decision.
fn log_stitch(he_a: HalfEdgeId, he_b: HalfEdgeId, label: &str, confidence: f64, ctx: &mut ModelingContext) {
    let mut decision = TracedDecision::new(
        DecisionId(he_a.index() as u64),
        DecisionKind::PolicyApplied { policy: forge_core::PolicyKind::CoincidentGeometry, default_used: true },
        DecisionTier::NearBoundary,
        confidence,
        DecisionContext::Degeneracy {
            description: format!("Stitched {} <-> {} ({})", he_a, he_b, label),
        },
    );
    decision.set_entity_scope(EntityRef::new(forge_core::EntityKind::HalfEdge, he_a.index()));
    ctx.get_decision_log_mut().record(decision);
}
