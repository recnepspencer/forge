//! Boundary certification algorithms.
//!
//! DOMAIN: Stateless 2D boundary certification for merge eligibility.
//! Two-phase algorithm: fast-path crossing check → fallback arrangement-based
//! weakly-simple recognizer (Akitaya-inspired, SoCG 2016).
//!
//! DEPENDENCIES: `worth-math` (orient2d exact predicates), `schema` types.
//! INVARIANTS: All functions are pure. No topology, no policy, no thresholds.
//! Certification predicates use exact Shewchuk orient2d (spec §4.5).
//!
//! NOTE: The fallback certifier constructs a rigorous planar arrangement by
//! calculating exact intersections and geometrically subdividing all segments.
//! Akitaya's recognition criteria are then applied to the resulting exact
//! arrangement graph combinatorial strands.

use worth_math::arithmetic::rational::Rational;
use worth_math::numeric::sign::TriSign;

use super::schema::{
    BoundaryArrangement, BoundaryCertError, BoundaryRejectReason, ProjectedBoundary2D,
    ProjectionFrame2D, Segment2D, WeakSimpleCertificate,
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
        FastPathResult::NeedsFallback => run_fallback_certifier(segments),
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

/// Map a `BoundaryCertError` to the best available `WeakSimpleCertificate::Rejected` variant.
///
/// Uses the segment that triggered the error (by index, if known) to produce a meaningful
/// witness point rather than always falling back to the first segment's start.
fn cert_error_to_rejected(
    err: BoundaryCertError,
    segments: &[Segment2D],
    offending_segment_idx: Option<usize>,
) -> WeakSimpleCertificate {
    let witness = offending_segment_idx
        .and_then(|i| segments.get(i))
        .map(|s| s.get_start())
        .unwrap_or_else(|| segments[0].get_start());

    let reason = match err {
        BoundaryCertError::OverlapDetected(w) => {
            return WeakSimpleCertificate::Rejected {
                reason: BoundaryRejectReason::OverlappingSegments,
                witness: w,
            };
        }
        BoundaryCertError::OutOfRangeParameter => BoundaryRejectReason::DegenerateBoundary,
        BoundaryCertError::PredicateFailure => BoundaryRejectReason::DegenerateBoundary,
        BoundaryCertError::DegenerateVector => BoundaryRejectReason::DegenerateBoundary,
    };

    WeakSimpleCertificate::Rejected { reason, witness }
}

/// Build exact arrangement graph and fully classify it. Akitaya-style 2D graph classification,
/// then determines the weakly-simple certificate from the event types. If exact predicates fail,
/// rejects the boundary with the specific error kind and the offending segment witness.
fn run_fallback_certifier(segments: &[Segment2D]) -> WeakSimpleCertificate {
    let arrangement = match build_arrangement(segments) {
        Ok(a) => a,
        Err(e) => return cert_error_to_rejected(e, segments, None),
    };

    match classify_arrangement(&arrangement) {
        Ok(cert) => cert,
        Err(e) => cert_error_to_rejected(e, segments, None),
    }
}

/// Build a boundary arrangement by computing exact atomic splits and vertices.
fn build_arrangement(segments: &[Segment2D]) -> Result<BoundaryArrangement, BoundaryCertError> {
    let (atomics, vertices) = crate::algorithms::boundary_cert::split::compute_splits(segments)?;

    Ok(BoundaryArrangement::new(
        segments.to_vec(),
        atomics,
        vertices,
    ))
}

/// Returns the CCW quadrant [0, 3] for an exact direction vector `(dx, dy)`.
///
/// Quadrant assignment (standard math convention):
/// - Q0: dx >= 0, dy >= 0 (but not both zero)
/// - Q1: dx < 0, dy >= 0
/// - Q2: dx < 0, dy < 0
/// - Q3: dx >= 0, dy < 0
///
/// Returns `Err(DegenerateVector)` if both components are exactly zero.
fn get_quadrant_from_exact_vec(dx: &Rational, dy: &Rational) -> Result<u8, BoundaryCertError> {
    let zero = Rational::zero();
    let dx_pos = *dx > zero;
    let dx_zero = *dx == zero;
    let dy_pos = *dy > zero;
    let dy_zero = *dy == zero;

    if dx_zero && dy_zero {
        return Err(BoundaryCertError::DegenerateVector);
    }

    if dx_pos || dx_zero {
        if dy_pos || (dy_zero && dx_pos) {
            return Ok(0);
        }
        return Ok(3);
    } else {
        if dy_pos || (dy_zero && !dx_pos) {
            return Ok(1);
        }
        return Ok(2);
    }
}

/// Classify an arrangement graph into a WeakSimpleCertificate.
///
/// Akitaya-inspired approach on explicit arrangement graph:
/// 1. Reject degenerate atomic segments
/// 2. Reject overlaps (multiple atomics sharing exact endpoints)
/// 3. Check high-valence vertices (>= 4 incident atomics)
/// 4. Angularly sort incident edges at high-valence vertices.
/// 5. Check interleaving pattern. ABAB = crossing (Reject), AABB = touch (Admit).
fn classify_arrangement(
    arrangement: &BoundaryArrangement,
) -> Result<WeakSimpleCertificate, BoundaryCertError> {
    let atomics = arrangement.get_atomic_segments();
    let vertices = arrangement.get_vertices();

    // 1. Degenerate atomics
    for atomic in atomics {
        if atomic.t_range[0] == atomic.t_range[1] {
            return Ok(WeakSimpleCertificate::Rejected {
                reason: BoundaryRejectReason::DegenerateBoundary,
                witness: atomic.start,
            });
        }
    }

    let n_sources = arrangement.get_source_segments().len();
    let mut ordered_tour = Vec::new();
    for src_id in 0..n_sources {
        let mut atomics_for_src: Vec<_> = atomics
            .iter()
            .enumerate()
            .filter(|(_, a)| a.source_segment == src_id)
            .collect();
        atomics_for_src
            .sort_by(|(_, a), (_, b)| a.t_range[0].as_rational().cmp(b.t_range[0].as_rational()));
        for (idx, _) in atomics_for_src {
            ordered_tour.push(idx);
        }
    }

    let mut touch_count = 0;

    // Analyze each vertex in the graph
    for v in vertices {
        let incident_edges = &v.incident_atomic_edges;

        // High-valence vertex: potential touch, crossing, or overlap
        if incident_edges.len() >= 4 {
            // Pre-compute exact outgoing direction vectors and quadrants before sorting.
            // This eliminates all fallible operations from sort closures — errors are
            // propagated eagerly here so that sort_by operates on infallible Rational values.
            struct OutgoingEdge<'a> {
                atomic_idx: usize,
                quadrant: u8,
                dx: Rational,
                dy: Rational,
                _other_end_exact: &'a [Rational; 2],
            }

            let mut outgoing: Vec<OutgoingEdge<'_>> = Vec::new();
            for &idx in incident_edges {
                let atomic = &atomics[idx];
                let other_end_exact = if atomic.end_exact == v.exact_position {
                    &atomic.start_exact
                } else {
                    &atomic.end_exact
                };

                let dx = other_end_exact[0].clone() - v.exact_position[0].clone();
                let dy = other_end_exact[1].clone() - v.exact_position[1].clone();
                let quadrant = get_quadrant_from_exact_vec(&dx, &dy)?;

                outgoing.push(OutgoingEdge {
                    atomic_idx: idx,
                    quadrant,
                    dx,
                    dy,
                    _other_end_exact: other_end_exact,
                });
            }

            // Sort by (quadrant, then cross-product within quadrant) — both infallible
            outgoing.sort_by(|a, b| {
                let q_cmp = a.quadrant.cmp(&b.quadrant);
                if q_cmp != std::cmp::Ordering::Equal {
                    return q_cmp;
                }
                // Same quadrant: cross product (a x b) > 0 means a is CCW of b
                let cross = a.dx.clone() * b.dy.clone() - a.dy.clone() * b.dx.clone();
                let zero = Rational::zero();
                if cross > zero {
                    std::cmp::Ordering::Less
                } else if cross < zero {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            });

            // Collinear overlap detection has been moved to compute_splits, where exact
            // pair-level intersection data is available. By the time we reach the arrangement
            // here, any OverlappingSegments intersection has already been reported.
            let mut radial_pos = std::collections::HashMap::new();
            for i in 0..outgoing.len() {
                radial_pos.insert(outgoing[i].atomic_idx, i);
            }

            // Extract topological strands from the global ordered tour
            let mut strands = Vec::new();
            let m = ordered_tour.len();
            for k in 0..m {
                let e_in = ordered_tour[k];
                let e_out = ordered_tour[(k + 1) % m];

                if radial_pos.contains_key(&e_in) && radial_pos.contains_key(&e_out) {
                    let p_in = radial_pos[&e_in];
                    let p_out = radial_pos[&e_out];
                    // Skip spurs (U-turns on the exact same ray)
                    if p_in != p_out {
                        strands.push((p_in, p_out));
                    }
                }
            }

            // Strand Interleaving Check (Akitaya): test pairs of strands for intersection
            let mut crossings = 0;
            for i in 0..strands.len() {
                for j in (i + 1)..strands.len() {
                    let (p1, p2) = strands[i];
                    let (q1, q2) = strands[j];

                    let (min1, max1) = if p1 < p2 { (p1, p2) } else { (p2, p1) };

                    let q1_in = q1 > min1 && q1 < max1;
                    let q2_in = q2 > min1 && q2 < max1;

                    if q1_in != q2_in {
                        crossings += 1;
                    }
                }
            }

            if crossings > 0 {
                return Ok(WeakSimpleCertificate::Rejected {
                    reason: BoundaryRejectReason::SelfCrossing,
                    witness: v.position,
                });
            }

            // Admissible touch
            touch_count += 1;
        }
    }

    if touch_count > 0 {
        Ok(WeakSimpleCertificate::WeaklySimple { touch_count })
    } else {
        Ok(WeakSimpleCertificate::Simple)
    }
}
