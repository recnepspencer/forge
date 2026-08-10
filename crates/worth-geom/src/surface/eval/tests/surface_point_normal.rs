use crate::surface::schema::SurfaceData;
use std::f64::consts::{FRAC_PI_2, PI, TAU};

#[test]
fn plane_point_at_origin() {
    let s = SurfaceData::plane([0.0, 0.0, 1.0], 0.0);
    let p = s.point_at(0.0, 0.0);
    assert!((p[2]).abs() < 1e-12);
}

#[test]
fn plane_point_at_offset() {
    let s = SurfaceData::plane([0.0, 0.0, 1.0], 5.0);
    let p = s.point_at(0.0, 0.0);
    assert!((p[2] - 5.0).abs() < 1e-12);
}

#[test]
fn plane_normal_is_constant() {
    let s = SurfaceData::plane([0.0, 1.0, 0.0], 3.0);
    let n1 = s.normal_at(0.0, 0.0);
    let n2 = s.normal_at(42.0, -17.0);
    assert_eq!(n1, n2);
    assert!((n1[1] - 1.0).abs() < 1e-12);
}

#[test]
fn cylinder_point_on_surface() {
    let s = SurfaceData::cylinder([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 2.0);
    let p = s.point_at(0.0, 5.0);
    let r = (p[0] * p[0] + p[1] * p[1]).sqrt();
    assert!((r - 2.0).abs() < 1e-12);
    assert!((p[2] - 5.0).abs() < 1e-12);
}

#[test]
fn cylinder_normal_is_radial() {
    let s = SurfaceData::cylinder([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 3.0);
    let n = s.normal_at(0.0, 0.0);
    let p = s.point_at(0.0, 0.0);
    let dot = n[0] * p[0] + n[1] * p[1];
    assert!(dot > 0.0);
    assert!(n[2].abs() < 1e-12);
}

#[test]
fn sphere_point_at_poles() {
    let s = SurfaceData::sphere([0.0, 0.0, 0.0], 1.0);
    let north = s.point_at(0.0, FRAC_PI_2);
    assert!((north[2] - 1.0).abs() < 1e-12);
    let south = s.point_at(0.0, -FRAC_PI_2);
    assert!((south[2] + 1.0).abs() < 1e-12);
}

#[test]
fn sphere_normal_is_outward() {
    let s = SurfaceData::sphere([1.0, 2.0, 3.0], 5.0);
    let p = s.point_at(0.0, 0.0);
    let n = s.normal_at(0.0, 0.0);
    let dir = [p[0] - 1.0, p[1] - 2.0, p[2] - 3.0];
    let dot = dir[0] * n[0] + dir[1] * n[1] + dir[2] * n[2];
    assert!(dot > 0.0);
}

#[test]
fn triaxial_ellipsoid_point_respects_distinct_principal_radii() {
    let s = SurfaceData::triaxial_ellipsoid(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
        5.0,
        3.0,
        2.0,
    )
    .expect("triaxial ellipsoid");
    assert_point_near(s.point_at(0.0, 0.0), [5.0, 0.0, 0.0]);
    assert_point_near(s.point_at(FRAC_PI_2, 0.0), [0.0, 3.0, 0.0]);
    assert_point_near(s.point_at(0.0, FRAC_PI_2), [0.0, 0.0, 2.0]);
}

// ── Cone evaluation ──────────────────────────────────────────────────────

#[test]
fn cone_apex_is_at_v_zero() {
    let s = SurfaceData::cone([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], PI / 4.0);
    let p = s.point_at(0.0, 0.0);
    assert!((p[0]).abs() < 1e-12);
    assert!((p[1]).abs() < 1e-12);
    assert!((p[2]).abs() < 1e-12);
}

#[test]
fn cone_radius_grows_with_v() {
    let half_angle = PI / 6.0; // 30°
    let s = SurfaceData::cone([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], half_angle);
    let v = 5.0;
    let p = s.point_at(0.0, v);
    let r = (p[0] * p[0] + p[1] * p[1]).sqrt();
    let expected_r = v * half_angle.tan();
    assert!(
        (r - expected_r).abs() < 1e-10,
        "r={} expected={}",
        r,
        expected_r
    );
    assert!((p[2] - v).abs() < 1e-10);
}

#[test]
fn cone_normal_perpendicular_to_surface() {
    let half_angle = PI / 4.0;
    let s = SurfaceData::cone([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], half_angle);
    let u = 0.3;
    let v = 2.0;
    let n = s.normal_at(u, v);
    let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
    assert!((len - 1.0).abs() < 1e-10, "normal not unit length: {}", len);
}

// ── Torus evaluation ─────────────────────────────────────────────────────

#[test]
fn torus_outer_equator_at_v_zero() {
    let major = 5.0;
    let minor = 1.0;
    let s = SurfaceData::torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], major, minor);
    let p = s.point_at(0.0, 0.0);
    let r = (p[0] * p[0] + p[1] * p[1]).sqrt();
    assert!(
        (r - (major + minor)).abs() < 1e-10,
        "outer equator r={}, expected={}",
        r,
        major + minor
    );
    assert!(p[2].abs() < 1e-10);
}

#[test]
fn torus_inner_equator_at_v_pi() {
    let major = 5.0;
    let minor = 1.0;
    let s = SurfaceData::torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], major, minor);
    let p = s.point_at(0.0, PI);
    let r = (p[0] * p[0] + p[1] * p[1]).sqrt();
    assert!(
        (r - (major - minor)).abs() < 1e-10,
        "inner equator r={}, expected={}",
        r,
        major - minor
    );
    assert!(p[2].abs() < 1e-10);
}

#[test]
fn torus_top_at_v_half_pi() {
    let major = 5.0;
    let minor = 1.0;
    let s = SurfaceData::torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], major, minor);
    let p = s.point_at(0.0, FRAC_PI_2);
    assert!(
        (p[2] - minor).abs() < 1e-10,
        "z={}, expected={}",
        p[2],
        minor
    );
    let r = (p[0] * p[0] + p[1] * p[1]).sqrt();
    assert!(
        (r - major).abs() < 1e-10,
        "at top of tube, r should equal major: r={}, major={}",
        r,
        major
    );
}

#[test]
fn torus_normal_is_unit_length() {
    let s = SurfaceData::torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.0, 1.0);
    for (u, v) in [
        (0.0, 0.0),
        (PI / 3.0, PI / 4.0),
        (PI, PI),
        (TAU * 0.7, FRAC_PI_2),
    ] {
        let n = s.normal_at(u, v);
        let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
        assert!(
            (len - 1.0).abs() < 1e-10,
            "normal not unit at u={}, v={}: len={}",
            u,
            v,
            len
        );
    }
}

fn assert_point_near(actual: [f64; 3], expected: [f64; 3]) {
    for i in 0..3 {
        assert!(
            (actual[i] - expected[i]).abs() < 1e-12,
            "point mismatch at axis {}: actual={:?} expected={:?}",
            i,
            actual,
            expected
        );
    }
}
