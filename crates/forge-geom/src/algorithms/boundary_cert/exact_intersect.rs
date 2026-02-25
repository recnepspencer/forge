//! Exact segment-segment intersection using rational arithmetic.
//!
//! DOMAIN: Computes mathematically exact parameters for 2D segment intersections
//! to build a perfectly robust boundary arrangement graph.
//!
//! DEPENDENCIES: `forge_math::arithmetic::Rational`, `forge_math::predicates::orient2d`
//! INVARIANTS: All `ExactParam` outputs are guaranteed to be true mathematical
//! rational fractions representing the exact intersection parameters.

use forge_math::arithmetic::rational::Rational;
use forge_math::numeric::sign::TriSign;
use forge_math::predicates::orient2d;

use super::schema::Segment2D;
use crate::algorithms::boundary_cert::schema::BoundaryCertError;

/// Exact rational parameter `t ∈ [0, 1]` along a segment.
///
/// Point = start + t * (end - start).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExactParam {
    t: Rational,
}

impl ExactParam {
    /// Unchecked constructor for the known-valid constants t=0 and t=1.
    ///
    /// Only call this with `Rational::zero()` or `Rational::one()`. All
    /// computed parameters from intersection logic must use `try_new`.
    pub(crate) fn zero() -> Self { Self { t: Rational::zero() } }
    pub(crate) fn one() -> Self { Self { t: Rational::one() } }

    /// Checked constructor for computed intersection parameters.
    ///
    /// Returns `Err(BoundaryCertError::OutOfRangeParameter)` if `t` is
    /// outside `[0, 1]`, which indicates a logic error in the caller.
    pub fn try_new(t: Rational) -> Result<Self, BoundaryCertError> {
        if t < Rational::zero() || t > Rational::one() {
            return Err(BoundaryCertError::OutOfRangeParameter);
        }
        Ok(Self { t })
    }

    /// Access the underlying exact rational fraction.
    pub fn as_rational(&self) -> &Rational {
        &self.t
    }

    /// True if parameter is exactly 0.
    pub fn is_start(&self) -> bool {
        self.t == Rational::zero()
    }

    /// True if parameter is exactly 1.
    pub fn is_end(&self) -> bool {
        self.t == Rational::one()
    }
}

/// Identifies which endpoint of a segment is involved in an interaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EndpointSide {
    Start,
    End,
}

/// Classification of the exact intersection between two segments.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExactIntersection {
    /// The segments do not touch or cross.
    Disjoint,
    /// Transverse crossing at exact rational parameters in the strict interiors `(0, 1)`.
    Crossing { t_a: ExactParam, t_b: ExactParam },
    /// An endpoint of one segment lies exactly on the other segment.
    /// `touching_seg` is 0 if an endpoint of A lies on B, 1 if an endpoint of B lies on A.
    /// Only emitted if it's not a shared endpoint. Includes the exact `t` on the *other* segment.
    EndpointTouch {
        touching_seg: usize,
        at_endpoint: EndpointSide,
        t_on_other: ExactParam,
    },
    /// The segments share at least one exact endpoint.
    /// `shared_a` is the endpoint on seg A, `shared_b` is the endpoint on seg B.
    SharedEndpoint {
        shared_a: EndpointSide,
        shared_b: EndpointSide,
    },
    /// The segments are collinear and overlap continuously over exact rational parameter intervals.
    Overlap {
        t_a_range: [ExactParam; 2],
        t_b_range: [ExactParam; 2],
    },
}

/// Compute the exact intersection between two 2D segments.
///
/// This evaluates purely in exact rational arithmetic and guarantees correct
/// classification and parameter values regardless of degeneracy.
pub fn intersect_segments_exact(seg_a: &Segment2D, seg_b: &Segment2D) -> Result<ExactIntersection, BoundaryCertError> {
    let pa1 = seg_a.get_start();
    let pa2 = seg_a.get_end();
    let pb1 = seg_b.get_start();
    let pb2 = seg_b.get_end();

    // Evaluate exact orientations
    let o_a1_a2_b1 = orient2d(pa1, pa2, pb1).map_err(|_| BoundaryCertError::PredicateFailure)?.0;
    let o_a1_a2_b2 = orient2d(pa1, pa2, pb2).map_err(|_| BoundaryCertError::PredicateFailure)?.0;
    let o_b1_b2_a1 = orient2d(pb1, pb2, pa1).map_err(|_| BoundaryCertError::PredicateFailure)?.0;
    let o_b1_b2_a2 = orient2d(pb1, pb2, pa2).map_err(|_| BoundaryCertError::PredicateFailure)?.0;

    // Collinear Overlap
    if o_a1_a2_b1.sign() == TriSign::Zero && o_a1_a2_b2.sign() == TriSign::Zero {
        let overlap = compute_collinear_overlap(pa1, pa2, pb1, pb2)?;
        if let ExactIntersection::Overlap { .. } = overlap {
            return Ok(overlap);
        }
    }

    // Fast check for exact shared endpoints
    let a1_eq_b1 = pa1 == pb1;
    let a1_eq_b2 = pa1 == pb2;
    let a2_eq_b1 = pa2 == pb1;
    let a2_eq_b2 = pa2 == pb2;

    if a1_eq_b1 {
        return Ok(ExactIntersection::SharedEndpoint { shared_a: EndpointSide::Start, shared_b: EndpointSide::Start });
    }
    if a1_eq_b2 {
        return Ok(ExactIntersection::SharedEndpoint { shared_a: EndpointSide::Start, shared_b: EndpointSide::End });
    }
    if a2_eq_b1 {
        return Ok(ExactIntersection::SharedEndpoint { shared_a: EndpointSide::End, shared_b: EndpointSide::Start });
    }
    if a2_eq_b2 {
        return Ok(ExactIntersection::SharedEndpoint { shared_a: EndpointSide::End, shared_b: EndpointSide::End });
    }

    let b_straddles_a = o_a1_a2_b1 != o_a1_a2_b2 && o_a1_a2_b1.sign() != TriSign::Zero && o_a1_a2_b2.sign() != TriSign::Zero;
    let a_straddles_b = o_b1_b2_a1 != o_b1_b2_a2 && o_b1_b2_a1.sign() != TriSign::Zero && o_b1_b2_a2.sign() != TriSign::Zero;

    // Strict transverse crossing
    if b_straddles_a && a_straddles_b {
        let (t_a, t_b) = compute_crossing_parameters(pa1, pa2, pb1, pb2)?;
        return Ok(ExactIntersection::Crossing {
            t_a: ExactParam::try_new(t_a)?,
            t_b: ExactParam::try_new(t_b)?,
        });
    }

    // Check for endpoint touches
    if o_a1_a2_b1.sign() == TriSign::Zero && is_point_on_segment(pb1, pa1, pa2)? {
        let t = compute_t_projection(pb1, pa1, pa2)?;
        return Ok(ExactIntersection::EndpointTouch { touching_seg: 1, at_endpoint: EndpointSide::Start, t_on_other: ExactParam::try_new(t)? });
    }
    if o_a1_a2_b2.sign() == TriSign::Zero && is_point_on_segment(pb2, pa1, pa2)? {
        let t = compute_t_projection(pb2, pa1, pa2)?;
        return Ok(ExactIntersection::EndpointTouch { touching_seg: 1, at_endpoint: EndpointSide::End, t_on_other: ExactParam::try_new(t)? });
    }
    if o_b1_b2_a1.sign() == TriSign::Zero && is_point_on_segment(pa1, pb1, pb2)? {
        let t = compute_t_projection(pa1, pb1, pb2)?;
        return Ok(ExactIntersection::EndpointTouch { touching_seg: 0, at_endpoint: EndpointSide::Start, t_on_other: ExactParam::try_new(t)? });
    }
    if o_b1_b2_a2.sign() == TriSign::Zero && is_point_on_segment(pa2, pb1, pb2)? {
        let t = compute_t_projection(pa2, pb1, pb2)?;
        return Ok(ExactIntersection::EndpointTouch { touching_seg: 0, at_endpoint: EndpointSide::End, t_on_other: ExactParam::try_new(t)? });
    }

    Ok(ExactIntersection::Disjoint)
}

/// Compute exact segment crossing parameters t_a and t_b using rational coefficients.
fn compute_crossing_parameters(pa1: [f64; 2], pa2: [f64; 2], pb1: [f64; 2], pb2: [f64; 2]) -> Result<(Rational, Rational), BoundaryCertError> {
    let r_pa1_x = Rational::try_from_f64(pa1[0]).map_err(|_| BoundaryCertError::PredicateFailure)?;
    let r_pa1_y = Rational::try_from_f64(pa1[1]).map_err(|_| BoundaryCertError::PredicateFailure)?;
    let r_pa2_x = Rational::try_from_f64(pa2[0]).map_err(|_| BoundaryCertError::PredicateFailure)?;
    let r_pa2_y = Rational::try_from_f64(pa2[1]).map_err(|_| BoundaryCertError::PredicateFailure)?;

    let r_pb1_x = Rational::try_from_f64(pb1[0]).map_err(|_| BoundaryCertError::PredicateFailure)?;
    let r_pb1_y = Rational::try_from_f64(pb1[1]).map_err(|_| BoundaryCertError::PredicateFailure)?;
    let r_pb2_x = Rational::try_from_f64(pb2[0]).map_err(|_| BoundaryCertError::PredicateFailure)?;
    let r_pb2_y = Rational::try_from_f64(pb2[1]).map_err(|_| BoundaryCertError::PredicateFailure)?;

    let v_a_x = r_pa2_x.clone() - r_pa1_x.clone();
    let v_a_y = r_pa2_y.clone() - r_pa1_y.clone();

    let v_b_x = r_pb2_x.clone() - r_pb1_x.clone();
    let v_b_y = r_pb2_y.clone() - r_pb1_y.clone();

    // Cross product (v_a x v_b)
    let denom = v_a_x.clone() * v_b_y.clone() - v_a_y.clone() * v_b_x.clone();

    let d_pa_pb_x = r_pa1_x.clone() - r_pb1_x.clone();
    let d_pa_pb_y = r_pa1_y.clone() - r_pb1_y.clone();

    // t_a = (v_b x d_pa_pb) / (v_a x v_b)
    let num_a = v_b_x.clone() * d_pa_pb_y.clone() - v_b_y.clone() * d_pa_pb_x.clone();
    // t_b = (v_a x d_pa_pb) / (v_a x v_b)
    let num_b = v_a_x * d_pa_pb_y - v_a_y * d_pa_pb_x;

    let t_a = num_a / denom.clone();
    let t_b = num_b / denom;

    Ok((t_a, t_b))
}

/// Compute 1D projection parameter t for point p on collinear segment [s1, s2].
fn compute_t_projection(p: [f64; 2], s1: [f64; 2], s2: [f64; 2]) -> Result<Rational, BoundaryCertError> {
    let dx = s2[0] - s1[0];
    let dy = s2[1] - s1[1];
    
    // Pick the dominant axis for stable 1D projection parameter calculation
    let r_p;
    let r_s1;
    let r_s2;

    if dx.abs() > dy.abs() {
        r_p = Rational::try_from_f64(p[0]).map_err(|_| BoundaryCertError::PredicateFailure)?;
        r_s1 = Rational::try_from_f64(s1[0]).map_err(|_| BoundaryCertError::PredicateFailure)?;
        r_s2 = Rational::try_from_f64(s2[0]).map_err(|_| BoundaryCertError::PredicateFailure)?;
    } else {
        r_p = Rational::try_from_f64(p[1]).map_err(|_| BoundaryCertError::PredicateFailure)?;
        r_s1 = Rational::try_from_f64(s1[1]).map_err(|_| BoundaryCertError::PredicateFailure)?;
        r_s2 = Rational::try_from_f64(s2[1]).map_err(|_| BoundaryCertError::PredicateFailure)?;
    }

    Ok((r_p - r_s1.clone()) / (r_s2 - r_s1))
}

/// Exact 1D check if collinear point p lies on [s1, s2].
fn is_point_on_segment(p: [f64; 2], s1: [f64; 2], s2: [f64; 2]) -> Result<bool, BoundaryCertError> {
    let t = compute_t_projection(p, s1, s2)?;
    Ok(t >= Rational::zero() && t <= Rational::one())
}

/// Fast Bounding Box Overlap for Segments
fn bbox_overlap(pa1: [f64; 2], pa2: [f64; 2], pb1: [f64; 2], pb2: [f64; 2]) -> bool {
    let min_ax = pa1[0].min(pa2[0]);
    let max_ax = pa1[0].max(pa2[0]);
    let min_ay = pa1[1].min(pa2[1]);
    let max_ay = pa1[1].max(pa2[1]);

    let min_bx = pb1[0].min(pb2[0]);
    let max_bx = pb1[0].max(pb2[0]);
    let min_by = pb1[1].min(pb2[1]);
    let max_by = pb1[1].max(pb2[1]);

    (max_ax >= min_bx) && (min_ax <= max_bx) && (max_ay >= min_by) && (min_ay <= max_by)
}

/// Compute exact overlap intervals for two collinear overlapping segments.
fn compute_collinear_overlap(pa1: [f64; 2], pa2: [f64; 2], pb1: [f64; 2], pb2: [f64; 2]) -> Result<ExactIntersection, BoundaryCertError> {
    if !bbox_overlap(pa1, pa2, pb1, pb2) {
        return Ok(ExactIntersection::Disjoint);
    }

    let t_b_on_a_1 = compute_t_projection(pb1, pa1, pa2)?;
    let t_b_on_a_2 = compute_t_projection(pb2, pa1, pa2)?;

    let t_a_min = Rational::zero();
    let t_a_max = Rational::one();

    let overlap_a_start = t_a_min.clone().max(t_b_on_a_1.clone().min(t_b_on_a_2.clone()));
    let overlap_a_end = t_a_max.clone().min(t_b_on_a_1.clone().max(t_b_on_a_2.clone()));

    if overlap_a_start >= overlap_a_end {
        return Ok(ExactIntersection::Disjoint);
    }

    let t_a_on_b_1 = compute_t_projection(pa1, pb1, pb2)?;
    let t_a_on_b_2 = compute_t_projection(pa2, pb1, pb2)?;

    let t_b_min = Rational::zero();
    let t_b_max = Rational::one();

    let overlap_b_start = t_b_min.clone().max(t_a_on_b_1.clone().min(t_a_on_b_2.clone()));
    let overlap_b_end = t_b_max.clone().min(t_a_on_b_1.max(t_a_on_b_2));

    Ok(ExactIntersection::Overlap {
        t_a_range: [ExactParam::try_new(overlap_a_start)?, ExactParam::try_new(overlap_a_end)?],
        t_b_range: [ExactParam::try_new(overlap_b_start)?, ExactParam::try_new(overlap_b_end)?],
    })
}
#[cfg(test)]
mod tests {
    use super::*;

    fn r(v: f64) -> Rational {
        Rational::try_from_f64(v).unwrap()
    }

    #[test]
    fn exact_intersect_disjoint() {
        let s1 = Segment2D::new([0.0, 0.0], [2.0, 2.0], 1);
        let s2 = Segment2D::new([0.0, 2.0], [1.0, 3.0], 2);
        assert_eq!(intersect_segments_exact(&s1, &s2), Ok(ExactIntersection::Disjoint));
    }

    #[test]
    fn exact_intersect_crossing() {
        // X-crossing at exactly (1, 1), t = 0.5 for both
        let s1 = Segment2D::new([0.0, 0.0], [2.0, 2.0], 1);
        let s2 = Segment2D::new([0.0, 2.0], [2.0, 0.0], 2);
        
        match intersect_segments_exact(&s1, &s2) {
            Ok(ExactIntersection::Crossing { t_a, t_b }) => {
                assert_eq!(t_a.as_rational(), &r(0.5));
                assert_eq!(t_b.as_rational(), &r(0.5));
            }
            res => panic!("Expected crossing, got {:?}", res),
        }
    }

    #[test]
    fn exact_intersect_crossing_params() {
        // Crossing off-center.
        // s1: y = 0, x from 0 to 4.
        // s2: x = 1, y from -2 to 2.
        // Intersects at (1, 0).
        // t_a = 1/4 = 0.25
        // t_b = 2/4 = 0.5
        let s1 = Segment2D::new([0.0, 0.0], [4.0, 0.0], 1);
        let s2 = Segment2D::new([1.0, -2.0], [1.0, 2.0], 2);

        match intersect_segments_exact(&s1, &s2) {
            Ok(ExactIntersection::Crossing { t_a, t_b }) => {
                assert_eq!(t_a.as_rational(), &r(0.25));
                assert_eq!(t_b.as_rational(), &r(0.5));
            }
            res => panic!("Expected crossing, got {:?}", res),
        }
    }

    #[test]
    fn exact_intersect_shared_endpoint() {
        let s1 = Segment2D::new([0.0, 0.0], [2.0, 2.0], 1);
        let s2 = Segment2D::new([2.0, 2.0], [4.0, 0.0], 2);
        assert_eq!(intersect_segments_exact(&s1, &s2), Ok(ExactIntersection::SharedEndpoint {
            shared_a: EndpointSide::End,
            shared_b: EndpointSide::Start,
        }));
    }

    #[test]
    fn exact_intersect_endpoint_touch() {
        // T-junction at (1, 1). s2 starts at the interior of s1.
        let s1 = Segment2D::new([0.0, 1.0], [2.0, 1.0], 1); // Horizontal
        let s2 = Segment2D::new([1.0, 1.0], [1.0, 3.0], 2); // Vertical upwards

        match intersect_segments_exact(&s1, &s2) {
            Ok(ExactIntersection::EndpointTouch { touching_seg, at_endpoint, t_on_other }) => {
                // seg 2's start point touches seg 1 at t=0.5
                assert_eq!(touching_seg, 1); // s2
                assert_eq!(at_endpoint, EndpointSide::Start);
                assert_eq!(t_on_other.as_rational(), &r(0.5));
            }
            res => panic!("Expected endpoint touch, got {:?}", res),
        }
    }

    #[test]
    fn exact_intersect_collinear_overlap() {
        // Two collinear overlapping segments on the X-axis
        let s1 = Segment2D::new([0.0, 0.0], [4.0, 0.0], 1);
        let s2 = Segment2D::new([1.0, 0.0], [5.0, 0.0], 2);

        match intersect_segments_exact(&s1, &s2) {
            Ok(ExactIntersection::Overlap { t_a_range, t_b_range }) => {
                // Overlap is from x=1 to x=4
                // On s1: t from 1/4 to 4/4 = [0.25, 1.0]
                assert_eq!(t_a_range[0].as_rational(), &r(0.25));
                assert_eq!(t_a_range[1].as_rational(), &r(1.0));
                
                // On s2: t from (1-1)/4 to (4-1)/4 = [0.0, 0.75]
                assert_eq!(t_b_range[0].as_rational(), &r(0.0));
                assert_eq!(t_b_range[1].as_rational(), &r(0.75));
            }
            res => panic!("Expected overlap, got {:?}", res),
        }
    }

    #[test]
    fn exact_intersect_collinear_disjoint() {
        let s1 = Segment2D::new([0.0, 0.0], [2.0, 0.0], 1);
        let s2 = Segment2D::new([3.0, 0.0], [5.0, 0.0], 2);
        assert_eq!(intersect_segments_exact(&s1, &s2), Ok(ExactIntersection::Disjoint));
    }
}
