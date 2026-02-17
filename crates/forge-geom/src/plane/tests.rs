//! Tests for the Plane primitive.

#[cfg(test)]
mod tests {
    use forge_math::sign::TriSign;
    use crate::plane::{Plane, PlaneRelation, classify_point, signed_distance, intersect_three_planes, to_plane_relation};

    /// Default degeneracy threshold for tests.
    const TEST_DEGENERACY: f64 = 1e-15;

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
        assert!(dist.abs() < 1e-10);
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
        assert!((dist - 3.0).abs() < 1e-10);
    }

    #[test]
    fn signed_distance_negative_below() {
        let plane = Plane::try_new([0.0, 0.0, 1.0], 0.0).unwrap();
        let dist = signed_distance(&plane, &[0.0, 0.0, -2.0]);
        assert!((dist + 2.0).abs() < 1e-10);
    }

    #[test]
    fn intersect_axis_aligned_planes_at_origin() {
        let px = Plane::try_new([1.0, 0.0, 0.0], 0.0).unwrap();
        let py = Plane::try_new([0.0, 1.0, 0.0], 0.0).unwrap();
        let pz = Plane::try_new([0.0, 0.0, 1.0], 0.0).unwrap();

        let point = intersect_three_planes(&px, &py, &pz, TEST_DEGENERACY).unwrap();
        assert!((point[0]).abs() < 1e-10);
        assert!((point[1]).abs() < 1e-10);
        assert!((point[2]).abs() < 1e-10);
    }

    #[test]
    fn intersect_offset_planes_at_known_point() {
        let px = Plane::try_new([1.0, 0.0, 0.0], -3.0).unwrap();
        let py = Plane::try_new([0.0, 1.0, 0.0], -4.0).unwrap();
        let pz = Plane::try_new([0.0, 0.0, 1.0], -5.0).unwrap();

        let point = intersect_three_planes(&px, &py, &pz, TEST_DEGENERACY).unwrap();
        assert!((point[0] - 3.0).abs() < 1e-10);
        assert!((point[1] - 4.0).abs() < 1e-10);
        assert!((point[2] - 5.0).abs() < 1e-10);
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
            (0, 2, 4), (0, 2, 5), (0, 3, 4), (0, 3, 5),
            (1, 2, 4), (1, 2, 5), (1, 3, 4), (1, 3, 5),
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
        assert!((len - 1.0).abs() < 1e-10);
    }

    #[test]
    fn raw_normal_preserves_original() {
        let plane = Plane::try_new([3.0, 4.0, 0.0], 10.0).unwrap();
        let raw = plane.raw_normal();
        assert!((raw[0] - 3.0).abs() < 1e-15);
        assert!((raw[1] - 4.0).abs() < 1e-15);
    }
}
