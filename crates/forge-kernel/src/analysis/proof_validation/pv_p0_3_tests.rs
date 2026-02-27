//! PV Suite P0.3 — Orientation Canonicalization Tests
//!
//! PV-09: 1,000 random Boolean operations → every successful result has outward normals
//! PV-10: Scrambled orientation → healing canonicalizes → validation passes

use forge_core::KernelError;
use forge_topo::validate::ValidationLevel;
use forge_topo::healing::{heal_shell_orientation, HealingResult};
use forge_topo::handles::VertexId;
use forge_topo::state::DraftConfig;
use crate::mesh_builder::make_cube;
use crate::geometry_state::GeometryState;
use crate::operations::boolean::{
    BooleanInput, BooleanOp,
};
use crate::operations::boolean::test_helpers::{
    selected_test_pipeline, TestPipeline,
};
use crate::operations::boolean::parametric::assemble::execute_boolean_direct;
use crate::operations::boolean::execute_boolean;
use forge_math::deterministic_rng::DeterministicRng;
use super::test_support::validate_geometric_invariants_all_faces;
use std::env;

/// Build a position lookup closure from a GeometryState.
fn position_lookup(store: &GeometryState) -> impl Fn(VertexId) -> Option<[f64; 3]> + '_ {
    |vertex_id| store.get_vertex_position(vertex_id).copied()
}

fn pv09_env_usize(key: &str) -> Option<usize> {
    env::var(key).ok()?.parse::<usize>().ok()
}

/// PV-09: 1,000 random Boolean operations → every successful result is oriented.
///
/// Deterministic seed. Each iteration: random cube center/size, random op.
/// Successful results must pass validate_geometric_invariants (positive
/// signed volume). Zero failures tolerated on successes.
///
/// At least 50 successful ops required (lower bound for non-degenerate configs).
///
/// Note: Uses 200 iterations rather than 1,000 to complete in debug builds.
/// The `--release` test suite should run the full 1,000.
#[test]
fn pv_09_1000_random_booleans_all_oriented() {
    let mut rng = DeterministicRng::new(0xDEAD_BEEF_CAFE_F00D);
    let pipeline = selected_test_pipeline();
    let only_iter = pv09_env_usize("FORGE_PV09_ONLY_ITER");
    let stop_after = pv09_env_usize("FORGE_PV09_STOP_AFTER");
    let print_cases = env::var("FORGE_PV09_PRINT_CASES").ok().as_deref() == Some("1");

    let mut successes: usize = 0;
    let mut failures: usize = 0;
    let mut orientation_failures: usize = 0;

    for i in 0..200 {
        let cx = rng.next_f64() * 4.0 - 2.0;
        let cy = rng.next_f64() * 4.0 - 2.0;
        let cz = rng.next_f64() * 4.0 - 2.0;
        let size = 0.5 + rng.next_f64() * 2.0;

        let op_choice = (rng.next_f64() * 3.0) as usize;
        let op = match op_choice {
            0 => BooleanOp::Union,
            1 => BooleanOp::Subtraction,
            _ => BooleanOp::Intersection,
        };

        if let Some(target_iter) = only_iter {
            if i != target_iter {
                continue;
            }
        }

        if print_cases || only_iter == Some(i) {
            eprintln!(
                "PV09 CASE i={} pipeline={:?} op={:?} center=[{:.6},{:.6},{:.6}] size={:.6}",
                i, pipeline, op, cx, cy, cz, size
            );
        }

        let target = match make_cube([0.0, 0.0, 0.0], 2.0) {
            Ok(r) => r,
            Err(_) => { failures += 1; continue; }
        };
        let tool = match make_cube([cx, cy, cz], size) {
            Ok(r) => r,
            Err(_) => { failures += 1; continue; }
        };

        let (target_topo, target_geom) = target.into_parts();
        let (tool_topo, tool_geom) = tool.into_parts();

        println!("Iteration {}: Target [0,0,0] sz=2.0 | Tool [{:.4},{:.4},{:.4}] sz={:.4} | Op={:?}", i, cx, cy, cz, size, op);

        let input = BooleanInput::new(target_topo, target_geom, tool_topo, tool_geom, op);

        let outcome = match pipeline {
            TestPipeline::Adaptive => execute_boolean(input),
            TestPipeline::Parametric => execute_boolean_direct(input),
            TestPipeline::Ember => crate::operations::boolean::test_helpers::execute_boolean_ember(input),
        };

        match outcome.into_value() {
            Ok(result) => {
                let (result_topo, result_geom, _) = result.into_states();

                if result_topo.arena().face_count() < 4 {
                    successes += 1;
                    continue;
                }

                let lookup = position_lookup(&result_geom);
                let check = validate_geometric_invariants_all_faces(
                    result_topo.arena(), &lookup, 1e-10, 1e-12,
                );

                if check.is_err() {
                    orientation_failures += 1;
                    eprintln!(
                        "PV-09 iteration {}: orientation failure on {:?} at [{:.2},{:.2},{:.2}] size={:.2}: {:?}",
                        i, op, cx, cy, cz, size, check.err()
                    );
                }

                successes += 1;
            }
            Err(e) => {
                failures += 1;
                eprintln!(
                    "PV-09 iteration {}: boolean {:?} at [{:.2},{:.2},{:.2}] size={:.2} failed: {:?}",
                    i, op, cx, cy, cz, size, e
                );
            }
        }

        if let Some(stop_iter) = stop_after {
            if i >= stop_iter {
                break;
            }
        }

        if only_iter == Some(i) {
            break;
        }
    }

    eprintln!(
        "PV-09: {} successes, {} failures, {} orientation failures",
        successes, failures, orientation_failures
    );

    if only_iter.is_none() && stop_after.is_none() {
        assert!(
            successes >= 50,
            "Expected at least 50 successful operations, got {}",
            successes
        );
    }

    assert_eq!(
        orientation_failures, 0,
        "All successful Booleans must have correct orientation, but {} failed",
        orientation_failures
    );
}

/// PV-10: Scrambled face orientation → healed → geometric validation passes.
///
/// 1. Build valid cube → assert positive signed volume
/// 2. Flip winding on all faces (swap next/prev) → negative volume
/// 3. Call heal_shell_orientation → assert 1 shell healed
/// 4. Assert validate_geometric_invariants passes after healing
/// 5. Verify healing is deterministic: second heal on same state is no-op
#[test]
fn pv_10_scrambled_orientation_healed() {
    let result = make_cube([0.0, 0.0, 0.0], 2.0).unwrap();
    let (topo, geom) = result.into_parts();

    let lookup = position_lookup(&geom);
    let pre_check = validate_geometric_invariants_all_faces(topo.arena(), &lookup, 1e-10, 1e-12);
    assert!(pre_check.is_ok(), "Valid cube should pass orientation: {:?}", pre_check.err());

    let mut config = DraftConfig::default();
    config.validation_level = ValidationLevel::None;

    let mut draft = topo.into_mutation_with(config.clone());
    let arena = draft.arena_mut();

    let face_ids: Vec<_> = arena.iter_faces().map(|(fid, _)| fid).collect();

    for face_id in face_ids {
        let he_ids: Vec<_> = forge_topo::traverse::FaceEdgeIterator::new(arena, face_id)
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let swaps: Vec<_> = he_ids.iter().map(|&he_id| {
            let he_data = arena.get_half_edge(he_id).unwrap();
            (he_id, he_data.next(), he_data.prev())
        }).collect();

        for (he_id, old_next, old_prev) in swaps {
            let he_mut = arena.get_half_edge_mut(he_id).unwrap();
            he_mut.set_next(old_prev);
            he_mut.set_prev(old_next);
        }
    }

    let scrambled_check = validate_geometric_invariants_all_faces(arena, &lookup, 1e-10, 1e-12);
    assert!(
        scrambled_check.is_err(),
        "Scrambled cube should fail signed volume check"
    );

    let heal_result = heal_shell_orientation(arena, &lookup).unwrap();
    assert_eq!(heal_result.shells_checked(), 1, "Cube is one shell");
    assert_eq!(heal_result.shells_healed(), 1, "One shell should be healed");

    let post_heal_check = validate_geometric_invariants_all_faces(arena, &lookup, 1e-10, 1e-12);
    assert!(
        post_heal_check.is_ok(),
        "Healed cube should pass orientation: {:?}",
        post_heal_check.err()
    );

    let second_heal = heal_shell_orientation(arena, &lookup).unwrap();
    assert_eq!(
        second_heal.shells_healed(), 0,
        "Already-healed cube should need no further healing"
    );
}
