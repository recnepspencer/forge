//! Exact segment intersection tests.

use super::super::schema::Segment2D;
use super::{intersect_segments_exact, EndpointSide, ExactIntersection};
use worth_math::arithmetic::rational::Rational;

fn r(v: f64) -> Rational {
    Rational::try_from_f64(v).unwrap()
}

#[test]
fn exact_intersect_disjoint() {
    let s1 = Segment2D::new([0.0, 0.0], [2.0, 2.0], 1);
    let s2 = Segment2D::new([0.0, 2.0], [1.0, 3.0], 2);
    assert_eq!(
        intersect_segments_exact(&s1, &s2),
        Ok(ExactIntersection::Disjoint)
    );
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
    assert_eq!(
        intersect_segments_exact(&s1, &s2),
        Ok(ExactIntersection::SharedEndpoint {
            shared_a: EndpointSide::End,
            shared_b: EndpointSide::Start,
        })
    );
}

#[test]
fn exact_intersect_endpoint_touch() {
    // T-junction at (1, 1). s2 starts at the interior of s1.
    let s1 = Segment2D::new([0.0, 1.0], [2.0, 1.0], 1); // Horizontal
    let s2 = Segment2D::new([1.0, 1.0], [1.0, 3.0], 2); // Vertical upwards

    match intersect_segments_exact(&s1, &s2) {
        Ok(ExactIntersection::EndpointTouch {
            touching_seg,
            at_endpoint,
            t_on_other,
        }) => {
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
        Ok(ExactIntersection::Overlap {
            t_a_range,
            t_b_range,
        }) => {
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
    assert_eq!(
        intersect_segments_exact(&s1, &s2),
        Ok(ExactIntersection::Disjoint)
    );
}
