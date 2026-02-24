//! Data shapes for 3D edge curves and their tolerance metadata.
//!
//! DOMAIN: Defines the curve type hierarchy — analytic (line, circle, ellipse),
//! symbolic (surface-surface intersection), and freeform (NURBS). Each curve
//! stores its parametric definition, certified tolerance tube radius, and
//! provenance for the audit trail.
//!
//! DEPENDENCIES: serde

use serde::{Deserialize, Serialize};

/// Index into the surface arena within the geometry store.
///
/// This is `forge-geom`'s internal index type for surface cross-references
/// (e.g., within `SurfaceIntersection` curves). The kernel bridges these
/// to `forge-topo::SurfaceRef` handles at the integration boundary.
pub type SurfaceIndex = u32;

/// Parametric 3D curve kinds supported by the kernel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CurveKind {
    /// Straight line segment.
    Line {
        /// A point on the line (t=0).
        origin: [f64; 3],
        /// Unit direction vector.
        direction: [f64; 3],
    },

    /// Circular arc in 3D.
    Circle {
        /// Center of the circle.
        center: [f64; 3],
        /// Normal to the plane of the circle.
        normal: [f64; 3],
        /// Radius (must be > 0).
        radius: f64,
    },

    /// Elliptical arc in 3D.
    Ellipse {
        /// Center of the ellipse.
        center: [f64; 3],
        /// Semi-major axis direction (length = semi-major radius).
        major: [f64; 3],
        /// Semi-minor axis direction (length = semi-minor radius).
        minor: [f64; 3],
    },

    /// Symbolic intersection of two surfaces — the aerospace correctness
    /// mechanism.
    ///
    /// Instead of discretizing the intersection into a polyline (which drifts
    /// after chained Booleans), the curve is stored as the pair of surfaces
    /// that produce it. A later Boolean can re-solve the 3-surface system
    /// exactly rather than intersecting against a lossy approximation.
    ///
    /// The `sp_curve_cache` provides a bounded-error B-spline approximation
    /// for fast downstream consumers (AABB, rendering). Only topological
    /// stitching re-evaluates the symbolic surfaces.
    SurfaceIntersection {
        /// Index of the first surface in the geometry store.
        surface_a: SurfaceIndex,
        /// Index of the second surface in the geometry store.
        surface_b: SurfaceIndex,
        /// Tightly bounded B-spline approximation computed at creation time.
        sp_curve_cache: SpCurveApproximation,
    },
}

/// Bounded-error B-spline approximation of a symbolic intersection curve.
///
/// The algebraic complexity explosion problem: if you Boolean A∩B to get
/// Symbolic Edge 1, then Boolean C through Edge 1, the solver must find
/// roots of three simultaneous surfaces. By the 4th Boolean, evaluating a
/// single point requires solving a 5D non-linear system.
///
/// The fix: `SurfaceIntersection` is lazy but cached. Fast consumers evaluate
/// this SP-curve. Only rigorous topological stitching falls back to the exact
/// symbolic surfaces.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpCurveApproximation {
    /// B-spline control points approximating the true intersection curve.
    pub control_points: Vec<[f64; 3]>,
    /// Knot vector.
    pub knots: Vec<f64>,
    /// Certified maximum deviation from the true symbolic curve.
    pub error_bound: f64,
    /// Parameter range.
    pub domain: (f64, f64),
}

/// How a curve's geometry and tolerance were established.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CurveProvenance {
    /// Curve is the exact intersection of two analytic surfaces.
    /// Tolerance is effectively zero (exact arithmetic).
    AnalyticIntersection {
        /// Indices of the two surfaces that produce this curve.
        surface_indices: [SurfaceIndex; 2],
    },

    /// Curve was computed by the SSI (Surface-Surface Intersection) solver.
    /// Tolerance is the certified solver residual.
    SsiSolver {
        /// SSI solver residual (3D tube radius).
        residual: f64,
        /// Number of subdivision iterations used.
        iterations: u32,
    },

    /// Curve was inherited from a parent edge during `SplitEdge`.
    SplitInherited {
        /// Raw index of the parent edge.
        parent_edge_index: u32,
        /// Parameter range of this segment on the parent curve.
        parameter_range: (f64, f64),
    },

    /// Curve was imported from an external source (STEP, IGES).
    Imported {
        /// The import healing tolerance.
        healing_tolerance: f64,
    },
}

/// Complete 3D curve geometry for one edge.
///
/// This is the geometry-layer mirror of `EdgeData` in `forge-topo`. The
/// topology stores an opaque `CurveRef` handle; the geometry store holds
/// the actual `CurveGeom` data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurveGeom {
    /// The 3D curve parametric definition.
    pub kind: CurveKind,
    /// Certified 3D uncertainty tube radius around this curve.
    /// For analytic curves this is ≈ 0. For SSI curves this is the solver
    /// residual. NEVER stored in `forge-topo`.
    pub tolerance: f64,
    /// How this curve was created and its tolerance derived.
    pub provenance: CurveProvenance,
}

impl CurveGeom {
    /// Create a curve geometry from an analytic intersection.
    pub fn from_analytic(kind: CurveKind, surfaces: [SurfaceIndex; 2]) -> Self {
        Self {
            kind,
            tolerance: 0.0,
            provenance: CurveProvenance::AnalyticIntersection {
                surface_indices: surfaces,
            },
        }
    }

    /// Create a curve geometry from an SSI solver result.
    pub fn from_ssi(kind: CurveKind, residual: f64, iterations: u32) -> Self {
        Self {
            kind,
            tolerance: residual,
            provenance: CurveProvenance::SsiSolver { residual, iterations },
        }
    }
}
