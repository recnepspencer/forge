//! Determinism verification for the geometry kernel.
//!
//! DOMAIN: A deterministic kernel must produce identical results
//! when given identical inputs. This module provides replay-based
//! determinism assertions using production hashers.
//!
//! Checks BOTH topology (arena adjacency) and geometry (vertex positions,
//! face planes) — catching geometry-only nondeterminism that topology-only
//! hashing would miss.
//!
//! Without this, determinism is theoretical.

use crate::engine::facade::SolidEnvelope;
use forge_core::envelope::OperationResult;
use forge_core::KernelError;

// ── Determinism assertion ────────────────────────────────────────────────────

/// Assert that running the same operation twice produces identical results.
///
/// Uses `SolidEnvelope::full_fingerprint()` — hashes topology arenas,
/// all vertex positions (f64 bit-exact), and all face plane normals + offsets.
///
/// If the full fingerprint diverges, also checks the topology-only fingerprint
/// to distinguish structural vs geometry-only nondeterminism.
///
/// ```rust,ignore
/// assert_deterministic(|| {
///     shapes::unit_cube()
/// });
/// ```
pub fn assert_deterministic<F>(build_fn: F)
where
    F: Fn() -> Result<OperationResult<SolidEnvelope>, KernelError>,
{
    let env1 = build_fn().expect("First run failed").into_value();
    let env2 = build_fn().expect("Second run failed").into_value();

    let full1 = env1.full_fingerprint();
    let full2 = env2.full_fingerprint();

    if full1 != full2 {
        // Diagnose: is it topology or geometry?
        let topo1 = env1.topology_fingerprint();
        let topo2 = env2.topology_fingerprint();

        if topo1 != topo2 {
            panic!(
                "Determinism violation: TOPOLOGY diverged between identical runs\n\
                 Run 1 topology: {:#034x}\n\
                 Run 2 topology: {:#034x}\n\
                 Run 1 full:     {:#034x}\n\
                 Run 2 full:     {:#034x}",
                topo1, topo2, full1, full2
            );
        } else {
            panic!(
                "Determinism violation: GEOMETRY diverged (topology identical)\n\
                 Shared topology: {:#034x}\n\
                 Run 1 full:      {:#034x}\n\
                 Run 2 full:      {:#034x}\n\
                 This means vertex positions or face planes differ between runs.",
                topo1, full1, full2
            );
        }
    }
}

/// Assert that running the same operation N times always produces the same hash.
///
/// More robust than 2-run: catches intermittent nondeterminism.
/// Uses `full_fingerprint()` for both topology and geometry coverage.
pub fn assert_deterministic_n<F>(build_fn: F, n: usize)
where
    F: Fn() -> Result<OperationResult<SolidEnvelope>, KernelError>,
{
    assert!(n >= 2, "Need at least 2 runs for determinism check");

    let first = build_fn().expect("First run failed").into_value();
    let expected_full = first.full_fingerprint();
    let expected_topo = first.topology_fingerprint();

    for i in 1..n {
        let env = build_fn().unwrap_or_else(|e| {
            panic!("Run {} failed: {:?}", i + 1, e);
        }).into_value();

        let full = env.full_fingerprint();
        if full != expected_full {
            let topo = env.topology_fingerprint();
            let divergence = if topo != expected_topo { "TOPOLOGY" } else { "GEOMETRY" };
            panic!(
                "Determinism violation on run {}/{}: {} diverged\n\
                 Expected full: {:#034x}\n\
                 Got full:      {:#034x}\n\
                 Expected topo: {:#034x}\n\
                 Got topo:      {:#034x}",
                i + 1, n, divergence,
                expected_full, full,
                expected_topo, topo
            );
        }
    }
}

