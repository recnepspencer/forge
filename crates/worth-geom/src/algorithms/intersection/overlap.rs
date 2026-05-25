//! Intersection helpers composed from lower-level geometric predicates.
//!
//! DOMAIN: Polygon/region overlap tests and projection-based intersection checks.

/// Test if two coplanar 3D polygons overlap in area.
///
/// Spec-aligned export wrapper for dominant-axis projection overlap.
pub fn polygons_overlap_3d(
    plane_normal: [f64; 3],
    poly_a: &[[f64; 3]],
    poly_b: &[[f64; 3]],
) -> bool {
    crate::algorithms::intersection::polygon_overlap::polygons_overlap_3d(
        plane_normal,
        poly_a,
        poly_b,
    )
}
