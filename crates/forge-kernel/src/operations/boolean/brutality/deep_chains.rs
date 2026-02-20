//! Deep Path-Dependent Chain Tests
//!
//! DOMAIN: Cascaded boolean operations where small errors in early steps
//! can compound into catastrophic failures at later steps.
//!
//! INVARIANTS:
//! - Euler χ = 2 at every intermediate step
//! - No panics or topology corruption after N chained operations
//! - If a step fails, the test reports exactly which step
//!
//! ═══════════════════════════════════════════════════════════════
//! REQUIRED CODE/MATH CHANGES TO PASS ALL TESTS:
//! ═══════════════════════════════════════════════════════════════
//!
//! 1. **Vertex Provenance Across Chains**: When a `BooleanResult` is used
//!    as input for the next operation, vertex `VertexMatchKey` provenance
//!    must be preserved so `assign_original_vertex_provenance` in the split
//!    phase can rebuild 3-plane keys. Currently, provenance is lost between
//!    operations because the copy phase doesn't carry it forward.
//!    THIS IS THE ROOT CAUSE of all chain failures.
//!
//! 2. **Spatial Vertex Welding Tolerance**: The `SpatialVertexIndex` in
//!    `copy.rs` uses `1e-18` squared tolerance for nearest-neighbor vertex
//!    matching. This is too tight for vertices that went through floating-point
//!    arithmetic in prior boolean ops. Needs to be relaxed to ~`1e-12` squared.
//!
//! 3. **Position-Based Stitch Fallback**: When `stitch_twins` can't find a
//!    matching reverse halfedge by vertex index, try matching by geometric
//!    position of the edge endpoints. This handles the case where the copy
//!    phase created duplicate vertices at the same position.
//!
//! 4. **Coordinate Drift Prevention**: After N chained operations, vertex
//!    positions accumulate floating-point roundoff. A global re-normalizer
//!    could snap vertices to their symbolic 3-plane intersection positions
//!    after each operation, preventing drift.
//!
//! 5. **Stitching Resilience**: The current stitch implementation should
//!    handle non-manifold junctions (>2 halfedges sharing a directed edge)
//!    via radial sorting. This is partially implemented in `select_best_twin`
//!    but needs testing under chained-op conditions.

use super::super::test_helpers::{
    build_cube, execute_boolean_logged, euler_audit,
};
use super::super::schema::{BooleanInput, BooleanOp};

// ══════════════════════════════════════════════════════════════
// §DC.1  UNION CHAIN (10 STEPS)
// ══════════════════════════════════════════════════════════════

/// DC.1 — ((A ∪ B) ∪ C) ∪ ... for 10 cubes at different offsets.
///
/// After each union, the Euler characteristic must be 2.
/// Tests whether boolean results remain usable as inputs.
#[test]
fn chain_union_10_steps() {
    let (mut topo, mut geom) = build_cube([0.0, 0.0, 0.0], 1.0);

    for step in 1..=10 {
        let offset = step as f64 * 0.8;
        let (topo_tool, geom_tool) = build_cube([offset, 0.0, 0.0], 1.0);

        let input = BooleanInput::new(
            topo, geom,
            topo_tool, geom_tool,
            BooleanOp::Union,
        );

        let result = execute_boolean_logged(input)
            .unwrap_or_else(|e| panic!("Union chain step {step} failed: {e:?}"));
        let r = result.into_value();

        let (v, e, f, chi) = euler_audit(r.topology().arena());
        assert_eq!(
            chi, 2,
            "Union chain step {step} Euler violation: V={v} E={e} F={f} χ={chi}"
        );

        let parts = r.into_topo_geom();
        topo = parts.0;
        geom = parts.1;
    }

    let final_f = topo.arena().face_count();
    assert!(
        final_f >= 6,
        "10-step union chain should produce at least 6 faces, got {final_f}"
    );
}

// ══════════════════════════════════════════════════════════════
// §DC.2  SUBTRACTION CHAIN (10 STEPS)
// ══════════════════════════════════════════════════════════════

/// DC.2 — Large cube minus 10 small cubes at different positions.
///
/// Each subtraction carves a notch. After each step, Euler must be 2.
#[test]
fn chain_subtract_10_steps() {
    let (mut topo, mut geom) = build_cube([0.0, 0.0, 0.0], 5.0);

    for step in 0..10 {
        let x = -4.0 + step as f64 * 0.9;
        let (topo_tool, geom_tool) = build_cube([x, 0.0, 4.5], 0.5);

        let input = BooleanInput::new(
            topo, geom,
            topo_tool, geom_tool,
            BooleanOp::Subtraction,
        );

        let result = execute_boolean_logged(input)
            .unwrap_or_else(|e| panic!("Subtract chain step {step} failed: {e:?}"));
        let r = result.into_value();

        let (v, e, f, chi) = euler_audit(r.topology().arena());
        assert_eq!(
            chi, 2,
            "Subtract chain step {step} Euler violation: V={v} E={e} F={f} χ={chi}"
        );

        let parts = r.into_topo_geom();
        topo = parts.0;
        geom = parts.1;
    }

    let final_f = topo.arena().face_count();
    assert!(
        final_f >= 6,
        "10-step subtract chain should produce at least 6 faces, got {final_f}"
    );
}

// ══════════════════════════════════════════════════════════════
// §DC.3  MIXED OPS CHAIN (10 STEPS)
// ══════════════════════════════════════════════════════════════

/// DC.3 — Alternating union/subtract for 10 steps.
///
/// Odd steps: union a cube. Even steps: subtract a cube at a different offset.
#[test]
fn chain_mixed_ops_10() {
    let (mut topo, mut geom) = build_cube([0.0, 0.0, 0.0], 2.0);

    for step in 1..=10 {
        let op = if step % 2 == 1 {
            BooleanOp::Union
        } else {
            BooleanOp::Subtraction
        };

        let offset = step as f64 * 0.4;
        let half = if step % 2 == 1 { 1.0 } else { 0.3 };
        let (topo_tool, geom_tool) = build_cube([offset, 0.0, 0.0], half);

        let input = BooleanInput::new(
            topo, geom,
            topo_tool, geom_tool,
            op,
        );

        match execute_boolean_logged(input) {
            Ok(envelope) => {
                let r = envelope.into_value();
                let (v, e, f, chi) = euler_audit(r.topology().arena());
                eprintln!("Mixed chain step {step} ({op:?}): V={v} E={e} F={f} χ={chi}");
                assert_eq!(
                    chi, 2,
                    "Mixed chain step {step} Euler violation: V={v} E={e} F={f} χ={chi}"
                );
                let parts = r.into_topo_geom();
                topo = parts.0;
                geom = parts.1;
            }
            Err(e) => {
                panic!("Mixed chain step {step} ({op:?}) failed: {e:?}");
            }
        }
    }
}

// ══════════════════════════════════════════════════════════════
// §DC.4  CHAIN WITH STEP IDENTIFICATION
// ══════════════════════════════════════════════════════════════

/// DC.4 — Same chain pattern but with explicit step labels for diagnostics.
///
/// Validates that if a step fails, the test output clearly identifies
/// which step caused the problem and dumps the topology state.
#[test]
fn chain_identifies_failing_step() {
    let (mut topo, mut geom) = build_cube([0.0, 0.0, 0.0], 3.0);

    let operations = vec![
        ([1.0, 0.0, 0.0], 1.0, BooleanOp::Union),
        ([0.0, 1.0, 0.0], 1.0, BooleanOp::Union),
        ([0.0, 0.0, 1.0], 1.0, BooleanOp::Union),
        ([0.5, 0.5, 0.5], 0.5, BooleanOp::Subtraction),
        ([-0.5, -0.5, 0.0], 0.8, BooleanOp::Union),
        ([1.5, 0.0, 0.0], 0.5, BooleanOp::Subtraction),
        ([0.0, 1.5, 0.0], 0.5, BooleanOp::Subtraction),
        ([0.0, 0.0, 1.5], 0.5, BooleanOp::Subtraction),
    ];

    for (step, (center, half, op)) in operations.iter().enumerate() {
        let pre_state = euler_audit(topo.arena());
        let (topo_tool, geom_tool) = build_cube(*center, *half);

        let input = BooleanInput::new(
            topo, geom,
            topo_tool, geom_tool,
            *op,
        );

        match execute_boolean_logged(input) {
            Ok(envelope) => {
                let r = envelope.into_value();
                let (v, e, f, chi) = euler_audit(r.topology().arena());
                eprintln!(
                    "Step {step} ({op:?} @ {center:?} h={half}): V={v} E={e} F={f} χ={chi}"
                );
                assert_eq!(
                    chi, 2,
                    "Step {step} ({op:?}) Euler violation: V={v} E={e} F={f}"
                );
                let parts = r.into_topo_geom();
                topo = parts.0;
                geom = parts.1;
            }
            Err(e) => {
                let (v, e_count, f, chi) = pre_state;
                eprintln!(
                    "FAILURE at step {step} ({op:?}): {e:?}\n\
                     State BEFORE failure: V={v} E={e_count} F={f} χ={chi}"
                );
                panic!("Chain failed at step {step}");
            }
        }
    }

}

// ══════════════════════════════════════════════════════════════
// §DC.5  MINIMAL REPRODUCTIONS
// ══════════════════════════════════════════════════════════════

/// Minimal repro: two overlapping subtracted notches.
///
/// This is the exact geometry from chain_subtract_10_steps step 0+1.
/// Step 0: subtract cube at (-4.0, 0, 4.5) half=0.5 → notch at x∈[-4.5,-3.5]
/// Step 1: subtract cube at (-3.1, 0, 4.5) half=0.5 → notch at x∈[-3.6,-2.6]
/// The two notches overlap in x∈[-3.6,-3.5].
#[test]
fn minimal_overlapping_notches() {
    let (topo, geom) = build_cube([0.0, 0.0, 0.0], 5.0);

    // Step 0: first notch
    let (tool0, tool0_g) = build_cube([-4.0, 0.0, 4.5], 0.5);
    let input0 = BooleanInput::new(topo, geom, tool0, tool0_g, BooleanOp::Subtraction);
    let r0 = execute_boolean_logged(input0).expect("Step 0 failed");
    let r0 = r0.into_value();
    let (v, e, f, chi) = euler_audit(r0.topology().arena());
    assert_eq!(chi, 2, "Step 0 Euler: V={v} E={e} F={f} χ={chi}");
    let (topo, geom) = r0.into_topo_geom();

    // Step 1: overlapping notch
    let (tool1, tool1_g) = build_cube([-3.1, 0.0, 4.5], 0.5);
    let input1 = BooleanInput::new(topo, geom, tool1, tool1_g, BooleanOp::Subtraction);
    let r1 = execute_boolean_logged(input1).expect("Step 1 failed (overlapping notch)");
    let r1 = r1.into_value();
    let (v, e, f, chi) = euler_audit(r1.topology().arena());
    assert_eq!(chi, 2, "Step 1 Euler: V={v} E={e} F={f} χ={chi}");
}

/// Control: two NON-overlapping subtracted notches.
///
/// Same as above but notches are spaced far apart (no overlap).
/// If this passes but overlapping fails, the bug is in how overlapping
/// geometry is handled (split/classify interaction with prior notch walls).
#[test]
fn minimal_nonoverlapping_notches() {
    let (topo, geom) = build_cube([0.0, 0.0, 0.0], 5.0);

    // Step 0: first notch at x=-4
    let (tool0, tool0_g) = build_cube([-4.0, 0.0, 4.5], 0.5);
    let input0 = BooleanInput::new(topo, geom, tool0, tool0_g, BooleanOp::Subtraction);
    let r0 = execute_boolean_logged(input0).expect("Step 0 failed");
    let r0 = r0.into_value();
    let (v, e, f, chi) = euler_audit(r0.topology().arena());
    assert_eq!(chi, 2, "Step 0 Euler: V={v} E={e} F={f} χ={chi}");
    
    // DIAGNOSTIC: print all edges that have at least one endpoint at z=5
    let arena = r0.topology().arena();
    let geom_ref = r0.geometry();
    eprintln!("=== STEP 0 RESULT: edges touching z=5 ===");
    for (he_id, _he) in arena.iter_half_edges() {
        let he_data = arena.get_half_edge(he_id).unwrap();
        let origin = he_data.origin();
        let next_data = arena.get_half_edge(he_data.next()).unwrap();
        let dest = next_data.origin();
        let p_o = geom_ref.get_vertex_position(origin).unwrap();
        let p_d = geom_ref.get_vertex_position(dest).unwrap();
        if (p_o[2] - 5.0).abs() < 1e-9 || (p_d[2] - 5.0).abs() < 1e-9 {
            let face = he_data.face();
            let twin = he_data.twin();
            let twin_face = arena.get_half_edge(twin).map(|t| t.face()).unwrap_or(face);
            eprintln!("  HE#{}: {origin}->{dest} [{:.3},{:.3},{:.3}]->[{:.3},{:.3},{:.3}] face={face} twin_face={twin_face}",
                he_id.index(), p_o[0], p_o[1], p_o[2], p_d[0], p_d[1], p_d[2]);
        }
    }
    eprintln!("=== END STEP 0 z=5 edges ===");
    
    let (topo, geom) = r0.into_topo_geom();

    // Step 1: NON-overlapping notch at x=+4 (far away)
    let (tool1, tool1_g) = build_cube([4.0, 0.0, 4.5], 0.5);
    let input1 = BooleanInput::new(topo, geom, tool1, tool1_g, BooleanOp::Subtraction);
    let r1 = execute_boolean_logged(input1).expect("Step 1 failed (non-overlapping)");
    let r1 = r1.into_value();
    let (v, e, f, chi) = euler_audit(r1.topology().arena());
    assert_eq!(chi, 2, "Step 1 Euler: V={v} E={e} F={f} χ={chi}");
}

/// Simplest case: single subtraction with flush z=5 boundary.
///
/// If this fails, the coplanar boundary problem exists even without chains.
#[test]
fn minimal_single_flush_subtraction() {
    let (topo, geom) = build_cube([0.0, 0.0, 0.0], 5.0);
    let (tool, tool_g) = build_cube([0.0, 0.0, 4.5], 0.5);
    let input = BooleanInput::new(topo, geom, tool, tool_g, BooleanOp::Subtraction);
    let r = execute_boolean_logged(input).expect("Single flush subtraction failed");
    let r = r.into_value();
    let (v, e, f, chi) = euler_audit(r.topology().arena());
    assert_eq!(chi, 2, "Euler: V={v} E={e} F={f} χ={chi}");
}

/// Two non-flush subtractions (tool fully inside, no touching boundary).
///
/// If this passes, the bug is specifically about flush coplanar boundaries.
#[test]
fn minimal_two_interior_subtractions() {
    let (topo, geom) = build_cube([0.0, 0.0, 0.0], 5.0);

    // Step 0: interior subtraction
    let (tool0, tool0_g) = build_cube([-3.0, 0.0, 0.0], 0.5);
    let input0 = BooleanInput::new(topo, geom, tool0, tool0_g, BooleanOp::Subtraction);
    let r0 = execute_boolean_logged(input0).expect("Step 0 failed");
    let r0 = r0.into_value();
    // Interior subtraction creates a cavity: V-E+F = 4 (two shells)
    let (v, e, f, chi) = euler_audit(r0.topology().arena());
    eprintln!("Step 0: V={v} E={e} F={f} χ={chi}");
    let (topo, geom) = r0.into_topo_geom();

    // Step 1: another interior subtraction, far away
    let (tool1, tool1_g) = build_cube([3.0, 0.0, 0.0], 0.5);
    let input1 = BooleanInput::new(topo, geom, tool1, tool1_g, BooleanOp::Subtraction);
    let r1 = execute_boolean_logged(input1).expect("Step 1 failed");
    let r1 = r1.into_value();
    let (v, e, f, chi) = euler_audit(r1.topology().arena());
    eprintln!("Step 1: V={v} E={e} F={f} χ={chi}");
}
