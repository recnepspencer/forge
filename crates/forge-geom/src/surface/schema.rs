//! Data shapes for parametric surfaces.
//!
//! DOMAIN: Defines the surface type hierarchy — analytic (plane, cylinder,
//! cone, sphere, torus) and freeform (NURBS). Each surface stores its
//! parametric definition and valid parameter domain.
//!
//! DEPENDENCIES: serde (serialization), forge-topo handles (SurfaceRef)

use serde::{Deserialize, Serialize};

/// Parametric surface kinds supported by the kernel.
///
/// Analytic surfaces have closed-form `point_at`/`normal_at` evaluation.
/// NURBS surfaces require numerical evaluation and are Phase 7.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SurfaceKind {
    /// Infinite plane: `n · p = d`.
    Plane {
        /// Outward-facing unit normal.
        normal: [f64; 3],
        /// Signed distance from origin along normal.
        offset: f64,
    },

    /// Right circular cylinder of infinite extent along `axis`.
    Cylinder {
        /// A point on the axis.
        origin: [f64; 3],
        /// Unit direction of the axis.
        axis: [f64; 3],
        /// Radius (must be > 0).
        radius: f64,
    },

    /// Right circular cone with apex at `apex`.
    Cone {
        /// Tip of the cone.
        apex: [f64; 3],
        /// Unit direction of the axis (from apex toward base).
        axis: [f64; 3],
        /// Half-angle in radians (0 < half_angle < π/2).
        half_angle: f64,
    },

    /// Sphere centered at `center`.
    Sphere {
        /// Center point.
        center: [f64; 3],
        /// Radius (must be > 0).
        radius: f64,
    },

    /// Torus (doughnut) centered at `center`.
    Torus {
        /// Center of the torus.
        center: [f64; 3],
        /// Unit axis of rotational symmetry.
        axis: [f64; 3],
        /// Major radius (center of tube to center of torus).
        major_r: f64,
        /// Minor radius (radius of the tube).
        minor_r: f64,
    },
}

/// Valid parameter domain for a surface.
///
/// For analytic surfaces, the domain is typically:
/// - Plane: `u ∈ (-∞, ∞)`, `v ∈ (-∞, ∞)` — clamped to a finite working range.
/// - Cylinder: `u ∈ [0, 2π)` (periodic), `v ∈ (-∞, ∞)`.
/// - Sphere: `u ∈ [0, 2π)` (periodic), `v ∈ [-π/2, π/2]`.
/// - Torus: both periodic `[0, 2π)`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDomain {
    /// Minimum of the u parameter.
    pub u_min: f64,
    /// Maximum of the u parameter.
    pub u_max: f64,
    /// Minimum of the v parameter.
    pub v_min: f64,
    /// Maximum of the v parameter.
    pub v_max: f64,
    /// Whether the u parameter wraps (e.g. angular parameters on cylinders).
    pub u_periodic: bool,
    /// Whether the v parameter wraps.
    pub v_periodic: bool,
}

impl ParameterDomain {
    /// Standard plane domain (finite working range for UV operations).
    pub fn plane() -> Self {
        Self {
            u_min: -1e6, u_max: 1e6,
            v_min: -1e6, v_max: 1e6,
            u_periodic: false, v_periodic: false,
        }
    }

    /// Standard cylinder domain: u ∈ [0, 2π) periodic, v ∈ [-1e6, 1e6].
    pub fn cylinder() -> Self {
        Self {
            u_min: 0.0, u_max: std::f64::consts::TAU,
            v_min: -1e6, v_max: 1e6,
            u_periodic: true, v_periodic: false,
        }
    }

    /// Standard cone domain: u ∈ [0, 2π) periodic, v ∈ [0, 1e6].
    pub fn cone() -> Self {
        Self {
            u_min: 0.0, u_max: std::f64::consts::TAU,
            v_min: 0.0, v_max: 1e6,
            u_periodic: true, v_periodic: false,
        }
    }

    /// Standard sphere domain: u ∈ [0, 2π) periodic, v ∈ [-π/2, π/2].
    pub fn sphere() -> Self {
        use std::f64::consts::{FRAC_PI_2, TAU};
        Self {
            u_min: 0.0, u_max: TAU,
            v_min: -FRAC_PI_2, v_max: FRAC_PI_2,
            u_periodic: true, v_periodic: false,
        }
    }

    /// Standard torus domain: both u, v ∈ [0, 2π) periodic.
    pub fn torus() -> Self {
        Self {
            u_min: 0.0, u_max: std::f64::consts::TAU,
            v_min: 0.0, v_max: std::f64::consts::TAU,
            u_periodic: true, v_periodic: true,
        }
    }
}

/// Complete surface data: parametric definition + domain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurfaceData {
    /// The parametric surface type.
    pub kind: SurfaceKind,
    /// Valid parameter range for (u, v).
    pub domain: ParameterDomain,
}

impl SurfaceData {
    /// Create a planar surface from a normal and offset.
    pub fn plane(normal: [f64; 3], offset: f64) -> Self {
        Self {
            kind: SurfaceKind::Plane { normal, offset },
            domain: ParameterDomain::plane(),
        }
    }

    /// Create a cylindrical surface.
    pub fn cylinder(origin: [f64; 3], axis: [f64; 3], radius: f64) -> Self {
        Self {
            kind: SurfaceKind::Cylinder { origin, axis, radius },
            domain: ParameterDomain::cylinder(),
        }
    }

    /// Create a conical surface.
    pub fn cone(apex: [f64; 3], axis: [f64; 3], half_angle: f64) -> Self {
        Self {
            kind: SurfaceKind::Cone { apex, axis, half_angle },
            domain: ParameterDomain::cone(),
        }
    }

    /// Create a spherical surface.
    pub fn sphere(center: [f64; 3], radius: f64) -> Self {
        Self {
            kind: SurfaceKind::Sphere { center, radius },
            domain: ParameterDomain::sphere(),
        }
    }

    /// Create a toroidal surface.
    pub fn torus(center: [f64; 3], axis: [f64; 3], major_r: f64, minor_r: f64) -> Self {
        Self {
            kind: SurfaceKind::Torus { center, axis, major_r, minor_r },
            domain: ParameterDomain::torus(),
        }
    }
}

/// Relationship between two surfaces (for analytic arbitration).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceRelation {
    /// Surfaces are analytically identical (same type, same parameters within
    /// machine epsilon). Boolean engine skips SSI and falls back to 2D graph
    /// merge in parameter space.
    Coincident,
    /// Surfaces are analytically known to never intersect (e.g. parallel
    /// planes, concentric spheres with different radii). Boolean engine
    /// skips this face pair entirely.
    Disjoint,
    /// Surfaces may intersect — must run the SSI solver.
    General,
}
