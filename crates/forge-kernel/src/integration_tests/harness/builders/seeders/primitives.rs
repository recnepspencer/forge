//! Primitive seeders — named, composable test scenarios.
//!
//! DOMAIN: Seeders are like Laravel's DatabaseSeeders. Each one tells
//! a specific story — "the hello-world manifold cube", "the degenerate
//! tetrahedron at tolerance boundary", "two disjoint solids". They
//! produce real solids through the real pipeline.
//!
//! Seeders call factories (shapes) internally, never raw `make_*`.

use crate::engine::facade::SolidEnvelope;
use forge_core::KernelError;

use crate::integration_tests::harness::builders::configs;
use crate::integration_tests::harness::shapes;

/// The hello-world. One manifold cube, fully validated.
///
/// Equivalent to Laravel's `UserSeeder` — the simplest possible valid entity.
pub fn seed_manifold_cube() -> Result<SolidEnvelope, KernelError> {
    shapes::unit_cube().map(|res| res.into_value())
}

/// Two cubes at different positions with no spatial overlap.
///
/// Equivalent to Laravel's two companies with no shared projects.
/// Useful for testing that operations on disjoint solids produce
/// independent results.
pub fn seed_disjoint_pair() -> Result<(SolidEnvelope, SolidEnvelope), KernelError> {
    let a = shapes::cube([0.0, 0.0, 0.0], 1.0)?.into_value();
    let b = shapes::cube([10.0, 0.0, 0.0], 1.0)?.into_value();
    Ok((a, b))
}

/// Tetrahedron at tolerance boundary — vertices nearly collapsed.
///
/// Equivalent to Laravel's `UserSeeder` with edge-case data
/// (empty name, max-length email). The `scale` should be near
/// the spatial tolerance to trigger near-degenerate geometry.
pub fn seed_degenerate_tetra(scale: f64) -> Result<SolidEnvelope, KernelError> {
    shapes::tetrahedron_at([0.0; 3], scale).map(|res| res.into_value())
}

/// A cube at a specific tolerance — useful for Euler operator tests.
///
/// Returns a `SolidEnvelope`. The caller can call `envelope.into_draft()`
/// to get a mutable draft for Euler operator testing.
pub fn seed_euler_playground() -> Result<SolidEnvelope, KernelError> {
    shapes::unit_cube().map(|res| res.into_value())
}

/// All primitive shapes at once — dodecahedron, prism, pyramid, wedge.
///
/// Useful for bulk invariant testing: "do ALL shapes satisfy Euler?"
pub fn seed_all_primitives() -> Result<Vec<SolidEnvelope>, KernelError> {
    Ok(vec![
        shapes::unit_cube()?.into_value(),
        shapes::tetrahedron()?.into_value(),
        shapes::dodecahedron([0.0; 3], 1.0)?.into_value(),
        shapes::prism([0.0; 3], 6, 1.0, 2.0)?.into_value(),
        shapes::pyramid([0.0; 3], 4, 1.0, 2.0)?.into_value(),
        shapes::wedge([0.0; 3], [1.0, 1.0, 1.0])?.into_value(),
    ])
}

/// Two identical cubes at the same position — stress test for
/// exact coincidence handling.
pub fn seed_coincident_pair() -> Result<(SolidEnvelope, SolidEnvelope), KernelError> {
    let a = shapes::unit_cube()?.into_value();
    let b = shapes::unit_cube()?.into_value();
    Ok((a, b))
}

/// Cube built with tight tolerance config.
///
/// Validates that the pipeline handles strict tolerance without
/// producing degenerate geometry.
pub fn seed_tight_tolerance_cube() -> Result<SolidEnvelope, KernelError> {
    let config = configs::config_tight();
    shapes::cube_with_config([0.0; 3], 1.0, &config).map(|res| res.into_value())
}
