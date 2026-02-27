use crate::primitives::plane::{
    classify_point, classify_point_exact, coplanar_eq, exact_eq, intersect_three_planes,
    intersect_three_planes_exact, signed_distance, to_plane_relation, Plane, PlaneRelation,
};
use forge_math::arithmetic::Rational;
use forge_math::sign::TriSign;

const TEST_DEGENERACY: f64 = 1e-15;
const TEST_TOLERANCE: f64 = 1e-10;

#[test]
fn construct_valid_plane() {
    let plane = Plane::try_new([0.0, 0.0, 1.0], 0.0);
    assert!(plane.is_ok());
}

#[test]
fn reject_zero_normal() {
    let plane = Plane::try_new([0.0, 0.0, 0.0], 1.0);
    assert!(plane.is_err());
}

#[test]
fn reject_nan_normal() {
    let plane = Plane::try_new([f64::NAN, 0.0, 1.0], 0.0);
    assert!(plane.is_err());
}

#[test]
fn reject_inf_offset() {
    let plane = Plane::try_new([0.0, 0.0, 1.0], f64::INFINITY);
    assert!(plane.is_err());
}

#[test]
fn reject_neg_inf_normal() {
    let plane = Plane::try_new([f64::NEG_INFINITY, 0.0, 0.0], 1.0);
    assert!(plane.is_err());
}

#[test]
fn from_point_normal_constructs_correctly() {
    let plane = Plane::from_point_normal([0.0, 0.0, 5.0], [0.0, 0.0, 1.0]).unwrap();
    let dist = signed_distance(&plane, &[0.0, 0.0, 5.0]);
    assert!(dist.abs() < TEST_TOLERANCE);
}

#[test]
fn classify_point_above_xy_plane() {
    let plane = Plane::try_new([0.0, 0.0, 1.0], 0.0).unwrap();
    let sign = classify_point(&plane, &[0.0, 0.0, 5.0]).unwrap();
    assert_eq!(to_plane_relation(&sign), PlaneRelation::Above);
}

#[test]
fn classify_point_below_xy_plane() {
    let plane = Plane::try_new([0.0, 0.0, 1.0], 0.0).unwrap();
    let sign = classify_point(&plane, &[0.0, 0.0, -3.0]).unwrap();
    assert_eq!(to_plane_relation(&sign), PlaneRelation::Below);
}

#[test]
fn classify_point_on_xy_plane() {
    let plane = Plane::try_new([0.0, 0.0, 1.0], 0.0).unwrap();
    let sign = classify_point(&plane, &[7.0, -3.0, 0.0]).unwrap();
    assert_eq!(sign.sign(), TriSign::Zero);
}

#[test]
fn signed_distance_positive_above() {
    let plane = Plane::try_new([0.0, 0.0, 1.0], 0.0).unwrap();
    let dist = signed_distance(&plane, &[0.0, 0.0, 3.0]);
    assert!((dist - 3.0).abs() < TEST_TOLERANCE);
}

#[test]
fn signed_distance_negative_below() {
    let plane = Plane::try_new([0.0, 0.0, 1.0], 0.0).unwrap();
    let dist = signed_distance(&plane, &[0.0, 0.0, -2.0]);
    assert!((dist + 2.0).abs() < TEST_TOLERANCE);
}

#[test]
fn intersect_axis_aligned_planes_at_origin() {
    let px = Plane::try_new([1.0, 0.0, 0.0], 0.0).unwrap();
    let py = Plane::try_new([0.0, 1.0, 0.0], 0.0).unwrap();
    let pz = Plane::try_new([0.0, 0.0, 1.0], 0.0).unwrap();

    let point = intersect_three_planes(&px, &py, &pz, TEST_DEGENERACY).unwrap();
    assert!((point[0]).abs() < TEST_TOLERANCE);
    assert!((point[1]).abs() < TEST_TOLERANCE);
    assert!((point[2]).abs() < TEST_TOLERANCE);
}

#[test]
fn intersect_offset_planes_at_known_point() {
    let px = Plane::try_new([1.0, 0.0, 0.0], -3.0).unwrap();
    let py = Plane::try_new([0.0, 1.0, 0.0], -4.0).unwrap();
    let pz = Plane::try_new([0.0, 0.0, 1.0], -5.0).unwrap();

    let point = intersect_three_planes(&px, &py, &pz, TEST_DEGENERACY).unwrap();
    assert!((point[0] - 3.0).abs() < TEST_TOLERANCE);
    assert!((point[1] - 4.0).abs() < TEST_TOLERANCE);
    assert!((point[2] - 5.0).abs() < TEST_TOLERANCE);
}

#[test]
fn intersect_parallel_planes_returns_error() {
    let p0 = Plane::try_new([0.0, 0.0, 1.0], 0.0).unwrap();
    let p1 = Plane::try_new([0.0, 0.0, 1.0], -1.0).unwrap();
    let p2 = Plane::try_new([1.0, 0.0, 0.0], 0.0).unwrap();

    let result = intersect_three_planes(&p0, &p1, &p2, TEST_DEGENERACY);
    assert!(result.is_err());
}

#[test]
fn cube_planes_produce_correct_vertex_count() {
    let planes = [
        Plane::try_new([1.0, 0.0, 0.0], 0.0).unwrap(),
        Plane::try_new([-1.0, 0.0, 0.0], 1.0).unwrap(),
        Plane::try_new([0.0, 1.0, 0.0], 0.0).unwrap(),
        Plane::try_new([0.0, -1.0, 0.0], 1.0).unwrap(),
        Plane::try_new([0.0, 0.0, 1.0], 0.0).unwrap(),
        Plane::try_new([0.0, 0.0, -1.0], 1.0).unwrap(),
    ];

    let mut valid_vertices = 0;
    let triples: [(usize, usize, usize); 8] = [
        (0, 2, 4),
        (0, 2, 5),
        (0, 3, 4),
        (0, 3, 5),
        (1, 2, 4),
        (1, 2, 5),
        (1, 3, 4),
        (1, 3, 5),
    ];

    for (i, j, k) in triples {
        let result = intersect_three_planes(&planes[i], &planes[j], &planes[k], TEST_DEGENERACY);
        if result.is_ok() {
            valid_vertices += 1;
        }
    }

    assert_eq!(valid_vertices, 8);
}

#[test]
fn plane_normal_is_normalized() {
    let plane = Plane::try_new([3.0, 4.0, 0.0], 10.0).unwrap();
    let n = plane.normal();
    let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
    assert!((len - 1.0).abs() < TEST_TOLERANCE);
}

#[test]
fn raw_normal_preserves_original() {
    let plane = Plane::try_new([3.0, 4.0, 0.0], 10.0).unwrap();
    let raw = plane.raw_normal();
    assert!((raw[0] - 3.0).abs() < TEST_TOLERANCE);
    assert!((raw[1] - 4.0).abs() < TEST_TOLERANCE);
}

#[test]
fn exact_eq_identical_planes() {
    let a = Plane::try_new([1.0, 0.0, 0.0], -5.0).unwrap();
    let b = Plane::try_new([1.0, 0.0, 0.0], -5.0).unwrap();
    assert!(exact_eq(&a, &b));
}

#[test]
fn exact_eq_scaled_planes() {
    let a = Plane::try_new([1.0, 0.0, 0.0], -5.0).unwrap();
    let b = Plane::try_new([2.0, 0.0, 0.0], -10.0).unwrap();
    assert!(exact_eq(&a, &b));
}

#[test]
fn exact_eq_opposite_normals_are_different() {
    let a = Plane::try_new([1.0, 0.0, 0.0], -5.0).unwrap();
    let b = Plane::try_new([-1.0, 0.0, 0.0], 5.0).unwrap();
    assert!(!exact_eq(&a, &b));
}

#[test]
fn coplanar_eq_same_direction() {
    let a = Plane::try_new([1.0, 0.0, 0.0], -5.0).unwrap();
    let b = Plane::try_new([1.0, 0.0, 0.0], -5.0).unwrap();
    assert!(coplanar_eq(&a, &b));
}

#[test]
fn coplanar_eq_opposite_normals() {
    let a = Plane::try_new([1.0, 0.0, 0.0], -5.0).unwrap();
    let b = Plane::try_new([-1.0, 0.0, 0.0], 5.0).unwrap();
    assert!(coplanar_eq(&a, &b));
}

#[test]
fn coplanar_eq_different_offset() {
    let a = Plane::try_new([0.0, 0.0, 1.0], -0.5).unwrap();
    let b = Plane::try_new([0.0, 0.0, 1.0], -1.5).unwrap();
    assert!(!coplanar_eq(&a, &b));
}

#[test]
fn coplanar_eq_z_axis_antiparallel_different_offset() {
    let a = Plane::try_new([0.0, 0.0, 1.0], -0.5).unwrap();
    let b = Plane::try_new([0.0, 0.0, -1.0], -0.5).unwrap();
    assert!(!coplanar_eq(&a, &b));
}

#[test]
fn coplanar_eq_z_axis_antiparallel_same_surface() {
    let a = Plane::try_new([0.0, 0.0, 1.0], -0.5).unwrap();
    let b = Plane::try_new([0.0, 0.0, -1.0], 0.5).unwrap();
    assert!(coplanar_eq(&a, &b));
}

#[test]
fn classify_point_exact_on_plane() {
    let plane = Plane::try_new([0.0, 0.0, 1.0], -5.0).unwrap();
    let point = [
        Rational::from_integer(0),
        Rational::from_integer(0),
        Rational::from_integer(5),
    ];
    assert_eq!(classify_point_exact(&plane, &point), TriSign::Zero);
}

#[test]
fn classify_point_exact_above_plane() {
    let plane = Plane::try_new([0.0, 0.0, 1.0], 0.0).unwrap();
    let point = [
        Rational::from_integer(0),
        Rational::from_integer(0),
        Rational::from_integer(5),
    ];
    assert_eq!(classify_point_exact(&plane, &point), TriSign::Pos);
}

#[test]
fn intersect_three_planes_exact_matches_f64() {
    let px = Plane::try_new([1.0, 0.0, 0.0], -3.0).unwrap();
    let py = Plane::try_new([0.0, 1.0, 0.0], -4.0).unwrap();
    let pz = Plane::try_new([0.0, 0.0, 1.0], -5.0).unwrap();

    let exact = intersect_three_planes_exact(&px, &py, &pz).unwrap();
    assert_eq!(exact[0], Rational::from_integer(3));
    assert_eq!(exact[1], Rational::from_integer(4));
    assert_eq!(exact[2], Rational::from_integer(5));
}

#[test]
fn axis_aligned_plane_constructs_correctly() {
    let plane = Plane::axis_aligned(2, 1, Rational::from_integer(-5)).unwrap();
    let n = plane.normal();
    assert!((n[2] - 1.0).abs() < TEST_TOLERANCE);
    assert!((plane.raw_offset() + 5.0).abs() < TEST_TOLERANCE);
}
