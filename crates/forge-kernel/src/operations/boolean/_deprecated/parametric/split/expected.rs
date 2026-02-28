//! Proof-system expected-cut hint matching.
//!
//! DOMAIN: Match cut vertex pairs to expected intersection endpoints from the
//!   proof system. Validates that the scaffold fallback is geometrically sound.
//! DEPENDENCIES: schema (ExpectedCutHint, EdgeCutMap), apply (execute_make_edge_face).
//! INVARIANTS:
//!   - `try_expected_pair` returns None immediately when no hint is present.
//!   - `can_use_scaffold_fallback` is side-effect-free (pure predicate).

use std::collections::BTreeSet;

use forge_core::KernelError;
use forge_topo::handles::{FaceId, VertexId};
use forge_topo::transactions::MutableDraft;

use crate::geom_facade::Plane;
use crate::core::ModelingContext;
use crate::geometry_state::GeometryState;

use super::apply::execute_make_edge_face;
use super::schema::{make_edge_key, EdgeCutMap, ExpectedCutHint};

/// Try to apply the cut pair most closely matching the expected endpoints.
///
/// Scores each candidate (v_a, v_b) pair against `hint.endpoints` by
/// closest-assignment distance, sorts by score, and attempts the best
/// pair below the tolerance threshold.
pub(super) fn try_expected_pair(
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
        if adjacent.contains(&make_edge_key(pair[0], pair[1])) {
            continue;
        }
        let Some(pa) = geometry.get_vertex_position(v_a) else {
            continue;
        };
        let Some(pb) = geometry.get_vertex_position(v_b) else {
            continue;
        };
        let (score, max_leg) = score_pair_against_expected(pa, pb, expected);
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
                face.index(), cut_plane_idx, v_a, v_b, score
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
            draft, geometry, edge_cut_map, face, face_plane, cut_plane_idx, v_a, v_b, ctx,
        ) {
            Some(result) => return Ok(Some(result)),
            None => eprintln!(
                "[cut-expected] face#{} plane#{} apply failed for {} {}",
                face.index(), cut_plane_idx, v_a, v_b
            ),
        }
    }

    if let Some((best_score, best_max_leg)) = best_seen {
        eprintln!(
            "[cut-expected] face#{} plane#{} reject best pair score={:.3e} max_leg={:.3e} allow={:.3e} scale={:.3e} sorted={} rejected={}",
            face.index(), cut_plane_idx, best_score, best_max_leg.sqrt(),
            max_leg_allow, scale, sorted.len(), rejected
        );
    }

    Ok(None)
}

/// Decide whether the scaffold fallback pair is geometrically valid.
///
/// Computes the expected projection intervals along the cut direction and
/// verifies that exactly one viable pair exists and it brackets those intervals.
pub(super) fn can_use_scaffold_fallback(
    sorted: &[VertexId],
    expected_hint: &ExpectedCutHint,
    geometry: &GeometryState,
    adjacent: &BTreeSet<(u32, u32)>,
    face_plane: &Plane,
    cut_plane: &Plane,
) -> bool {
    let dir = forge_math::linalg::plane_cut_direction(
        face_plane.raw_normal(),
        cut_plane.raw_normal(),
        1e-24,
    );
    let expected_intervals = scaffold_expected_intervals(expected_hint, dir);
    if expected_intervals.is_empty() {
        return false;
    }

    let mut viable_pairs = 0usize;
    let mut bracketed_pairs = 0usize;

    for pair in sorted.chunks_exact(2) {
        let v_a = pair[0];
        let v_b = pair[1];
        if v_a == v_b || adjacent.contains(&make_edge_key(pair[0], pair[1])) {
            continue;
        }
        let Some(pa) = geometry.get_vertex_position(v_a) else { continue; };
        let Some(pb) = geometry.get_vertex_position(v_b) else { continue; };
        let a_t = forge_math::linalg::dot(*pa, dir);
        let b_t = forge_math::linalg::dot(*pb, dir);
        let cand_min = a_t.min(b_t);
        let cand_max = a_t.max(b_t);
        let bracketed = expected_intervals.iter().any(|(exp_min, exp_max, exp_scale)| {
            let bracket_tol = (exp_scale * 0.10).max(1e-6);
            cand_min <= *exp_min + bracket_tol && cand_max >= *exp_max - bracket_tol
        });
        if bracketed {
            bracketed_pairs += 1;
        }
        viable_pairs += 1;
    }

    // Fallback is only safe when exactly 1 viable pair exists AND it brackets the expected interval.
    viable_pairs == 1 && bracketed_pairs == 1
}

// ── Internal helpers ─────────────────────────────────────────────────────────

use forge_math::linalg::distance_sq;

fn scaffold_expected_intervals(hint: &ExpectedCutHint, dir: [f64; 3]) -> Vec<(f64, f64, f64)> {
    let mut out = Vec::new();
    if !hint.intervals.is_empty() {
        for iv in &hint.intervals {
            if let Some((a, b, s)) =
                forge_geom::algorithms::chord::project_interval_onto_direction([iv.p0, iv.p1], dir)
            {
                out.push((a.min(b), a.max(b), s.max(1e-9)));
            }
        }
        return out;
    }
    if let Some((a, b, s)) =
        forge_geom::algorithms::chord::project_interval_onto_direction(hint.endpoints.iter().copied(), dir)
    {
        out.push((a.min(b), a.max(b), s.max(1e-9)));
    }
    out
}

/// Score `(a, b)` against expected endpoints by minimum sum-of-squared distances.
fn score_pair_against_expected(
    a: &[f64; 3],
    b: &[f64; 3],
    expected: &[[f64; 3]],
) -> (f64, f64) {
    let mut best = f64::INFINITY;
    let mut best_max_leg = f64::INFINITY;
    for i in 0..expected.len() {
        for j in (i + 1)..expected.len() {
            let d_ai = distance_sq(*a, expected[i]);
            let d_bj = distance_sq(*b, expected[j]);
            let d_aj = distance_sq(*a, expected[j]);
            let d_bi = distance_sq(*b, expected[i]);
            let s1 = d_ai + d_bj;
            let s2 = d_aj + d_bi;
            let (s, max_leg) = if s1 <= s2 { (s1, d_ai.max(d_bj)) } else { (s2, d_aj.max(d_bi)) };
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
            best = best.max(distance_sq(expected[i], expected[j]));
        }
    }
    best.sqrt()
}

