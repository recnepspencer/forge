//! Expected-cut-hint normalization.
//!
//! DOMAIN: Deduplicate and canonicalize ExpectedCutHint endpoint/interval lists.
//! DEPENDENCIES: schema (ExpectedCutEndpointMap, ExpectedCutHint, ExpectedCutInterval).
//! INVARIANTS: All operations are purely transformational — no topology mutation.

use crate::shared_ops::vertex::dedup::dedup_points_by_tolerance;
use super::schema::{ExpectedCutEndpointMap, ExpectedCutHint, ExpectedCutInterval};

/// Deduplicate and canonicalize all hints in an `ExpectedCutEndpointMap` in place.
pub(super) fn normalize_hint_map(map: &mut ExpectedCutEndpointMap, tol: f64) {
    for hint in map.values_mut() {
        hint.endpoints =
            dedup_points_by_tolerance(std::mem::take(&mut hint.endpoints), tol);
        hint.intervals = normalize_intervals(std::mem::take(&mut hint.intervals), tol);
    }
}

/// Localize an `ExpectedCutHint` to the overlap between the hint intervals and a face chord.
///
/// Returns `None` if no interval overlaps the chord (face is irrelevant for this hint).
pub(super) fn localize_expected_hint(
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
            face_chord,
            (iv.p0, iv.p1),
            min_len,
        ) {
            out.endpoints.push(p0);
            out.endpoints.push(p1);
            out.intervals
                .push(ExpectedCutInterval { p0, p1 });
        }
    }

    if out.intervals.is_empty() {
        return None;
    }

    out.endpoints = dedup_points_by_tolerance(out.endpoints, min_len.max(1e-9));
    Some(out)
}

// ── Internal helpers ─────────────────────────────────────────────────────────

fn normalize_intervals(
    intervals: Vec<ExpectedCutInterval>,
    tol: f64,
) -> Vec<ExpectedCutInterval> {
    let tol_sq = tol * tol;
    let mut out: Vec<ExpectedCutInterval> = Vec::new();

    'outer: for mut iv in intervals {
        if forge_math::linalg::distance_sq(iv.p0, iv.p1) <= tol_sq {
            continue;
        }
        canonicalize_interval(&mut iv);
        for existing in &out {
            let same_dir = (forge_math::linalg::distance_sq(iv.p0, existing.p0) <= tol_sq
                && forge_math::linalg::distance_sq(iv.p1, existing.p1) <= tol_sq)
                || (forge_math::linalg::distance_sq(iv.p0, existing.p1) <= tol_sq
                    && forge_math::linalg::distance_sq(iv.p1, existing.p0) <= tol_sq);
            if same_dir {
                continue 'outer;
            }
        }
        out.push(iv);
    }

    out.sort_by(interval_sort_key);
    out
}

fn canonicalize_interval(iv: &mut ExpectedCutInterval) {
    let a = iv.p0;
    let b = iv.p1;
    if forge_math::linalg::compare_points_lex(&a, &b).is_gt() {
        iv.p0 = b;
        iv.p1 = a;
    }
}

fn interval_sort_key(a: &ExpectedCutInterval, b: &ExpectedCutInterval) -> std::cmp::Ordering {
    forge_math::linalg::compare_points_lex(&a.p0, &b.p0)
        .then_with(|| forge_math::linalg::compare_points_lex(&a.p1, &b.p1))
}


