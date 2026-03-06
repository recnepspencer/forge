#![cfg(any())]
//! Phase 1: Disjoint solid operations.
//!
//! The simplest Boolean cases — two solids that do not touch.
//! These test the basic pipeline wiring (validate → classify → select → assemble)
//! without exercising the split or coplanar phases.
//!
//! Expected results:
//! - Union: both solids kept, 12 faces total
//! - Subtraction: target kept unchanged, 6 faces
//! - Intersection: empty result (no shared volume)

use crate::operations::boolean::schema::BooleanOp;
use crate::operations::boolean::test_helpers::*;

/// Union of two non-touching cubes produces both solids (12 faces).
#[test]
fn union_disjoint_cubes() {
    let result = run_boolean(
        [0.0, 0.0, 0.0],
        1.0, // cube A at origin
        [5.0, 0.0, 0.0],
        1.0, // cube B far away
        BooleanOp::Union,
    );
    assert_eq!(
        result.topology().arena().face_count(),
        12,
        "Union of disjoint cubes should have 12 faces (6 + 6)"
    );
}

/// Subtraction with disjoint tool: target is unchanged (6 faces).
#[test]
fn subtract_disjoint_cubes() {
    let result = run_boolean(
        [0.0, 0.0, 0.0],
        1.0,
        [5.0, 0.0, 0.0],
        1.0,
        BooleanOp::Subtraction,
    );
    assert_eq!(
        result.topology().arena().face_count(),
        6,
        "Subtracting a disjoint tool should leave target unchanged (6 faces)"
    );
}

/// Intersection of disjoint cubes: no shared volume → should error or be empty.
#[test]
fn intersect_disjoint_cubes() {
    let result = try_boolean(
        [0.0, 0.0, 0.0],
        1.0,
        [5.0, 0.0, 0.0],
        1.0,
        BooleanOp::Intersection,
    );
    // Disjoint intersection has no shared volume — either empty result or error
    match result {
        Ok(r) => assert_eq!(
            r.topology().arena().face_count(),
            0,
            "Intersection of disjoint cubes should be empty"
        ),
        Err(_) => {} // acceptable — empty intersection can be an error
    }
}
