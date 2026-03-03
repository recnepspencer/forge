//! Determinism verification for the geometry kernel.
//!
//! DOMAIN: A deterministic kernel must produce identical results
//! when given identical inputs. This module provides replay-based
//! determinism assertions using the production topology hasher.
//!
//! Without this, determinism is theoretical.

use crate::engine::facade::SolidEnvelope;
use forge_core::KernelError;
use forge_topo::transactions::compute_arena_topology_hash;

// ── Determinism assertion ────────────────────────────────────────────────────

/// Assert that running the same operation twice produces identical results.
///
/// Uses the production `compute_arena_topology_hash` — the same hasher
/// that the transaction system uses for structural signatures.
///
/// ```rust,ignore
/// assert_deterministic(|| {
///     shapes::unit_cube()
/// });
/// ```
pub fn assert_deterministic<F>(build_fn: F)
where
    F: Fn() -> Result<SolidEnvelope, KernelError>,
{
    let env1 = build_fn().expect("First run failed");
    let env2 = build_fn().expect("Second run failed");

    let hash1 = compute_arena_topology_hash(env1.topology().arena());
    let hash2 = compute_arena_topology_hash(env2.topology().arena());

    assert_eq!(
        hash1, hash2,
        "Determinism violation: identical operations produced different topology hashes\n\
         Run 1: {:#034x}\n\
         Run 2: {:#034x}",
        hash1, hash2
    );
}

/// Assert that running the same operation N times always produces the same hash.
///
/// More robust than 2-run: catches intermittent nondeterminism.
pub fn assert_deterministic_n<F>(build_fn: F, n: usize)
where
    F: Fn() -> Result<SolidEnvelope, KernelError>,
{
    assert!(n >= 2, "Need at least 2 runs for determinism check");

    let first = build_fn().expect("First run failed");
    let expected_hash = compute_arena_topology_hash(first.topology().arena());

    for i in 1..n {
        let env = build_fn().unwrap_or_else(|e| {
            panic!("Run {} failed: {:?}", i + 1, e);
        });
        let hash = compute_arena_topology_hash(env.topology().arena());
        assert_eq!(
            hash, expected_hash,
            "Determinism violation on run {}/{}: hash {:#034x} != expected {:#034x}",
            i + 1, n, hash, expected_hash
        );
    }
}
