//! Representation traits for visual output.
//!
//! DOMAIN: Contracts for converting geometry into visual representations.
//! DEPENDENCIES: `TriangleMesh` (schema)

use crate::TriangleMesh;

/// Trait for geometry that can provide an SDF (Signed Distance Field).
///
/// Implementors produce a distance value for any point in 3D space.
/// Negative = inside, positive = outside, zero = on surface.
///
/// This is consumed by the real-time preview engine for 60fps
/// ray-marched visualization of the model while B-Rep materializes
/// in the background (see Product Pitch, Pillar 4).
pub trait Viewable {
    /// Evaluate the signed distance from `point` to the surface.
    ///
    /// - Negative: point is inside the solid
    /// - Positive: point is outside the solid
    /// - Zero: point is on the surface boundary
    fn evaluate_sdf(&self, point: [f64; 3]) -> f64;

    /// Axis-aligned bounding box as `(min, max)` corners.
    fn bounding_box(&self) -> ([f64; 3], [f64; 3]);
}

/// Trait for geometry that can be tessellated into a triangle mesh.
///
/// Implementors produce a `TriangleMesh` at a given tolerance.
/// Smaller tolerance = more triangles, higher fidelity.
pub trait Tessellatable {
    /// Generate a triangle mesh approximation of this geometry.
    ///
    /// `tolerance` controls the maximum deviation from the true
    /// surface — smaller values produce denser, more accurate meshes.
    fn tessellate(&self, tolerance: f64) -> TriangleMesh;
}
