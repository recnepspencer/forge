//! Exact segment-segment intersection using rational arithmetic.

use worth_math::arithmetic::rational::Rational;
use worth_math::numeric::sign::{CertifiedTriSign, TriSign};
use worth_math::predicates::orient2d;

use super::schema::Segment2D;
use crate::algorithms::boundary_cert::schema::BoundaryCertError;

/// Exact rational parameter `t ∈ [0, 1]` along a segment.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExactParam {
    t: Rational,
}

impl ExactParam {
    /// Unchecked constructor for the known-valid constants t=0 and t=1.
    pub(crate) fn zero() -> Self {
        Self {
            t: Rational::zero(),
        }
    }
    pub(crate) fn one() -> Self {
        Self { t: Rational::one() }
    }

    /// Checked constructor for computed intersection parameters.
    pub fn try_new(t: Rational) -> Result<Self, BoundaryCertError> {
        if t < Rational::zero() || t > Rational::one() {
            return Err(BoundaryCertError::OutOfRangeParameter);
        }
        Ok(Self { t })
    }

    pub fn as_rational(&self) -> &Rational {
        &self.t
    }

    pub fn is_start(&self) -> bool {
        self.t == Rational::zero()
    }

    pub fn is_end(&self) -> bool {
        self.t == Rational::one()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EndpointSide {
    Start,
    End,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExactIntersection {
    Disjoint,
    Crossing {
        t_a: ExactParam,
        t_b: ExactParam,
    },
    EndpointTouch {
        touching_seg: usize,
        at_endpoint: EndpointSide,
        t_on_other: ExactParam,
    },
    SharedEndpoint {
        shared_a: EndpointSide,
        shared_b: EndpointSide,
    },
    Overlap {
        t_a_range: [ExactParam; 2],
        t_b_range: [ExactParam; 2],
    },
}

/// Compute the exact intersection between two 2D segments.
pub fn intersect_segments_exact(
    seg_a: &Segment2D,
    seg_b: &Segment2D,
) -> Result<ExactIntersection, BoundaryCertError> {
    let pa1 = seg_a.get_start();
    let pa2 = seg_a.get_end();
    let pb1 = seg_b.get_start();
    let pb2 = seg_b.get_end();

    let orientations = acquire_orientations(pa1, pa2, pb1, pb2)?;
    if let Some(overlap) = classify_fully_collinear(pa1, pa2, pb1, pb2, &orientations)? {
        return Ok(overlap);
    }
    if let Some(shared_endpoint) = classify_shared_endpoint(pa1, pa2, pb1, pb2) {
        return Ok(shared_endpoint);
    }
    if let Some(crossing) = classify_proper_crossing(pa1, pa2, pb1, pb2, &orientations)? {
        return Ok(crossing);
    }
    if let Some(endpoint_touch) = classify_endpoint_touch(pa1, pa2, pb1, pb2, &orientations)? {
        return Ok(endpoint_touch);
    }
    Ok(ExactIntersection::Disjoint)
}

struct SegmentOrientations {
    a1_a2_b1: CertifiedTriSign,
    a1_a2_b2: CertifiedTriSign,
    b1_b2_a1: CertifiedTriSign,
    b1_b2_a2: CertifiedTriSign,
}

fn acquire_orientations(
    pa1: [f64; 2],
    pa2: [f64; 2],
    pb1: [f64; 2],
    pb2: [f64; 2],
) -> Result<SegmentOrientations, BoundaryCertError> {
    let a1_a2_b1 = orient2d(pa1, pa2, pb1)
        .map_err(|_| BoundaryCertError::PredicateFailure)?
        .0;
    let a1_a2_b2 = orient2d(pa1, pa2, pb2)
        .map_err(|_| BoundaryCertError::PredicateFailure)?
        .0;
    let b1_b2_a1 = orient2d(pb1, pb2, pa1)
        .map_err(|_| BoundaryCertError::PredicateFailure)?
        .0;
    let b1_b2_a2 = orient2d(pb1, pb2, pa2)
        .map_err(|_| BoundaryCertError::PredicateFailure)?
        .0;
    Ok(SegmentOrientations {
        a1_a2_b1,
        a1_a2_b2,
        b1_b2_a1,
        b1_b2_a2,
    })
}

fn classify_fully_collinear(
    pa1: [f64; 2],
    pa2: [f64; 2],
    pb1: [f64; 2],
    pb2: [f64; 2],
    orientations: &SegmentOrientations,
) -> Result<Option<ExactIntersection>, BoundaryCertError> {
    if orientations.a1_a2_b1.sign() == TriSign::Zero
        && orientations.a1_a2_b2.sign() == TriSign::Zero
    {
        let overlap = compute_collinear_overlap(pa1, pa2, pb1, pb2)?;
        if let ExactIntersection::Overlap { .. } = overlap {
            return Ok(Some(overlap));
        }
    }
    Ok(None)
}

fn classify_shared_endpoint(
    pa1: [f64; 2],
    pa2: [f64; 2],
    pb1: [f64; 2],
    pb2: [f64; 2],
) -> Option<ExactIntersection> {
    let a1_eq_b1 = pa1 == pb1;
    let a1_eq_b2 = pa1 == pb2;
    let a2_eq_b1 = pa2 == pb1;
    let a2_eq_b2 = pa2 == pb2;

    if a1_eq_b1 {
        return Some(ExactIntersection::SharedEndpoint {
            shared_a: EndpointSide::Start,
            shared_b: EndpointSide::Start,
        });
    }
    if a1_eq_b2 {
        return Some(ExactIntersection::SharedEndpoint {
            shared_a: EndpointSide::Start,
            shared_b: EndpointSide::End,
        });
    }
    if a2_eq_b1 {
        return Some(ExactIntersection::SharedEndpoint {
            shared_a: EndpointSide::End,
            shared_b: EndpointSide::Start,
        });
    }
    if a2_eq_b2 {
        return Some(ExactIntersection::SharedEndpoint {
            shared_a: EndpointSide::End,
            shared_b: EndpointSide::End,
        });
    }
    None
}

fn classify_proper_crossing(
    pa1: [f64; 2],
    pa2: [f64; 2],
    pb1: [f64; 2],
    pb2: [f64; 2],
    orientations: &SegmentOrientations,
) -> Result<Option<ExactIntersection>, BoundaryCertError> {
    let b_straddles_a = orientations.a1_a2_b1 != orientations.a1_a2_b2
        && orientations.a1_a2_b1.sign() != TriSign::Zero
        && orientations.a1_a2_b2.sign() != TriSign::Zero;
    let a_straddles_b = orientations.b1_b2_a1 != orientations.b1_b2_a2
        && orientations.b1_b2_a1.sign() != TriSign::Zero
        && orientations.b1_b2_a2.sign() != TriSign::Zero;

    if b_straddles_a && a_straddles_b {
        let (t_a, t_b) = compute_crossing_parameters(pa1, pa2, pb1, pb2)?;
        return Ok(Some(ExactIntersection::Crossing {
            t_a: ExactParam::try_new(t_a)?,
            t_b: ExactParam::try_new(t_b)?,
        }));
    }
    Ok(None)
}

fn classify_endpoint_touch(
    pa1: [f64; 2],
    pa2: [f64; 2],
    pb1: [f64; 2],
    pb2: [f64; 2],
    orientations: &SegmentOrientations,
) -> Result<Option<ExactIntersection>, BoundaryCertError> {
    if orientations.a1_a2_b1.sign() == TriSign::Zero && is_point_on_segment(pb1, pa1, pa2)? {
        let t = compute_t_projection(pb1, pa1, pa2)?;
        return Ok(Some(ExactIntersection::EndpointTouch {
            touching_seg: 1,
            at_endpoint: EndpointSide::Start,
            t_on_other: ExactParam::try_new(t)?,
        }));
    }
    if orientations.a1_a2_b2.sign() == TriSign::Zero && is_point_on_segment(pb2, pa1, pa2)? {
        let t = compute_t_projection(pb2, pa1, pa2)?;
        return Ok(Some(ExactIntersection::EndpointTouch {
            touching_seg: 1,
            at_endpoint: EndpointSide::End,
            t_on_other: ExactParam::try_new(t)?,
        }));
    }
    if orientations.b1_b2_a1.sign() == TriSign::Zero && is_point_on_segment(pa1, pb1, pb2)? {
        let t = compute_t_projection(pa1, pb1, pb2)?;
        return Ok(Some(ExactIntersection::EndpointTouch {
            touching_seg: 0,
            at_endpoint: EndpointSide::Start,
            t_on_other: ExactParam::try_new(t)?,
        }));
    }
    if orientations.b1_b2_a2.sign() == TriSign::Zero && is_point_on_segment(pa2, pb1, pb2)? {
        let t = compute_t_projection(pa2, pb1, pb2)?;
        return Ok(Some(ExactIntersection::EndpointTouch {
            touching_seg: 0,
            at_endpoint: EndpointSide::End,
            t_on_other: ExactParam::try_new(t)?,
        }));
    }
    Ok(None)
}

/// Compute exact segment crossing parameters t_a and t_b using rational coefficients.
fn compute_crossing_parameters(
    pa1: [f64; 2],
    pa2: [f64; 2],
    pb1: [f64; 2],
    pb2: [f64; 2],
) -> Result<(Rational, Rational), BoundaryCertError> {
    let r_pa1_x =
        Rational::try_from_f64(pa1[0]).map_err(|_| BoundaryCertError::PredicateFailure)?;
    let r_pa1_y =
        Rational::try_from_f64(pa1[1]).map_err(|_| BoundaryCertError::PredicateFailure)?;
    let r_pa2_x =
        Rational::try_from_f64(pa2[0]).map_err(|_| BoundaryCertError::PredicateFailure)?;
    let r_pa2_y =
        Rational::try_from_f64(pa2[1]).map_err(|_| BoundaryCertError::PredicateFailure)?;

    let r_pb1_x =
        Rational::try_from_f64(pb1[0]).map_err(|_| BoundaryCertError::PredicateFailure)?;
    let r_pb1_y =
        Rational::try_from_f64(pb1[1]).map_err(|_| BoundaryCertError::PredicateFailure)?;
    let r_pb2_x =
        Rational::try_from_f64(pb2[0]).map_err(|_| BoundaryCertError::PredicateFailure)?;
    let r_pb2_y =
        Rational::try_from_f64(pb2[1]).map_err(|_| BoundaryCertError::PredicateFailure)?;

    let v_a_x = r_pa2_x.clone() - r_pa1_x.clone();
    let v_a_y = r_pa2_y.clone() - r_pa1_y.clone();

    let v_b_x = r_pb2_x.clone() - r_pb1_x.clone();
    let v_b_y = r_pb2_y.clone() - r_pb1_y.clone();

    let denom = v_a_x.clone() * v_b_y.clone() - v_a_y.clone() * v_b_x.clone();

    let d_pa_pb_x = r_pa1_x.clone() - r_pb1_x.clone();
    let d_pa_pb_y = r_pa1_y.clone() - r_pb1_y.clone();

    let num_a = v_b_x.clone() * d_pa_pb_y.clone() - v_b_y.clone() * d_pa_pb_x.clone();
    let num_b = v_a_x * d_pa_pb_y - v_a_y * d_pa_pb_x;

    let t_a = num_a / denom.clone();
    let t_b = num_b / denom;

    Ok((t_a, t_b))
}

/// Compute 1D projection parameter t for point p on collinear segment [s1, s2].
fn compute_t_projection(
    p: [f64; 2],
    s1: [f64; 2],
    s2: [f64; 2],
) -> Result<Rational, BoundaryCertError> {
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
fn compute_collinear_overlap(
    pa1: [f64; 2],
    pa2: [f64; 2],
    pb1: [f64; 2],
    pb2: [f64; 2],
) -> Result<ExactIntersection, BoundaryCertError> {
    if !bbox_overlap(pa1, pa2, pb1, pb2) {
        return Ok(ExactIntersection::Disjoint);
    }

    let t_b_on_a_1 = compute_t_projection(pb1, pa1, pa2)?;
    let t_b_on_a_2 = compute_t_projection(pb2, pa1, pa2)?;

    let t_a_min = Rational::zero();
    let t_a_max = Rational::one();

    let overlap_a_start = t_a_min
        .clone()
        .max(t_b_on_a_1.clone().min(t_b_on_a_2.clone()));
    let overlap_a_end = t_a_max
        .clone()
        .min(t_b_on_a_1.clone().max(t_b_on_a_2.clone()));

    if overlap_a_start >= overlap_a_end {
        return Ok(ExactIntersection::Disjoint);
    }

    let t_a_on_b_1 = compute_t_projection(pa1, pb1, pb2)?;
    let t_a_on_b_2 = compute_t_projection(pa2, pb1, pb2)?;

    let t_b_min = Rational::zero();
    let t_b_max = Rational::one();

    let overlap_b_start = t_b_min
        .clone()
        .max(t_a_on_b_1.clone().min(t_a_on_b_2.clone()));
    let overlap_b_end = t_b_max.clone().min(t_a_on_b_1.max(t_a_on_b_2));

    Ok(ExactIntersection::Overlap {
        t_a_range: [
            ExactParam::try_new(overlap_a_start)?,
            ExactParam::try_new(overlap_a_end)?,
        ],
        t_b_range: [
            ExactParam::try_new(overlap_b_start)?,
            ExactParam::try_new(overlap_b_end)?,
        ],
    })
}
#[cfg(test)]
mod tests;
