//! Sector classification tests for kissing contacts.
//!
//! DOMAIN: Verify that the multi-sample boundary fallback correctly
//! classifies faces when centroids land exactly on shared boundaries.
//!
//! INVARIANTS: No crashes, correct face counts, valid Euler characteristic.

use super::super::schema::BooleanOp;
use super::super::test_helpers::{build_cube, euler_audit, run_boolean};

// ── Touching vertex (no volume overlap) ──────────────────────────────────────

/// Two cubes touching at exactly one vertex: union preserves both.
///
/// Cube A: [-1,1]^3, Cube B: [1,3]^3.
/// Shared vertex at (1,1,1). No volume overlap.
/// Union should produce 12 faces (6 per cube).
#[test]
fn touching_vertex_union_face_count() {
    let result = run_boolean([0.0, 0.0, 0.0], 1.0, [2.0, 2.0, 2.0], 1.0, BooleanOp::Union);
    assert_eq!(
        result.topology().arena().face_count(),
        12,
        "Union of vertex-touching cubes should keep all 12 faces"
    );
}

/// Two cubes touching at one vertex: subtraction leaves target intact.
///
/// No volume overlap means A - B == A.
#[test]
fn touching_vertex_subtraction_preserves_target() {
    let result = run_boolean(
        [0.0, 0.0, 0.0],
        1.0,
        [2.0, 2.0, 2.0],
        1.0,
        BooleanOp::Subtraction,
    );
    assert_eq!(
        result.topology().arena().face_count(),
        6,
        "Subtraction with vertex-only contact should preserve target 6 faces"
    );
}

// ── Edge-on-edge contact (no volume overlap) ─────────────────────────────────

/// Two cubes sharing exactly one edge: union preserves both.
///
/// Cube A: [-1,1]^3, Cube B: [1,3]x[1,3]x[-1,1].
/// Shared edge at x=1, y=1, z∈[-1,1]. No volume overlap.
#[test]
fn edge_contact_union_face_count() {
    let result = run_boolean([0.0, 0.0, 0.0], 1.0, [2.0, 2.0, 0.0], 1.0, BooleanOp::Union);
    assert_eq!(
        result.topology().arena().face_count(),
        12,
        "Union of edge-touching cubes should keep all 12 faces"
    );
}

/// Two cubes sharing one edge: subtraction leaves target intact.
#[test]
fn edge_contact_subtraction_preserves_target() {
    let result = run_boolean(
        [0.0, 0.0, 0.0],
        1.0,
        [2.0, 2.0, 0.0],
        1.0,
        BooleanOp::Subtraction,
    );
    assert_eq!(
        result.topology().arena().face_count(),
        6,
        "Subtraction with edge-only contact should preserve target 6 faces"
    );
}

// ── Euler characteristic validation ──────────────────────────────────────────

/// Touching-vertex union produces valid Euler characteristic.
#[test]
fn touching_vertex_union_euler() {
    let result = run_boolean([0.0, 0.0, 0.0], 1.0, [2.0, 2.0, 2.0], 1.0, BooleanOp::Union);
    let (_v, _e, _f, chi) = euler_audit(result.topology().arena());
    assert!(
        chi == 2 || chi == 4,
        "Euler χ should be 2 (if merged into one shell) or 4 (two disjoint shells), got {chi}"
    );
}

/// Edge-contact union produces valid Euler characteristic.
#[test]
fn edge_contact_union_euler() {
    let result = run_boolean([0.0, 0.0, 0.0], 1.0, [2.0, 2.0, 0.0], 1.0, BooleanOp::Union);
    let (_v, _e, _f, chi) = euler_audit(result.topology().arena());
    assert!(chi == 2 || chi == 4, "Euler χ should be 2 or 4, got {chi}");
}
