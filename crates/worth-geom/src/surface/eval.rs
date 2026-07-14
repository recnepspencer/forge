//! Surface evaluation — point_at, normal_at, and analytic arbitration.
//!
//! DOMAIN: Pure stateless evaluation of parametric surfaces. Returns
//! `PolicyResult` when a classification falls within the ambiguity zone
//! (Doctrine D2). No tolerance decisions, no geometry store access.
//!
//! DEPENDENCIES: surface schema types and local ambiguity-policy support.

use super::schema::{SurfaceData, SurfaceKind, SurfaceRelation};
use crate::support::{PolicyKind, PolicyQuery, PolicyResult};

impl SurfaceData {
    /// Evaluate the surface at parameter (u, v), returning the 3D point.
    ///
    /// For analytic surfaces this is exact (closed-form). NURBS surfaces
    /// require numerical evaluation (Phase 7).
    pub fn point_at(&self, u: f64, v: f64) -> [f64; 3] {
        match &self.kind {
            SurfaceKind::Plane { normal, offset } => {
                let (tangent_u, tangent_v) = plane_tangent_frame(normal);
                [
                    normal[0] * offset + tangent_u[0] * u + tangent_v[0] * v,
                    normal[1] * offset + tangent_u[1] * u + tangent_v[1] * v,
                    normal[2] * offset + tangent_u[2] * u + tangent_v[2] * v,
                ]
            }
            SurfaceKind::Cylinder {
                origin,
                axis,
                radius,
            } => {
                let (u_dir, v_dir) = cylinder_frame(axis);
                let cu = u.cos();
                let su = u.sin();
                [
                    origin[0] + radius * (cu * u_dir[0] + su * v_dir[0]) + v * axis[0],
                    origin[1] + radius * (cu * u_dir[1] + su * v_dir[1]) + v * axis[1],
                    origin[2] + radius * (cu * u_dir[2] + su * v_dir[2]) + v * axis[2],
                ]
            }
            SurfaceKind::Cone {
                apex,
                axis,
                half_angle,
            } => {
                let (u_dir, v_dir) = cylinder_frame(axis);
                let r = v * half_angle.tan();
                let cu = u.cos();
                let su = u.sin();
                [
                    apex[0] + v * axis[0] + r * (cu * u_dir[0] + su * v_dir[0]),
                    apex[1] + v * axis[1] + r * (cu * u_dir[1] + su * v_dir[1]),
                    apex[2] + v * axis[2] + r * (cu * u_dir[2] + su * v_dir[2]),
                ]
            }
            SurfaceKind::Sphere { center, radius } => {
                let cv = v.cos();
                let sv = v.sin();
                let cu = u.cos();
                let su = u.sin();
                [
                    center[0] + radius * cv * cu,
                    center[1] + radius * cv * su,
                    center[2] + radius * sv,
                ]
            }
            SurfaceKind::TriaxialEllipsoid {
                center,
                axis_u,
                axis_v,
                axis_w,
                radius_a,
                radius_b,
                radius_c,
            } => {
                let cv = v.cos();
                let sv = v.sin();
                let cu = u.cos();
                let su = u.sin();
                let local = [radius_a * cv * cu, radius_b * cv * su, radius_c * sv];
                point_from_frame(center, axis_u, axis_v, axis_w, local)
            }
            SurfaceKind::Torus {
                center,
                axis,
                major_r,
                minor_r,
            } => {
                let (u_dir, v_dir) = cylinder_frame(axis);
                let cu = u.cos();
                let su = u.sin();
                let cv = v.cos();
                let sv = v.sin();
                let ring_r = *major_r + *minor_r * cv;
                let radial = [
                    cu * u_dir[0] + su * v_dir[0],
                    cu * u_dir[1] + su * v_dir[1],
                    cu * u_dir[2] + su * v_dir[2],
                ];
                [
                    center[0] + ring_r * radial[0] + minor_r * sv * axis[0],
                    center[1] + ring_r * radial[1] + minor_r * sv * axis[1],
                    center[2] + ring_r * radial[2] + minor_r * sv * axis[2],
                ]
            }
        }
    }

    /// Evaluate the outward unit normal at parameter (u, v).
    pub fn normal_at(&self, u: f64, v: f64) -> [f64; 3] {
        match &self.kind {
            SurfaceKind::Plane { normal, .. } => *normal,

            SurfaceKind::Cylinder { axis, .. } => {
                let (u_dir, v_dir) = cylinder_frame(axis);
                let cu = u.cos();
                let su = u.sin();
                [
                    cu * u_dir[0] + su * v_dir[0],
                    cu * u_dir[1] + su * v_dir[1],
                    cu * u_dir[2] + su * v_dir[2],
                ]
            }
            SurfaceKind::Cone {
                axis, half_angle, ..
            } => {
                let (u_dir, v_dir) = cylinder_frame(axis);
                let cu = u.cos();
                let su = u.sin();
                let ca = half_angle.cos();
                let sa = half_angle.sin();
                let radial = [
                    cu * u_dir[0] + su * v_dir[0],
                    cu * u_dir[1] + su * v_dir[1],
                    cu * u_dir[2] + su * v_dir[2],
                ];
                let n = [
                    ca * radial[0] - sa * axis[0],
                    ca * radial[1] - sa * axis[1],
                    ca * radial[2] - sa * axis[2],
                ];
                normalize(n)
            }
            SurfaceKind::Sphere { .. } => {
                let cv = v.cos();
                let sv = v.sin();
                let cu = u.cos();
                let su = u.sin();
                [cv * cu, cv * su, sv]
            }
            SurfaceKind::TriaxialEllipsoid {
                axis_u,
                axis_v,
                axis_w,
                radius_a,
                radius_b,
                radius_c,
                ..
            } => {
                let cv = v.cos();
                let sv = v.sin();
                let cu = u.cos();
                let su = u.sin();
                normalize(combine_frame(
                    axis_u,
                    axis_v,
                    axis_w,
                    [cv * cu / radius_a, cv * su / radius_b, sv / radius_c],
                ))
            }
            SurfaceKind::Torus {
                axis,
                major_r,
                minor_r,
                ..
            } => {
                let (u_dir, v_dir) = cylinder_frame(axis);
                let cu = u.cos();
                let su = u.sin();
                let cv = v.cos();
                let sv = v.sin();
                let radial = [
                    cu * u_dir[0] + su * v_dir[0],
                    cu * u_dir[1] + su * v_dir[1],
                    cu * u_dir[2] + su * v_dir[2],
                ];
                let _ = major_r;
                let _ = minor_r;
                let n = [
                    cv * radial[0] + sv * axis[0],
                    cv * radial[1] + sv * axis[1],
                    cv * radial[2] + sv * axis[2],
                ];
                normalize(n)
            }
        }
    }
}

/// Analytic surface-surface relation classification.
///
/// Compares abstract surface definitions to determine if two surfaces are
/// coincident (same surface), disjoint (cannot intersect), or general
/// (require SSI computation). This is the "analytic arbitration" step in
/// the hybrid Boolean pipeline (Architecture §4.3.4).
///
/// Returns `PolicyResult<SurfaceRelation>` per Doctrine D2: when a measure
/// falls within the ambiguity band (`tol..ambiguity_factor*tol`), the solver
/// returns `Ambiguous` and lets the kernel decide via `ModelingContext` policy.
///
/// `ambiguity_factor` is passed from `ToleranceConfig::get_ambiguity_band_factor()`.
pub fn classify_surface_pair(
    a: &SurfaceData,
    b: &SurfaceData,
    tol: f64,
    ambiguity_factor: f64,
) -> PolicyResult<SurfaceRelation> {
    match (&a.kind, &b.kind) {
        (
            SurfaceKind::Plane {
                normal: n1,
                offset: d1,
            },
            SurfaceKind::Plane {
                normal: n2,
                offset: d2,
            },
        ) => classify_plane_plane(n1, *d1, n2, *d2, tol, ambiguity_factor),

        (
            SurfaceKind::Sphere {
                center: c1,
                radius: r1,
            },
            SurfaceKind::Sphere {
                center: c2,
                radius: r2,
            },
        ) => classify_sphere_sphere(c1, *r1, c2, *r2, tol, ambiguity_factor),

        (
            SurfaceKind::Cylinder {
                origin: o1,
                axis: a1,
                radius: r1,
            },
            SurfaceKind::Cylinder {
                origin: o2,
                axis: a2,
                radius: r2,
            },
        ) => classify_cylinder_cylinder(o1, a1, *r1, o2, a2, *r2, tol, ambiguity_factor),

        (
            SurfaceKind::Cone {
                apex: a1,
                axis: ax1,
                half_angle: ha1,
            },
            SurfaceKind::Cone {
                apex: a2,
                axis: ax2,
                half_angle: ha2,
            },
        ) => classify_cone_cone(a1, ax1, *ha1, a2, ax2, *ha2, tol, ambiguity_factor),

        (
            SurfaceKind::TriaxialEllipsoid {
                center: c1,
                axis_u: u1,
                axis_v: v1,
                axis_w: w1,
                radius_a: a1,
                radius_b: b1,
                radius_c: c_radius1,
            },
            SurfaceKind::TriaxialEllipsoid {
                center: c2,
                axis_u: u2,
                axis_v: v2,
                axis_w: w2,
                radius_a: a2,
                radius_b: b2,
                radius_c: c_radius2,
            },
        ) => classify_triaxial_ellipsoid_triaxial_ellipsoid(
            c1,
            u1,
            v1,
            w1,
            *a1,
            *b1,
            *c_radius1,
            c2,
            u2,
            v2,
            w2,
            *a2,
            *b2,
            *c_radius2,
            tol,
            ambiguity_factor,
        ),

        (
            SurfaceKind::Torus {
                center: c1,
                axis: ax1,
                major_r: mj1,
                minor_r: mn1,
            },
            SurfaceKind::Torus {
                center: c2,
                axis: ax2,
                major_r: mj2,
                minor_r: mn2,
            },
        ) => classify_torus_torus(
            c1,
            ax1,
            *mj1,
            *mn1,
            c2,
            ax2,
            *mj2,
            *mn2,
            tol,
            ambiguity_factor,
        ),

        _ => PolicyResult::Success(SurfaceRelation::General),
    }
}

/// Build an `Ambiguous` result for near-coincident/near-disjoint situations.
fn ambiguous(relation: SurfaceRelation, margin: f64) -> PolicyResult<SurfaceRelation> {
    PolicyResult::Ambiguous {
        query: PolicyQuery {
            kind: PolicyKind::CoincidentGeometry,
            location: [0.0, 0.0, 0.0],
            margin,
            overridable: true,
        },
        potential_value: relation,
    }
}

/// Classify two planes: coincident (same or antiparallel normals, same offset),
/// disjoint (parallel but different offset), or general (intersecting).
fn classify_plane_plane(
    n1: &[f64; 3],
    d1: f64,
    n2: &[f64; 3],
    d2: f64,
    tol: f64,
    af: f64,
) -> PolicyResult<SurfaceRelation> {
    let dot = n1[0] * n2[0] + n1[1] * n2[1] + n1[2] * n2[2];
    let angle_deviation = (dot.abs() - 1.0).abs();

    if angle_deviation < tol {
        let effective_d2 = if dot > 0.0 { d2 } else { -d2 };
        let offset_diff = (d1 - effective_d2).abs();

        if offset_diff < tol {
            PolicyResult::Success(SurfaceRelation::Coincident)
        } else if offset_diff < tol * af {
            ambiguous(SurfaceRelation::Coincident, offset_diff)
        } else {
            PolicyResult::Success(SurfaceRelation::Disjoint)
        }
    } else if angle_deviation < tol * af {
        ambiguous(SurfaceRelation::General, angle_deviation)
    } else {
        PolicyResult::Success(SurfaceRelation::General)
    }
}

/// Classify two spheres.
fn classify_sphere_sphere(
    c1: &[f64; 3],
    r1: f64,
    c2: &[f64; 3],
    r2: f64,
    tol: f64,
    af: f64,
) -> PolicyResult<SurfaceRelation> {
    let dx = c1[0] - c2[0];
    let dy = c1[1] - c2[1];
    let dz = c1[2] - c2[2];
    let dist_sq = dx * dx + dy * dy + dz * dz;
    let radius_diff = (r1 - r2).abs();

    if dist_sq < tol * tol && radius_diff < tol {
        PolicyResult::Success(SurfaceRelation::Coincident)
    } else if dist_sq < (tol * af).powi(2) && radius_diff < tol * af {
        ambiguous(SurfaceRelation::Coincident, dist_sq.sqrt().max(radius_diff))
    } else {
        let dist = dist_sq.sqrt();
        if dist > r1 + r2 + tol * af {
            PolicyResult::Success(SurfaceRelation::Disjoint)
        } else if dist > r1 + r2 + tol {
            ambiguous(SurfaceRelation::Disjoint, dist - (r1 + r2))
        } else if dist + r1.min(r2) + tol * af < r1.max(r2) {
            PolicyResult::Success(SurfaceRelation::Disjoint)
        } else if dist + r1.min(r2) + tol < r1.max(r2) {
            let gap = r1.max(r2) - dist - r1.min(r2);
            ambiguous(SurfaceRelation::Disjoint, gap)
        } else {
            PolicyResult::Success(SurfaceRelation::General)
        }
    }
}

/// Classify two cylinders: coincident if coaxial with same radius.
fn classify_cylinder_cylinder(
    o1: &[f64; 3],
    a1: &[f64; 3],
    r1: f64,
    o2: &[f64; 3],
    a2: &[f64; 3],
    r2: f64,
    tol: f64,
    af: f64,
) -> PolicyResult<SurfaceRelation> {
    let dot = a1[0] * a2[0] + a1[1] * a2[1] + a1[2] * a2[2];
    let angle_dev = (dot.abs() - 1.0).abs();

    if angle_dev > tol * af {
        return PolicyResult::Success(SurfaceRelation::General);
    }
    if angle_dev > tol {
        return ambiguous(SurfaceRelation::General, angle_dev);
    }

    let radius_diff = (r1 - r2).abs();
    if radius_diff > tol * af {
        return PolicyResult::Success(SurfaceRelation::General);
    }
    if radius_diff > tol {
        return ambiguous(SurfaceRelation::General, radius_diff);
    }

    let d = [o2[0] - o1[0], o2[1] - o1[1], o2[2] - o1[2]];
    let proj = d[0] * a1[0] + d[1] * a1[1] + d[2] * a1[2];
    let perp_sq = (d[0] - proj * a1[0]).powi(2)
        + (d[1] - proj * a1[1]).powi(2)
        + (d[2] - proj * a1[2]).powi(2);

    if perp_sq < tol * tol {
        PolicyResult::Success(SurfaceRelation::Coincident)
    } else if perp_sq < (tol * af).powi(2) {
        ambiguous(SurfaceRelation::Coincident, perp_sq.sqrt())
    } else {
        PolicyResult::Success(SurfaceRelation::General)
    }
}

/// Classify two cones: coincident if same apex, axis, half_angle.
fn classify_cone_cone(
    a1: &[f64; 3],
    ax1: &[f64; 3],
    ha1: f64,
    a2: &[f64; 3],
    ax2: &[f64; 3],
    ha2: f64,
    tol: f64,
    af: f64,
) -> PolicyResult<SurfaceRelation> {
    let dot = ax1[0] * ax2[0] + ax1[1] * ax2[1] + ax1[2] * ax2[2];
    let angle_dev = (dot.abs() - 1.0).abs();

    if angle_dev > tol * af {
        return PolicyResult::Success(SurfaceRelation::General);
    }
    if angle_dev > tol {
        return ambiguous(SurfaceRelation::General, angle_dev);
    }

    let ha_diff = (ha1 - ha2).abs();
    if ha_diff > tol * af {
        return PolicyResult::Success(SurfaceRelation::General);
    }
    if ha_diff > tol {
        return ambiguous(SurfaceRelation::General, ha_diff);
    }

    let d = [a2[0] - a1[0], a2[1] - a1[1], a2[2] - a1[2]];
    let dist_sq = d[0] * d[0] + d[1] * d[1] + d[2] * d[2];

    if dist_sq < tol * tol {
        PolicyResult::Success(SurfaceRelation::Coincident)
    } else if dist_sq < (tol * af).powi(2) {
        ambiguous(SurfaceRelation::Coincident, dist_sq.sqrt())
    } else {
        PolicyResult::Success(SurfaceRelation::General)
    }
}

/// Classify two tori: coincident if same center, axis, major/minor radius.
fn classify_torus_torus(
    c1: &[f64; 3],
    ax1: &[f64; 3],
    major1: f64,
    minor1: f64,
    c2: &[f64; 3],
    ax2: &[f64; 3],
    major2: f64,
    minor2: f64,
    tol: f64,
    af: f64,
) -> PolicyResult<SurfaceRelation> {
    let dot = ax1[0] * ax2[0] + ax1[1] * ax2[1] + ax1[2] * ax2[2];
    let angle_dev = (dot.abs() - 1.0).abs();

    if angle_dev > tol * af {
        return PolicyResult::Success(SurfaceRelation::General);
    }
    if angle_dev > tol {
        return ambiguous(SurfaceRelation::General, angle_dev);
    }

    let major_diff = (major1 - major2).abs();
    let minor_diff = (minor1 - minor2).abs();
    let max_radius_diff = major_diff.max(minor_diff);

    if max_radius_diff > tol * af {
        return PolicyResult::Success(SurfaceRelation::General);
    }
    if max_radius_diff > tol {
        return ambiguous(SurfaceRelation::General, max_radius_diff);
    }

    let d = [c2[0] - c1[0], c2[1] - c1[1], c2[2] - c1[2]];
    let dist_sq = d[0] * d[0] + d[1] * d[1] + d[2] * d[2];

    if dist_sq < tol * tol {
        PolicyResult::Success(SurfaceRelation::Coincident)
    } else if dist_sq < (tol * af).powi(2) {
        ambiguous(SurfaceRelation::Coincident, dist_sq.sqrt())
    } else {
        PolicyResult::Success(SurfaceRelation::General)
    }
}

fn classify_triaxial_ellipsoid_triaxial_ellipsoid(
    c1: &[f64; 3],
    u1: &[f64; 3],
    v1: &[f64; 3],
    w1: &[f64; 3],
    a1: f64,
    b1: f64,
    c_radius1: f64,
    c2: &[f64; 3],
    u2: &[f64; 3],
    v2: &[f64; 3],
    w2: &[f64; 3],
    a2: f64,
    b2: f64,
    c_radius2: f64,
    tol: f64,
    af: f64,
) -> PolicyResult<SurfaceRelation> {
    let center_delta = norm([c1[0] - c2[0], c1[1] - c2[1], c1[2] - c2[2]]);
    let axis_delta = frame_max_delta(u1, v1, w1, u2, v2, w2);
    let radius_delta = (a1 - a2)
        .abs()
        .max((b1 - b2).abs())
        .max((c_radius1 - c_radius2).abs());
    let max_delta = center_delta.max(axis_delta).max(radius_delta);

    if max_delta < tol {
        PolicyResult::Success(SurfaceRelation::Coincident)
    } else if max_delta < tol * af {
        ambiguous(SurfaceRelation::Coincident, max_delta)
    } else {
        PolicyResult::Success(SurfaceRelation::General)
    }
}

/// Build an orthonormal tangent frame for a plane given its normal.
fn plane_tangent_frame(normal: &[f64; 3]) -> ([f64; 3], [f64; 3]) {
    let seed = if normal[0].abs() < 0.9 {
        [1.0, 0.0, 0.0]
    } else {
        [0.0, 1.0, 0.0]
    };
    let u = normalize(cross(&seed, normal));
    let v = cross(normal, &u);
    (u, v)
}

/// Build a local frame for a cylinder/cone/torus given its axis direction.
fn cylinder_frame(axis: &[f64; 3]) -> ([f64; 3], [f64; 3]) {
    plane_tangent_frame(axis)
}

fn combine_frame(
    axis_u: &[f64; 3],
    axis_v: &[f64; 3],
    axis_w: &[f64; 3],
    local: [f64; 3],
) -> [f64; 3] {
    [
        local[0] * axis_u[0] + local[1] * axis_v[0] + local[2] * axis_w[0],
        local[0] * axis_u[1] + local[1] * axis_v[1] + local[2] * axis_w[1],
        local[0] * axis_u[2] + local[1] * axis_v[2] + local[2] * axis_w[2],
    ]
}

fn point_from_frame(
    center: &[f64; 3],
    axis_u: &[f64; 3],
    axis_v: &[f64; 3],
    axis_w: &[f64; 3],
    local: [f64; 3],
) -> [f64; 3] {
    let offset = combine_frame(axis_u, axis_v, axis_w, local);
    [
        center[0] + offset[0],
        center[1] + offset[1],
        center[2] + offset[2],
    ]
}

fn frame_max_delta(
    u1: &[f64; 3],
    v1: &[f64; 3],
    w1: &[f64; 3],
    u2: &[f64; 3],
    v2: &[f64; 3],
    w2: &[f64; 3],
) -> f64 {
    axis_delta(u1, u2)
        .max(axis_delta(v1, v2))
        .max(axis_delta(w1, w2))
}

fn axis_delta(a: &[f64; 3], b: &[f64; 3]) -> f64 {
    norm([a[0] - b[0], a[1] - b[1], a[2] - b[2]])
}

fn norm(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

fn cross(a: &[f64; 3], b: &[f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn normalize(v: [f64; 3]) -> [f64; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len < 1e-30 {
        return [0.0, 0.0, 0.0];
    }
    [v[0] / len, v[1] / len, v[2] / len]
}

#[cfg(test)]
mod tests {
    use super::*;
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

    #[test]
    fn axis_swapped_triaxial_ellipsoids_are_general_not_coincident() {
        let canonical = SurfaceData::triaxial_ellipsoid(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            5.0,
            3.0,
            2.0,
        )
        .expect("canonical");
        let swapped = SurfaceData::triaxial_ellipsoid(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [0.0, 1.0, 0.0],
            5.0,
            2.0,
            3.0,
        )
        .expect("swapped");
        assert_eq!(
            classify_surface_pair(&canonical, &swapped, 1e-12, 10.0)
                .into_result_strict()
                .unwrap(),
            SurfaceRelation::General
        );
    }

    #[test]
    fn coincident_planes_detected() {
        let a = SurfaceData::plane([0.0, 0.0, 1.0], 5.0);
        let b = SurfaceData::plane([0.0, 0.0, 1.0], 5.0);
        assert_eq!(
            classify_surface_pair(&a, &b, 1e-12, 10.0)
                .into_result_strict()
                .unwrap(),
            SurfaceRelation::Coincident
        );
    }

    #[test]
    fn antiparallel_coincident_planes_detected() {
        let a = SurfaceData::plane([0.0, 0.0, 1.0], 5.0);
        let b = SurfaceData::plane([0.0, 0.0, -1.0], -5.0);
        assert_eq!(
            classify_surface_pair(&a, &b, 1e-12, 10.0)
                .into_result_strict()
                .unwrap(),
            SurfaceRelation::Coincident
        );
    }

    #[test]
    fn parallel_disjoint_planes() {
        let a = SurfaceData::plane([0.0, 0.0, 1.0], 3.0);
        let b = SurfaceData::plane([0.0, 0.0, 1.0], 7.0);
        assert_eq!(
            classify_surface_pair(&a, &b, 1e-12, 10.0)
                .into_result_strict()
                .unwrap(),
            SurfaceRelation::Disjoint
        );
    }

    #[test]
    fn intersecting_planes_are_general() {
        let a = SurfaceData::plane([1.0, 0.0, 0.0], 0.0);
        let b = SurfaceData::plane([0.0, 1.0, 0.0], 0.0);
        assert_eq!(
            classify_surface_pair(&a, &b, 1e-12, 10.0)
                .into_result_strict()
                .unwrap(),
            SurfaceRelation::General
        );
    }

    #[test]
    fn coincident_spheres() {
        let a = SurfaceData::sphere([0.0, 0.0, 0.0], 5.0);
        let b = SurfaceData::sphere([0.0, 0.0, 0.0], 5.0);
        assert_eq!(
            classify_surface_pair(&a, &b, 1e-12, 10.0)
                .into_result_strict()
                .unwrap(),
            SurfaceRelation::Coincident
        );
    }

    #[test]
    fn disjoint_spheres() {
        let a = SurfaceData::sphere([0.0, 0.0, 0.0], 1.0);
        let b = SurfaceData::sphere([10.0, 0.0, 0.0], 1.0);
        assert_eq!(
            classify_surface_pair(&a, &b, 1e-12, 10.0)
                .into_result_strict()
                .unwrap(),
            SurfaceRelation::Disjoint
        );
    }

    #[test]
    fn contained_sphere_is_disjoint() {
        let a = SurfaceData::sphere([0.0, 0.0, 0.0], 10.0);
        let b = SurfaceData::sphere([0.0, 0.0, 0.0], 1.0);
        assert_eq!(
            classify_surface_pair(&a, &b, 1e-12, 10.0)
                .into_result_strict()
                .unwrap(),
            SurfaceRelation::Disjoint
        );
    }

    #[test]
    fn coincident_cylinders() {
        let a = SurfaceData::cylinder([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 3.0);
        let b = SurfaceData::cylinder([0.0, 0.0, 5.0], [0.0, 0.0, 1.0], 3.0);
        assert_eq!(
            classify_surface_pair(&a, &b, 1e-12, 10.0)
                .into_result_strict()
                .unwrap(),
            SurfaceRelation::Coincident
        );
    }

    // ── Cone/Torus Classification Tests ─────────────────────────────────

    #[test]
    fn same_cone_detected_as_coincident() {
        let a = SurfaceData::cone([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], PI / 4.0);
        let b = SurfaceData::cone([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], PI / 4.0);
        assert_eq!(
            classify_surface_pair(&a, &b, 1e-12, 10.0)
                .into_result_strict()
                .unwrap(),
            SurfaceRelation::Coincident
        );
    }

    #[test]
    fn same_cone_antiparallel_axis_detected() {
        let a = SurfaceData::cone([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], PI / 4.0);
        let b = SurfaceData::cone([0.0, 0.0, 0.0], [0.0, 0.0, -1.0], PI / 4.0);
        assert_eq!(
            classify_surface_pair(&a, &b, 1e-12, 10.0)
                .into_result_strict()
                .unwrap(),
            SurfaceRelation::Coincident
        );
    }

    #[test]
    fn different_cone_angle_is_general() {
        let a = SurfaceData::cone([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], PI / 4.0);
        let b = SurfaceData::cone([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], PI / 3.0);
        assert_eq!(
            classify_surface_pair(&a, &b, 1e-12, 10.0)
                .into_result_strict()
                .unwrap(),
            SurfaceRelation::General
        );
    }

    #[test]
    fn same_torus_detected_as_coincident() {
        let a = SurfaceData::torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.0, 1.0);
        let b = SurfaceData::torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.0, 1.0);
        assert_eq!(
            classify_surface_pair(&a, &b, 1e-12, 10.0)
                .into_result_strict()
                .unwrap(),
            SurfaceRelation::Coincident
        );
    }

    #[test]
    fn same_torus_antiparallel_axis_detected() {
        let a = SurfaceData::torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.0, 1.0);
        let b = SurfaceData::torus([0.0, 0.0, 0.0], [0.0, 0.0, -1.0], 5.0, 1.0);
        assert_eq!(
            classify_surface_pair(&a, &b, 1e-12, 10.0)
                .into_result_strict()
                .unwrap(),
            SurfaceRelation::Coincident
        );
    }

    #[test]
    fn different_torus_major_radius_is_general() {
        let a = SurfaceData::torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.0, 1.0);
        let b = SurfaceData::torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 6.0, 1.0);
        assert_eq!(
            classify_surface_pair(&a, &b, 1e-12, 10.0)
                .into_result_strict()
                .unwrap(),
            SurfaceRelation::General
        );
    }

    // ── Cone evaluation ─────────────────────────────────────────────────

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

    // ── Torus evaluation ────────────────────────────────────────────────

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

    // ── Normal consistency (numerical derivative check) ─────────────────

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
            let dot = analytic[0] * numerical[0]
                + analytic[1] * numerical[1]
                + analytic[2] * numerical[2];
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
            let dot = analytic[0] * numerical[0]
                + analytic[1] * numerical[1]
                + analytic[2] * numerical[2];
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
            let dot = analytic[0] * numerical[0]
                + analytic[1] * numerical[1]
                + analytic[2] * numerical[2];
            assert!(
                dot.abs() > 0.999,
                "torus normal mismatch at u={}, v={}: dot={}",
                u,
                v,
                dot
            );
        }
    }
}
