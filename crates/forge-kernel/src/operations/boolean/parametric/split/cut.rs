//! Per-face plane-cut application.
//!
//! DOMAIN: Apply a single face cut using MakeEdgeFace after the gate passes.
//! DEPENDENCIES: gate (compute_face_chord), schema (CutPoint, SplitConfig).
//! INVARIANTS:
//!   - `gate::compute_face_chord` is called first to decide IF a cut happens.
//!   - `find_cut_points_provenance` decides WHERE (vertex sign walk).
//!   - Exactly ONE cut pair is applied per call; both fragments are re-enqueued
//!     by the caller for re-testing against remaining planes.

use std::collections::{BTreeMap, BTreeSet};

use forge_core::KernelError;
use forge_core::{
    DecisionContext, DecisionId, DecisionKind, DecisionTier, EntityRef, TracedDecision,
};
use crate::geom_facade::{intersect_three_planes_exact, Plane};
use forge_math::arithmetic::Rational;
use forge_math::linalg::{plane_cut_direction, sort_points_along_direction};
use forge_math::sign::is_sign_crossing;
use forge_topo::euler::make_edge_face::MakeEdgeFace;
use forge_topo::euler::split_edge::SplitEdge;
use forge_topo::handles::{FaceId, HalfEdgeId, VertexId};
use forge_topo::operator::apply_op;
use forge_topo::state::MutableDraft;
use forge_topo::traverse::FaceAllEdgesIterator;
use forge_topo::topology::queries::polygon::face_adjacent_vertex_pairs;

use crate::core::ModelingContext;
use crate::geometry_state::GeometryState;
use crate::operations::boolean::shared::edge_intersection::compute_edge_plane_intersection_position;
use crate::shared_ops::point_dedup::dedup_points_by_tolerance;
use crate::shared_ops::vertex_identity::{build_vertex_provenance, VertexMatchKey};

use super::gate::compute_face_chord;
use super::schema::{
    make_edge_key, CutPoint, EdgeCutMap, ExpectedCutHint, LocalVertexDedup, PlaneTable,
    SharedVertexRegistry, SplitConfig,
};
use super::signs::exact_sign_for_vertex;

/// Split a face by a cutting plane — applies exactly ONE cut pair per call.
///
/// Gate: `compute_face_chord` decides IF the face needs cutting.
/// Location: `find_cut_points_provenance` decides WHERE (vertex sign walk).
///
/// Returns `[new_face, original_face]` on success so the caller can
/// re-enqueue both with the current cut plane prepended.
pub fn split_face_by_plane(
    draft: &mut MutableDraft,
    geometry: &mut GeometryState,
    dedup: &mut LocalVertexDedup,
    edge_cut_map: &mut EdgeCutMap,
    face: FaceId,
    face_plane: &Plane,
    cut_plane: &Plane,
    cut_plane_idx: usize,
    split_cfg: &SplitConfig<'_>,
    shared_registry: &mut SharedVertexRegistry,
    expected_hint: Option<&ExpectedCutHint>,
    ctx: &mut ModelingContext,
) -> Result<Vec<FaceId>, KernelError> {
    let Some(face_chord) = gate_chord(
        draft,
        geometry,
        face,
        face_plane,
        cut_plane,
        cut_plane_idx,
        split_cfg,
        ctx,
    )?
    else {
        return Ok(Vec::new());
    };

    let cut_points = find_cut_points_provenance(
        draft.arena(),
        geometry,
        face,
        cut_plane,
        cut_plane_idx,
        dedup,
        shared_registry,
        split_cfg,
    )?;

    let resolved = resolve_all_cut_points(&cut_points, draft, geometry, dedup)?;
    if resolved.len() < 2 {
        let nan3 = [f64::NAN; 3];
        eprintln!(
            "[cut-diag] Face#{} by plane#{}: {} cut_points found, {} unique after dedup",
            face.index(),
            cut_plane_idx,
            cut_points.len(),
            resolved.len()
        );
        for (i, vid) in resolved.iter().enumerate() {
            let pos = geometry.get_vertex_position(*vid).unwrap_or(&nan3);
            eprintln!(
                "  [cut-diag]   resolved[{}]: vid={} pos=[{:.6},{:.6},{:.6}]",
                i, vid, pos[0], pos[1], pos[2]
            );
        }
        log_rejection(
            face,
            cut_plane_idx,
            &format!(
                "{} resolved vertices after dedup (need >=2)",
                resolved.len()
            ),
            ctx,
        );
        return Ok(Vec::new());
    }

    let sorted = sort_along_cut_direction(resolved, face_plane, cut_plane, geometry);
    let had_expected_hint = expected_hint.is_some();
    let localized_expected_hint = expected_hint.and_then(|hint| {
        localize_expected_hint(hint, face_chord, split_cfg.tolerance.get_min_edge_length())
    });
    if had_expected_hint && localized_expected_hint.is_none() {
        log_rejection(
            face,
            cut_plane_idx,
            "deferred: fragment chord does not overlap expected segment interval",
            ctx,
        );
        return Ok(Vec::new());
    }

    apply_one_cut(
        sorted,
        draft,
        geometry,
        edge_cut_map,
        face,
        face_plane,
        cut_plane,
        cut_plane_idx,
        localized_expected_hint.as_ref(),
        ctx,
    )
}

/// Run the chord-gate and return the current face chord.
fn gate_chord(
    draft: &MutableDraft,
    geometry: &GeometryState,
    face: FaceId,
    face_plane: &Plane,
    cut_plane: &Plane,
    cut_plane_idx: usize,
    split_cfg: &SplitConfig<'_>,
    ctx: &mut ModelingContext,
) -> Result<Option<([f64; 3], [f64; 3])>, KernelError> {
    let chord = compute_face_chord(
        draft.arena(),
        geometry,
        face,
        face_plane,
        cut_plane,
        split_cfg.tolerance,
    )?;
    if chord.is_none() {
        log_rejection(face, cut_plane_idx, "rejected by chord gate", ctx);
        return Ok(None);
    }
    Ok(chord)
}

/// Resolve CutPoints to VertexIds, dedup, and validate count.
fn resolve_all_cut_points(
    cut_points: &[CutPoint],
    draft: &mut MutableDraft,
    geometry: &mut GeometryState,
    dedup: &mut LocalVertexDedup,
) -> Result<Vec<VertexId>, KernelError> {
    let mut resolved: Vec<VertexId> = Vec::new();
    for cp in cut_points {
        resolved.push(resolve_cut_point(cp, draft, geometry, dedup)?);
    }
    resolved.dedup_by_key(|v| v.index());
    Ok(resolved)
}

/// Sort resolved vertices along the cut chord direction.
fn sort_along_cut_direction(
    verts: Vec<VertexId>,
    face_plane: &Plane,
    cut_plane: &Plane,
    geometry: &GeometryState,
) -> Vec<VertexId> {
    let dir = plane_cut_direction(face_plane.raw_normal(), cut_plane.raw_normal(), 1e-24);
    let items: Vec<(VertexId, [f64; 3])> = verts
        .into_iter()
        .map(|v| {
            let pos = geometry.get_vertex_position(v).copied().unwrap_or([0.0; 3]);
            (v, pos)
        })
        .collect();
    sort_points_along_direction(items, dir)
        .into_iter()
        .map(|(v, _)| v)
        .collect()
}

/// Apply ONE MakeEdgeFace cut from sorted vertex pairs.
///
/// Skips pairs that are already adjacent on the face (no-op cuts)
/// and tries each non-adjacent pair until one succeeds.
fn apply_one_cut(
    sorted: Vec<VertexId>,
    draft: &mut MutableDraft,
    geometry: &mut GeometryState,
    edge_cut_map: &mut EdgeCutMap,
    face: FaceId,
    face_plane: &Plane,
    cut_plane: &Plane,
    cut_plane_idx: usize,
    expected_hint: Option<&ExpectedCutHint>,
    ctx: &mut ModelingContext,
) -> Result<Vec<FaceId>, KernelError> {
    let adjacent = face_adjacent_vertex_pairs(draft.arena(), face)?;

    if let Some(result) = try_expected_pair(
        &sorted,
        expected_hint,
        &adjacent,
        draft,
        geometry,
        edge_cut_map,
        face,
        face_plane,
        cut_plane_idx,
        ctx,
    )? {
        return Ok(result);
    }

    if expected_hint.is_some()
        && !can_use_scaffold_fallback(
            &sorted,
            expected_hint.unwrap(),
            geometry,
            &adjacent,
            face_plane,
            cut_plane,
        )
    {
        log_rejection(
            face,
            cut_plane_idx,
            "deferred: expected overlap endpoints not bracketed by scaffold fragment",
            ctx,
        );
        return Ok(Vec::new());
    }

    let pairs = select_non_adjacent_pairs(&sorted, &adjacent);
    if let Some(result) = apply_scaffold_pass(
        &pairs,
        draft,
        geometry,
        edge_cut_map,
        face,
        face_plane,
        cut_plane_idx,
        expected_hint,
        ctx,
    )? {
        return Ok(result);
    }

    log_rejection(face, cut_plane_idx, "no valid cut pair found", ctx);
    Ok(Vec::new())
}

/// Filter a sorted vertex list down to non-identical, non-adjacent chunk pairs.
///
/// Returns `(v_a, v_b)` tuples in chunk order, skipping:
/// - Pairs where both vertices are the same.
/// - Pairs that already share a boundary edge on the face.
///
/// Pure function — no mutation, easily unit-testable in isolation.
fn select_non_adjacent_pairs(
    sorted: &[VertexId],
    adjacent: &BTreeSet<(u32, u32)>,
) -> Vec<(VertexId, VertexId)> {
    sorted
        .chunks_exact(2)
        .filter(|pair| pair[0] != pair[1])
        .filter(|pair| !adjacent.contains(&make_edge_key(pair[0], pair[1])))
        .map(|pair| (pair[0], pair[1]))
        .collect()
}

/// Apply a `MakeEdgeFace` cut between `v_a` and `v_b` on `face`.
///
/// On success: records the cut in `edge_cut_map`, assigns the face plane
/// to the new fragment, logs the decision, and returns `Some([new_face, face])`.
/// On topology failure: returns `None` (caller tries the next pair).
fn execute_make_edge_face(
    draft: &mut MutableDraft,
    geometry: &mut GeometryState,
    edge_cut_map: &mut EdgeCutMap,
    face: FaceId,
    face_plane: &Plane,
    cut_plane_idx: usize,
    v_a: VertexId,
    v_b: VertexId,
    ctx: &mut ModelingContext,
) -> Option<Vec<FaceId>> {
    let op = MakeEdgeFace {
        vertex_a: v_a,
        vertex_b: v_b,
        face,
    };
    match apply_op(draft, op) {
        Ok(res) => {
            edge_cut_map.insert(make_edge_key(v_a, v_b), cut_plane_idx);
            let new_face = res.get_value().new_face;
            geometry.set_face_plane(new_face, face_plane.clone());
            log_split_success(face, cut_plane_idx, new_face, ctx);
            Some(vec![new_face, face])
        }
        Err(_) => None,
    }
}

/// Try each non-adjacent pair in order until one `MakeEdgeFace` succeeds.
///
/// This is the "scaffold pass" — used when no expected hint was set or
/// when the expected pair path already failed.
fn apply_scaffold_pass(
    pairs: &[(VertexId, VertexId)],
    draft: &mut MutableDraft,
    geometry: &mut GeometryState,
    edge_cut_map: &mut EdgeCutMap,
    face: FaceId,
    face_plane: &Plane,
    cut_plane_idx: usize,
    expected_hint: Option<&ExpectedCutHint>,
    ctx: &mut ModelingContext,
) -> Result<Option<Vec<FaceId>>, KernelError> {
    for &(v_a, v_b) in pairs {
        if expected_hint.is_some() {
            eprintln!(
                "[cut-expected] face#{} plane#{} fallback trying {} {}",
                face.index(),
                cut_plane_idx,
                v_a,
                v_b
            );
        }
        if let Some(result) =
            execute_make_edge_face(draft, geometry, edge_cut_map, face, face_plane, cut_plane_idx, v_a, v_b, ctx)
        {
            return Ok(Some(result));
        }
    }
    Ok(None)
}


fn can_use_scaffold_fallback(
    sorted: &[VertexId],
    expected_hint: &ExpectedCutHint,
    geometry: &GeometryState,
    adjacent: &BTreeSet<(u32, u32)>,
    face_plane: &Plane,
    cut_plane: &Plane,
) -> bool {
    let dir = plane_cut_direction(face_plane.raw_normal(), cut_plane.raw_normal(), 1e-24);
    let expected_intervals = scaffold_expected_intervals(expected_hint, dir);
    if expected_intervals.is_empty() {
        return false;
    }

    let mut viable_pairs = 0usize;
    let mut bracketed_pairs = 0usize;

    for pair in sorted.chunks_exact(2) {
        let v_a = pair[0];
        let v_b = pair[1];
        if v_a == v_b {
            continue;
        }
        let key = if v_a.index() <= v_b.index() {
            (v_a.index(), v_b.index())
        } else {
            (v_b.index(), v_a.index())
        };
        if adjacent.contains(&key) {
            continue;
        }

        let Some(pa) = geometry.get_vertex_position(v_a) else {
            continue;
        };
        let Some(pb) = geometry.get_vertex_position(v_b) else {
            continue;
        };
        let a_t = forge_math::linalg::dot(*pa, dir);
        let b_t = forge_math::linalg::dot(*pb, dir);
        let cand_min = a_t.min(b_t);
        let cand_max = a_t.max(b_t);
        let bracketed = expected_intervals
            .iter()
            .any(|(exp_min, exp_max, exp_scale)| {
                let bracket_tol = (exp_scale * 0.10).max(1e-6);
                cand_min <= *exp_min + bracket_tol && cand_max >= *exp_max - bracket_tol
            });

        if bracketed {
            bracketed_pairs += 1;
        }

        viable_pairs += 1;
    }

    // Fallback is only allowed if there is at most 1 viable pair AND it brackets the expected endpoints.
    // If there is >1 viable bounding candidate, we defer to avoid crossing cuts.
    viable_pairs == 1 && bracketed_pairs == 1
}


fn localize_expected_hint(
    hint: &ExpectedCutHint,
    face_chord: ([f64; 3], [f64; 3]),
    min_len: f64,
) -> Option<ExpectedCutHint> {
    let mut out = ExpectedCutHint::default();
    if hint.intervals.is_empty() {
        return Some(hint.clone());
    }

    for iv in &hint.intervals {
        if let Some((p0, p1)) = forge_geom::algorithms::chord::chord_overlap_segment(
            face_chord, (iv.p0, iv.p1), min_len,
        ) {
            out.endpoints.push(p0);
            out.endpoints.push(p1);
            out.intervals
                .push(super::schema::ExpectedCutInterval { p0, p1 });
        }
    }

    if out.intervals.is_empty() {
        return None;
    }

    out.endpoints = dedup_points_by_tolerance(out.endpoints, min_len.max(1e-9));
    Some(out)
}


fn scaffold_expected_intervals(hint: &ExpectedCutHint, dir: [f64; 3]) -> Vec<(f64, f64, f64)> {
    let mut out = Vec::new();
    if !hint.intervals.is_empty() {
        for iv in &hint.intervals {
            if let Some((a, b, s)) = forge_geom::algorithms::chord::project_interval_onto_direction(
                [iv.p0, iv.p1], dir,
            ) {
                out.push((a.min(b), a.max(b), s.max(1e-9)));
            }
        }
        return out;
    }
    if let Some((a, b, s)) = forge_geom::algorithms::chord::project_interval_onto_direction(
        hint.endpoints.iter().copied(), dir,
    ) {
        out.push((a.min(b), a.max(b), s.max(1e-9)));
    }
    out
}

fn try_expected_pair(
    sorted: &[VertexId],
    expected_hint: Option<&ExpectedCutHint>,
    adjacent: &BTreeSet<(u32, u32)>,
    draft: &mut MutableDraft,
    geometry: &mut GeometryState,
    edge_cut_map: &mut EdgeCutMap,
    face: FaceId,
    face_plane: &Plane,
    cut_plane_idx: usize,
    ctx: &mut ModelingContext,
) -> Result<Option<Vec<FaceId>>, KernelError> {
    let Some(expected_hint) = expected_hint else {
        return Ok(None);
    };
    let expected = &expected_hint.endpoints;
    if expected.len() < 2 || sorted.len() < 2 {
        return Ok(None);
    }

    let mut candidates: Vec<(VertexId, VertexId, f64, f64)> = Vec::new();
    for pair in sorted.chunks_exact(2) {
        let v_a = pair[0];
        let v_b = pair[1];
        if v_a == v_b {
            continue;
        }
        let key = if v_a.index() <= v_b.index() {
            (v_a.index(), v_b.index())
        } else {
            (v_b.index(), v_a.index())
        };
        if adjacent.contains(&key) {
            continue;
        }
        let Some(pa) = geometry.get_vertex_position(v_a) else {
            continue;
        };
        let Some(pb) = geometry.get_vertex_position(v_b) else {
            continue;
        };
        let (score, max_leg) = expected_pair_score(pa, pb, expected);
        candidates.push((v_a, v_b, score, max_leg));
    }

    if candidates.is_empty() {
        return Ok(None);
    }

    let scale = expected_extent(expected).max(1e-9);
    let max_leg_allow = scale * 0.05 + 1e-6;
    let score_allow = scale * scale * 0.25;
    candidates.sort_by(|a, b| {
        a.2.partial_cmp(&b.2)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.3.partial_cmp(&b.3).unwrap_or(std::cmp::Ordering::Equal))
    });

    let mut best_seen: Option<(f64, f64)> = None;
    let mut rejected = 0usize;
    for (v_a, v_b, score, max_leg) in candidates {
        if best_seen.is_none() {
            best_seen = Some((score, max_leg));
        }
        if score > score_allow || max_leg > max_leg_allow * max_leg_allow {
            rejected += 1;
            continue;
        }

        if let (Some(pa), Some(pb)) = (
            geometry.get_vertex_position(v_a),
            geometry.get_vertex_position(v_b),
        ) {
            eprintln!(
                "[cut-expected] face#{} plane#{} choose {} {} score={:.3e}",
                face.index(),
                cut_plane_idx,
                v_a,
                v_b,
                score
            );
            eprintln!(
                "[cut-expected]   pair A=[{:.6},{:.6},{:.6}] B=[{:.6},{:.6},{:.6}]",
                pa[0], pa[1], pa[2], pb[0], pb[1], pb[2]
            );
            for (i, p) in expected.iter().enumerate() {
                eprintln!(
                    "[cut-expected]   expected[{}]=[{:.6},{:.6},{:.6}]",
                    i, p[0], p[1], p[2]
                );
            }
        }

        match execute_make_edge_face(
            draft,
            geometry,
            edge_cut_map,
            face,
            face_plane,
            cut_plane_idx,
            v_a,
            v_b,
            ctx,
        ) {
            Some(result) => return Ok(Some(result)),
            None => {
                eprintln!(
                    "[cut-expected] face#{} plane#{} apply failed for {} {}",
                    face.index(),
                    cut_plane_idx,
                    v_a,
                    v_b
                );
            }
        }
    }

    if let Some((best_score, best_max_leg)) = best_seen {
        eprintln!(
            "[cut-expected] face#{} plane#{} reject best pair score={:.3e} max_leg={:.3e} allow={:.3e} scale={:.3e} sorted={} rejected={}",
            face.index(),
            cut_plane_idx,
            best_score,
            best_max_leg.sqrt(),
            max_leg_allow,
            scale,
            sorted.len(),
            rejected
        );
    }

    Ok(None)
}

fn expected_pair_score(a: &[f64; 3], b: &[f64; 3], expected: &[[f64; 3]]) -> (f64, f64) {
    let mut best = f64::INFINITY;
    let mut best_max_leg = f64::INFINITY;
    for i in 0..expected.len() {
        for j in (i + 1)..expected.len() {
            let d_ai = dist_sq(a, &expected[i]);
            let d_bj = dist_sq(b, &expected[j]);
            let d_aj = dist_sq(a, &expected[j]);
            let d_bi = dist_sq(b, &expected[i]);
            let s1 = d_ai + d_bj;
            let s2 = d_aj + d_bi;
            let (s, max_leg) = if s1 <= s2 {
                (s1, d_ai.max(d_bj))
            } else {
                (s2, d_aj.max(d_bi))
            };
            if s < best || (s == best && max_leg < best_max_leg) {
                best = s;
                best_max_leg = max_leg;
            }
        }
    }
    (best, best_max_leg)
}

fn expected_extent(expected: &[[f64; 3]]) -> f64 {
    let mut best: f64 = 0.0;
    for i in 0..expected.len() {
        for j in (i + 1)..expected.len() {
            best = best.max(dist_sq(&expected[i], &expected[j]));
        }
    }
    best.sqrt()
}

fn dist_sq(a: &[f64; 3], b: &[f64; 3]) -> f64 {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    let dz = a[2] - b[2];
    dx * dx + dy * dy + dz * dz
}

/// Build the set of vertex pairs already adjacent on a face.

// ── Cut-point location (vertex sign walk) ───────────────────────────────────

/// Find where the cut plane enters the face boundary — exact sign-walk.
///
/// Walks every edge of the face:
/// - Origin vertex with TriSign::Zero → existing vertex CutPoint
/// - Edge crossing Pos↔Neg → new vertex CutPoint on the edge
fn find_cut_points_provenance(
    arena: &forge_topo::arena::TopologyArena,
    geometry: &GeometryState,
    face: FaceId,
    cut_plane: &Plane,
    cut_plane_idx: usize,
    dedup: &LocalVertexDedup,
    shared_registry: &mut SharedVertexRegistry,
    split_cfg: &SplitConfig<'_>,
) -> Result<Vec<CutPoint>, KernelError> {
    let mut points = Vec::new();
    let mut sign_cache: BTreeMap<VertexId, forge_math::sign::TriSign> = BTreeMap::new();

    for he in FaceAllEdgesIterator::new(arena, face)? {
        let he = he?;
        let he_data = arena.get_half_edge(he)?;
        let origin = he_data.origin();
        let next_data = arena.get_half_edge(he_data.next())?;
        let dest = next_data.origin();

        if let Some(p_o) = geometry.get_vertex_position(origin) {
            if let Some(p_d) = geometry.get_vertex_position(dest) {
                let s_o = *sign_cache.entry(origin).or_insert_with(|| {
                    exact_sign_for_vertex(geometry, origin, p_o, cut_plane, cut_plane_idx)
                });
                let s_d = *sign_cache.entry(dest).or_insert_with(|| {
                    exact_sign_for_vertex(geometry, dest, p_d, cut_plane, cut_plane_idx)
                });

                if s_o == forge_math::sign::TriSign::Zero && s_d != forge_math::sign::TriSign::Zero
                {
                    points.push(CutPoint::Existing(origin));
                } else if is_sign_crossing(s_o, s_d) {
                    let cp = compute_crossing_cut_point(
                        arena,
                        geometry,
                        he,
                        face,
                        cut_plane,
                        cut_plane_idx,
                        p_o,
                        p_d,
                        dedup,
                        shared_registry,
                        split_cfg,
                    )?;
                    points.push(cp);
                }
            }
        }
    }

    Ok(points)
}

/// True when signs indicate a Pos↔Neg edge crossing.
/// Delegates to the canonical predicate in `forge_math::sign`.

/// Compute the CutPoint for a Pos↔Neg edge crossing.
///
/// Attempts exact 3-plane intersection when the twin face has a different
/// plane, otherwise falls back to edge-plane intersection.
fn compute_crossing_cut_point(
    arena: &forge_topo::arena::TopologyArena,
    _geometry: &GeometryState,
    he: HalfEdgeId,
    face: FaceId,
    cut_plane: &Plane,
    cut_plane_idx: usize,
    p_o: &[f64; 3],
    p_d: &[f64; 3],
    dedup: &LocalVertexDedup,
    shared_registry: &mut SharedVertexRegistry,
    split_cfg: &SplitConfig<'_>,
) -> Result<CutPoint, KernelError> {
    let he_data = arena.get_half_edge(he)?;
    let twin = he_data.radial_next();
    let twin_face = arena.get_half_edge(twin)?.face();

    let p_face_idx = *split_cfg.face_plane_map.get(&face).unwrap_or(&0);
    let p_twin_idx = *split_cfg
        .face_plane_map
        .get(&twin_face)
        .unwrap_or(&p_face_idx);

    let (exact_pos, computed_pos, symbolic_planes) = compute_edge_plane_intersection_position(
        p_face_idx,
        p_twin_idx,
        cut_plane_idx,
        split_cfg.plane_table.planes(),
        cut_plane,
        p_o,
        p_d,
        split_cfg.tolerance,
    );

    let provenance = build_vertex_provenance(&exact_pos, computed_pos);
    let canonical_pos = shared_registry.canonical_position(&provenance, computed_pos);

    if let Some(vid) = dedup.find_by_provenance(&provenance) {
        return Ok(CutPoint::Existing(vid));
    }

    Ok(CutPoint::NewOnEdge {
        half_edge: he,
        provenance,
        position: canonical_pos,
        exact_position: exact_pos,
        symbolic_planes,
    })
}

/// Compute the intersection position for a new cut vertex.
///
/// If the face and twin have different planes, uses exact 3-plane
/// intersection. Otherwise falls back to edge-plane intersection
/// with f64→Rational promotion.

// ── Decision logging ─────────────────────────────────────────────────────────

/// Log a face-cut rejection decision.
fn log_rejection(face: FaceId, cut_plane_idx: usize, reason: &str, ctx: &mut ModelingContext) {
    let mut decision = TracedDecision::new(
        DecisionId(face.index() as u64),
        DecisionKind::Exact,
        DecisionTier::Deterministic,
        1.0,
        DecisionContext::Degeneracy {
            description: format!("Face #{} {reason} (plane #{cut_plane_idx})", face.index()),
        },
    );
    decision.set_entity_scope(EntityRef::new(forge_core::EntityKind::Face, face.index()));
    ctx.get_decision_log_mut().record(decision);
}

/// Log a successful face split decision.
fn log_split_success(
    face: FaceId,
    cut_plane_idx: usize,
    new_face: FaceId,
    ctx: &mut ModelingContext,
) {
    let mut decision = TracedDecision::new(
        DecisionId(face.index() as u64),
        DecisionKind::PolicyApplied {
            policy: forge_core::PolicyKind::CoincidentGeometry,
            default_used: true,
        },
        DecisionTier::Deterministic,
        1.0,
        DecisionContext::Degeneracy {
            description: format!(
                "Split face #{} by plane #{} -> new face #{}",
                face.index(),
                cut_plane_idx,
                new_face.index()
            ),
        },
    );
    decision.set_entity_scope(EntityRef::new(forge_core::EntityKind::Face, face.index()));
    ctx.get_decision_log_mut().record(decision);
}

// ── Utilities ────────────────────────────────────────────────────────────────

/// Resolve a CutPoint to a concrete VertexId, performing SplitEdge when needed.
pub fn resolve_cut_point(
    cp: &CutPoint,
    draft: &mut MutableDraft,
    geom: &mut GeometryState,
    dedup: &mut LocalVertexDedup,
) -> Result<VertexId, KernelError> {
    match cp {
        CutPoint::Existing(v) => Ok(*v),
        CutPoint::NewOnEdge {
            half_edge,
            provenance,
            position,
            exact_position,
            symbolic_planes,
        } => {
            let res = apply_op(
                draft,
                SplitEdge {
                    edge: *half_edge,
                    parameter: 0.5,
                },
            )?;
            let v = res.get_value().new_vertex;
            if let Some(exact) = exact_position {
                if let Some(planes) = symbolic_planes {
                    geom.set_vertex_position_symbolic(v, exact.clone(), *position, *planes);
                } else {
                    geom.set_vertex_position_exact(v, exact.clone());
                }
            } else {
                geom.set_vertex_position(v, *position);
            }
            dedup.insert(v, provenance.clone());
            Ok(v)
        }
    }
}

/// Build a vertex provenance key from an optional exact position.
/// Delegates to `shared_ops::vertex_identity::build_vertex_provenance`.
