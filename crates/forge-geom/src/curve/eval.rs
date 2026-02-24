//! Curve evaluation — point_at, tangent_at for 3D edge curves.
//!
//! DOMAIN: Pure stateless evaluation of parametric 3D curves.
//!
//! DEPENDENCIES: curve schema types

use super::schema::CurveKind;

impl CurveKind {
    /// Evaluate the 3D curve at parameter `t`.
    ///
    /// - Line: `origin + t * direction` (t unbounded).
    /// - Circle: `center + R * cos(t) * u_dir + R * sin(t) * v_dir`, t ∈ [0, 2π).
    /// - Ellipse: `center + cos(t) * major + sin(t) * minor`, t ∈ [0, 2π).
    /// - SurfaceIntersection: evaluates the SP-curve approximation.
    pub fn point_at(&self, t: f64) -> [f64; 3] {
        match self {
            CurveKind::Line { origin, direction } => [
                origin[0] + t * direction[0],
                origin[1] + t * direction[1],
                origin[2] + t * direction[2],
            ],
            CurveKind::Circle { center, normal, radius } => {
                let (u_dir, v_dir) = circle_frame(normal);
                let ct = t.cos();
                let st = t.sin();
                [
                    center[0] + radius * (ct * u_dir[0] + st * v_dir[0]),
                    center[1] + radius * (ct * u_dir[1] + st * v_dir[1]),
                    center[2] + radius * (ct * u_dir[2] + st * v_dir[2]),
                ]
            }
            CurveKind::Ellipse { center, major, minor } => {
                let ct = t.cos();
                let st = t.sin();
                [
                    center[0] + ct * major[0] + st * minor[0],
                    center[1] + ct * major[1] + st * minor[1],
                    center[2] + ct * major[2] + st * minor[2],
                ]
            }
            CurveKind::SurfaceIntersection { sp_curve_cache, .. } => {
                evaluate_bspline_point(&sp_curve_cache.control_points, &sp_curve_cache.knots, sp_curve_cache.domain, t)
            }
        }
    }

    /// Evaluate the unit tangent vector at parameter `t`.
    pub fn tangent_at(&self, t: f64) -> [f64; 3] {
        match self {
            CurveKind::Line { direction, .. } => *direction,
            CurveKind::Circle { normal, radius, .. } => {
                let (u_dir, v_dir) = circle_frame(normal);
                let ct = t.cos();
                let st = t.sin();
                let raw = [
                    radius * (-st * u_dir[0] + ct * v_dir[0]),
                    radius * (-st * u_dir[1] + ct * v_dir[1]),
                    radius * (-st * u_dir[2] + ct * v_dir[2]),
                ];
                normalize(raw)
            }
            CurveKind::Ellipse { major, minor, .. } => {
                let ct = t.cos();
                let st = t.sin();
                let raw = [
                    -st * major[0] + ct * minor[0],
                    -st * major[1] + ct * minor[1],
                    -st * major[2] + ct * minor[2],
                ];
                normalize(raw)
            }
            CurveKind::SurfaceIntersection { sp_curve_cache, .. } => {
                let dt = 1e-8;
                let p0 = evaluate_bspline_point(&sp_curve_cache.control_points, &sp_curve_cache.knots, sp_curve_cache.domain, t);
                let p1 = evaluate_bspline_point(&sp_curve_cache.control_points, &sp_curve_cache.knots, sp_curve_cache.domain, t + dt);
                normalize([
                    (p1[0] - p0[0]) / dt,
                    (p1[1] - p0[1]) / dt,
                    (p1[2] - p0[2]) / dt,
                ])
            }
        }
    }
}

/// Build an orthonormal frame for a circle given its normal.
fn circle_frame(normal: &[f64; 3]) -> ([f64; 3], [f64; 3]) {
    let seed = if normal[0].abs() < 0.9 {
        [1.0, 0.0, 0.0]
    } else {
        [0.0, 1.0, 0.0]
    };
    let u = normalize(cross(&seed, normal));
    let v = cross(normal, &u);
    (u, v)
}

/// Simple linear interpolation B-spline evaluation (degree 1) for the SP-curve cache.
///
/// Full B-spline evaluation with de Boor's algorithm is Phase 7. For now,
/// the SP-curve cache is evaluated as a piecewise linear interpolation of
/// control points, which is sufficient for AABB broad-phase queries.
fn evaluate_bspline_point(
    control_points: &[[f64; 3]],
    _knots: &[f64],
    domain: (f64, f64),
    t: f64,
) -> [f64; 3] {
    if control_points.is_empty() {
        return [0.0, 0.0, 0.0];
    }
    if control_points.len() == 1 {
        return control_points[0];
    }

    let n = control_points.len() - 1;
    let normalized = (t - domain.0) / (domain.1 - domain.0);
    let clamped = normalized.clamp(0.0, 1.0);
    let scaled = clamped * n as f64;
    let seg = (scaled as usize).min(n - 1);
    let frac = scaled - seg as f64;

    let p0 = &control_points[seg];
    let p1 = &control_points[seg + 1];
    [
        p0[0] + frac * (p1[0] - p0[0]),
        p0[1] + frac * (p1[1] - p0[1]),
        p0[2] + frac * (p1[2] - p0[2]),
    ]
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

    #[test]
    fn line_point_at() {
        let c = CurveKind::Line {
            origin: [0.0, 0.0, 0.0],
            direction: [1.0, 0.0, 0.0],
        };
        let p = c.point_at(5.0);
        assert!((p[0] - 5.0).abs() < 1e-12);
        assert!(p[1].abs() < 1e-12);
    }

    #[test]
    fn line_tangent_is_constant() {
        let c = CurveKind::Line {
            origin: [1.0, 2.0, 3.0],
            direction: [0.0, 0.0, 1.0],
        };
        let t1 = c.tangent_at(0.0);
        let t2 = c.tangent_at(100.0);
        assert_eq!(t1, t2);
        assert!((t1[2] - 1.0).abs() < 1e-12);
    }

    #[test]
    fn circle_point_on_circle() {
        let c = CurveKind::Circle {
            center: [0.0, 0.0, 0.0],
            normal: [0.0, 0.0, 1.0],
            radius: 3.0,
        };
        let p = c.point_at(0.0);
        let r = (p[0] * p[0] + p[1] * p[1]).sqrt();
        assert!((r - 3.0).abs() < 1e-12);
        assert!(p[2].abs() < 1e-12);
    }

    #[test]
    fn circle_tangent_is_perpendicular_to_radius() {
        let c = CurveKind::Circle {
            center: [0.0, 0.0, 0.0],
            normal: [0.0, 0.0, 1.0],
            radius: 2.0,
        };
        let p = c.point_at(0.0);
        let t = c.tangent_at(0.0);
        let dot = p[0] * t[0] + p[1] * t[1] + p[2] * t[2];
        assert!(dot.abs() < 1e-10);
    }

    #[test]
    fn ellipse_point_on_ellipse() {
        let c = CurveKind::Ellipse {
            center: [0.0, 0.0, 0.0],
            major: [3.0, 0.0, 0.0],
            minor: [0.0, 2.0, 0.0],
        };
        let p0 = c.point_at(0.0);
        assert!((p0[0] - 3.0).abs() < 1e-12);
        let p_half_pi = c.point_at(std::f64::consts::FRAC_PI_2);
        assert!((p_half_pi[1] - 2.0).abs() < 1e-12);
    }
}
