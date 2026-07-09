//! Data shapes for parametric surfaces.
//!
//! DOMAIN: Defines the surface type hierarchy — analytic (plane, cylinder,
//! cone, sphere, torus) and freeform (NURBS). Each surface stores its
//! parametric definition and valid parameter domain.
//!
//! DEPENDENCIES: serde (serialization), WORTH-topo handles (SurfaceRef)

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

    /// Triaxial ellipsoid centered at `center` with three distinct principal
    /// radii aligned to the provided orthonormal frame.
    TriaxialEllipsoid {
        /// Center point.
        center: [f64; 3],
        /// Unit principal axis for radius_a.
        axis_u: [f64; 3],
        /// Unit principal axis for radius_b.
        axis_v: [f64; 3],
        /// Unit principal axis for radius_c.
        axis_w: [f64; 3],
        /// Principal radius along `axis_u`.
        radius_a: f64,
        /// Principal radius along `axis_v`.
        radius_b: f64,
        /// Principal radius along `axis_w`.
        radius_c: f64,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriaxialEllipsoidDefinitionError {
    RadiusMustBePositive,
    RadiiMustBeDistinct,
    AxisFrameMustBeUnitAndOrthonormal,
}

/// Valid parameter domain for a surface.
///
/// For analytic surfaces, the domain is typically:
/// - Plane: `u ∈ (-∞, ∞)`, `v ∈ (-∞, ∞)` — clamped to a finite working range.
/// - Cylinder: `u ∈ [0, 2π)` (periodic), `v ∈ (-∞, ∞)`.
/// - Sphere: `u ∈ [0, 2π)` (periodic), `v ∈ [-π/2, π/2]`.
/// - Torus: both periodic `[0, 2π)`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
            u_min: -1e6,
            u_max: 1e6,
            v_min: -1e6,
            v_max: 1e6,
            u_periodic: false,
            v_periodic: false,
        }
    }

    /// Standard cylinder domain: u ∈ [0, 2π) periodic, v ∈ [-1e6, 1e6].
    pub fn cylinder() -> Self {
        Self {
            u_min: 0.0,
            u_max: std::f64::consts::TAU,
            v_min: -1e6,
            v_max: 1e6,
            u_periodic: true,
            v_periodic: false,
        }
    }

    /// Standard cone domain: u ∈ [0, 2π) periodic, v ∈ [0, 1e6].
    pub fn cone() -> Self {
        Self {
            u_min: 0.0,
            u_max: std::f64::consts::TAU,
            v_min: 0.0,
            v_max: 1e6,
            u_periodic: true,
            v_periodic: false,
        }
    }

    /// Standard sphere domain: u ∈ [0, 2π) periodic, v ∈ [-π/2, π/2].
    pub fn sphere() -> Self {
        use std::f64::consts::{FRAC_PI_2, TAU};
        Self {
            u_min: 0.0,
            u_max: TAU,
            v_min: -FRAC_PI_2,
            v_max: FRAC_PI_2,
            u_periodic: true,
            v_periodic: false,
        }
    }

    /// Standard torus domain: both u, v ∈ [0, 2π) periodic.
    pub fn torus() -> Self {
        Self {
            u_min: 0.0,
            u_max: std::f64::consts::TAU,
            v_min: 0.0,
            v_max: std::f64::consts::TAU,
            u_periodic: true,
            v_periodic: true,
        }
    }

    /// Standard triaxial ellipsoid domain: u ∈ [0, 2π) periodic, v ∈ [-π/2, π/2].
    pub fn triaxial_ellipsoid() -> Self {
        Self::sphere()
    }

    pub fn structural_signature(&self) -> String {
        format!(
            concat!("u:[{:016x},{:016x}]:{}|", "v:[{:016x},{:016x}]:{}"),
            self.u_min.to_bits(),
            self.u_max.to_bits(),
            self.u_periodic,
            self.v_min.to_bits(),
            self.v_max.to_bits(),
            self.v_periodic
        )
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
            kind: SurfaceKind::Cylinder {
                origin,
                axis,
                radius,
            },
            domain: ParameterDomain::cylinder(),
        }
    }

    /// Create a conical surface.
    pub fn cone(apex: [f64; 3], axis: [f64; 3], half_angle: f64) -> Self {
        Self {
            kind: SurfaceKind::Cone {
                apex,
                axis,
                half_angle,
            },
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

    /// Create a triaxial ellipsoid surface.
    pub fn triaxial_ellipsoid(
        center: [f64; 3],
        axis_u: [f64; 3],
        axis_v: [f64; 3],
        axis_w: [f64; 3],
        radius_a: f64,
        radius_b: f64,
        radius_c: f64,
    ) -> Result<Self, TriaxialEllipsoidDefinitionError> {
        validate_triaxial_ellipsoid_definition(
            axis_u, axis_v, axis_w, radius_a, radius_b, radius_c,
        )?;
        Ok(Self {
            kind: SurfaceKind::TriaxialEllipsoid {
                center,
                axis_u,
                axis_v,
                axis_w,
                radius_a,
                radius_b,
                radius_c,
            },
            domain: ParameterDomain::triaxial_ellipsoid(),
        })
    }

    /// Create a toroidal surface.
    pub fn torus(center: [f64; 3], axis: [f64; 3], major_r: f64, minor_r: f64) -> Self {
        Self {
            kind: SurfaceKind::Torus {
                center,
                axis,
                major_r,
                minor_r,
            },
            domain: ParameterDomain::torus(),
        }
    }
}

fn validate_triaxial_ellipsoid_definition(
    axis_u: [f64; 3],
    axis_v: [f64; 3],
    axis_w: [f64; 3],
    radius_a: f64,
    radius_b: f64,
    radius_c: f64,
) -> Result<(), TriaxialEllipsoidDefinitionError> {
    if radius_a <= 0.0 || radius_b <= 0.0 || radius_c <= 0.0 {
        return Err(TriaxialEllipsoidDefinitionError::RadiusMustBePositive);
    }
    if radius_a.to_bits() == radius_b.to_bits()
        || radius_a.to_bits() == radius_c.to_bits()
        || radius_b.to_bits() == radius_c.to_bits()
    {
        return Err(TriaxialEllipsoidDefinitionError::RadiiMustBeDistinct);
    }
    if !is_unit(axis_u) || !is_unit(axis_v) || !is_unit(axis_w) {
        return Err(TriaxialEllipsoidDefinitionError::AxisFrameMustBeUnitAndOrthonormal);
    }
    if dot(axis_u, axis_v).abs() > 1e-12
        || dot(axis_u, axis_w).abs() > 1e-12
        || dot(axis_v, axis_w).abs() > 1e-12
    {
        return Err(TriaxialEllipsoidDefinitionError::AxisFrameMustBeUnitAndOrthonormal);
    }
    Ok(())
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn is_unit(axis: [f64; 3]) -> bool {
    (dot(axis, axis) - 1.0).abs() <= 1e-12
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
    /// Classification could not be safely decided under bounded precision.
    ///
    /// Kernel-side policy: fail-closed by default for merge eligibility.
    /// Must emit a `TracedDecision` (precision/policy escalation), never
    /// silently proceed. The kernel may choose to escalate to exact
    /// arithmetic or reject the merge.
    Undetermined,
}
