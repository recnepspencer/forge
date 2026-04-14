//! Data shapes for UV-space coedges (trim curves).
//!
//! DOMAIN: Each halfedge in a B-Rep has a coedge — a 2D curve in its face's
//! surface parameter space. This is the mechanism that prevents 3D positional
//! drift in deep boolean chains: `surface.point_at(coedge.uv_at(t))` is on
//! the surface by construction.
//!
//! DEPENDENCIES: serde

use super::curve::SurfaceIndex;
use serde::{Deserialize, Serialize};

/// A 2D parametric curve in surface parameter space.
///
/// These are the trim curves that bound a face in UV space. For planar
/// faces they are trivial straight lines. For curved surfaces they become
/// non-trivial (e.g., a cylindrical face's trim curves are lines in (θ, z)
/// space; a NURBS face's trim curves are NURBS 2D curves).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParametricCurve2D {
    /// Straight line segment in UV space.
    Line {
        /// Start point (u, v) at t=0.
        start: [f64; 2],
        /// End point (u, v) at t=1.
        end: [f64; 2],
    },

    /// Circular arc in UV space.
    Circle {
        /// Center in UV space.
        center: [f64; 2],
        /// Radius in UV parameter units.
        radius: f64,
        /// Start angle (radians).
        start_angle: f64,
        /// End angle (radians).
        end_angle: f64,
    },

    /// NURBS curve in UV space (Phase 7).
    Nurbs {
        /// Control points in (u, v) parameter space.
        control_points: Vec<[f64; 2]>,
        /// Knot vector.
        knots: Vec<f64>,
        /// Polynomial degree.
        degree: u8,
    },
}

impl ParametricCurve2D {
    /// Evaluate the UV position at parameter `t ∈ [0, 1]`.
    pub fn uv_at(&self, t: f64) -> [f64; 2] {
        match self {
            ParametricCurve2D::Line { start, end } => [
                start[0] + t * (end[0] - start[0]),
                start[1] + t * (end[1] - start[1]),
            ],
            ParametricCurve2D::Circle {
                center,
                radius,
                start_angle,
                end_angle,
            } => {
                let angle = start_angle + t * (end_angle - start_angle);
                [
                    center[0] + radius * angle.cos(),
                    center[1] + radius * angle.sin(),
                ]
            }
            ParametricCurve2D::Nurbs { control_points, .. } => {
                if control_points.is_empty() {
                    return [0.0, 0.0];
                }
                if control_points.len() == 1 {
                    return control_points[0];
                }
                let n = control_points.len() - 1;
                let clamped = t.clamp(0.0, 1.0);
                let scaled = clamped * n as f64;
                let seg = (scaled as usize).min(n - 1);
                let frac = scaled - seg as f64;
                let p0 = &control_points[seg];
                let p1 = &control_points[seg + 1];
                [
                    p0[0] + frac * (p1[0] - p0[0]),
                    p0[1] + frac * (p1[1] - p0[1]),
                ]
            }
        }
    }
}

/// A directed trim curve in a face's (u, v) parameter space.
///
/// This is the geometry-layer mirror of the `HalfEdgeData.coedge` handle
/// in `forge-topo`. Each halfedge bordering a curved face has a `Coedge`
/// that defines its boundary path in the face's surface parameter space.
///
/// The key anti-drift property: `surface.point_at(coedge.uv_at(t))` is
/// guaranteed to lie exactly on the surface, regardless of how many
/// boolean operations have been chained. The 3D position is *derived*
/// from the UV position, never the other way around.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coedge {
    /// The 2D path in this face's surface parameter space.
    pub uv_curve: ParametricCurve2D,
    /// Index of the surface this coedge is anchored to.
    pub surface: SurfaceIndex,
}

impl Coedge {
    /// Evaluate the UV position at parameter `t ∈ [0, 1]`.
    pub fn uv_at(&self, t: f64) -> [f64; 2] {
        self.uv_curve.uv_at(t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_coedge_endpoints() {
        let c = ParametricCurve2D::Line {
            start: [0.0, 0.0],
            end: [1.0, 1.0],
        };
        let start = c.uv_at(0.0);
        let end = c.uv_at(1.0);
        assert!((start[0]).abs() < 1e-12);
        assert!((end[0] - 1.0).abs() < 1e-12);
    }

    #[test]
    fn line_coedge_midpoint() {
        let c = ParametricCurve2D::Line {
            start: [0.0, 0.0],
            end: [2.0, 4.0],
        };
        let mid = c.uv_at(0.5);
        assert!((mid[0] - 1.0).abs() < 1e-12);
        assert!((mid[1] - 2.0).abs() < 1e-12);
    }

    #[test]
    fn circle_coedge_start_angle() {
        let c = ParametricCurve2D::Circle {
            center: [0.5, 0.5],
            radius: 0.3,
            start_angle: 0.0,
            end_angle: std::f64::consts::TAU,
        };
        let p = c.uv_at(0.0);
        assert!((p[0] - 0.8).abs() < 1e-12);
        assert!((p[1] - 0.5).abs() < 1e-12);
    }

    #[test]
    fn coedge_delegates_to_uv_curve() {
        let ce = Coedge {
            uv_curve: ParametricCurve2D::Line {
                start: [0.0, 0.0],
                end: [1.0, 0.0],
            },
            surface: 0,
        };
        let uv = ce.uv_at(0.5);
        assert!((uv[0] - 0.5).abs() < 1e-12);
    }

    // ── Anti-drift property tests ───────────────────────────────────────
    // The entire point of coedges: surface.point_at(coedge.uv_at(t))
    // is on the surface by construction. This test verifies that property.

    #[test]
    fn anti_drift_cylinder_uv_to_3d_lands_on_surface() {
        use crate::surface::SurfaceData;
        let radius = 3.0;
        let surface = SurfaceData::cylinder([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], radius);
        let coedge = Coedge {
            uv_curve: ParametricCurve2D::Line {
                start: [0.0, 0.0],
                end: [std::f64::consts::PI, 5.0],
            },
            surface: 0,
        };

        for i in 0..=10 {
            let t = i as f64 / 10.0;
            let uv = coedge.uv_at(t);
            let p = surface.point_at(uv[0], uv[1]);
            let r = (p[0] * p[0] + p[1] * p[1]).sqrt();
            assert!(
                (r - radius).abs() < 1e-10,
                "Point at t={} not on cylinder: r={}, expected={}",
                t,
                r,
                radius
            );
        }
    }

    #[test]
    fn anti_drift_sphere_uv_to_3d_lands_on_surface() {
        use crate::surface::SurfaceData;
        let radius = 5.0;
        let surface = SurfaceData::sphere([1.0, 2.0, 3.0], radius);
        let coedge = Coedge {
            uv_curve: ParametricCurve2D::Line {
                start: [0.0, 0.0],
                end: [std::f64::consts::PI, std::f64::consts::FRAC_PI_4],
            },
            surface: 0,
        };

        for i in 0..=10 {
            let t = i as f64 / 10.0;
            let uv = coedge.uv_at(t);
            let p = surface.point_at(uv[0], uv[1]);
            let dx = p[0] - 1.0;
            let dy = p[1] - 2.0;
            let dz = p[2] - 3.0;
            let r = (dx * dx + dy * dy + dz * dz).sqrt();
            assert!(
                (r - radius).abs() < 1e-10,
                "Point at t={} not on sphere: r={}, expected={}",
                t,
                r,
                radius
            );
        }
    }

    #[test]
    fn anti_drift_torus_uv_to_3d_lands_on_surface() {
        use crate::surface::SurfaceData;
        let major = 5.0;
        let minor = 1.0;
        let surface = SurfaceData::torus([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], major, minor);
        let coedge = Coedge {
            uv_curve: ParametricCurve2D::Line {
                start: [0.0, 0.0],
                end: [std::f64::consts::TAU, std::f64::consts::TAU],
            },
            surface: 0,
        };

        for i in 0..=20 {
            let t = i as f64 / 20.0;
            let uv = coedge.uv_at(t);
            let p = surface.point_at(uv[0], uv[1]);

            let xy_r = (p[0] * p[0] + p[1] * p[1]).sqrt();
            let dist_to_ring_center = ((xy_r - major).powi(2) + p[2].powi(2)).sqrt();
            assert!(
                (dist_to_ring_center - minor).abs() < 1e-10,
                "Point at t={} not on torus: dist_to_tube_center={}, expected={}",
                t,
                dist_to_ring_center,
                minor
            );
        }
    }
}
