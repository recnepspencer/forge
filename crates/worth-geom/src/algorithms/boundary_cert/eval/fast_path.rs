//! Exact fast-path checks for boundary certification.
//!
//! DOMAIN: Reject definite crossings or degeneracies before arrangement
//! construction, while preserving exact-predicate failure as a certification
//! outcome.

use worth_math::numeric::sign::TriSign;

use super::super::schema::{BoundaryRejectReason, Segment2D};

/// Result of the fast-path crossing check.
pub(super) enum FastPathResult {
    /// No crossings, no degeneracies — boundary is simple.
    Simple,
    /// Definite rejection (proper crossing found or predicate failure).
    Rejected {
        reason: BoundaryRejectReason,
        witness: [f64; 2],
    },
    /// Ambiguous — needs fallback arrangement analysis.
    NeedsFallback,
}

/// Fast-path: check all non-adjacent segment pairs for proper crossing.
///
/// Uses exact orient2d predicates (Shewchuk) for topology-driving decisions.
/// Returns `Simple` only if no evidence of any degeneracy is found.
/// Returns `Rejected` if any orient2d predicate evaluation fails.
pub(super) fn try_fast_path(segments: &[Segment2D]) -> FastPathResult {
    let n = segments.len();
    let mut needs_fallback = false;

    for i in 0..n {
        let a_start = segments[i].get_start();
        let a_end = segments[i].get_end();

        for j in (i + 2)..n {
            if i == 0 && j == n - 1 {
                continue;
            }

            let b_start = segments[j].get_start();
            let b_end = segments[j].get_end();

            match classify_segment_pair_exact(a_start, a_end, b_start, b_end) {
                SegmentPairClass::Disjoint => {}
                SegmentPairClass::ProperCrossing { witness } => {
                    return FastPathResult::Rejected {
                        reason: BoundaryRejectReason::SelfCrossing,
                        witness,
                    };
                }
                SegmentPairClass::EndpointTouch
                | SegmentPairClass::Collinear
                | SegmentPairClass::Ambiguous => {
                    needs_fallback = true;
                }
                SegmentPairClass::PredicateFailure { witness } => {
                    return FastPathResult::Rejected {
                        reason: BoundaryRejectReason::DegenerateBoundary,
                        witness,
                    };
                }
            }
        }
    }

    if detect_repeated_vertices(segments) {
        needs_fallback = true;
    }

    if needs_fallback {
        FastPathResult::NeedsFallback
    } else {
        FastPathResult::Simple
    }
}

/// Classification of a pair of segments using exact predicates.
enum SegmentPairClass {
    /// Segments are clearly disjoint (no interaction).
    Disjoint,
    /// Segments cross transversally (proper crossing).
    ProperCrossing { witness: [f64; 2] },
    /// An endpoint touches the other segment (not a crossing).
    EndpointTouch,
    /// Segments are collinear (may overlap).
    Collinear,
    /// Could not classify cleanly (degenerate case).
    Ambiguous,
    /// Exact predicate evaluation failed — cannot certify.
    PredicateFailure { witness: [f64; 2] },
}

/// Classify two segments using exact orient2d predicates.
///
/// Determines if they cross, touch, are collinear, or are disjoint.
/// All orientation tests use Shewchuk adaptive exact predicates.
/// Propagates predicate errors as `PredicateFailure` — never silently
/// maps them to `TriSign::Zero`.
fn classify_segment_pair_exact(
    a0: [f64; 2],
    a1: [f64; 2],
    b0: [f64; 2],
    b1: [f64; 2],
) -> SegmentPairClass {
    let d1 = match orient2d_sign(a0, a1, b0) {
        Ok(s) => s,
        Err(_) => return SegmentPairClass::PredicateFailure { witness: b0 },
    };
    let d2 = match orient2d_sign(a0, a1, b1) {
        Ok(s) => s,
        Err(_) => return SegmentPairClass::PredicateFailure { witness: b1 },
    };
    let d3 = match orient2d_sign(b0, b1, a0) {
        Ok(s) => s,
        Err(_) => return SegmentPairClass::PredicateFailure { witness: a0 },
    };
    let d4 = match orient2d_sign(b0, b1, a1) {
        Ok(s) => s,
        Err(_) => return SegmentPairClass::PredicateFailure { witness: a1 },
    };

    if d1 == TriSign::Zero && d2 == TriSign::Zero {
        return SegmentPairClass::Collinear;
    }

    if d1 == TriSign::Zero || d2 == TriSign::Zero || d3 == TriSign::Zero || d4 == TriSign::Zero {
        let any_straddling = (d1 != d2 || d1 == TriSign::Zero || d2 == TriSign::Zero)
            && (d3 != d4 || d3 == TriSign::Zero || d4 == TriSign::Zero);
        if any_straddling {
            return SegmentPairClass::EndpointTouch;
        }
        return SegmentPairClass::Ambiguous;
    }

    if d1 != d2 && d3 != d4 {
        let witness = approximate_crossing_point(a0, a1, b0, b1);
        return SegmentPairClass::ProperCrossing { witness };
    }

    SegmentPairClass::Disjoint
}

/// Compute exact orient2d sign using Shewchuk predicates.
///
/// Returns `Err` on predicate evaluation failure — callers MUST NOT
/// silently map this to `TriSign::Zero`.
fn orient2d_sign(
    pa: [f64; 2],
    pb: [f64; 2],
    pc: [f64; 2],
) -> Result<TriSign, worth_math::MathError> {
    let (certified, _) = worth_math::predicates::orient2d(pa, pb, pc)?;
    Ok(certified.sign())
}

/// Approximate crossing point for witness reporting.
///
/// Uses parametric line intersection. Not exact — for diagnostics only.
fn approximate_crossing_point(a0: [f64; 2], a1: [f64; 2], b0: [f64; 2], b1: [f64; 2]) -> [f64; 2] {
    let da = [a1[0] - a0[0], a1[1] - a0[1]];
    let db = [b1[0] - b0[0], b1[1] - b0[1]];
    let denom = da[0] * db[1] - da[1] * db[0];

    if denom == 0.0 {
        return [(a0[0] + b0[0]) * 0.5, (a0[1] + b0[1]) * 0.5];
    }

    let t = ((b0[0] - a0[0]) * db[1] - (b0[1] - a0[1]) * db[0]) / denom;
    [a0[0] + t * da[0], a0[1] + t * da[1]]
}

/// Check for zero-length segments after projection using exact vertex equality.
///
/// Two endpoints are considered identical when all coordinates match bitwise.
/// No floating-point threshold — if the coordinates are distinct IEEE754 values,
/// the segment is non-degenerate.
pub(super) fn find_degenerate_segment(segments: &[Segment2D]) -> Option<[f64; 2]> {
    for seg in segments {
        let s = seg.get_start();
        let e = seg.get_end();
        if s[0] == e[0] && s[1] == e[1] {
            return Some(s);
        }
    }
    None
}

/// Detect if all boundary vertices are collinear (zero enclosed area).
///
/// A boundary where every vertex lies on the same line encloses no area and
/// is geometrically degenerate. Uses orient2d against the first segment's
/// endpoints to check every other vertex. If all are collinear, returns the
/// first vertex as the witness.
pub(super) fn detect_all_collinear(segments: &[Segment2D]) -> Option<[f64; 2]> {
    if segments.len() < 3 {
        return None;
    }

    let p0 = segments[0].get_start();
    let p1 = segments[0].get_end();

    for seg in &segments[1..] {
        let q = seg.get_end();
        match orient2d_sign(p0, p1, q) {
            Ok(sign) => {
                if sign != TriSign::Zero {
                    return None;
                }
            }
            Err(_) => return None,
        }
    }

    Some(p0)
}

/// Detect non-adjacent repeated projected vertices using exact equality.
///
/// Two vertices are considered repeated when all coordinates match bitwise.
fn detect_repeated_vertices(segments: &[Segment2D]) -> bool {
    let n = segments.len();

    for i in 0..n {
        let vi = segments[i].get_start();
        for j in (i + 2)..n {
            if i == 0 && j == n - 1 {
                continue;
            }
            let vj = segments[j].get_start();
            if vi[0] == vj[0] && vi[1] == vj[1] {
                return true;
            }
        }
    }
    false
}
