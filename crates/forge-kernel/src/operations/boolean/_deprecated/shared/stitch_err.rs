//! Boolean-specific error generation for missing twin topology.
//!
//! DOMAIN: Extracts 2-ring topological neighborhoods and lineage to report boolean assembly failures.

use forge_core::{EntityRef, KernelError, ToleranceProvider};
use forge_topo::handles::HalfEdgeId;
use forge_topo::transactions::MutableDraft;

use crate::core::ModelingContext;
use crate::geometry_state::GeometryState;

fn debug_stitch_enabled() -> bool {
    std::env::var("FORGE_DEBUG_STITCH")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Build a structured error for remaining unpaired halfedges.
///
/// Includes per-entity decision ancestry and a 2-ring extracted region
/// for each unpaired halfedge, enabling root-cause tracing and local
/// geometry reconstruction.
pub fn build_stitch_failure_error(
    unpaired: &[HalfEdgeId],
    draft: &MutableDraft,
    geom: &GeometryState,
    ctx: &ModelingContext,
) -> KernelError {
    let mut detail_lines: Vec<String> = Vec::new();
    detail_lines.push(format!(
        "{} halfedges remain unpaired after stitching",
        unpaired.len(),
    ));

    let decision_log = ctx.get_decision_log();
    let max_report = unpaired.len().min(5);

    for &he_id in unpaired.iter().take(max_report) {
        let he_ref = EntityRef::new(forge_core::EntityKind::HalfEdge, he_id.index());
        let he_ref = EntityRef::new(forge_core::EntityKind::HalfEdge, he_id.index(), 0);
        let face_index = draft
            .arena()
            .get_half_edge(he_id)
            .map(|he| he.face().index())
            .unwrap_or(u32::MAX);
        let face_id_opt = draft.arena().get_half_edge(he_id).ok().map(|he| he.face());
        let face_ref = EntityRef::new(forge_core::EntityKind::Face, face_index, 0);

        let related_decisions: Vec<String> = decision_log
            .decisions()
            .filter(|d| {
                d.get_entity_scope()
                    .map(|e| *e == he_ref || *e == face_ref)
                    .unwrap_or(false)
            })
            .map(|d| {
                format!(
                    "    [{}] {} margin={:.2e} | {}",
                    d.get_tier(),
                    d.get_kind(),
                    d.get_margin(),
                    d.get_context(),
                )
            })
            .collect();

        detail_lines.push(format!(
            "  HalfEdge#{} (Face#{})",
            he_id.index(),
            face_index
        ));
        if let Some(face_id) = face_id_opt {
            if let Ok(face_data) = draft.arena().get_face(face_id) {
                if let Some(lin) = face_data.lineage() {
                    detail_lines.push(format!(
                        "    face_lineage: op={} ancestry={:032x} features={:?}",
                        lin.get_creation_op(),
                        lin.get_ancestry_hash(),
                        lin.get_origin_features(),
                    ));
                }
            }
        }
        if debug_stitch_enabled() {
            let edge_id = draft
                .arena()
                .get_half_edge(he_id)
                .map(|he| he.edge())
                .unwrap_or(forge_topo::handles::EdgeId::new(0, 0));
            if let Ok((origin, dest)) = draft.arena().get_edge_endpoints(edge_id) {
                let p0 = geom.get_vertex_position(origin);
                let p1 = geom.get_vertex_position(dest);
                if let (Some(a), Some(b)) = (p0, p1) {
                    detail_lines.push(format!(
                        "    endpoints: V#{} [{:.6},{:.6},{:.6}] -> V#{} [{:.6},{:.6},{:.6}]",
                        origin.index(),
                        a[0],
                        a[1],
                        a[2],
                        dest.index(),
                        b[0],
                        b[1],
                        b[2]
                    ));
                } else {
                    detail_lines.push(format!(
                        "    endpoints: V#{} -> V#{} (geometry missing)",
                        origin.index(),
                        dest.index()
                    ));
                }
            }
            if let Some(line) = find_near_reverse_edge_debug(he_id, draft, geom, ctx) {
                detail_lines.push(format!("    near-reverse: {}", line));
            }
        }
        if related_decisions.is_empty() {
            detail_lines.push("    (no entity-scoped decisions found)".to_string());
        } else {
            for line in related_decisions {
                detail_lines.push(line);
            }
        }

        let face_id = forge_topo::handles::FaceId::new(face_index, 0);
        if let Ok(region) =
            crate::proof::region_extractor::extract_n_ring(draft.arena(), geom, face_id, 2)
        {
            detail_lines.push(format!(
                "  2-ring: {}F {}HE {}V",
                region.face_count(),
                region.half_edge_count(),
                region.vertex_count(),
            ));
            for (&fidx, plane) in region.get_face_planes() {
                let n = plane.get_normal();
                detail_lines.push(format!(
                    "    Face#{}: n=[{:.2},{:.2},{:.2}] d={:.2}",
                    fidx,
                    n[0],
                    n[1],
                    n[2],
                    plane.get_offset(),
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

pub fn find_near_reverse_edge_debug(
    source_he: HalfEdgeId,
    draft: &MutableDraft,
    geom: &GeometryState,
    ctx: &ModelingContext,
) -> Option<String> {
    let edge_id = draft.arena().get_half_edge(source_he).ok()?.edge();
    let (src_o, src_d) = draft.arena().get_edge_endpoints(edge_id).ok()?;
    let a = *geom.get_vertex_position(src_o)?;
    let b = *geom.get_vertex_position(src_d)?;

    let tol = geom
        .global_default()
        .max(ctx.get_gap_closure().get_max_gap() * 4.0);
    let tol_sq = tol * tol;

    let mut best_any: Option<(HalfEdgeId, f64)> = None;
    let mut best_close: Option<(HalfEdgeId, f64)> = None;
    for (cand_id, _) in draft.arena().iter_half_edges() {
        if cand_id == source_he {
            continue;
        }
        let cand_edge_id = draft.arena().get_half_edge(cand_id).ok()?.edge();
        let (co, cd) = match draft.arena().get_edge_endpoints(cand_edge_id) {
            Ok(v) => v,
            Err(_) => continue,
        };
        let pa = match geom.get_vertex_position(co) {
            Some(p) => *p,
            None => continue,
        };
        let pb = match geom.get_vertex_position(cd) {
            Some(p) => *p,
            None => continue,
        };

        let d0 = forge_math::linalg::distance_sq(a, pb);
        let d1 = forge_math::linalg::distance_sq(b, pa);
        let score = d0 + d1;
        let is_better_any = best_any.map(|(_, s)| score < s).unwrap_or(true);
        if is_better_any {
            best_any = Some((cand_id, score));
        }
        if d0 <= tol_sq * 100.0 && d1 <= tol_sq * 100.0 {
            let is_better = best_close.map(|(_, s)| score < s).unwrap_or(true);
            if is_better {
                best_close = Some((cand_id, score));
            }
        }
    }

    let (best_id, score, close) = if let Some((id, s)) = best_close {
        (id, s, true)
    } else if let Some((id, s)) = best_any {
        (id, s, false)
    } else {
        return None;
    };
    let face = draft.arena().get_half_edge(best_id).ok()?.face();
    let lineage = draft
        .arena()
        .get_face(face)
        .ok()
        .and_then(|f| f.lineage())
        .map(|lin| {
            format!(
                "{}#{}",
                lin.get_creation_op().get_name(),
                lin.get_creation_op().get_invocation_id()
            )
        })
        .unwrap_or_else(|| "no-lineage".to_string());
    Some(format!(
        "HE#{} F#{} score={:.3e} {}",
        best_id.index(),
        face.index(),
        score.sqrt(),
        if close {
            lineage
        } else {
            format!("{} [far]", lineage)
        }
    ))
}
