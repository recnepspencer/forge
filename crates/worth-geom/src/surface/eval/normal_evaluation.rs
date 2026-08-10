use crate::surface::schema::{SurfaceData, SurfaceKind};

use super::geometry::{cylinder_frame, normalize};

impl SurfaceData {
    /// Evaluate the outward unit normal at parameter (u, v).
    pub fn normal_at(&self, u: f64, v: f64) -> [f64; 3] {
        match &self.kind {
            SurfaceKind::Plane { normal, .. } => plane_normal(normal),
            SurfaceKind::Cylinder { axis, .. } => cylinder_normal(axis, u),
            SurfaceKind::Cone {
                axis, half_angle, ..
            } => cone_normal(axis, *half_angle, u),
            SurfaceKind::Sphere { .. } => sphere_normal(u, v),
            SurfaceKind::TriaxialEllipsoid {
                axis_u,
                axis_v,
                axis_w,
                radius_a,
                radius_b,
                radius_c,
                ..
            } => triaxial_ellipsoid_normal(
                axis_u, axis_v, axis_w, *radius_a, *radius_b, *radius_c, u, v,
            ),
            SurfaceKind::Torus {
                axis,
                major_r,
                minor_r,
                ..
            } => torus_normal(axis, *major_r, *minor_r, u, v),
        }
    }
}

fn plane_normal(normal: &[f64; 3]) -> [f64; 3] {
    *normal
}

fn cylinder_normal(axis: &[f64; 3], u: f64) -> [f64; 3] {
    let (u_dir, v_dir) = cylinder_frame(axis);
    let cu = u.cos();
    let su = u.sin();
    [
        cu * u_dir[0] + su * v_dir[0],
        cu * u_dir[1] + su * v_dir[1],
        cu * u_dir[2] + su * v_dir[2],
    ]
}

fn cone_normal(axis: &[f64; 3], half_angle: f64, u: f64) -> [f64; 3] {
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

fn sphere_normal(u: f64, v: f64) -> [f64; 3] {
    let cv = v.cos();
    let sv = v.sin();
    let cu = u.cos();
    let su = u.sin();
    [cv * cu, cv * su, sv]
}

fn triaxial_ellipsoid_normal(
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
    normalize(super::geometry::combine_frame(
        axis_u,
        axis_v,
        axis_w,
        [cv * cu / radius_a, cv * su / radius_b, sv / radius_c],
    ))
}

fn torus_normal(axis: &[f64; 3], major_r: f64, minor_r: f64, u: f64, v: f64) -> [f64; 3] {
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
