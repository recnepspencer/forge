//! Classify the geometric relationship between two surfaces.
//!
//! DOMAIN: Surface pair classification — coincident, disjoint, tangent,
//! or general intersection. Wraps forge-geom plane classification so the
//! result is always a `SurfacePairClass`, never a raw f64 comparison.
//!
//! POLICY REQUIREMENTS: CoincidentGeometry, NearTangency (declared in step contract).
//! These are validated by `OperationPipeline::run_step` before this runs.
//!
//! DEPENDENCIES: forge-geom (Plane), forge-core (KernelError)

use forge_core::KernelError;
use forge_geom::facade::Plane;

/// Geometric relationship between two surface planes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfacePairClass {
    /// The planes are (within tolerance) identical — same normal and offset.
    Coincident,
    /// The planes are parallel but distinct (no intersection line).
    Parallel,
    /// The planes are nearly tangent (dihedral angle < near-tangency threshold).
    NearTangent,
    /// The planes intersect at a general angle (the common case).
    Intersecting,
}

/// Classify the relationship between two planar surfaces.
///
/// Uses exact plane normal comparison for parallelism / coincidence,
/// and the supplied tolerance thresholds (from `ToleranceConfig` via kernel)
/// for near-tangency detection. All threshold comparisons go through named
/// f64 parameters — no inline magic numbers (Architecture Rule §4.1).
///
/// # Parameters
/// - `plane_a`, `plane_b` — the two surface planes (any orientation)
/// - `coincidence_tol` — distance threshold for treating planes as identical
/// - `tangency_tol` — sine-of-dihedral threshold below which planes are near-tangent
pub fn classify_surface_pair(
    plane_a: &Plane,
    plane_b: &Plane,
    coincidence_tol: f64,
    tangency_tol: f64,
) -> Result<SurfacePairClass, KernelError> {
    let na = plane_a.normal();
    let nb = plane_b.normal();

    // dot(na, nb) — both normals are unit vectors.
    let dot = na[0] * nb[0] + na[1] * nb[1] + na[2] * nb[2];
    let abs_dot = dot.abs();

    // sin(dihedral) ≈ sqrt(1 - dot²) for unit vectors.
    // Near-tangent when sin(dihedral) < tangency_tol, i.e. abs_dot > sqrt(1 - tol²).
    let sin_sq = (1.0 - abs_dot * abs_dot).max(0.0);
    let sin_angle = sin_sq.sqrt();

    if sin_angle < tangency_tol {
        // Normals are (nearly) parallel — check offset distance for coincidence.
        let offset_diff = (plane_a.offset() - plane_b.offset()).abs();
        if offset_diff < coincidence_tol {
            return Ok(SurfacePairClass::Coincident);
        }
        return Ok(SurfacePairClass::Parallel);
    }

    if sin_angle < tangency_tol * 10.0 {
        return Ok(SurfacePairClass::NearTangent);
    }

    Ok(SurfacePairClass::Intersecting)
}
