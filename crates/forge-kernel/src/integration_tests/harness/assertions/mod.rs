//! Assertion modules for the test harness.
//!
//! DOMAIN: Split into three domains — topology wiring, geometric data,
//! and decision log correctness. This module re-exports all public
//! symbols from each sub-module so existing `harness::assertions::*`
//! imports continue to work unchanged.

pub mod topology;
pub mod geometry;
pub mod decisions;

// ── Re-exports for backward compatibility ────────────────────────────────────
// Existing tests use `harness::assertions::assert_all_invariants` etc.
// These re-exports preserve that import path.

pub use topology::{
    EntityCounts,
    assert_reciprocity,
    assert_face_ownership,
    assert_vertex_orbits,
    assert_edge_consistency,
    assert_loop_face_consistency,
    assert_closed_loops,
    assert_counts,
    assert_euler_formula,
    assert_all_invariants,
    assert_structural_invariants,
    assert_face_valence,
};

pub use decisions::{
    assert_decisions_well_formed,
    assert_vertex_decisions,
};

pub use geometry::{
    assert_geometry_complete,
    assert_positive_face_areas,
    assert_bounds,
    assert_volume,
    assert_face_plane,
    assert_edge_lengths,
};

// ── Master assertion ─────────────────────────────────────────────────────────

use crate::engine::facade::SolidEnvelope;
use forge_core::DecisionLog;

/// Run the full validation suite on a solid + its decision log.
///
/// This is the "nuclear option" — topology wiring, geometry completeness,
/// face area sanity, and decision well-formedness. Call after any
/// operation that produces a closed solid.
pub fn assert_valid_solid(env: &SolidEnvelope, decisions: &DecisionLog) {
    topology::assert_all_invariants(env.topology().arena());
    geometry::assert_geometry_complete(env);
    geometry::assert_positive_face_areas(env, 1e-12);
    decisions::assert_decisions_well_formed(decisions);
}
