//! Surface construction from geometric constraints.
//!
//! DOMAIN: Constructs a `Plane` from geometric inputs — currently planar
//! surfaces from 3+ co-planar points or an explicit normal+offset.
//! Pure geometry — no topology handles, no GeometryState.
//!
//! DEPENDENCIES: forge-geom (Plane), forge-core (KernelError)

use forge_core::KernelError;
use forge_geom::facade::Plane;

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
    let ab = forge_geom::facade::distance(&a, &b);
    let ac = forge_geom::facade::distance(&a, &c);
    // Use triangle_area_3d for collinearity detection (area → 0 when collinear)
    let area = forge_geom::facade::triangle_area_3d(&a, &b, &c);
    // Area of a triangle = 0.5 * |AB × AC|, so cross magnitude = 2 * area
    let cross_mag = 2.0 * area;

    if cross_mag < degeneracy_tol {
        return Err(KernelError::InvalidInput {
            message: format!(
                "construct_surface: points are collinear \
                 (cross-product magnitude {:.2e}, threshold {:.2e})",
                cross_mag, degeneracy_tol
            ),
            context: None,
        });
    }

    // Construct normal via forge-geom: sub + cross + normalize_checked
    // This is a one-off geometry construction, not an inline math hotpath.
    let ab_vec = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let ac_vec = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
    let cross = [
        ab_vec[1] * ac_vec[2] - ab_vec[2] * ac_vec[1],
        ab_vec[2] * ac_vec[0] - ab_vec[0] * ac_vec[2],
        ab_vec[0] * ac_vec[1] - ab_vec[1] * ac_vec[0],
    ];
    let normal = [cross[0] / cross_mag, cross[1] / cross_mag, cross[2] / cross_mag];
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
