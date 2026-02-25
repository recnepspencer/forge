//! Split parameter collection and atomic segment emission.
//!
//! DOMAIN: Converts a list of potentially intersecting boundary segments into
//! a set of explicitly non-intersecting AtomicSegment2Ds, divided exactly at intersections.
//!
//! DEPENDENCIES: `forge_geom::algorithms::boundary_cert::exact_intersect`, `forge_math::arithmetic::Rational`

use std::collections::{BTreeMap, BTreeSet};

use forge_math::arithmetic::rational::Rational;

use super::schema::{BoundaryCertError, Segment2D};
use super::exact_intersect::{intersect_segments_exact, ExactIntersection, ExactParam};

/// An unbroken segment of the boundary arrangement.
/// Guaranteed to contain no interior geometric events.
#[derive(Debug, Clone, PartialEq)]
pub struct AtomicSegment2D {
    pub start: [f64; 2],
    pub end: [f64; 2],
    pub start_exact: [Rational; 2],
    pub end_exact: [Rational; 2],
    pub source_segment: usize,
    pub t_range: [ExactParam; 2],
}

/// A certified exact vertex in the planar arrangement graph.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArrangementVertexId(usize);

#[derive(Debug, Clone, PartialEq)]
pub struct ArrangementVertex {
    pub position: [f64; 2],
    pub exact_position: [Rational; 2],
    pub incident_atomic_edges: Vec<usize>,
    pub incident_sources: Vec<usize>,
}

pub fn compute_splits(
    segments: &[Segment2D],
) -> Result<(Vec<AtomicSegment2D>, Vec<ArrangementVertex>), crate::algorithms::boundary_cert::schema::BoundaryCertError> {
    let n = segments.len();
    
    // Store exact split parameters for each segment
    let mut segment_splits: Vec<BTreeSet<ExactParam>> = vec![BTreeSet::new(); n];

    // Seed with endpoints for every segment
    for i in 0..n {
        segment_splits[i].insert(ExactParam::zero());
        segment_splits[i].insert(ExactParam::one());
    }

    // N^2 all-pairs exact intersections
    for i in 0..n {
        for j in (i + 1)..n {
            // Note: If i and j are adjacent in a simple loop, they share an endpoint.
            // The exact intersect will discover it naturally. We don't skip adjacencies
            // because in NMT geometry, a single Segment2D might intersect its sequential neighbor
            // in the middle as well (figure-8 loops).
            
            let intersection = intersect_segments_exact(&segments[i], &segments[j])?;
            match intersection {
                ExactIntersection::Disjoint => {}
                ExactIntersection::Crossing { t_a, t_b } => {
                    segment_splits[i].insert(t_a);
                    segment_splits[j].insert(t_b);
                }
                ExactIntersection::EndpointTouch { touching_seg, t_on_other, .. } => {
                    if touching_seg == 0 {
                        // j touches i; j's endpoint is already at 0 or 1.
                        segment_splits[i].insert(t_on_other);
                    } else {
                        // i touches j.
                        segment_splits[j].insert(t_on_other);
                    }
                }
                ExactIntersection::SharedEndpoint { .. } => {
                    // Shared endpoints are already at t=0 or t=1, which are seeded.
                }
                ExactIntersection::Overlap { t_a_range, .. } => {
                    // Collinear overlap detected between source segments i and j.
                    // Return early: this is an unambiguous OverlappingSegments violation.
                    // Use the approximate midpoint of the overlap on segment i as the witness.
                    let t_mid = t_a_range[0].as_rational().clone()
                        + (t_a_range[1].as_rational().clone() - t_a_range[0].as_rational().clone())
                        * forge_math::arithmetic::rational::Rational::try_from_f64(0.5)
                            .map_err(|_| BoundaryCertError::PredicateFailure)?;
                    let p0 = segments[i].get_start();
                    let p1 = segments[i].get_end();
                    let t_f64 = t_mid.to_f64_approx();
                    let witness = [p0[0] + (p1[0] - p0[0]) * t_f64, p0[1] + (p1[1] - p0[1]) * t_f64];
                    return Err(BoundaryCertError::OverlapDetected(witness));
                }
            }
        }
    }

    let mut atomics: Vec<AtomicSegment2D> = Vec::new();
    let mut exact_vertices: BTreeMap<[Rational; 2], ArrangementVertex> = BTreeMap::new();

    // Generate atomic segments
    for i in 0..n {
        let splits: Vec<&ExactParam> = segment_splits[i].iter().collect();
        for k in 0..(splits.len() - 1) {
            let t_start = splits[k];
            let t_end = splits[k + 1];

            // Filter out exact zero-length segments if they occur.
            // (E.g. multiple events coincident at the exact same rational T).
            if t_start == t_end {
                continue;
            }

            let start_f64 = interpolate_point(segments[i].get_start(), segments[i].get_end(), t_start);
            let end_f64 = interpolate_point(segments[i].get_start(), segments[i].get_end(), t_end);

            let start_exact = interpolate_exact(segments[i].get_start(), segments[i].get_end(), t_start)?;
            let end_exact = interpolate_exact(segments[i].get_start(), segments[i].get_end(), t_end)?;

            let atomic_id = atomics.len();
            atomics.push(AtomicSegment2D {
                start: start_f64,
                end: end_f64,
                start_exact: start_exact.clone(),
                end_exact: end_exact.clone(),
                source_segment: i,
                t_range: [t_start.clone(), t_end.clone()],
            });

            // Register start vertex
            let start_vtx = exact_vertices.entry(start_exact.clone()).or_insert_with(|| ArrangementVertex {
                position: start_f64,
                exact_position: start_exact,
                incident_atomic_edges: Vec::new(),
                incident_sources: Vec::new(),
            });
            start_vtx.incident_atomic_edges.push(atomic_id);
            if !start_vtx.incident_sources.contains(&i) {
                start_vtx.incident_sources.push(i);
            }

            // Register end vertex
            let end_vtx = exact_vertices.entry(end_exact.clone()).or_insert_with(|| ArrangementVertex {
                position: end_f64,
                exact_position: end_exact,
                incident_atomic_edges: Vec::new(),
                incident_sources: Vec::new(),
            });
            end_vtx.incident_atomic_edges.push(atomic_id);
            if !end_vtx.incident_sources.contains(&i) {
                end_vtx.incident_sources.push(i);
            }
        }
    }

    let vertices: Vec<ArrangementVertex> = exact_vertices.into_values().collect();

    Ok((atomics, vertices))
}

fn interpolate_point(p1: [f64; 2], p2: [f64; 2], t: &ExactParam) -> [f64; 2] {
    let t_approx = t.as_rational().to_f64_approx();
    [
        p1[0] + (p2[0] - p1[0]) * t_approx,
        p1[1] + (p2[1] - p1[1]) * t_approx,
    ]
}

fn interpolate_exact(p1: [f64; 2], p2: [f64; 2], t: &ExactParam) -> Result<[Rational; 2], crate::algorithms::boundary_cert::schema::BoundaryCertError> {
    use crate::algorithms::boundary_cert::schema::BoundaryCertError;
    let r_p1_x = Rational::try_from_f64(p1[0]).map_err(|_| BoundaryCertError::PredicateFailure)?;
    let r_p1_y = Rational::try_from_f64(p1[1]).map_err(|_| BoundaryCertError::PredicateFailure)?;
    
    if t.is_start() {
        return Ok([r_p1_x, r_p1_y]);
    }
    if t.is_end() {
        let r_p2_x = Rational::try_from_f64(p2[0]).map_err(|_| BoundaryCertError::PredicateFailure)?;
        let r_p2_y = Rational::try_from_f64(p2[1]).map_err(|_| BoundaryCertError::PredicateFailure)?;
        return Ok([r_p2_x, r_p2_y]);
    }

    let r_p2_x = Rational::try_from_f64(p2[0]).map_err(|_| BoundaryCertError::PredicateFailure)?;
    let r_p2_y = Rational::try_from_f64(p2[1]).map_err(|_| BoundaryCertError::PredicateFailure)?;

    let dx = r_p2_x - r_p1_x.clone();
    let dy = r_p2_y - r_p1_y.clone();

    let tr = t.as_rational().clone();

    // start + t * (end - start)
    let rx = r_p1_x + (dx * tr.clone());
    let ry = r_p1_y + (dy * tr);

    Ok([rx, ry])
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn exact_split_crossing() {
        let segments = vec![
            Segment2D::new([0.0, 0.0], [2.0, 2.0], 0),
            Segment2D::new([0.0, 2.0], [2.0, 0.0], 1),
        ];

        let Ok((atomics, vertices)) = compute_splits(&segments) else { panic!("Expected Ok") };

        // 2 segments crossing perfectly in the middle -> 4 atomic segments, 5 vertices
        assert_eq!(atomics.len(), 4);
        assert_eq!(vertices.len(), 5);

        // Find the central vertex
        let center_v = vertices.iter().find(|v| v.incident_atomic_edges.len() == 4).unwrap();
        assert_eq!(center_v.position, [1.0, 1.0]);
        let exact_one = Rational::one();
        assert_eq!(center_v.exact_position, [exact_one.clone(), exact_one]);
    }

    #[test]
    fn exact_split_disjoint() {
        let segments = vec![
            Segment2D::new([0.0, 0.0], [1.0, 0.0], 0),
            Segment2D::new([0.0, 1.0], [1.0, 1.0], 1),
        ];

        let Ok((atomics, vertices)) = compute_splits(&segments) else { panic!("Expected Ok") };

        assert_eq!(atomics.len(), 2);
        assert_eq!(vertices.len(), 4);
    }

    #[test]
    fn collinear_overlap_splits_at_boundaries() {
        // Two collinear segments with a partial overlap: [0,4] and [1,5] on y=0.
        // Since overlap detection is now done at the pair level in compute_splits,
        // this must return Err(OverlapDetected) — not Ok with atomics.
        let segments = vec![
            Segment2D::new([0.0, 0.0], [4.0, 0.0], 0),
            Segment2D::new([1.0, 0.0], [5.0, 0.0], 1),
        ];
        let result = compute_splits(&segments);
        match result {
            Err(BoundaryCertError::OverlapDetected(_)) => {}
            other => panic!("Expected OverlapDetected, got {:?}", other.map(|_| "Ok")),
        }
    }
}
