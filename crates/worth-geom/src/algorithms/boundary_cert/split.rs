//! Split parameter collection and atomic segment emission.
//!
//! DOMAIN: Converts a list of potentially intersecting boundary segments into
//! a set of explicitly non-intersecting AtomicSegment2Ds, divided exactly at intersections.
//!
//! DEPENDENCIES: `worth_geom::algorithms::boundary_cert::exact_intersect`, `worth_math::arithmetic::Rational`

use std::collections::{BTreeMap, BTreeSet};

use worth_math::arithmetic::rational::Rational;

use super::exact_intersect::{intersect_segments_exact, ExactIntersection, ExactParam};
use super::schema::{BoundaryCertError, Segment2D};

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
) -> Result<
    (Vec<AtomicSegment2D>, Vec<ArrangementVertex>),
    crate::algorithms::boundary_cert::schema::BoundaryCertError,
> {
    let mut segment_splits = seed_endpoint_splits(segments.len());
    collect_pair_intersections(segments, &mut segment_splits)?;
    let (atomics, exact_vertices) = materialize_atomic_segments(segments, &segment_splits)?;
    let vertices: Vec<ArrangementVertex> = exact_vertices.into_values().collect();

    Ok((atomics, vertices))
}

fn seed_endpoint_splits(segment_count: usize) -> Vec<BTreeSet<ExactParam>> {
    let mut segment_splits: Vec<BTreeSet<ExactParam>> = vec![BTreeSet::new(); segment_count];
    for splits in &mut segment_splits {
        splits.insert(ExactParam::zero());
        splits.insert(ExactParam::one());
    }
    segment_splits
}

fn collect_pair_intersections(
    segments: &[Segment2D],
    segment_splits: &mut [BTreeSet<ExactParam>],
) -> Result<(), BoundaryCertError> {
    let n = segments.len();
    for i in 0..n {
        for j in (i + 1)..n {
            let intersection = intersect_segments_exact(&segments[i], &segments[j])?;
            match intersection {
                ExactIntersection::Disjoint => {}
                ExactIntersection::Crossing { t_a, t_b } => {
                    segment_splits[i].insert(t_a);
                    segment_splits[j].insert(t_b);
                }
                ExactIntersection::EndpointTouch {
                    touching_seg,
                    t_on_other,
                    ..
                } => {
                    if touching_seg == 0 {
                        segment_splits[i].insert(t_on_other);
                    } else {
                        segment_splits[j].insert(t_on_other);
                    }
                }
                ExactIntersection::SharedEndpoint { .. } => {}
                ExactIntersection::Overlap { t_a_range, .. } => {
                    let witness = build_overlap_witness(segments, i, t_a_range)?;
                    return Err(BoundaryCertError::OverlapDetected(witness));
                }
            }
        }
    }
    Ok(())
}

fn build_overlap_witness(
    segments: &[Segment2D],
    segment_idx: usize,
    t_a_range: [ExactParam; 2],
) -> Result<[f64; 2], BoundaryCertError> {
    let t_mid = t_a_range[0].as_rational().clone()
        + (t_a_range[1].as_rational().clone() - t_a_range[0].as_rational().clone())
            * Rational::try_from_f64(0.5).map_err(|_| BoundaryCertError::PredicateFailure)?;
    let p0 = segments[segment_idx].get_start();
    let p1 = segments[segment_idx].get_end();
    let t_f64 = t_mid.to_f64_approx();
    Ok([
        p0[0] + (p1[0] - p0[0]) * t_f64,
        p0[1] + (p1[1] - p0[1]) * t_f64,
    ])
}

fn materialize_atomic_segments(
    segments: &[Segment2D],
    segment_splits: &[BTreeSet<ExactParam>],
) -> Result<
    (
        Vec<AtomicSegment2D>,
        BTreeMap<[Rational; 2], ArrangementVertex>,
    ),
    BoundaryCertError,
> {
    let mut atomics: Vec<AtomicSegment2D> = Vec::new();
    let mut exact_vertices: BTreeMap<[Rational; 2], ArrangementVertex> = BTreeMap::new();

    for i in 0..segments.len() {
        let splits: Vec<&ExactParam> = segment_splits[i].iter().collect();
        for k in 0..(splits.len() - 1) {
            let t_start = splits[k];
            let t_end = splits[k + 1];
            if t_start == t_end {
                continue;
            }

            let start_f64 =
                interpolate_point(segments[i].get_start(), segments[i].get_end(), t_start);
            let end_f64 = interpolate_point(segments[i].get_start(), segments[i].get_end(), t_end);
            let start_exact =
                interpolate_exact(segments[i].get_start(), segments[i].get_end(), t_start)?;
            let end_exact =
                interpolate_exact(segments[i].get_start(), segments[i].get_end(), t_end)?;

            let atomic_id = atomics.len();
            atomics.push(AtomicSegment2D {
                start: start_f64,
                end: end_f64,
                start_exact: start_exact.clone(),
                end_exact: end_exact.clone(),
                source_segment: i,
                t_range: [t_start.clone(), t_end.clone()],
            });

            register_atomic_vertex(&mut exact_vertices, start_f64, start_exact, atomic_id, i);
            register_atomic_vertex(&mut exact_vertices, end_f64, end_exact, atomic_id, i);
        }
    }

    Ok((atomics, exact_vertices))
}

fn register_atomic_vertex(
    exact_vertices: &mut BTreeMap<[Rational; 2], ArrangementVertex>,
    position: [f64; 2],
    exact_position: [Rational; 2],
    atomic_id: usize,
    source_segment: usize,
) {
    let vertex = exact_vertices
        .entry(exact_position.clone())
        .or_insert_with(|| ArrangementVertex {
            position,
            exact_position,
            incident_atomic_edges: Vec::new(),
            incident_sources: Vec::new(),
        });
    vertex.incident_atomic_edges.push(atomic_id);
    if !vertex.incident_sources.contains(&source_segment) {
        vertex.incident_sources.push(source_segment);
    }
}

fn interpolate_point(p1: [f64; 2], p2: [f64; 2], t: &ExactParam) -> [f64; 2] {
    let t_approx = t.as_rational().to_f64_approx();
    [
        p1[0] + (p2[0] - p1[0]) * t_approx,
        p1[1] + (p2[1] - p1[1]) * t_approx,
    ]
}

fn interpolate_exact(
    p1: [f64; 2],
    p2: [f64; 2],
    t: &ExactParam,
) -> Result<[Rational; 2], crate::algorithms::boundary_cert::schema::BoundaryCertError> {
    use crate::algorithms::boundary_cert::schema::BoundaryCertError;
    let r_p1_x = Rational::try_from_f64(p1[0]).map_err(|_| BoundaryCertError::PredicateFailure)?;
    let r_p1_y = Rational::try_from_f64(p1[1]).map_err(|_| BoundaryCertError::PredicateFailure)?;

    if t.is_start() {
        return Ok([r_p1_x, r_p1_y]);
    }
    if t.is_end() {
        let r_p2_x =
            Rational::try_from_f64(p2[0]).map_err(|_| BoundaryCertError::PredicateFailure)?;
        let r_p2_y =
            Rational::try_from_f64(p2[1]).map_err(|_| BoundaryCertError::PredicateFailure)?;
        return Ok([r_p2_x, r_p2_y]);
    }

    let r_p2_x = Rational::try_from_f64(p2[0]).map_err(|_| BoundaryCertError::PredicateFailure)?;
    let r_p2_y = Rational::try_from_f64(p2[1]).map_err(|_| BoundaryCertError::PredicateFailure)?;

    let dx = r_p2_x - r_p1_x.clone();
    let dy = r_p2_y - r_p1_y.clone();
    let tr = t.as_rational().clone();
    let rx = r_p1_x + (dx * tr.clone());
    let ry = r_p1_y + (dy * tr);

    Ok([rx, ry])
}

#[cfg(test)]
mod tests {
    use super::{compute_splits, BoundaryCertError, Segment2D};
    use worth_math::arithmetic::rational::Rational;

    #[test]
    fn exact_split_crossing() {
        let segments = vec![
            Segment2D::new([0.0, 0.0], [2.0, 2.0], 0),
            Segment2D::new([0.0, 2.0], [2.0, 0.0], 1),
        ];

        let Ok((atomics, vertices)) = compute_splits(&segments) else {
            panic!("Expected Ok")
        };

        assert_eq!(atomics.len(), 4);
        assert_eq!(vertices.len(), 5);

        let center_v = vertices
            .iter()
            .find(|v| v.incident_atomic_edges.len() == 4)
            .unwrap();
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

        let Ok((atomics, vertices)) = compute_splits(&segments) else {
            panic!("Expected Ok")
        };

        assert_eq!(atomics.len(), 2);
        assert_eq!(vertices.len(), 4);
    }

    #[test]
    fn collinear_overlap_splits_at_boundaries() {
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
