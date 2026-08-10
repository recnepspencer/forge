//! Arrangement fallback for weakly-simple boundary certification.
//!
//! DOMAIN: Converts exact split results into arrangement classifications and
//! maps their failures to the boundary certification contract.

use worth_math::arithmetic::rational::Rational;

use super::super::schema::{
    BoundaryArrangement, BoundaryCertError, BoundaryRejectReason, Segment2D, WeakSimpleCertificate,
};
use super::super::split::{ArrangementVertex, AtomicSegment2D};

/// Run the exact arrangement certifier after the fast path finds ambiguity.
pub(super) fn run_fallback_certifier(segments: &[Segment2D]) -> WeakSimpleCertificate {
    let arrangement = match build_overlap_free_arrangement(segments) {
        Ok(a) => a,
        Err(e) => return cert_error_to_rejected(e, segments, None),
    };

    match classify_arrangement(&arrangement) {
        Ok(cert) => cert,
        Err(e) => cert_error_to_rejected(e, segments, None),
    }
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

/// Build a boundary arrangement after exact pairwise overlap admission.
fn build_overlap_free_arrangement(
    segments: &[Segment2D],
) -> Result<BoundaryArrangement, BoundaryCertError> {
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
/// The arrangement is already overlap-free because pairwise overlap admission
/// occurs while exact splits are collected. This function therefore owns the
/// remaining ordered phases: degenerate atomics, source tour construction, and
/// high-valence angular interleaving.
fn classify_arrangement(
    arrangement: &BoundaryArrangement,
) -> Result<WeakSimpleCertificate, BoundaryCertError> {
    let atomics = arrangement.get_atomic_segments();
    if let Some(rejection) = reject_degenerate_atomics(atomics) {
        return Ok(rejection);
    }

    let ordered_tour = build_ordered_tour(atomics, arrangement.get_source_segments().len());
    let touch_count =
        match classify_high_valence_vertices(atomics, arrangement.get_vertices(), &ordered_tour)? {
            HighValenceClassification::Rejected(rejection) => return Ok(rejection),
            HighValenceClassification::TouchCount(count) => count,
        };

    if touch_count > 0 {
        Ok(WeakSimpleCertificate::WeaklySimple { touch_count })
    } else {
        Ok(WeakSimpleCertificate::Simple)
    }
}

fn reject_degenerate_atomics(atomics: &[AtomicSegment2D]) -> Option<WeakSimpleCertificate> {
    for atomic in atomics {
        if atomic.t_range[0] == atomic.t_range[1] {
            return Some(WeakSimpleCertificate::Rejected {
                reason: BoundaryRejectReason::DegenerateBoundary,
                witness: atomic.start,
            });
        }
    }
    None
}

fn build_ordered_tour(atomics: &[AtomicSegment2D], n_sources: usize) -> Vec<usize> {
    let mut ordered_tour = Vec::new();
    for src_id in 0..n_sources {
        let mut atomics_for_src: Vec<_> = atomics
            .iter()
            .enumerate()
            .filter(|(_, atomic)| atomic.source_segment == src_id)
            .collect();
        atomics_for_src
            .sort_by(|(_, a), (_, b)| a.t_range[0].as_rational().cmp(b.t_range[0].as_rational()));
        for (idx, _) in atomics_for_src {
            ordered_tour.push(idx);
        }
    }
    ordered_tour
}

enum HighValenceClassification {
    Rejected(WeakSimpleCertificate),
    TouchCount(usize),
}

fn classify_high_valence_vertices(
    atomics: &[AtomicSegment2D],
    vertices: &[ArrangementVertex],
    ordered_tour: &[usize],
) -> Result<HighValenceClassification, BoundaryCertError> {
    let mut touch_count = 0;
    for vertex in vertices {
        if vertex.incident_atomic_edges.len() < 4 {
            continue;
        }
        if let Some(witness) = classify_high_valence_vertex(atomics, vertex, ordered_tour)? {
            return Ok(HighValenceClassification::Rejected(
                WeakSimpleCertificate::Rejected {
                    reason: BoundaryRejectReason::SelfCrossing,
                    witness,
                },
            ));
        }
        touch_count += 1;
    }
    Ok(HighValenceClassification::TouchCount(touch_count))
}

fn classify_high_valence_vertex(
    atomics: &[AtomicSegment2D],
    vertex: &ArrangementVertex,
    ordered_tour: &[usize],
) -> Result<Option<[f64; 2]>, BoundaryCertError> {
    let outgoing = build_outgoing_edges(atomics, vertex)?;
    let radial_pos = build_radial_positions(&outgoing);
    let strands = extract_vertex_strands(ordered_tour, &radial_pos);
    if count_strand_crossings(&strands) > 0 {
        return Ok(Some(vertex.position));
    }
    Ok(None)
}

struct OutgoingEdge<'a> {
    atomic_idx: usize,
    quadrant: u8,
    dx: Rational,
    dy: Rational,
    _other_end_exact: &'a [Rational; 2],
}

fn build_outgoing_edges<'a>(
    atomics: &'a [AtomicSegment2D],
    vertex: &ArrangementVertex,
) -> Result<Vec<OutgoingEdge<'a>>, BoundaryCertError> {
    let mut outgoing: Vec<OutgoingEdge<'_>> = Vec::new();
    for &idx in &vertex.incident_atomic_edges {
        let atomic = &atomics[idx];
        let other_end_exact = if atomic.end_exact == vertex.exact_position {
            &atomic.start_exact
        } else {
            &atomic.end_exact
        };

        let dx = other_end_exact[0].clone() - vertex.exact_position[0].clone();
        let dy = other_end_exact[1].clone() - vertex.exact_position[1].clone();
        let quadrant = get_quadrant_from_exact_vec(&dx, &dy)?;

        outgoing.push(OutgoingEdge {
            atomic_idx: idx,
            quadrant,
            dx,
            dy,
            _other_end_exact: other_end_exact,
        });
    }

    outgoing.sort_by(|a, b| {
        let q_cmp = a.quadrant.cmp(&b.quadrant);
        if q_cmp != std::cmp::Ordering::Equal {
            return q_cmp;
        }
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
    Ok(outgoing)
}

fn build_radial_positions(
    outgoing: &[OutgoingEdge<'_>],
) -> std::collections::HashMap<usize, usize> {
    let mut radial_pos = std::collections::HashMap::new();
    for i in 0..outgoing.len() {
        radial_pos.insert(outgoing[i].atomic_idx, i);
    }
    radial_pos
}

fn extract_vertex_strands(
    ordered_tour: &[usize],
    radial_pos: &std::collections::HashMap<usize, usize>,
) -> Vec<(usize, usize)> {
    let mut strands = Vec::new();
    let m = ordered_tour.len();
    for k in 0..m {
        let e_in = ordered_tour[k];
        let e_out = ordered_tour[(k + 1) % m];

        if radial_pos.contains_key(&e_in) && radial_pos.contains_key(&e_out) {
            let p_in = radial_pos[&e_in];
            let p_out = radial_pos[&e_out];
            if p_in != p_out {
                strands.push((p_in, p_out));
            }
        }
    }
    strands
}

fn count_strand_crossings(strands: &[(usize, usize)]) -> usize {
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
    crossings
}
