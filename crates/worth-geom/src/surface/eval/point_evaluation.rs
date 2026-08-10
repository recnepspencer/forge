use crate::surface::schema::{SurfaceData, SurfaceKind};

use super::geometry::{cylinder_frame, plane_tangent_frame, point_from_frame};

impl SurfaceData {
    /// Evaluate the surface at parameter (u, v), returning the 3D point.
    ///
    /// For analytic surfaces this is exact (closed-form). NURBS surfaces
    /// require numerical evaluation (Phase 7).
    pub fn point_at(&self, u: f64, v: f64) -> [f64; 3] {
        match &self.kind {
            SurfaceKind::Plane { normal, offset } => plane_point(normal, *offset, u, v),
            SurfaceKind::Cylinder {
                origin,
                axis,
                radius,
            } => cylinder_point(origin, axis, *radius, u, v),
            SurfaceKind::Cone {
                apex,
                axis,
                half_angle,
            } => cone_point(apex, axis, *half_angle, u, v),
            SurfaceKind::Sphere { center, radius } => sphere_point(center, *radius, u, v),
            SurfaceKind::TriaxialEllipsoid {
                center,
                axis_u,
                axis_v,
                axis_w,
                radius_a,
                radius_b,
                radius_c,
            } => triaxial_ellipsoid_point(
                center, axis_u, axis_v, axis_w, *radius_a, *radius_b, *radius_c, u, v,
            ),
            SurfaceKind::Torus {
                center,
                axis,
                major_r,
                minor_r,
            } => torus_point(center, axis, *major_r, *minor_r, u, v),
        }
    }
}

fn plane_point(normal: &[f64; 3], offset: f64, u: f64, v: f64) -> [f64; 3] {
    let (tangent_u, tangent_v) = plane_tangent_frame(normal);
    [
        normal[0] * offset + tangent_u[0] * u + tangent_v[0] * v,
        normal[1] * offset + tangent_u[1] * u + tangent_v[1] * v,
        normal[2] * offset + tangent_u[2] * u + tangent_v[2] * v,
    ]
}

fn cylinder_point(origin: &[f64; 3], axis: &[f64; 3], radius: f64, u: f64, v: f64) -> [f64; 3] {
    let (u_dir, v_dir) = cylinder_frame(axis);
    let cu = u.cos();
    let su = u.sin();
    [
        origin[0] + radius * (cu * u_dir[0] + su * v_dir[0]) + v * axis[0],
        origin[1] + radius * (cu * u_dir[1] + su * v_dir[1]) + v * axis[1],
        origin[2] + radius * (cu * u_dir[2] + su * v_dir[2]) + v * axis[2],
    ]
}

fn cone_point(apex: &[f64; 3], axis: &[f64; 3], half_angle: f64, u: f64, v: f64) -> [f64; 3] {
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

fn sphere_point(center: &[f64; 3], radius: f64, u: f64, v: f64) -> [f64; 3] {
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

fn triaxial_ellipsoid_point(
    center: &[f64; 3],
    axis_u: &[f64; 3],
    axis_v: &[f64; 3],
    axis_w: &[f64; 3],
    radius_a: f64,
    radius_b: f64,
    radius_c: f64,
    u: f64,
    v: f64,
) -> [f64; 3] {
    let cv = v.cos();
    let sv = v.sin();
    let cu = u.cos();
    let su = u.sin();
    let local = [radius_a * cv * cu, radius_b * cv * su, radius_c * sv];
    point_from_frame(center, axis_u, axis_v, axis_w, local)
}

fn torus_point(
    center: &[f64; 3],
    axis: &[f64; 3],
    major_r: f64,
    minor_r: f64,
    u: f64,
    v: f64,
) -> [f64; 3] {
    let (u_dir, v_dir) = cylinder_frame(axis);
    let cu = u.cos();
    let su = u.sin();
    let cv = v.cos();
    let sv = v.sin();
    let ring_r = major_r + minor_r * cv;
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
