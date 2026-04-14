//! Surface evaluation trait contracts.
//!
//! DOMAIN: Defines the parametric surface evaluation API that future SSI
//! solvers and UV merge algorithms will program against. Existing
//! `SurfaceData::point_at`/`normal_at` methods remain as concrete
//! implementations; this trait is the abstraction layer for generic
//! algorithms.

use super::schema::ParameterDomain;

/// Parametric surface evaluation contract.
///
/// Implementors provide point, normal, and tangent evaluation over a
/// bounded parameter domain. This is the abstraction that decouples
/// merge/SSI algorithms from specific surface representations.
///
/// DESIGN TARGET: Not yet implemented by `SurfaceData`. Implementation
/// will be added when curved merge execution is built (post-Epic C).
pub trait EvaluateSurface {
    /// Evaluate the 3D point at parameter (u, v).
    fn point_at_uv(&self, u: f64, v: f64) -> [f64; 3];

    /// Evaluate the outward unit normal at parameter (u, v).
    fn normal_at_uv(&self, u: f64, v: f64) -> [f64; 3];

    /// Evaluate the tangent in the u-direction at parameter (u, v).
    fn tangent_u_at_uv(&self, u: f64, v: f64) -> [f64; 3];

    /// Evaluate the tangent in the v-direction at parameter (u, v).
    fn tangent_v_at_uv(&self, u: f64, v: f64) -> [f64; 3];

    /// The valid parameter domain for this surface.
    fn domain(&self) -> &ParameterDomain;
}
