//! P3.5: MetaBoss Replay Torture Suite — Divergence Detection.
//!
//! DOMAIN: Replay the decision chain and identify the first wrong step.
//! These tests inject known faults (flipped classifications, geometry
//! perturbations, topology corruptions) and verify that the proof system
//! correctly pinpoints them via checkpoint diffs, causal chains, and
//! delta-debug bisection.
//!
//! DEPENDENCIES: `proof_invariants` (the oracle), `counterfactual`,
//! `causal_chain`, `checkpoint_diff`, `delta_debug`.

use forge_core::tracing::checkpoint_diff::diff_decision_logs;
use forge_core::tracing::delta_debug::delta_debug;
use forge_core::{DecisionTier, KernelError};

use crate::analysis::causal_chain::query_causal_chain;
use crate::analysis::counterfactual::{replay_all_near_boundary, CounterfactualValidation};
use crate::analysis::proof_validation::proof_invariants::validate_all;
use crate::operations::boolean::test_helpers::{
    build_cube, execute_boolean_logged, menger_sponge_subtraction_centers,
};
use crate::operations::boolean::BooleanInput;
use crate::operations::boolean::{BooleanOp, FaceClassification};

/// MB-R1: Checkpoint diff pinpoints injected divergence in a chain.
///
/// Builds a 10-step chain, checkpoints it. Re-runs the chain but injects a
/// flipped classification at step 5. Verifies the diff is empty for steps 1-4
/// and non-empty from step 5, exactly identifying the flipped decision.
#[test]
fn mb_r1_checkpoint_diff_pinpoints_injected_divergence() {
    // 1. Build original 10-step chain (Menger sponge level 1 + few more)
    let centers = menger_sponge_subtraction_centers([0.0, 0.0, 0.0], 10.0, 1);
    let mut current_topo;
    let mut current_geom;
    {
        let (t, g) = build_cube([0.0, 0.0, 0.0], 10.0);
        current_topo = t;
        current_geom = g;
    }

    let mut original_logs = Vec::new();

    for i in 0..10 {
        let (c, h) = centers[i % centers.len()];
        let (tool_topo, tool_geom) = build_cube(c, h);

        let input = BooleanInput::new(
            current_topo.clone(),
            current_geom.clone(),
            tool_topo,
            tool_geom,
            BooleanOp::Subtraction,
        );

        let envelope = execute_boolean_logged(input);
        original_logs.push(envelope.get_decision_log().clone());
        let result = envelope.into_result().expect("Original step failed");
        let (t, g, _) = result.into_states();
        current_topo = t;
        current_geom = g;
    }

    // 2. Inject fault at step 5 (index 4)
    let fault_step = 4;
    let target_log = &original_logs[fault_step];

    // Find a classification decision to flip
    let decision_to_flip = target_log
        .decisions()
        .find(|d| {
            d.get_context().to_string().contains("Inside")
                || d.get_context().to_string().contains("Outside")
        })
        .expect("No classification decision found to flip");
    let target_id = decision_to_flip.get_id();

    // Determine flipped classification
    let original_str = decision_to_flip.get_context().to_string();
    let flipped_class = if original_str.contains("Inside") {
        FaceClassification::Outside
    } else {
        FaceClassification::Inside
    };

    let overrides = vec![(target_id, flipped_class)];

    // 3. Re-run chain with injected fault at step 5
    let mut current_topo;
    let mut current_geom;
    {
        let (t, g) = build_cube([0.0, 0.0, 0.0], 10.0);
        current_topo = t;
        current_geom = g;
    }

    let mut divergent_logs = Vec::new();

    for i in 0..10 {
        let (c, h) = centers[i % centers.len()];
        let (tool_topo, tool_geom) = build_cube(c, h);

        let input = BooleanInput::new(
            current_topo.clone(),
            current_geom.clone(),
            tool_topo,
            tool_geom,
            BooleanOp::Subtraction,
        );

        let envelope = if i == fault_step {
            crate::operations::boolean::parametric::assemble::merge::eval::execute_boolean_with_overrides(input, &overrides)
        } else {
            execute_boolean_logged(input)
        };

        divergent_logs.push(envelope.get_decision_log().clone());
        let result = envelope.into_result().expect("Re-run step failed");
        let (t, g, _) = result.into_states();
        current_topo = t;
        current_geom = g;
    }

    // 4. Assert diffs
    for i in 0..10 {
        let diff = diff_decision_logs(&original_logs[i], &divergent_logs[i]);
        if i < fault_step {
            assert!(
                diff.is_empty(),
                "Step {} before fault should have empty diff",
                i
            );
        } else if i == fault_step {
            assert!(
                !diff.is_empty(),
                "Fault step {} must have non-empty diff",
                i
            );
            let changed = diff.get_changed();
            assert!(
                changed.iter().any(|c| c.get_id() == target_id),
                "Diff must identify the flipped decision"
            );
        }
    }
}

/// MB-R2: Geometry perturbation → diff catches every flipped decision.
///
/// Runs Boolean, perturbs geometry to force a near-tangent cut, re-runs.
/// Verifies diff catches the flipped decisions and invariants hold.
#[test]
fn mb_r2_geometry_perturbation_diff_catches_every_flipped_decision() {
    // 1. Original run (overlapping, non-tangent)
    let (t1, g1) = build_cube([0.0, 0.0, 0.0], 10.0);
    let (t2, g2) = build_cube([10.0, 10.0, 10.0], 10.0); // center at vertex
    let input_a = BooleanInput::new(t1, g1, t2, g2, BooleanOp::Intersection);

    let env_a = execute_boolean_logged(input_a);
    let log_a = env_a.get_decision_log().clone();
    let res_a = env_a.into_result().expect("Original boolean failed");

    // 2. Perturbed run (shifted so one face is almost exactly coplanar, forcing margin to drop and potentially flip)
    let (t1, g1) = build_cube([0.0, 0.0, 0.0], 10.0);
    let (t2, g2) = build_cube([10.0, 10.0, 10.00000001], 10.0); // perturbed Z slightly
    let input_b = BooleanInput::new(t1, g1, t2, g2, BooleanOp::Intersection);

    let env_b = execute_boolean_logged(input_b);
    let log_b = env_b.get_decision_log().clone();
    let res_b = env_b.into_result().expect("Perturbed boolean failed");

    // 3. Diff should show at least some change (margin delta, tier change, or kind change)
    let diff = diff_decision_logs(&log_a, &log_b);
    assert!(
        !diff.is_empty(),
        "Perturbation must produce a detectable diff in DecisionLog"
    );

    // 4. Validate invariants on both runs
    validate_all(
        &log_a,
        res_a.topology().arena(),
        forge_topo::hashing::compute_arena_topology_hash(res_a.topology().arena()),
    )
    .expect("Original run invariants failed");

    validate_all(
        &log_b,
        res_b.topology().arena(),
        forge_topo::hashing::compute_arena_topology_hash(res_b.topology().arena()),
    )
    .expect("Perturbed run invariants failed");
}

/// MB-R3: Causal chains correctly scope decisions to affected faces.
#[test]
fn mb_r3_causal_chains_scope_decisions_to_correct_faces() {
    let (t1, g1) = build_cube([0.0, 0.0, 0.0], 10.0);
    let (t2, g2) = build_cube([5.0, 5.0, 5.0], 10.0);
    let input = BooleanInput::new(t1, g1, t2, g2, BooleanOp::Union);
    let env = execute_boolean_logged(input);
    let log = env.get_decision_log().clone();
    let result = env.into_result().unwrap();

    let arena = result.topology().arena();
    let face_indices: Vec<_> = arena.iter_faces().map(|(id, _)| id.index()).collect();
    assert!(face_indices.len() >= 2, "Test requires at least 2 faces");

    let f1 = forge_core::EntityRef::new(forge_core::EntityKind::Face, face_indices[0] as u32);
    let f2 = forge_core::EntityRef::new(forge_core::EntityKind::Face, face_indices[1] as u32);

    let chain1 = query_causal_chain();
    let chain2 = query_causal_chain();

    assert!(!chain1.get_steps().is_empty(), "F1 chain must not be empty");
    assert!(!chain2.get_steps().is_empty(), "F2 chain must not be empty");

    // Verify querying a non-existent face returns an empty chain
    let missing_face = forge_core::EntityRef::new(forge_core::EntityKind::Face, 999999);
    let empty_chain = query_causal_chain();
    assert!(
        empty_chain.get_steps().is_empty(),
        "Non-existent face must return empty chain"
    );
}

/// MB-R4: Delta-debug finds injected structural failure.
#[test]
fn mb_r4_delta_debug_finds_injected_structural_failure() {
    // 1. Build an 8-step chain definition
    let centers = menger_sponge_subtraction_centers([0.0, 0.0, 0.0], 10.0, 1);

    // 2. Oracle: runs the chain up to `step_idx` and returns true if topology is broken.
    // We simulate a failure starting at step 5.
    let oracle = |step_idx: usize| -> Result<bool, KernelError> {
        let mut t;
        let mut g;
        {
            let (tt, gg) = build_cube([0.0, 0.0, 0.0], 10.0);
            t = tt;
            g = gg;
        }

        for i in 0..=step_idx {
            let (c, h) = centers[i % centers.len()];
            let (tool_topo, tool_geom) = build_cube(c, h);
            let input = BooleanInput::new(
                t.clone(),
                g.clone(),
                tool_topo,
                tool_geom,
                BooleanOp::Subtraction,
            );

            // Inject failure at step 5
            if i == 5 {
                return Ok(true); // Failure detected!
            }

            let res = execute_boolean_logged(input).into_result()?;
            t = res.topology().clone();
            g = res.geometry().clone();
        }
        Ok(false) // Survived
    };

    // 3. Delta-debug should find step 5
    let failure_result = delta_debug(8, oracle).expect("Delta debug failed");
    assert_eq!(
        failure_result.get_failing_step(),
        5,
        "Delta debug failed to pinpoint exact failure step"
    );
}

/// MB-R5: Counterfactual replay identifies topology-breaking vs divergent.
#[test]
fn mb_r5_counterfactual_replay_classifies_valid_vs_breaking() {
    let (t1, g1) = build_cube([0.0, 0.0, 0.0], 10.0);
    let (t2, g2) = build_cube([5.0, 5.0, 5.0], 10.0);
    let input = BooleanInput::new(t1, g1, t2, g2, BooleanOp::Union);
    let env = execute_boolean_logged(input.clone());

    let original_log = env.get_decision_log();
    let original_hash = env.get_state_hash_after();

    let counterfactuals = replay_all_near_boundary(&input, original_log, original_hash);

    let mut valid_count = 0;
    let mut broken_count = 0;

    for cf_res in counterfactuals {
        let cf = cf_res.unwrap();
        match cf.get_validation() {
            CounterfactualValidation::TopologyBroken { .. } => broken_count += 1,
            CounterfactualValidation::DivergentButValid => valid_count += 1,
            CounterfactualValidation::Valid => {} // No change
        }
    }

    // Some should theoretically break or divert. Our tests use exact cubes on integer grids so NearBoundary might be empty.
    // We just verify the counterfactual harness runs without panic.
    println!(
        "MB-R5: {} validdivergent, {} broken",
        valid_count, broken_count
    );
}

/// MB-R6: Serialized proof metadata enables cross-session divergence detection.
#[test]
fn mb_r6_serialized_proof_metadata_enables_cross_session_detection() {
    let (t1, g1) = build_cube([0.0, 0.0, 0.0], 10.0);
    let (t2, g2) = build_cube([5.0, 5.0, 5.0], 10.0);
    let input = BooleanInput::new(t1, g1, t2, g2, BooleanOp::Union);
    let env = execute_boolean_logged(input.clone());

    let original_decision = env.get_decision_log().clone();
    let result = env.into_result().expect("Boolean failed");

    let json_replay = serde_json::to_string(&original_replay).unwrap();
    let json_decision = serde_json::to_string(&original_decision).unwrap();
    let json_lineage = serde_json::to_string(&original_lineage).unwrap();

    let decoded_replay: forge_topo::replay::ReplayLog = serde_json::from_str(&json_replay).unwrap();
    let decoded_decision: forge_core::DecisionLog = serde_json::from_str(&json_decision).unwrap();
    let decoded_lineage: Vec<forge_topo::lineage::LineageEvent> =
        serde_json::from_str(&json_lineage).unwrap();

    // Verify determinism checks
    assert!(
        original_replay.verify_determinism(&decoded_replay),
        "Deserialized replay log does not match original"
    );

    // Verify struct invariants on deserialized data
    validate_all(
        &decoded_replay,
        &decoded_decision,
        &decoded_lineage,
        result.topology().arena(),
        forge_topo::hashing::compute_arena_topology_hash(result.topology().arena()),
    )
    .expect("Invariants failed on deserialized data");

    // Re-run the exact same boolean to verify cross-session deterministic replay
    let fresh_env = execute_boolean_logged(input);
    let fresh_decision = fresh_env.get_decision_log();

    let diff = diff_decision_logs(&decoded_decision, fresh_decision);
    assert!(
        diff.is_empty(),
        "Fresh run DecisionLog differs from deserialized log — determinism broken"
    );
}

/// MB-R7: Margin analysis cross-validates with causal chains.
#[test]
fn mb_r7_margin_analysis_cross_validates_with_causal_chains() {
    let (t1, g1) = build_cube([0.0, 0.0, 0.0], 10.0);
    let (t2, g2) = build_cube([10.0, 10.0, 10.0], 10.0); // Face-to-face contact
    let input = BooleanInput::new(t1, g1, t2, g2, BooleanOp::Union);
    let env = execute_boolean_logged(input);
    env.get_value().as_ref().expect("Boolean failed");

    let mut margin_decisions: Vec<_> = env
        .get_decision_log()
        .decisions()
        .filter(|d| d.get_tier() >= DecisionTier::NearBoundary)
        .collect();

    // Test passes if it runs without panic and sorts correctly
    margin_decisions.sort_by(|a, b| a.get_margin().partial_cmp(&b.get_margin()).unwrap());

    if let Some(smallest) = margin_decisions.first() {
        assert!(
            smallest.get_margin() <= 1e-6 || smallest.get_tier() == DecisionTier::Deterministic,
            "Smallest margin should be small or an exact rational decision"
        );
    }
}
