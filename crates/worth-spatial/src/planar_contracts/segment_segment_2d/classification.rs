use worth_math::sign::TriSign;

use super::basis::CertifiedSegmentSegment2DBasis;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CertifiedSegmentSegment2DClassification {
    Disjoint,
    ProperCrossing,
    EndpointTouch,
    CollinearDisjoint,
    CollinearOverlap,
    Identical,
    ReverseIdentical,
    PolicyRequiredOrUncertain,
}

impl CertifiedSegmentSegment2DClassification {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Disjoint => "disjoint",
            Self::ProperCrossing => "proper-crossing",
            Self::EndpointTouch => "endpoint-touch",
            Self::CollinearDisjoint => "collinear-disjoint",
            Self::CollinearOverlap => "collinear-overlap",
            Self::Identical => "identical",
            Self::ReverseIdentical => "reverse-identical",
            Self::PolicyRequiredOrUncertain => "policy-required-or-uncertain",
        }
    }
}

pub(crate) fn classify_segment_segment_2d(
    basis: &CertifiedSegmentSegment2DBasis,
) -> CertifiedSegmentSegment2DClassification {
    let signs = basis.orientation_signs();
    if signs.iter().all(|sign| sign.is_zero()) {
        return classify_collinear(basis);
    }
    if signs.iter().any(|sign| sign.is_zero()) {
        return classify_endpoint_touch_or_disjoint(signs);
    }
    if signs[0] != signs[1] && signs[2] != signs[3] {
        CertifiedSegmentSegment2DClassification::ProperCrossing
    } else {
        CertifiedSegmentSegment2DClassification::Disjoint
    }
}

fn classify_endpoint_touch_or_disjoint(
    signs: [TriSign; 4],
) -> CertifiedSegmentSegment2DClassification {
    let first_pair_touches = signs[0].is_zero() || signs[1].is_zero();
    let second_pair_touches = signs[2].is_zero() || signs[3].is_zero();
    let first_pair_spans = signs[0].is_zero() || signs[1].is_zero() || signs[0] != signs[1];
    let second_pair_spans = signs[2].is_zero() || signs[3].is_zero() || signs[2] != signs[3];
    if first_pair_touches && second_pair_touches && first_pair_spans && second_pair_spans {
        CertifiedSegmentSegment2DClassification::EndpointTouch
    } else {
        CertifiedSegmentSegment2DClassification::Disjoint
    }
}

fn classify_collinear(
    basis: &CertifiedSegmentSegment2DBasis,
) -> CertifiedSegmentSegment2DClassification {
    let a0 = basis.first_start_point_2d();
    let a1 = basis.first_end_point_2d();
    let b0 = basis.second_start_point_2d();
    let b1 = basis.second_end_point_2d();
    if a0 == b0 && a1 == b1 {
        return CertifiedSegmentSegment2DClassification::Identical;
    }
    if a0 == b1 && a1 == b0 {
        return CertifiedSegmentSegment2DClassification::ReverseIdentical;
    }

    let use_x_axis = (a1[0] - a0[0]).abs() >= (a1[1] - a0[1]).abs();
    let axis = usize::from(!use_x_axis);
    let (a_min, a_max) = ordered_pair(a0[axis], a1[axis]);
    let (b_min, b_max) = ordered_pair(b0[axis], b1[axis]);
    if a_max < b_min || b_max < a_min {
        CertifiedSegmentSegment2DClassification::CollinearDisjoint
    } else if a_max == b_min || b_max == a_min {
        CertifiedSegmentSegment2DClassification::EndpointTouch
    } else if basis.contact_policy_identity() == "require-imprint-for-collinear-overlap" {
        CertifiedSegmentSegment2DClassification::PolicyRequiredOrUncertain
    } else {
        CertifiedSegmentSegment2DClassification::CollinearOverlap
    }
}

fn ordered_pair(left: f64, right: f64) -> (f64, f64) {
    if left <= right {
        (left, right)
    } else {
        (right, left)
    }
}
