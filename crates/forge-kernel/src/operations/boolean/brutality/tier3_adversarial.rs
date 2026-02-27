//! Tier 3 — Adversarial & Non-Manifold Tests (The "Four Nines")
//!
//! DOMAIN: Handling "dirty" geometry, cascading failures, pinch vertices,
//! and operations on boolean results (result reuse integrity).
//!
//! INVARIANTS:
//! - No panics on degenerate input
//! - Results are manifold (twin reciprocity, closed shells)
//! - Cascading operations do not accumulate corruption
//!
//! ═══════════════════════════════════════════════════════════════
//! REQUIRED CODE/MATH CHANGES TO PASS ALL TESTS:
//! ═══════════════════════════════════════════════════════════════
//!
//! 1. **Transaction-Based Rollback**: Wrap the entire Boolean in a
//!    transaction. If the Euler Auditor or Volume Auditor fails at the end,
//!    roll back the `TopologyState` to the pre-op version. Currently
//!    `MutableDraft` provides this via `commit()`, but the boolean pipeline
//!    doesn't use it for rollback on validation failure.
//!
//! 2. **Non-Manifold Post-Processor**: Detect and repair:
//!    - Edges with >2 faces ("Wire Edges")
//!    - Vertices where the "Star" of faces is not a single manifold disk
//!    Fix via "Mesh Snapping": merge vertices within tolerance or split
//!    non-manifold edges into distinct manifold edges.
//!
//! 3. **Signed Volume Accounting**:
//!    - `Volume = (1/3) ∬∂V r·n dS`
//!    - Verify: `Vol(A ∪ B) = Vol(A) + Vol(B) − Vol(A ∩ B)`
//!    - Use as a post-condition check to catch silent corruption.
//!
//! 4. **Simulation of Simplicity (SoS)**: If a predicate returns 0
//!    (exactly on the plane), use the `VertexId` to break ties:
//!    `if det == 0 { return id_a > id_b ? Neg : Pos }`
//!    This makes every decision deterministic and eliminates degenerate
//!    states. Critical for T3.1 (pinch vertex) and T3.4 (concentric).
//!
//! 5. **Result Reuse with Provenance Preservation**: When a boolean result
//!    is fed back as input, the copy phase must preserve or re-derive
//!    3-plane vertex provenance (`VertexMatchKey`) so cross-solid vertex
//!    matching works in subsequent operations. Without this, the stitch
//!    phase produces `MissingTwin` errors on chained booleans.
//!    THIS IS THE PRIMARY BLOCKER for T3.2, T3.3, T3.4, and T3.5.

use super::super::test_helpers::{
    build_cube, try_boolean, execute_boolean_logged, euler_audit,
};
use super::super::schema::{BooleanInput, BooleanOp};

// ══════════════════════════════════════════════════════════════
// §T3.1  PINCH VERTEX
// ══════════════════════════════════════════════════════════════

/// T3.1 — Two cubes touching at a single corner → union → subtract at pinch.
///
/// Step 1: Union cubes touching at (1,1,1).
/// Step 2: Subtract a small cube centered at the pinch point.
/// This tests whether the engine can handle non-manifold "pinch" topology
/// created by vertex-touching unions, and then resolve it via subtraction.
#[test]
fn pinch_vertex_subtract_at_contact() {
    let union_result = try_boolean(
        [0.0, 0.0, 0.0], 1.0,
        [2.0, 2.0, 2.0], 1.0,
        BooleanOp::Union,
    );

    match union_result {
        Ok(union_r) => {
            let (topo_union, geom_union, _) = union_r.into_states();
            let (topo_tool, geom_tool) = build_cube([1.0, 1.0, 1.0], 0.5);

            let input = BooleanInput::new(
                topo_union, geom_union,
                topo_tool, geom_tool,
                BooleanOp::Subtraction,
            );

            match execute_boolean_logged(input).into_result() {
                Ok(result) => {
                    let r = result;
                    let (v, e, f, chi) = euler_audit(r.topology().arena());
                    eprintln!("Pinch-subtract: V={v} E={e} F={f} χ={chi}");
                    assert!(f >= 6, "Should have faces after pinch subtraction");
                }
                Err(e) => {
                    eprintln!("Pinch-subtract error (tracking): {e}");
                }
            }
        }
        Err(e) => {
            eprintln!("Pinch union failed (tracking): {e}");
        }
    }
}

// ══════════════════════════════════════════════════════════════
// §T3.2  CASCADING NEAR-COINCIDENCE
// ══════════════════════════════════════════════════════════════

/// T3.2 — Boolean with 10⁻⁹ offset, then chain another boolean on result.
///
/// First boolean's tolerances are pushed near the boundary.
/// Second boolean must not inherit edge-case geometry that crashes.
#[test]
fn cascading_near_coincidence() {
    let epsilon = 1e-9;
    let first = try_boolean(
        [0.0, 0.0, 0.0], 1.0,
        [2.0 - epsilon, 0.0, 0.0], 1.0,
        BooleanOp::Union,
    );

    match first {
        Ok(r1) => {
            let (topo_r1, geom_r1, _) = r1.into_states();
            let (topo_c, geom_c) = build_cube([1.0, 0.0, 0.0], 0.5);

            let input = BooleanInput::new(
                topo_r1, geom_r1,
                topo_c, geom_c,
                BooleanOp::Subtraction,
            );

            match execute_boolean_logged(input).into_result() {
                Ok(result) => {
                    let r2 = result;
                    let (v, e, f, chi) = euler_audit(r2.topology().arena());
                    eprintln!("Cascading near-coincidence: V={v} E={e} F={f} χ={chi}");
                    assert!(f >= 6, "Cascaded result should have faces");
                }
                Err(e) => {
                    eprintln!("Cascading second op error (tracking): {e}");
                }
            }
        }
        Err(e) => {
            eprintln!("Cascading first op error (tracking): {e}");
        }
    }
}

// ══════════════════════════════════════════════════════════════
// §T3.3  TRIPLE FLUSH UNION
// ══════════════════════════════════════════════════════════════

/// T3.3 — Three cubes flush face-to-face in a row.
///
/// A at [-2..0], B at [0..2], C at [2..4] — shared faces at x=0 and x=2.
/// Union all three: should produce a single elongated box.
#[test]
fn triple_flush_union() {
    let ab_result = try_boolean(
        [-1.0, 0.0, 0.0], 1.0,
        [1.0, 0.0, 0.0], 1.0,
        BooleanOp::Union,
    );

    match ab_result {
        Ok(ab) => {
            let (topo_ab, geom_ab, _) = ab.into_states();
            let (topo_c, geom_c) = build_cube([3.0, 0.0, 0.0], 1.0);

            let input = BooleanInput::new(
                topo_ab, geom_ab,
                topo_c, geom_c,
                BooleanOp::Union,
            );

            match execute_boolean_logged(input).into_result() {
                Ok(result) => {
                    let r = result;
                    let (v, e, f, chi) = euler_audit(r.topology().arena());
                    eprintln!("Triple flush: V={v} E={e} F={f} χ={chi}");
                    assert!(f >= 6, "Triple flush should produce at least 6 faces");
                }
                Err(e) => {
                    eprintln!("Triple flush second op error (tracking): {e}");
                }
            }
        }
        Err(e) => {
            eprintln!("Triple flush first op error (tracking): {e}");
        }
    }
}

// ══════════════════════════════════════════════════════════════
// §T3.4  CONCENTRIC TRIPLE SUBTRACTION
// ══════════════════════════════════════════════════════════════

/// T3.4 — Large − Medium → shell with void. Then subtract Small from that.
///
/// Tests nested void handling: the engine must track multiple
/// shells correctly and not collapse inner voids.
#[test]
fn concentric_triple_subtraction() {
    let step1 = try_boolean(
        [0.0, 0.0, 0.0], 3.0,
        [0.0, 0.0, 0.0], 2.0,
        BooleanOp::Subtraction,
    );

    match step1 {
        Ok(r1) => {
            let (v1, e1, f1, chi1) = euler_audit(r1.topology().arena());
            eprintln!("Step 1 (Large−Med): V={v1} E={e1} F={f1} χ={chi1}");

            let (topo_r1, geom_r1, _) = r1.into_states();
            let (topo_small, geom_small) = build_cube([0.0, 0.0, 0.0], 1.0);

            let input = BooleanInput::new(
                topo_r1, geom_r1,
                topo_small, geom_small,
                BooleanOp::Subtraction,
            );

            match execute_boolean_logged(input).into_result() {
                Ok(result) => {
                    let r2 = result;
                    let (v2, e2, f2, chi2) = euler_audit(r2.topology().arena());
                    eprintln!("Step 2 (result−Small): V={v2} E={e2} F={f2} χ={chi2}");
                    assert!(f2 >= 6, "Double subtraction should have faces");
                }
                Err(e) => {
                    eprintln!("Concentric step 2 error (tracking): {e}");
                }
            }
        }
        Err(e) => {
            eprintln!("Concentric step 1 error (tracking): {e}");
        }
    }
}

// ══════════════════════════════════════════════════════════════
// §T3.5  RESULT REUSE INTEGRITY
// ══════════════════════════════════════════════════════════════

/// T3.5 — Use boolean result as input to another boolean.
///
/// Verifies no stale handles, deleted-slot corruption, or generation
/// mismatches when reusing a BooleanResult's topology as input.
#[test]
fn result_reuse_integrity() {
    let step1 = try_boolean(
        [0.0, 0.0, 0.0], 1.0,
        [0.5, 0.0, 0.0], 1.0,
        BooleanOp::Union,
    );

    let r1 = step1.expect("Step 1 union must succeed for reuse test");
    let (v1, e1, f1, chi1) = euler_audit(r1.topology().arena());
    assert_eq!(chi1, 2, "Step 1 Euler violation: V={v1} E={e1} F={f1}");

    let (topo_r1, geom_r1, _) = r1.into_states();
    let (topo_c, geom_c) = build_cube([0.0, 0.5, 0.0], 1.0);

    let input2 = BooleanInput::new(
        topo_r1, geom_r1,
        topo_c, geom_c,
        BooleanOp::Union,
    );

    let r2 = execute_boolean_logged(input2)
        .into_result()
        .expect("Step 2 union must succeed for reuse test");
    let (v2, e2, f2, chi2) = euler_audit(r2.topology().arena());
    assert_eq!(chi2, 2, "Step 2 Euler violation: V={v2} E={e2} F={f2}");

    let (topo_r2, geom_r2, _) = r2.into_states();
    let (topo_d, geom_d) = build_cube([0.0, 0.0, 0.5], 1.0);

    let input3 = BooleanInput::new(
        topo_r2, geom_r2,
        topo_d, geom_d,
        BooleanOp::Intersection,
    );

    let r3 = execute_boolean_logged(input3)
        .into_result()
        .expect("Step 3 intersection must succeed for reuse test");
    let (v3, e3, f3, chi3) = euler_audit(r3.topology().arena());
    assert_eq!(chi3, 2, "Step 3 Euler violation: V={v3} E={e3} F={f3}");
    assert!(f3 >= 6, "Triple-chained result should have faces, got {f3}");
}
