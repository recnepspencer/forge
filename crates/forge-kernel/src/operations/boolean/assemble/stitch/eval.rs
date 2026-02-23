//! Core twin-stitching logic.
//!
//! DOMAIN: Match half-edges by directed edge identity and set twin pointers.
//! DEPENDENCIES: fallback (position-based stitching), GeometryStore.
//! INVARIANTS: Two passes — exact vertex match first, then among remaining
//! unpaired edges. Position-based fallback handles geometric near-misses.
//! Returns `StitchReport` so callers decide if unpaired is acceptable.

use std::collections::{BTreeMap, BTreeSet};
use forge_core::KernelError;
use forge_core::{TracedDecision, DecisionId, DecisionKind, DecisionTier, DecisionContext, EntityRef};
use forge_topo::handles::{HalfEdgeId, VertexId};
use forge_topo::state::MutableDraft;
use crate::core::{ModelingContext, ArenaSnapshot, compute_topology_delta};
use crate::geometry_store::GeometryStore;

use super::fallback::stitch_position_fallback;

/// Structured result from stitching — callers decide if unpaired is acceptable.
pub struct StitchReport {
    /// Total halfedges that were paired in this pass.
    pub paired_count: usize,
    /// Halfedge IDs that remain unpaired after all passes.
    pub unpaired_ids: Vec<HalfEdgeId>,
}

impl StitchReport {
    /// All halfedges were successfully paired.
    pub fn is_fully_paired(&self) -> bool {
        self.unpaired_ids.is_empty()
    }

    /// Require all halfedges paired, or return an error with diagnostics.
    pub fn require_fully_paired(
        &self,
        draft: &MutableDraft,
        geom: &GeometryStore,
        ctx: &ModelingContext,
    ) -> Result<(), KernelError> {
        if self.is_fully_paired() {
            return Ok(());
        }
        Err(build_stitch_failure_error(&self.unpaired_ids, draft, geom, ctx))
    }
}

/// Stitch twin pointers by matching directed edges.
///
/// Pass 1: exact vertex-ID matching against all half-edges.
/// Pass 2: retry among remaining unpaired edges.
/// Fallback: position-based matching for geometric near-misses.
///
/// Returns `StitchReport` with paired count and unpaired IDs.
/// Callers decide whether unpaired is an error (closed shell expected)
/// or acceptable (disjoint shells in same arena).
pub fn stitch_twins(
    draft: &mut MutableDraft,
    all_he_ids: &[HalfEdgeId],
    geom: &GeometryStore,
    weld_tolerance_sq: f64,
    ctx: &mut ModelingContext,
) -> Result<StitchReport, KernelError> {
    let (forward_map, zero_length) = build_edge_map(draft, all_he_ids)?;

    let paired = run_stitch_pass(
        draft, geom, all_he_ids, &forward_map, &BTreeSet::new(), &zero_length,
        DecisionKind::Exact, ctx,
    )?;

    let unpaired_ids = collect_unpaired(all_he_ids, &paired, &zero_length);
    if unpaired_ids.is_empty() {
        return Ok(StitchReport {
            paired_count: paired.len(),
            unpaired_ids: Vec::new(),
        });
    }

    let unpaired_map = build_directed_map(draft, &unpaired_ids)?;
    let paired_retry = run_stitch_pass(
        draft, geom, &unpaired_ids, &unpaired_map, &BTreeSet::new(), &zero_length,
        DecisionKind::PolicyApplied { policy: forge_core::PolicyKind::CoincidentGeometry, default_used: true },
        ctx,
    )?;

    let still_unpaired: Vec<HalfEdgeId> = unpaired_ids.iter()
        .filter(|id| !paired_retry.contains(&id.index()))
        .copied()
        .collect();

    let total_paired = paired.len() + paired_retry.len();

    if !still_unpaired.is_empty() {
        let pre_snapshot = ArenaSnapshot::capture(draft.arena());

        let fallback_result = stitch_position_fallback(
            draft, geom, &still_unpaired, weld_tolerance_sq, ctx,
        );

        let delta = compute_topology_delta(&pre_snapshot, draft.arena());
        if !delta.is_empty() {
            let mut decision = TracedDecision::new(
                DecisionId(still_unpaired.len() as u64),
                DecisionKind::Forced {
                    reason: format!("Position fallback stitched {} unpaired HEs", still_unpaired.len()),
                },
                DecisionTier::PolicyApplied,
                1.0,
                DecisionContext::Degeneracy {
                    description: format!(
                        "Stitch fallback created {} V, {} HE, {} F",
                        delta.created_vertices.len(),
                        delta.created_halfedges.len(),
                        delta.created_faces.len(),
                    ),
                },
            );
            decision.set_topology_delta(delta);
            ctx.get_decision_log_mut().record(decision);
        }

        match fallback_result {
            Ok(fallback_report) => {
                return Ok(StitchReport {
                    paired_count: total_paired + fallback_report.paired_count,
                    unpaired_ids: fallback_report.unpaired_ids,
                });
            }
            Err(_) => {
                return Ok(StitchReport {
                    paired_count: total_paired,
                    unpaired_ids: still_unpaired,
                });
            }
        }
    }

    Ok(StitchReport {
        paired_count: total_paired,
        unpaired_ids: Vec::new(),
    })
}

/// Build a forward map from (origin, dest) → Vec<HalfEdgeId>, filtering zero-length edges.
fn build_edge_map(
    draft: &MutableDraft,
    all_he_ids: &[HalfEdgeId],
) -> Result<(BTreeMap<(u32, u32), Vec<HalfEdgeId>>, BTreeSet<u32>), KernelError> {
    let mut forward_map: BTreeMap<(u32, u32), Vec<HalfEdgeId>> = BTreeMap::new();
    let mut zero_length: BTreeSet<u32> = BTreeSet::new();

    for &he_id in all_he_ids {
        let (origin, dest) = get_edge_endpoints(draft, he_id)?;
        if origin == dest {
            zero_length.insert(he_id.index());
        } else {
            forward_map.entry((origin.index(), dest.index())).or_default().push(he_id);
        }
    }
    Ok((forward_map, zero_length))
}

/// Build a directed map for a subset of half-edge IDs.
fn build_directed_map(
    draft: &MutableDraft,
    ids: &[HalfEdgeId],
) -> Result<BTreeMap<(u32, u32), Vec<HalfEdgeId>>, KernelError> {
    let mut map: BTreeMap<(u32, u32), Vec<HalfEdgeId>> = BTreeMap::new();
    for &he_id in ids {
        let (origin, dest) = get_edge_endpoints(draft, he_id)?;
        map.entry((origin.index(), dest.index())).or_default().push(he_id);
    }
    Ok(map)
}

/// Run one stitch pass: match each half-edge against reverse candidates.
fn run_stitch_pass(
    draft: &mut MutableDraft,
    geom: &GeometryStore,
    candidates: &[HalfEdgeId],
    edge_map: &BTreeMap<(u32, u32), Vec<HalfEdgeId>>,
    already_paired: &BTreeSet<u32>,
    zero_length: &BTreeSet<u32>,
    decision_kind: DecisionKind,
    ctx: &mut ModelingContext,
) -> Result<BTreeSet<u32>, KernelError> {
    let mut paired = already_paired.clone();

    for &he_id in candidates {
        let is_eligible = !paired.contains(&he_id.index()) && !zero_length.contains(&he_id.index());

        if is_eligible {
            let he_face = draft.arena().get_half_edge(he_id)?.face();
            let (origin, dest) = get_edge_endpoints(draft, he_id)?;
            let reverse_key = (dest.index(), origin.index());

            if let Some(reverse_candidates) = edge_map.get(&reverse_key) {
                let unpaired: Vec<HalfEdgeId> = reverse_candidates.iter()
                    .filter(|&&c| {
                        c != he_id
                            && !paired.contains(&c.index())
                            && draft.arena().get_half_edge(c).map(|d| d.face() != he_face).unwrap_or(false)
                    })
                    .copied()
                    .collect();

                if !unpaired.is_empty() {
                    let best = if unpaired.len() == 1 {
                        unpaired[0]
                    } else {
                        select_best_twin(draft, geom, he_id, &unpaired)
                    };

                    draft.arena_mut().get_half_edge_mut(he_id)?.set_twin(best);
                    draft.arena_mut().get_half_edge_mut(best)?.set_twin(he_id);
                    paired.insert(he_id.index());
                    paired.insert(best.index());

                    log_stitch_decision(he_id, best, &decision_kind, ctx);
                }
            }
        }
    }

    Ok(paired)
}

/// Get origin and destination vertices of a half-edge.
fn get_edge_endpoints(draft: &MutableDraft, he_id: HalfEdgeId) -> Result<(VertexId, VertexId), KernelError> {
    let he_data = draft.arena().get_half_edge(he_id)?;
    let origin = he_data.origin();
    let dest = draft.arena().get_half_edge(he_data.next())?.origin();
    Ok((origin, dest))
}

/// Collect half-edge IDs that are not yet paired and not zero-length.
fn collect_unpaired(all: &[HalfEdgeId], paired: &BTreeSet<u32>, zero: &BTreeSet<u32>) -> Vec<HalfEdgeId> {
    all.iter()
        .filter(|id| !paired.contains(&id.index()) && !zero.contains(&id.index()))
        .copied()
        .collect()
}

/// Log a twin-stitch decision.
fn log_stitch_decision(he_a: HalfEdgeId, he_b: HalfEdgeId, kind: &DecisionKind, ctx: &mut ModelingContext) {
    let suffix = match kind {
        DecisionKind::PolicyApplied { .. } => " (retry)",
        _ => "",
    };
    let mut decision = TracedDecision::new(
        DecisionId(he_a.index() as u64),
        kind.clone(),
        DecisionTier::Deterministic,
        1.0,
        DecisionContext::Degeneracy {
            description: format!("Stitched {} <-> {}{}", he_a, he_b, suffix),
        },
    );
    decision.set_entity_scope(EntityRef::new(forge_core::EntityKind::HalfEdge, he_a.index()));
    ctx.get_decision_log_mut().record(decision);
}

/// Select the best twin candidate using face normal dot product.
pub fn select_best_twin(
    draft: &MutableDraft,
    geom: &GeometryStore,
    source_he: HalfEdgeId,
    candidates: &[HalfEdgeId],
) -> HalfEdgeId {
    let source_normal = draft.arena().get_half_edge(source_he)
        .ok()
        .map(|d| d.face())
        .and_then(|f| geom.get_face_plane(f))
        .map(|p| p.raw_normal());

    let Some(sn) = source_normal else {
        return *candidates.iter().min_by_key(|c| c.index()).unwrap_or(&candidates[0]);
    };

    let mut best = candidates[0];
    let mut best_dot = f64::NEG_INFINITY;

    for &cand in candidates {
        let cand_normal = draft.arena().get_half_edge(cand)
            .ok()
            .map(|d| d.face())
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

/// Build a structured error for remaining unpaired halfedges.
///
/// Includes per-entity decision ancestry and a 2-ring extracted region
/// for each unpaired halfedge, enabling root-cause tracing and local
/// geometry reconstruction.
fn build_stitch_failure_error(
    unpaired: &[HalfEdgeId],
    draft: &MutableDraft,
    geom: &GeometryStore,
    ctx: &ModelingContext,
) -> KernelError {
    let mut detail_lines: Vec<String> = Vec::new();
    detail_lines.push(format!(
        "{} halfedges remain unpaired after stitching", unpaired.len(),
    ));

    let decision_log = ctx.get_decision_log();
    let max_report = unpaired.len().min(5);

    for &he_id in unpaired.iter().take(max_report) {
        let he_ref = EntityRef::new(forge_core::EntityKind::HalfEdge, he_id.index());
        let face_index = draft.arena().get_half_edge(he_id)
            .map(|he| he.face().index())
            .unwrap_or(u32::MAX);
        let face_ref = EntityRef::new(forge_core::EntityKind::Face, face_index);

        let related_decisions: Vec<String> = decision_log.decisions()
            .filter(|d| {
                d.get_entity_scope()
                    .map(|e| *e == he_ref || *e == face_ref)
                    .unwrap_or(false)
            })
            .map(|d| format!(
                "    [{}] {} margin={:.2e} | {}",
                d.get_tier(), d.get_kind(), d.get_margin(), d.get_context(),
            ))
            .collect();

        detail_lines.push(format!("  HalfEdge#{} (Face#{})", he_id.index(), face_index));
        if related_decisions.is_empty() {
            detail_lines.push("    (no entity-scoped decisions found)".to_string());
        } else {
            for line in related_decisions {
                detail_lines.push(line);
            }
        }

        let face_id = forge_topo::handles::FaceId::from_raw_parts(face_index, 0);
        if let Ok(region) = crate::analysis::region_extractor::extract_n_ring(
            draft.arena(), geom, face_id, 2,
        ) {
            detail_lines.push(format!(
                "  2-ring: {}F {}HE {}V",
                region.face_count(), region.half_edge_count(), region.vertex_count(),
            ));
            for (&fidx, plane) in region.get_face_planes() {
                let n = plane.get_normal();
                detail_lines.push(format!(
                    "    Face#{}: n=[{:.2},{:.2},{:.2}] d={:.2}",
                    fidx, n[0], n[1], n[2], plane.get_offset(),
                ));
            }
        }
    }

    if unpaired.len() > max_report {
        detail_lines.push(format!("  ... and {} more", unpaired.len() - max_report));
    }

    KernelError::TopologyViolation {
        err: forge_core::TopologyError::MissingTwin {
            halfedge_index: unpaired[0].index(),
        },
        context: Some(forge_core::ErrorContext {
            scope: forge_core::ErrorScope::Global,
            suggested_fixes: Vec::new(),
            detail: detail_lines.join("\n"),
        }),
    }
}
