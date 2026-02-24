//! Boundary certification algorithms.
//!
//! DOMAIN: Stateless 2D boundary certification for merge eligibility.
//! Two-phase algorithm: fast-path crossing check → fallback arrangement-based
//! weakly-simple recognizer (Akitaya-inspired, SoCG 2016).
//!
//! DEPENDENCIES: `forge-math` (orient2d exact predicates), `schema` types.
//! INVARIANTS: All functions are pure. No topology, no policy, no thresholds.
//! Certification predicates use exact Shewchuk orient2d (spec §4.5).
//!
//! NOTE: The fallback certifier classifies segment interactions but does NOT
//! split segments at intersection points. The `BoundaryArrangement` stores
//! the original segments plus classified events — it is an event-based
//! classification, not a full planar subdivision. This is sufficient for
//! weakly-simple recognition per Akitaya et al.

use forge_math::predicates::orient2d::orient2d;
use forge_math::sign::TriSign;

use super::schema::{
    BoundaryArrangement, BoundaryEvent, BoundaryEventKind, BoundaryRejectReason,
    ProjectedBoundary2D, ProjectionFrame2D, Segment2D, WeakSimpleCertificate,
};

/// Build a deterministic projection frame from a 3D plane normal.
///
/// Drops the axis with the largest absolute normal component.
/// Tie-break: X > Y > Z (spec §4.6).
/// The orientation sign preserves winding direction after projection.
pub fn build_projection_frame(normal: [f64; 3]) -> ProjectionFrame2D {
    let abs_n = [normal[0].abs(), normal[1].abs(), normal[2].abs()];

    let (drop_axis, u_axis, v_axis) = if abs_n[0] >= abs_n[1] && abs_n[0] >= abs_n[2] {
        (0, 1, 2)
    } else if abs_n[1] >= abs_n[2] {
        (1, 0, 2)
    } else {
        (2, 0, 1)
    };

    let orientation_sign = if normal[drop_axis] >= 0.0 { 1.0 } else { -1.0 };

    ProjectionFrame2D::new(drop_axis, u_axis, v_axis, orientation_sign)
}

/// Project a 3D point onto 2D using the given frame.
pub fn project_point(point: [f64; 3], frame: &ProjectionFrame2D) -> [f64; 2] {
    let u = point[frame.get_u_axis()];
    let v = point[frame.get_v_axis()];
    if frame.get_orientation_sign() < 0.0 {
        [v, u]
    } else {
        [u, v]
    }
}

/// Project 3D boundary segments to 2D using the given plane normal.
///
/// Builds a `ProjectionFrame2D` from the normal, then projects each segment.
pub fn project_boundary_to_2d(
    segments_3d: &[([f64; 3], [f64; 3], u64)],
    normal: [f64; 3],
) -> ProjectedBoundary2D {
    let frame = build_projection_frame(normal);
    let segments: Vec<Segment2D> = segments_3d
        .iter()
        .map(|(start, end, prov)| {
            let s2d = project_point(*start, &frame);
            let e2d = project_point(*end, &frame);
            Segment2D::new(s2d, e2d, *prov)
        })
        .collect();
    ProjectedBoundary2D::new(segments, frame)
}

/// Certify whether a projected 2D boundary is weakly simple.
///
/// Two-phase algorithm (spec §4.7):
/// 1. **Fast path**: exact orient2d crossing check on all segment pairs.
///    Returns `Simple` if no degeneracy evidence.
/// 2. **Fallback**: classifies all segment interactions, then runs
///    weakly-simple recognition on the classified events.
///
/// Returns `Rejected` if exact predicates cannot be evaluated (predicate
/// failure is treated as a certification failure, never silently ignored).
pub fn certify_boundary(boundary: &ProjectedBoundary2D) -> WeakSimpleCertificate {
    let segments = boundary.get_segments();

    if segments.len() < 3 {
        return WeakSimpleCertificate::Rejected {
            reason: BoundaryRejectReason::DegenerateBoundary,
            witness: [0.0, 0.0],
        };
    }

    if let Some(degenerate_witness) = find_degenerate_segment(segments) {
        return WeakSimpleCertificate::Rejected {
            reason: BoundaryRejectReason::DegenerateBoundary,
            witness: degenerate_witness,
        };
    }

    if let Some(witness) = detect_all_collinear(segments) {
        return WeakSimpleCertificate::Rejected {
            reason: BoundaryRejectReason::DegenerateBoundary,
            witness,
        };
    }

    match try_fast_path(segments) {
        FastPathResult::Simple => WeakSimpleCertificate::Simple,
        FastPathResult::Rejected { reason, witness } => {
            WeakSimpleCertificate::Rejected { reason, witness }
        }
        FastPathResult::NeedsFallback => {
            run_fallback_certifier(segments)
        }
    }
}

/// Result of the fast-path crossing check.
enum FastPathResult {
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
fn try_fast_path(segments: &[Segment2D]) -> FastPathResult {
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
    a0: [f64; 2], a1: [f64; 2],
    b0: [f64; 2], b1: [f64; 2],
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
fn orient2d_sign(pa: [f64; 2], pb: [f64; 2], pc: [f64; 2]) -> Result<TriSign, forge_math::MathError> {
    let (certified, _) = orient2d(pa, pb, pc)?;
    Ok(certified.sign())
}

/// Approximate crossing point for witness reporting.
///
/// Uses parametric line intersection. Not exact — for diagnostics only.
fn approximate_crossing_point(
    a0: [f64; 2], a1: [f64; 2],
    b0: [f64; 2], b1: [f64; 2],
) -> [f64; 2] {
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
fn find_degenerate_segment(segments: &[Segment2D]) -> Option<[f64; 2]> {
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
fn detect_all_collinear(segments: &[Segment2D]) -> Option<[f64; 2]> {
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

/// Fallback certifier: classify all interactions and run recognition.
///
/// Classifies all non-adjacent segment pair interactions using exact predicates,
/// then determines the weakly-simple certificate from the event types.
fn run_fallback_certifier(segments: &[Segment2D]) -> WeakSimpleCertificate {
    let arrangement = build_arrangement(segments);
    classify_arrangement(&arrangement)
}

/// Build a boundary arrangement by classifying all interactions between segments.
///
/// This produces an event-based classification (not a planar subdivision).
/// Events are sorted deterministically: primary by x, secondary by y,
/// tertiary by event kind ordinal.
fn build_arrangement(segments: &[Segment2D]) -> BoundaryArrangement {
    let n = segments.len();
    let mut events = Vec::new();

    for i in 0..n {
        let a0 = segments[i].get_start();
        let a1 = segments[i].get_end();

        if a0[0] == a1[0] && a0[1] == a1[1] {
            events.push(BoundaryEvent::new(
                BoundaryEventKind::DegenerateSegment,
                a0,
                [i, i],
            ));
            continue;
        }

        for j in (i + 2)..n {
            if i == 0 && j == n - 1 {
                continue;
            }

            let b0 = segments[j].get_start();
            let b1 = segments[j].get_end();

            let d1 = match orient2d_sign(a0, a1, b0) {
                Ok(s) => s,
                Err(_) => {
                    events.push(BoundaryEvent::new(
                        BoundaryEventKind::DegenerateSegment,
                        b0,
                        [i, j],
                    ));
                    continue;
                }
            };
            let d2 = match orient2d_sign(a0, a1, b1) {
                Ok(s) => s,
                Err(_) => {
                    events.push(BoundaryEvent::new(
                        BoundaryEventKind::DegenerateSegment,
                        b1,
                        [i, j],
                    ));
                    continue;
                }
            };
            let d3 = match orient2d_sign(b0, b1, a0) {
                Ok(s) => s,
                Err(_) => {
                    events.push(BoundaryEvent::new(
                        BoundaryEventKind::DegenerateSegment,
                        a0,
                        [i, j],
                    ));
                    continue;
                }
            };
            let d4 = match orient2d_sign(b0, b1, a1) {
                Ok(s) => s,
                Err(_) => {
                    events.push(BoundaryEvent::new(
                        BoundaryEventKind::DegenerateSegment,
                        a1,
                        [i, j],
                    ));
                    continue;
                }
            };

            if d1 == TriSign::Zero && d2 == TriSign::Zero {
                if segments_collinear_overlap_exact(a0, a1, b0, b1) {
                    let loc = [(a0[0] + b0[0]) * 0.5, (a0[1] + b0[1]) * 0.5];
                    events.push(BoundaryEvent::new(
                        BoundaryEventKind::OverlapStart,
                        loc,
                        [i, j],
                    ));
                }
                continue;
            }

            let has_zero = d1 == TriSign::Zero
                || d2 == TriSign::Zero
                || d3 == TriSign::Zero
                || d4 == TriSign::Zero;

            if has_zero {
                let touch_loc = find_touch_location(a0, a1, b0, b1, d1, d2, d3, d4);
                events.push(BoundaryEvent::new(
                    BoundaryEventKind::EndpointTouch,
                    touch_loc,
                    [i, j],
                ));
            } else if d1 != d2 && d3 != d4 {
                let witness = approximate_crossing_point(a0, a1, b0, b1);
                events.push(BoundaryEvent::new(
                    BoundaryEventKind::ProperCrossing,
                    witness,
                    [i, j],
                ));
            }
        }
    }

    events.sort_by(|a, b| {
        let xa = a.get_location()[0];
        let xb = b.get_location()[0];
        xa.partial_cmp(&xb)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| {
                let ya = a.get_location()[1];
                let yb = b.get_location()[1];
                ya.partial_cmp(&yb).unwrap_or(std::cmp::Ordering::Equal)
            })
            .then_with(|| event_kind_ordinal(a.get_kind()).cmp(&event_kind_ordinal(b.get_kind())))
    });

    BoundaryArrangement::new(segments.to_vec(), events)
}

/// Deterministic ordering of event kinds.
fn event_kind_ordinal(kind: BoundaryEventKind) -> u8 {
    match kind {
        BoundaryEventKind::DegenerateSegment => 0,
        BoundaryEventKind::OverlapStart => 1,
        BoundaryEventKind::OverlapEnd => 2,
        BoundaryEventKind::EndpointTouch => 3,
        BoundaryEventKind::ProperCrossing => 4,
    }
}

/// Check if two collinear segments overlap using exact coordinate comparisons.
///
/// Projects onto the axis with greater extent (no floating-point threshold).
fn segments_collinear_overlap_exact(
    a0: [f64; 2], a1: [f64; 2],
    b0: [f64; 2], b1: [f64; 2],
) -> bool {
    let axis = if (a1[0] - a0[0]).abs() > (a1[1] - a0[1]).abs() { 0 } else { 1 };

    let (a_min, a_max) = if a0[axis] <= a1[axis] {
        (a0[axis], a1[axis])
    } else {
        (a1[axis], a0[axis])
    };
    let (b_min, b_max) = if b0[axis] <= b1[axis] {
        (b0[axis], b1[axis])
    } else {
        (b1[axis], b0[axis])
    };

    a_min < b_max && b_min < a_max
}

/// Find the location of an endpoint-touch event.
fn find_touch_location(
    a0: [f64; 2], a1: [f64; 2],
    b0: [f64; 2], b1: [f64; 2],
    d1: TriSign, d2: TriSign, d3: TriSign, d4: TriSign,
) -> [f64; 2] {
    if d3 == TriSign::Zero { return a0; }
    if d4 == TriSign::Zero { return a1; }
    if d1 == TriSign::Zero { return b0; }
    if d2 == TriSign::Zero { return b1; }
    [(a0[0] + b0[0]) * 0.5, (a0[1] + b0[1]) * 0.5]
}

/// Classify an arrangement into a WeakSimpleCertificate.
///
/// Akitaya-inspired approach:
/// - Any ProperCrossing → Rejected { SelfCrossing }
/// - Any OverlapStart/OverlapEnd → Rejected { OverlappingSegments }
/// - Any DegenerateSegment → Rejected { DegenerateBoundary }
/// - Only EndpointTouch events → WeaklySimple { touch_count }
/// - No events → Simple
fn classify_arrangement(arrangement: &BoundaryArrangement) -> WeakSimpleCertificate {
    let events = arrangement.get_events();

    if events.is_empty() {
        return WeakSimpleCertificate::Simple;
    }

    for event in events {
        match event.get_kind() {
            BoundaryEventKind::ProperCrossing => {
                return WeakSimpleCertificate::Rejected {
                    reason: BoundaryRejectReason::SelfCrossing,
                    witness: event.get_location(),
                };
            }
            BoundaryEventKind::OverlapStart | BoundaryEventKind::OverlapEnd => {
                return WeakSimpleCertificate::Rejected {
                    reason: BoundaryRejectReason::OverlappingSegments,
                    witness: event.get_location(),
                };
            }
            BoundaryEventKind::DegenerateSegment => {
                return WeakSimpleCertificate::Rejected {
                    reason: BoundaryRejectReason::DegenerateBoundary,
                    witness: event.get_location(),
                };
            }
            BoundaryEventKind::EndpointTouch => {}
        }
    }

    let touch_count = events
        .iter()
        .filter(|e| e.get_kind() == BoundaryEventKind::EndpointTouch)
        .count();

    WeakSimpleCertificate::WeaklySimple { touch_count }
}
