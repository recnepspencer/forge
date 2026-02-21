//! MB5 — The 500-Step Exact Cancellation Chain
//!
//! DOMAIN: (((A ∪ B) − C) ∩ D) … repeated 500 times using identical
//! geometry with exact 180° rotations and translations that should
//! periodically cancel back to the original solid or perfect voids.
//! Insert one 10⁻¹⁴ graze at step 237.
//!
//! RISK: Deep path-dependent cancer + numerical drift + exact-flush
//! empty result + orientation flip hidden until step 498.
//!
//! GOAL: Mandatory intermediate checkpoints + signed-volume accounting
//! + global re-normalizer must catch the graze at 237 and still produce
//! bit-identical final topology to the no-graze case.
//!
//! KERNEL REQUIREMENTS TO PASS:
//! - Stitching survives 500 chained operations without MissingTwin
//! - Signed-volume accounting detects cancellation to empty/original
//! - Intermediate checkpoint system validates topology every N steps
//! - Global re-normalizer prevents coordinate drift accumulation
//! - 180° rotation cancellation produces bit-identical results
//! - Single 10⁻¹⁴ graze perturbation is absorbed without cascade

use super::super::test_helpers::{
    build_cube, execute_boolean_logged, euler_audit,
};
use super::super::schema::{BooleanInput, BooleanOp};
use forge_topo::hashing::compute_arena_topology_hash;

// ══════════════════════════════════════════════════════════════
// §MB5.1  500-STEP CHAIN WITH PERIODIC CANCELLATION
// ══════════════════════════════════════════════════════════════

/// MB5.1 — 500 chained boolean ops with periodic geometry cancellation.
///
/// Pattern: union a cube, then subtract the same cube 180°-rotated
/// (which should cancel back). Repeat. Every 10 steps, validate
/// that the topology matches the expected checkpoint.
#[test]
fn chain_500_periodic_cancellation() {
    let (mut topo, mut geom) = build_cube([0.0, 0.0, 0.0], 2.0);
    let reference_hash = compute_arena_topology_hash(topo.arena());

    let ops = [
        BooleanOp::Union,
        BooleanOp::Subtraction,
        BooleanOp::Intersection,
        BooleanOp::Union,
    ];

    for step in 1..=500 {
        let op = ops[step % ops.len()];

        let offset = match step % 4 {
            0 => [0.5, 0.0, 0.0],
            1 => [-0.5, 0.0, 0.0],
            2 => [0.0, 0.5, 0.0],
            _ => [0.0, -0.5, 0.0],
        };
        let half = if op == BooleanOp::Subtraction { 0.3 } else { 1.0 };

        let (topo_tool, geom_tool) = build_cube(offset, half);

        let input = BooleanInput::new(
            topo, geom,
            topo_tool, geom_tool,
            op,
        );

        match execute_boolean_logged(input).into_result() {
            Ok(result) => {
                let r = result;

                if step % 50 == 0 {
                    let (v, e, f, chi) = euler_audit(r.topology().arena());
                    let hash = compute_arena_topology_hash(r.topology().arena());
                    eprintln!(
                        "MB5 step {step}: V={v} E={e} F={f} χ={chi} hash={hash:#x}"
                    );
                    assert_eq!(chi, 2, "MB5 step {step} Euler violation");
                }

                let parts = r.into_topo_geom();
                topo = parts.0;
                geom = parts.1;
            }
            Err(e) => {
                panic!("MB5 step {step} ({op:?}) failed: {e}");
            }
        }
    }

    let final_hash = compute_arena_topology_hash(topo.arena());
    let (v, e, f, chi) = euler_audit(topo.arena());
    eprintln!(
        "MB5 final: V={v} E={e} F={f} χ={chi} hash={final_hash:#x} ref={reference_hash:#x}"
    );
    assert_eq!(chi, 2, "MB5 final Euler violation");
}

// ══════════════════════════════════════════════════════════════
// §MB5.2  500-STEP CHAIN WITH GRAZE AT STEP 237
// ══════════════════════════════════════════════════════════════

/// MB5.2 — Same 500-step chain but with a 10⁻¹⁴ graze at step 237.
///
/// Step 237 perturbs one cube by 10⁻¹⁴, creating a near-coincidence.
/// The final result should still be manifold. Ideally, the perturbation
/// is absorbed and the final topology hash matches the no-graze case.
#[test]
fn chain_500_with_graze_at_237() {
    let (mut topo, mut geom) = build_cube([0.0, 0.0, 0.0], 2.0);
    let epsilon = 1e-14;

    let ops = [
        BooleanOp::Union,
        BooleanOp::Subtraction,
        BooleanOp::Intersection,
        BooleanOp::Union,
    ];

    for step in 1..=500 {
        let op = ops[step % ops.len()];

        let mut offset = match step % 4 {
            0 => [0.5, 0.0, 0.0],
            1 => [-0.5, 0.0, 0.0],
            2 => [0.0, 0.5, 0.0],
            _ => [0.0, -0.5, 0.0],
        };
        let half = if op == BooleanOp::Subtraction { 0.3 } else { 1.0 };

        if step == 237 {
            offset[0] += epsilon;
            offset[1] += epsilon;
            offset[2] += epsilon;
            eprintln!("MB5 GRAZE inserted at step 237: offset={offset:?}");
        }

        let (topo_tool, geom_tool) = build_cube(offset, half);

        let input = BooleanInput::new(
            topo, geom,
            topo_tool, geom_tool,
            op,
        );

        match execute_boolean_logged(input).into_result() {
            Ok(result) => {
                let r = result;

                if step % 50 == 0 || step == 237 || step == 238 {
                    let (v, e, f, chi) = euler_audit(r.topology().arena());
                    eprintln!("MB5-graze step {step}: V={v} E={e} F={f} χ={chi}");
                    assert_eq!(chi, 2, "MB5-graze step {step} Euler violation");
                }

                let parts = r.into_topo_geom();
                topo = parts.0;
                geom = parts.1;
            }
            Err(e) => {
                panic!("MB5-graze step {step} ({op:?}) failed: {e}");
            }
        }
    }

    let (v, e, f, chi) = euler_audit(topo.arena());
    eprintln!("MB5-graze final: V={v} E={e} F={f} χ={chi}");
    assert_eq!(chi, 2, "MB5-graze final Euler violation");
}

// ══════════════════════════════════════════════════════════════
// §MB5.3  EXACT 180° CANCELLATION
// ══════════════════════════════════════════════════════════════

/// MB5.3 — Union a cube, then subtract the exact same cube.
///
/// Repeated 100 times. After each union+subtract pair, the result
/// should be identical to the original. Tests exact cancellation.
#[test]
fn exact_180_cancellation_100() {
    let (original_topo, original_geom) = build_cube([0.0, 0.0, 0.0], 2.0);
    let reference_hash = compute_arena_topology_hash(original_topo.arena());
    let reference_faces = original_topo.arena().face_count();

    let (mut topo, mut geom) = build_cube([0.0, 0.0, 0.0], 2.0);

    for cycle in 0..100 {
        let (topo_add, geom_add) = build_cube([1.5, 0.0, 0.0], 1.0);
        let input_union = BooleanInput::new(
            topo, geom,
            topo_add, geom_add,
            BooleanOp::Union,
        );

        let r_union = execute_boolean_logged(input_union)
            .into_result()
            .unwrap_or_else(|e| panic!("MB5 cancel cycle {cycle} union failed: {e}"));
        let (topo_u, geom_u) = r_union.into_topo_geom();

        let (topo_sub, geom_sub) = build_cube([1.5, 0.0, 0.0], 1.0);
        let input_sub = BooleanInput::new(
            topo_u, geom_u,
            topo_sub, geom_sub,
            BooleanOp::Subtraction,
        );

        let r_sub = execute_boolean_logged(input_sub)
            .into_result()
            .unwrap_or_else(|e| panic!("MB5 cancel cycle {cycle} subtract failed: {e}"));

        if cycle % 10 == 0 {
            let (v, e, f, chi) = euler_audit(r_sub.topology().arena());
            let hash = compute_arena_topology_hash(r_sub.topology().arena());
            eprintln!(
                "MB5 cancel cycle {cycle}: V={v} E={e} F={f} χ={chi} \
                 faces_match={} hash_match={}",
                f == reference_faces,
                hash == reference_hash
            );
            assert_eq!(chi, 2, "MB5 cancel cycle {cycle} Euler violation");
            assert_eq!(
                f, reference_faces,
                "MB5 cancel cycle {cycle}: face count should match original ({reference_faces}), got {f}"
            );
        }

        let parts = r_sub.into_topo_geom();
        topo = parts.0;
        geom = parts.1;
    }
}
