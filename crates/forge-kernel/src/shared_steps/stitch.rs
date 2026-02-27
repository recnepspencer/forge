//! Top-level topology stitching step.
//!
//! DOMAIN: Pairs newly generated halfedges into face loops and records twin decisions.

use std::collections::{BTreeMap, BTreeSet};

use forge_core::{
    KernelError, DecisionContext, DecisionId, DecisionKind, DecisionTier, EntityRef, TracedDecision
};
use forge_topo::state::MutableDraft;
use forge_topo::handles::HalfEdgeId;
use forge_topo::operator::apply_op;
use forge_topo::euler::sew_edge::SewEdge;
use crate::geom::{fuzzy_match_edges, DirectedEdge, FuzzyMatchMode, select_best_radial_match};

use crate::core::ModelingContext;
use crate::core::macros::declare_step;
use crate::geometry_state::GeometryState;
use crate::shared_ops::stitch::StitchReport;

fn debug_stitch_enabled() -> bool {
    std::env::var("FORGE_DEBUG_STITCH")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

declare_step! {
    /// Stitch twins for newly assembled faces and resolve non-manifold junctions.
    pub struct StitchTwinsStep<'a> {
        draft: &'a mut MutableDraft,
        all_he_ids: &'a [HalfEdgeId],
        geom: &'a GeometryState,
        weld_tolerance_sq: f64,
    }

    fn execute(self, ctx: &mut ModelingContext) -> Result<StitchReport, KernelError> {
        let draft = self.draft;
        let geom = self.geom;
        let all_he_ids = self.all_he_ids;

        let edge_map_result = forge_topo::topology::queries::edge_map::build_edge_map(draft.arena(), all_he_ids)?;
        let forward_map = edge_map_result.forward_map;
        let zero_length = edge_map_result.zero_length;

        let paired = run_stitch_pass(
            draft,
            geom,
            all_he_ids,
            &forward_map,
            &BTreeSet::new(),
            &zero_length,
            DecisionKind::Exact,
            ctx,
        )?;

        let unpaired_ids = collect_unpaired(all_he_ids, &paired, &zero_length);
        if unpaired_ids.is_empty() {
            return Ok(StitchReport {
                paired_count: paired.len(),
                unpaired_ids: Vec::new(),
            });
        }

        let unpaired_map = forge_topo::topology::queries::edge_map::build_directed_map(draft.arena(), &unpaired_ids)?;
        let paired_retry = run_stitch_pass(
            draft,
            geom,
            &unpaired_ids,
            &unpaired_map,
            &BTreeSet::new(),
            &zero_length,
            DecisionKind::PolicyApplied {
                policy: forge_core::PolicyKind::CoincidentGeometry,
                default_used: true,
            },
            ctx,
        )?;

        let still_unpaired: Vec<HalfEdgeId> = unpaired_ids
            .iter()
            .filter(|id| !paired_retry.contains(&id.index()))
            .copied()
            .collect();

        let total_paired = paired.len() + paired_retry.len();

        if !still_unpaired.is_empty() {
            let pre_snapshot = crate::core::ArenaSnapshot::capture(draft.arena());

            let fallback_result = stitch_position_fallback(
                draft,
                geom,
                &still_unpaired,
                self.weld_tolerance_sq,
                ctx,
            );

            let delta = crate::core::compute_topology_delta(&pre_snapshot, draft.arena());
            if !delta.is_empty() {
                let mut decision = TracedDecision::new(
                    DecisionId(still_unpaired.len() as u64),
                    DecisionKind::Forced {
                        reason: format!(
                            "Position fallback stitched {} unpaired HEs",
                            still_unpaired.len()
                        ),
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

            if let Ok(fallback_report) = fallback_result {
                return Ok(StitchReport {
                    paired_count: total_paired + fallback_report.paired_count,
                    unpaired_ids: fallback_report.unpaired_ids,
                });
            } else {
                return Ok(StitchReport {
                    paired_count: total_paired,
                    unpaired_ids: still_unpaired,
                });
            }
        }

        Ok(StitchReport {
            paired_count: total_paired,
            unpaired_ids: Vec::new(),
        })
    }
}

pub fn stitch_twins(
    draft: &mut MutableDraft,
    all_he_ids: &[HalfEdgeId],
    geom: &GeometryState,
    weld_tolerance_sq: f64,
    ctx: &mut ModelingContext,
) -> Result<StitchReport, KernelError> {
    StitchTwinsStep {
        draft,
        all_he_ids,
        geom,
        weld_tolerance_sq,
    }
    .execute(ctx)
}

fn run_stitch_pass(
    draft: &mut MutableDraft,
    geom: &GeometryState,
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
            let edge_id = draft.arena().get_half_edge(he_id)?.edge();
            let (origin, dest) = draft.arena().get_edge_endpoints(edge_id)?;
            let reverse_key = (dest.index(), origin.index());

            if let Some(reverse_candidates) = edge_map.get(&reverse_key) {
                let unpaired: Vec<HalfEdgeId> = reverse_candidates
                    .iter()
                    .filter(|&&c| {
                        c != he_id
                            && !paired.contains(&c.index())
                            && draft
                                .arena()
                                .get_half_edge(c)
                                .map(|d| d.face() != he_face)
                                .unwrap_or(false)
                    })
                    .copied()
                    .collect();

                if debug_stitch_enabled() && unpaired.is_empty() {
                    let reverse_summary: Vec<String> = reverse_candidates
                        .iter()
                        .map(|&c| {
                            let face = draft
                                .arena()
                                .get_half_edge(c)
                                .map(|d| d.face().index())
                                .unwrap_or(u32::MAX);
                            let paired_flag = paired.contains(&c.index());
                            format!("HE#{}(F#{},paired={})", c.index(), face, paired_flag)
                        })
                        .collect();
                    eprintln!(
                        "[stitch] HE#{} F#{} {:?}->{:?} reverse {:?} exists but no eligible candidates: {}",
                        he_id.index(),
                        he_face.index(),
                        origin.index(),
                        dest.index(),
                        reverse_key,
                        reverse_summary.join(", ")
                    );
                }

                if !unpaired.is_empty() {
                    let best = if unpaired.len() == 1 {
                        unpaired[0]
                    } else {
                        select_best_twin(draft, geom, he_id, &unpaired)
                    };

                    let _ = apply_op(
                        draft,
                        SewEdge {
                            he_a: he_id,
                            he_b: best,
                        },
                    )?;
                    paired.insert(he_id.index());
                    paired.insert(best.index());

                    log_stitch_decision(he_id, best, &decision_kind, ctx);
                }
            }
            if debug_stitch_enabled() && !edge_map.contains_key(&reverse_key) {
                let same_key = (origin.index(), dest.index());
                let same_summary = edge_map
                    .get(&same_key)
                    .map(|v| {
                        v.iter()
                            .map(|&c| {
                                let face = draft
                                    .arena()
                                    .get_half_edge(c)
                                    .map(|d| d.face().index())
                                    .unwrap_or(u32::MAX);
                                format!("HE#{}(F#{})", c.index(), face)
                            })
                            .collect::<Vec<_>>()
                            .join(", ")
                    })
                    .unwrap_or_else(|| "<none>".to_string());
                eprintln!(
                    "[stitch] HE#{} F#{} {:?}->{:?} has no reverse key {:?}; same-dir={}",
                    he_id.index(),
                    he_face.index(),
                    origin.index(),
                    dest.index(),
                    reverse_key,
                    same_summary
                );
            }
        }
    }

    Ok(paired)
}

fn collect_unpaired(
    all: &[HalfEdgeId],
    paired: &BTreeSet<u32>,
    zero: &BTreeSet<u32>,
) -> Vec<HalfEdgeId> {
    all.iter()
        .filter(|id| !paired.contains(&id.index()) && !zero.contains(&id.index()))
        .copied()
        .collect()
}

fn log_stitch_decision(
    he_a: HalfEdgeId,
    he_b: HalfEdgeId,
    kind: &DecisionKind,
    ctx: &mut ModelingContext,
) {
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
    decision.set_entity_scope(EntityRef::new(
        forge_core::EntityKind::HalfEdge,
        he_a.index(),
    ));
    ctx.get_decision_log_mut().record(decision);
}

pub fn select_best_twin(
    draft: &MutableDraft,
    geom: &GeometryState,
    source_he: HalfEdgeId,
    candidates: &[HalfEdgeId],
) -> HalfEdgeId {
    let source_normal = draft
        .arena()
        .get_half_edge(source_he)
        .ok()
        .map(|d| d.face())
        .and_then(|f| geom.get_face_plane(f))
        .map(|p| p.raw_normal());

    let Some(sn) = source_normal else {
        return *candidates
            .iter()
            .min_by_key(|c| c.index())
            .unwrap_or(&candidates[0]);
    };

    let mut cad_normals = Vec::new();
    for &cand in candidates {
        let cand_normal = draft
            .arena()
            .get_half_edge(cand)
            .ok()
            .map(|d| d.face())
            .and_then(|f| geom.get_face_plane(f))
            .map(|p| p.raw_normal());

        if let Some(cn) = cand_normal {
            cad_normals.push((cand.index(), cn));
        }
    }

    if cad_normals.is_empty() {
        return *candidates
            .iter()
            .min_by_key(|c| c.index())
            .unwrap_or(&candidates[0]);
    }

    let best_id = select_best_radial_match(sn, &cad_normals);
    candidates.iter().find(|c| c.index() == best_id).copied().unwrap_or(candidates[0])
}

pub(super) fn stitch_position_fallback(
    draft: &mut MutableDraft,
    geom: &GeometryState,
    still_unpaired: &[HalfEdgeId],
    weld_tolerance_sq: f64,
    ctx: &mut ModelingContext,
) -> Result<StitchReport, KernelError> {
    let stitch_tol_sq = weld_tolerance_sq * 10000.0;
    let mut paired: BTreeSet<u32> = BTreeSet::new();

    run_full_position_pass(draft, geom, still_unpaired, stitch_tol_sq, &mut paired, ctx)?;
    run_single_vertex_pass(draft, geom, still_unpaired, stitch_tol_sq, &mut paired, ctx)?;

    let final_unpaired: Vec<HalfEdgeId> = still_unpaired
        .iter()
        .filter(|he| !paired.contains(&he.index()))
        .copied()
        .collect();

    Ok(StitchReport {
        paired_count: paired.len(),
        unpaired_ids: final_unpaired,
    })
}

fn run_full_position_pass(
    draft: &mut MutableDraft,
    geom: &GeometryState,
    halfedges: &[HalfEdgeId],
    tol_sq: f64,
    paired: &mut BTreeSet<u32>,
    ctx: &mut ModelingContext,
) -> Result<(), KernelError> {
    let edges = build_directed_edges(draft, geom, halfedges, false);
    let id_map = build_id_map(halfedges);
    let matches = fuzzy_match_edges(edges, tol_sq, FuzzyMatchMode::FullEndpoint);
    for m in &matches {
        apply_match(
            draft,
            &id_map,
            m.edge_a,
            m.edge_b,
            paired,
            "position fallback",
            0.8,
            ctx,
        )?;
    }
    Ok(())
}

fn run_single_vertex_pass(
    draft: &mut MutableDraft,
    geom: &GeometryState,
    halfedges: &[HalfEdgeId],
    tol_sq: f64,
    paired: &mut BTreeSet<u32>,
    ctx: &mut ModelingContext,
) -> Result<(), KernelError> {
    let remaining: Vec<HalfEdgeId> = halfedges
        .iter()
        .filter(|he| !paired.contains(&he.index()))
        .copied()
        .collect();

    if remaining.is_empty() {
        return Ok(());
    }

    let edges = build_directed_edges(draft, geom, &remaining, true);
    let id_map = build_id_map(&remaining);
    let matches = fuzzy_match_edges(edges, tol_sq, FuzzyMatchMode::SingleVertex);
    for m in &matches {
        apply_match(
            draft,
            &id_map,
            m.edge_a,
            m.edge_b,
            paired,
            "single-vertex fallback",
            0.6,
            ctx,
        )?;
    }
    Ok(())
}

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

fn build_id_map(halfedges: &[HalfEdgeId]) -> std::collections::BTreeMap<u32, HalfEdgeId> {
    halfedges.iter().map(|&he| (he.index(), he)).collect()
}

fn build_directed_edges(
    draft: &MutableDraft,
    geom: &GeometryState,
    halfedges: &[HalfEdgeId],
    include_indices: bool,
) -> Vec<DirectedEdge> {
    halfedges
        .iter()
        .filter_map(|&he_id| {
            let he = draft.arena().get_half_edge(he_id).ok()?;
            let origin = he.origin();
            let dest = draft.arena().get_half_edge(he.next()).ok()?.origin();
            let p_o = geom.get_vertex_position(origin)?;
            let p_d = geom.get_vertex_position(dest)?;
            Some(DirectedEdge {
                id: he_id.index(),
                group: Some(he.face().index()),
                origin_index: if include_indices {
                    Some(origin.index())
                } else {
                    None
                },
                dest_index: if include_indices {
                    Some(dest.index())
                } else {
                    None
                },
                origin: *p_o,
                dest: *p_d,
            })
        })
        .collect()
}

fn log_stitch(
    he_a: HalfEdgeId,
    he_b: HalfEdgeId,
    label: &str,
    confidence: f64,
    ctx: &mut ModelingContext,
) {
    let mut decision = TracedDecision::new(
        DecisionId(he_a.index() as u64),
        DecisionKind::PolicyApplied {
            policy: forge_core::PolicyKind::CoincidentGeometry,
            default_used: true,
        },
        DecisionTier::NearBoundary,
        confidence,
        DecisionContext::Degeneracy {
            description: format!("Stitched {} <-> {} ({})", he_a, he_b, label),
        },
    );
    decision.set_entity_scope(EntityRef::new(
        forge_core::EntityKind::HalfEdge,
        he_a.index(),
    ));
    ctx.get_decision_log_mut().record(decision);
}
