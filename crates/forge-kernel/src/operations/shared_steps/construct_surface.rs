//! Surface construction from geometric constraints.
//!
//! DOMAIN: Constructs a `Plane` from geometric inputs — currently planar
//! surfaces from 3+ co-planar points or an explicit normal+offset.
//! Pure geometry — no topology handles, no GeometryState.
//!
//! DEPENDENCIES: forge-geom (Plane), forge-core (KernelError)

use forge_core::KernelError;
use crate::geom::Plane;

/// A surface constructed from geometric inputs.
#[derive(Debug, Clone)]
pub enum ConstructedSurface {
    /// A planar surface.
    Planar(Plane),
}

/// Construct a plane from 3 co-planar 3D points (CCW winding gives outward normal).
///
/// Returns `InvalidInput` if the three points are collinear.
///
/// # Parameters
/// - `a`, `b`, `c` — three non-collinear 3D points on the plane
/// - `degeneracy_tol` — cross-product magnitude threshold for degeneracy detection
pub fn construct_planar_surface_from_points(
    a: [f64; 3],
    b: [f64; 3],
    c: [f64; 3],
    degeneracy_tol: f64,
) -> Result<ConstructedSurface, KernelError> {
    let ab = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let ac = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
    let cross = [
        ab[1] * ac[2] - ab[2] * ac[1],
        ab[2] * ac[0] - ab[0] * ac[2],
        ab[0] * ac[1] - ab[1] * ac[0],
    ];
    let mag = (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt();

    if mag < degeneracy_tol {
        return Err(KernelError::InvalidInput {
            message: format!(
                "construct_surface: points are collinear \
                 (cross-product magnitude {:.2e}, threshold {:.2e})",
                mag, degeneracy_tol
            ),
            context: None,
        });
    }

    let normal = [cross[0] / mag, cross[1] / mag, cross[2] / mag];
    let offset = normal[0] * a[0] + normal[1] * a[1] + normal[2] * a[2];

    Plane::try_new(normal, offset)
        .map(ConstructedSurface::Planar)
        .map_err(|e| KernelError::InvalidInput {
            message: format!("construct_surface: {}", e),
            context: None,
        })
}

/// Construct a plane from an explicit unit normal and signed offset.
///
/// Returns `InvalidInput` if the normal is not unit length within tolerance.
pub fn construct_planar_surface_from_normal(
    normal: [f64; 3],
    offset: f64,
    unit_tol: f64,
) -> Result<ConstructedSurface, KernelError> {
    let mag_sq = normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2];
    if (mag_sq - 1.0).abs() > unit_tol {
        return Err(KernelError::InvalidInput {
            message: format!(
                "construct_surface: normal is not unit length (|n|² = {:.6e})",
                mag_sq
            ),
            context: None,
        });
    }

    Plane::try_new(normal, offset)
        .map(ConstructedSurface::Planar)
        .map_err(|e| KernelError::InvalidInput {
            message: format!("construct_surface: {}", e),
            context: None,
        })
}
