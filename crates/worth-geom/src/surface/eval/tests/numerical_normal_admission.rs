use crate::surface::schema::SurfaceData;
use std::f64::consts::PI;

fn numerical_normal(s: &SurfaceData, u: f64, v: f64) -> [f64; 3] {
    let dt = 1e-7;
    let du = [
        (s.point_at(u + dt, v)[0] - s.point_at(u - dt, v)[0]) / (2.0 * dt),
        (s.point_at(u + dt, v)[1] - s.point_at(u - dt, v)[1]) / (2.0 * dt),
        (s.point_at(u + dt, v)[2] - s.point_at(u - dt, v)[2]) / (2.0 * dt),
    ];
    let dv = [
        (s.point_at(u, v + dt)[0] - s.point_at(u, v - dt)[0]) / (2.0 * dt),
        (s.point_at(u, v + dt)[1] - s.point_at(u, v - dt)[1]) / (2.0 * dt),
        (s.point_at(u, v + dt)[2] - s.point_at(u, v - dt)[2]) / (2.0 * dt),
    ];
    let raw = [
        du[1] * dv[2] - du[2] * dv[1],
        du[2] * dv[0] - du[0] * dv[2],
        du[0] * dv[1] - du[1] * dv[0],
    ];
    let len = (raw[0] * raw[0] + raw[1] * raw[1] + raw[2] * raw[2]).sqrt();
    if len < 1e-15 {
        return [0.0, 0.0, 0.0];
    }
    [raw[0] / len, raw[1] / len, raw[2] / len]
}

#[test]
fn triaxial_ellipsoid_rejects_symmetric_or_non_orthonormal_definitions() {
    assert_eq!(
        SurfaceData::triaxial_ellipsoid(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            5.0,
            5.0,
            2.0,
        )
        .unwrap_err(),
        crate::surface::schema::TriaxialEllipsoidDefinitionError::RadiiMustBeDistinct
    );
    assert_eq!(
        SurfaceData::triaxial_ellipsoid(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            5.0,
            3.0,
            2.0,
        )
        .unwrap_err(),
        crate::surface::schema::TriaxialEllipsoidDefinitionError::AxisFrameMustBeUnitAndOrthonormal
    );
}

#[test]
fn cylinder_normal_agrees_with_numerical_derivative() {
    let s = SurfaceData::cylinder([1.0, 2.0, 3.0], [0.0, 0.0, 1.0], 4.0);
    for &u in &[0.5, 1.0, 2.5, 4.0] {
        let analytic = s.normal_at(u, 0.0);
        let numerical = numerical_normal(&s, u, 0.0);
        let dot =
            analytic[0] * numerical[0] + analytic[1] * numerical[1] + analytic[2] * numerical[2];
        assert!(
            dot.abs() > 0.999,
            "cylinder normal mismatch at u={}: analytic={:?} numerical={:?} dot={}",
            u,
            analytic,
            numerical,
            dot
        );
    }
}

#[test]
fn sphere_normal_agrees_with_numerical_derivative() {
    let s = SurfaceData::sphere([0.0, 0.0, 0.0], 3.0);
    for &(u, v) in &[(0.5, 0.3), (PI, 0.0), (1.0, -0.5)] {
        let analytic = s.normal_at(u, v);
        let numerical = numerical_normal(&s, u, v);
        let dot =
            analytic[0] * numerical[0] + analytic[1] * numerical[1] + analytic[2] * numerical[2];
        assert!(
            dot.abs() > 0.999,
            "sphere normal mismatch at u={}, v={}: dot={}",
            u,
            v,
            dot
        );
    }
}

#[test]
fn torus_normal_agrees_with_numerical_derivative() {
    let s = SurfaceData::torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.0, 1.0);
    for &(u, v) in &[(0.5, 0.3), (PI, PI / 4.0), (2.0, 1.0)] {
        let analytic = s.normal_at(u, v);
        let numerical = numerical_normal(&s, u, v);
        let dot =
            analytic[0] * numerical[0] + analytic[1] * numerical[1] + analytic[2] * numerical[2];
        assert!(
            dot.abs() > 0.999,
            "torus normal mismatch at u={}, v={}: dot={}",
            u,
            v,
            dot
        );
    }
}
