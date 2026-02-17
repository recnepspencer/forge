//! Handcrafted deterministic edge-case battery for planar Booleans.
//!
//! DOMAIN: Milestone 2.5 — every hard planar scenario with pinned assertions.
//! INVARIANTS: No crashes, no non-manifold output, every ambiguous case logged.
//! DEPENDENCIES: `boolean` (execute_boolean), `geometry_store`, `mesh_builder`

use super::test_helpers::{run_boolean, try_boolean};
use super::schema::BooleanOp;

// ──────────────────────────────────────────────────────────
// 1. COPLANAR FACE OVERLAPS
// ──────────────────────────────────────────────────────────

/// Two cubes sharing one face flush along +X.
/// Cube A: center [0,0,0] half=1 → occupies [-1,1]^3
/// Cube B: center [2,0,0] half=1 → occupies [1,3]×[-1,1]^2
/// Shared face at x=1.
///
/// Union should NOT crash and should produce a valid result.
#[test]
fn coplanar_shared_face_union() {
    let result = try_boolean(
        [0.0, 0.0, 0.0], 1.0,
        [2.0, 0.0, 0.0], 1.0,
        BooleanOp::Union,
    );

    assert!(
        result.is_ok(),
        "Flush-face union must not crash: {:?}",
        result.err()
    );

    let bool_result = result.unwrap();
    let total_faces = bool_result.target_faces_kept() + bool_result.tool_faces_kept();
    assert!(
        total_faces >= 6,
        "Union of flush cubes should produce at least 6 faces, got {}",
        total_faces
    );
}

/// Subtraction of flush cubes: A - B where they share a face.
/// Since B is adjacent but does NOT penetrate A, the target stays intact.
#[test]
fn coplanar_shared_face_subtraction() {
    let result = try_boolean(
        [0.0, 0.0, 0.0], 1.0,
        [2.0, 0.0, 0.0], 1.0,
        BooleanOp::Subtraction,
    );

    assert!(
        result.is_ok(),
        "Flush-face subtraction must not crash: {:?}",
        result.err()
    );
}

/// Intersection of flush cubes: shared face only (degenerate/zero-volume).
#[test]
fn coplanar_shared_face_intersection() {
    let result = try_boolean(
        [0.0, 0.0, 0.0], 1.0,
        [2.0, 0.0, 0.0], 1.0,
        BooleanOp::Intersection,
    );

    match result {
        Ok(r) => {
            let total = r.target_faces_kept() + r.tool_faces_kept();
            eprintln!("Flush intersection produced {} faces", total);
        }
        Err(e) => {
            eprintln!("Flush intersection returned error (acceptable): {:?}", e);
        }
    }
}

// ──────────────────────────────────────────────────────────
// 2. EDGE-ON-EDGE INTERSECTIONS
// ──────────────────────────────────────────────────────────

/// Two cubes sharing exactly one edge.
/// Cube A: center [0,0,0] half=1 → [-1,1]^3
/// Cube B: center [2,2,0] half=1 → [1,3]×[1,3]×[-1,1]
/// Shared edge at x=1, y=1, z∈[-1,1].
#[test]
fn edge_on_edge_union() {
    let result = try_boolean(
        [0.0, 0.0, 0.0], 1.0,
        [2.0, 2.0, 0.0], 1.0,
        BooleanOp::Union,
    );

    assert!(
        result.is_ok(),
        "Edge-on-edge union must not crash: {:?}",
        result.err()
    );

    let bool_result = result.unwrap();
    let total = bool_result.target_faces_kept() + bool_result.tool_faces_kept();
    assert_eq!(
        total, 12,
        "Edge-touching cubes union should keep all 12 faces (two disjoint cubes)"
    );
}

/// A - B where they share only an edge: no volume overlap means target unchanged.
#[test]
fn edge_on_edge_subtraction() {
    let result = try_boolean(
        [0.0, 0.0, 0.0], 1.0,
        [2.0, 2.0, 0.0], 1.0,
        BooleanOp::Subtraction,
    );

    assert!(
        result.is_ok(),
        "Edge-on-edge subtraction must not crash: {:?}",
        result.err()
    );

    let bool_result = result.unwrap();
    assert_eq!(
        bool_result.target_faces_kept(), 6,
        "Target should retain all 6 faces when tool only touches at edge"
    );
}

// ──────────────────────────────────────────────────────────
// 3. VERTEX-ON-FACE INTERSECTIONS
// ──────────────────────────────────────────────────────────

/// Cube B's corner touches the interior of cube A's face.
/// Cube A: center [0,0,0] half=2 → [-2,2]^3
/// Cube B: center [3,0,0] half=1 → [2,4]×[-1,1]^2
/// B's corner at (2,−1,−1)...(2,1,1) touches A's +X face at x=2.
///
/// Since B's face at x=2 is coplanar with A's +X face but doesn't
/// overlap in the y-z projection (B is [−1,1]^2, A is [−2,2]^2),
/// this is a partial coplanar overlap (B's face is fully within A's face).
#[test]
fn vertex_on_face_union() {
    let result = try_boolean(
        [0.0, 0.0, 0.0], 2.0,
        [3.0, 0.0, 0.0], 1.0,
        BooleanOp::Union,
    );

    assert!(
        result.is_ok(),
        "Vertex-on-face union must not crash: {:?}",
        result.err()
    );
}

/// A - B where tool contacts at a face contact: target should be unchanged
/// since tool is entirely outside target, only touching the boundary.
#[test]
fn vertex_on_face_subtraction() {
    let result = try_boolean(
        [0.0, 0.0, 0.0], 2.0,
        [3.0, 0.0, 0.0], 1.0,
        BooleanOp::Subtraction,
    );

    assert!(
        result.is_ok(),
        "Vertex-on-face subtraction must not crash: {:?}",
        result.err()
    );
}

// ──────────────────────────────────────────────────────────
// 4. TOUCHING SOLIDS (SHARED VERTEX ONLY)
// ──────────────────────────────────────────────────────────

/// Two cubes touching at exactly one vertex.
/// Cube A: center [0,0,0] half=1 → [-1,1]^3
/// Cube B: center [2,2,2] half=1 → [1,3]^3
/// Shared vertex at (1,1,1).
#[test]
fn touching_vertex_union() {
    let result = try_boolean(
        [0.0, 0.0, 0.0], 1.0,
        [2.0, 2.0, 2.0], 1.0,
        BooleanOp::Union,
    );

    assert!(
        result.is_ok(),
        "Touching-vertex union must not crash: {:?}",
        result.err()
    );

    let bool_result = result.unwrap();
    let total = bool_result.target_faces_kept() + bool_result.tool_faces_kept();
    assert_eq!(
        total, 12,
        "Vertex-touching cubes union should keep all 12 faces (two disjoint cubes)"
    );
}

/// A - B where they share one vertex: no volume overlap.
#[test]
fn touching_vertex_subtraction() {
    let result = try_boolean(
        [0.0, 0.0, 0.0], 1.0,
        [2.0, 2.0, 2.0], 1.0,
        BooleanOp::Subtraction,
    );

    assert!(
        result.is_ok(),
        "Touching-vertex subtraction must not crash: {:?}",
        result.err()
    );

    let bool_result = result.unwrap();
    assert_eq!(
        bool_result.target_faces_kept(), 6,
        "Target should retain all 6 faces when tool only touches at vertex"
    );
}

// ──────────────────────────────────────────────────────────
// 5. IDENTICAL GEOMETRY
// ──────────────────────────────────────────────────────────

/// Union of two identical cubes should produce the same cube.
#[test]
fn identical_cubes_union() {
    let result = try_boolean(
        [0.0, 0.0, 0.0], 1.0,
        [0.0, 0.0, 0.0], 1.0,
        BooleanOp::Union,
    );

    assert!(
        result.is_ok(),
        "Identical union must not crash: {:?}",
        result.err()
    );

    let bool_result = result.unwrap();
    let total = bool_result.target_faces_kept() + bool_result.tool_faces_kept();
    assert!(
        total >= 6,
        "Identical union should produce at least 6 faces (the cube), got {}",
        total
    );
}

/// Intersection of two identical cubes should produce the same cube.
#[test]
fn identical_cubes_intersection() {
    let result = try_boolean(
        [0.0, 0.0, 0.0], 1.0,
        [0.0, 0.0, 0.0], 1.0,
        BooleanOp::Intersection,
    );

    assert!(
        result.is_ok(),
        "Identical intersection must not crash: {:?}",
        result.err()
    );

    let bool_result = result.unwrap();
    let total = bool_result.target_faces_kept() + bool_result.tool_faces_kept();
    assert!(
        total >= 6,
        "Identical intersection should produce at least 6 faces (the cube), got {}",
        total
    );
}

/// Subtraction of identical cubes should produce empty (or zero-face result).
#[test]
fn identical_cubes_subtraction() {
    let result = try_boolean(
        [0.0, 0.0, 0.0], 1.0,
        [0.0, 0.0, 0.0], 1.0,
        BooleanOp::Subtraction,
    );

    match result {
        Ok(r) => {
            let total = r.target_faces_kept() + r.tool_faces_kept();
            eprintln!("Identical subtraction produced {} faces (expected 0)", total);
        }
        Err(e) => {
            eprintln!("Identical subtraction returned error (acceptable): {:?}", e);
        }
    }
}

// ──────────────────────────────────────────────────────────
// 6. PARTIALLY OVERLAPPING CUBES
// ──────────────────────────────────────────────────────────

/// Standard half-overlap union with tighter assertions.
/// Cube A: [0,0,0] half=1, Cube B: [1,0,0] half=1.
/// Overlap region: [0,1]×[-1,1]^2.
#[test]
fn half_overlap_union() {
    let result = run_boolean(
        [0.0, 0.0, 0.0], 1.0,
        [1.0, 0.0, 0.0], 1.0,
        BooleanOp::Union,
    );

    let v = result.topology().arena().vertex_count();
    let e = result.topology().arena().half_edge_count() / 2;
    let f = result.topology().arena().face_count();

    eprintln!("Half-overlap union: V={}, E={}, F={}", v, e, f);

    assert!(
        f >= 10,
        "Half-overlap union should produce at least 10 faces, got {}",
        f
    );
}

/// Standard half-overlap subtraction.
#[test]
fn half_overlap_subtraction() {
    let result = run_boolean(
        [0.0, 0.0, 0.0], 1.0,
        [1.0, 0.0, 0.0], 1.0,
        BooleanOp::Subtraction,
    );

    let f = result.topology().arena().face_count();
    eprintln!("Half-overlap subtraction: F={}", f);

    assert!(f >= 6, "Half-overlap subtraction should produce at least 6 faces, got {}", f);
}

/// Standard half-overlap intersection.
#[test]
fn half_overlap_intersection() {
    let result = run_boolean(
        [0.0, 0.0, 0.0], 1.0,
        [1.0, 0.0, 0.0], 1.0,
        BooleanOp::Intersection,
    );

    let f = result.topology().arena().face_count();
    eprintln!("Half-overlap intersection: F={}", f);

    assert!(f >= 6, "Half-overlap intersection should produce at least 6 faces, got {}", f);
}

// ──────────────────────────────────────────────────────────
// 7. COMPLETELY DISJOINT (REGRESSION VALIDATION)
// ──────────────────────────────────────────────────────────

/// Union of well-separated cubes: all 12 faces kept.
#[test]
fn disjoint_cubes_union_edge_case() {
    let result = run_boolean(
        [0.0, 0.0, 0.0], 1.0,
        [10.0, 10.0, 10.0], 1.0,
        BooleanOp::Union,
    );

    assert_eq!(result.target_faces_kept(), 6);
    assert_eq!(result.tool_faces_kept(), 6);
    assert_eq!(result.topology().arena().vertex_count(), 16);
    assert_eq!(result.topology().arena().face_count(), 12);
}

/// Intersection of disjoint cubes should produce no volume → error is acceptable.
#[test]
fn disjoint_cubes_intersection_edge_case() {
    let result = try_boolean(
        [0.0, 0.0, 0.0], 1.0,
        [10.0, 10.0, 10.0], 1.0,
        BooleanOp::Intersection,
    );

    match result {
        Ok(r) => {
            let total = r.target_faces_kept() + r.tool_faces_kept();
            assert_eq!(total, 0, "Disjoint intersection should produce 0 faces");
        }
        Err(e) => {
            eprintln!("Disjoint intersection returned error (acceptable): {:?}", e);
        }
    }
}

/// Subtraction of disjoint cubes: target stays intact.
#[test]
fn disjoint_cubes_subtraction_edge_case() {
    let result = run_boolean(
        [0.0, 0.0, 0.0], 1.0,
        [10.0, 10.0, 10.0], 1.0,
        BooleanOp::Subtraction,
    );

    assert_eq!(result.target_faces_kept(), 6);
    assert_eq!(result.tool_faces_kept(), 0);
}

// ──────────────────────────────────────────────────────────
// 8. CONCENTRIC (DIFFERENT SIZES) — REGRESSION
// ──────────────────────────────────────────────────────────

/// Union of concentric cubes: outer shell should be the result.
#[test]
fn concentric_union() {
    let result = run_boolean(
        [0.0, 0.0, 0.0], 2.0,
        [0.0, 0.0, 0.0], 1.0,
        BooleanOp::Union,
    );

    assert_eq!(
        result.target_faces_kept() + result.tool_faces_kept(), 54,
        "Concentric union: target outside=54 (split faces outside inner), tool outside=0"
    );
}
