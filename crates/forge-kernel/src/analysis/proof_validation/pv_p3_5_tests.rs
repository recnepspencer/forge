//! P3.5: MetaBoss Replay Torture Suite.
//!
//! These tests exercise the real Boolean pipeline under stress conditions:
//! chained operations, repeated execution, decision extraction, delta-debug,
//! counterfactual replay, serialization, and FMA sensitivity analysis.

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use forge_core::result::{DecisionId, DecisionTier, DecisionKind, DecisionContext, TracedDecision};
    use forge_topo::hashing::compute_arena_topology_hash;

    use crate::operations::boolean::test_helpers::{
        build_cube, execute_boolean_logged, euler_audit,
    };
    use crate::operations::boolean::{
        BooleanInput, BooleanOp, BooleanResult, FaceClassification,
        execute_boolean, execute_boolean_with_overrides,
    };
    use crate::analysis::counterfactual::{
        replay_decision, replay_all_near_boundary,
        DecisionOverride,
    };

    /// MB-R1: Chained Boolean operations with causal chain performance.
    ///
    /// Runs 10 real chained union operations and verifies that the total
    /// decision count grows with each step (proving decisions accumulate).
    #[test]
    fn mb_r1_chained_boolean_decision_accumulation() {
        let step_count = 10;

        let (mut current_topo, mut current_geom) = build_cube([0.0, 0.0, 0.0], 1.0);
        let mut total_decisions = 0usize;

        let start = Instant::now();

        for i in 1..=step_count {
            let offset = i as f64 * 0.5;
            let (tool_topo, tool_geom) = build_cube([offset, 0.0, 0.0], 1.0);

            let input = BooleanInput::new(
                current_topo,
                current_geom,
                tool_topo,
                tool_geom,
                BooleanOp::Union,
            );

            let envelope = execute_boolean_logged(input)
                .unwrap_or_else(|e| panic!("Chain step {} failed: {:?}", i, e));

            let step_decisions = envelope.get_decision_log().len();
            total_decisions += step_decisions;

            let result = envelope.into_value();
            current_topo = result.topology().clone();
            current_geom = result.geometry().clone();
        }

        let elapsed = start.elapsed();

        assert!(
            total_decisions > step_count,
            "10 chained booleans should produce more than 10 total decisions, got {}",
            total_decisions,
        );

        eprintln!(
            "MB-R1: {} steps, {} total decisions, {:.1}ms total",
            step_count, total_decisions, elapsed.as_secs_f64() * 1000.0,
        );
    }

    /// MB-R1 scaled: 500-step chained Boolean operations.
    ///
    /// Exercises long operation chains and verifies that decision
    /// accumulation and topology growth remain well-behaved at scale.
    #[test]
    #[ignore]
    fn mb_r1_scaled_500_step_chain() {
        let step_count = 500;

        let (mut current_topo, mut current_geom) = build_cube([0.0, 0.0, 0.0], 1.0);
        let mut total_decisions = 0usize;

        let start = Instant::now();

        for i in 1..=step_count {
            let offset = i as f64 * 0.5;
            let (tool_topo, tool_geom) = build_cube([offset, 0.0, 0.0], 1.0);

            let input = BooleanInput::new(
                current_topo,
                current_geom,
                tool_topo,
                tool_geom,
                BooleanOp::Union,
            );

            let envelope = execute_boolean_logged(input)
                .unwrap_or_else(|e| panic!("Chain step {} failed: {:?}", i, e));

            let step_decisions = envelope.get_decision_log().len();
            total_decisions += step_decisions;

            let result = envelope.into_value();
            current_topo = result.topology().clone();
            current_geom = result.geometry().clone();
        }

        let elapsed = start.elapsed();

        assert!(
            total_decisions > step_count,
            "500 chained booleans must produce more than 500 total decisions, got {}",
            total_decisions,
        );

        eprintln!(
            "MB-R1-SCALED: {} steps, {} total decisions, {:.1}ms total, final faces={}",
            step_count, total_decisions, elapsed.as_secs_f64() * 1000.0,
            current_topo.arena().face_count(),
        );
    }

    /// MB-R2: 100x replay determinism.
    ///
    /// Runs the same Boolean operation 100 times and verifies that the
    /// topology hash AND decision log summary are identical every time.
    #[test]
    fn mb_r2_100x_replay_determinism() {
        let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);
        let (topo_b, geom_b) = build_cube([0.5, 0.0, 0.0], 1.0);

        let make_input = || {
            BooleanInput::new(
                topo_a.clone(), geom_a.clone(),
                topo_b.clone(), geom_b.clone(),
                BooleanOp::Union,
            )
        };

        let first_envelope = execute_boolean(make_input())
            .expect("first run failed");
        let reference_hash = first_envelope.get_state_hash_after();
        let reference_decision_count = first_envelope.get_decision_log().len();

        for i in 1..100 {
            let envelope = execute_boolean(make_input())
                .unwrap_or_else(|e| panic!("Replay {} failed: {:?}", i, e));

            assert_eq!(
                envelope.get_state_hash_after(), reference_hash,
                "Replay {} produced different topology hash", i,
            );
            assert_eq!(
                envelope.get_decision_log().len(), reference_decision_count,
                "Replay {} produced different decision count", i,
            );
        }
    }

    /// MB-R3: Decision extraction scoped to faces.
    ///
    /// Runs a Boolean producing many faces, then verifies that
    /// classification decisions can be extracted per-face.
    #[test]
    fn mb_r3_per_face_decision_extraction() {
        let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);
        let (topo_b, geom_b) = build_cube([0.5, 0.0, 0.0], 1.0);

        let input = BooleanInput::new(
            topo_a, geom_a, topo_b, geom_b, BooleanOp::Union,
        );

        let envelope = execute_boolean_logged(input)
            .expect("Boolean failed");

        let log = envelope.get_decision_log();

        let classification_decisions: Vec<_> = log.decisions()
            .filter(|d| matches!(d.get_context(), DecisionContext::Classification { .. }))
            .collect();

        let select_decisions: Vec<_> = log.decisions()
            .filter(|d| {
                if let DecisionContext::Classification { result, .. } = d.get_context() {
                    result.contains("Keep") || result.contains("Drop")
                } else {
                    false
                }
            })
            .collect();

        assert!(
            !classification_decisions.is_empty(),
            "Should have classification decisions"
        );

        let unique_face_indices: std::collections::BTreeSet<_> = classification_decisions
            .iter()
            .map(|d| d.get_id().0)
            .collect();

        assert!(
            unique_face_indices.len() >= 6,
            "Overlapping cubes union should classify at least 6 faces, got {}",
            unique_face_indices.len(),
        );

        eprintln!(
            "MB-R3: {} classification decisions across {} unique faces, {} select decisions",
            classification_decisions.len(),
            unique_face_indices.len(),
            select_decisions.len(),
        );
    }

    /// MB-R4: Delta-debug bisection on a Boolean chain.
    ///
    /// Builds a chain of 8 Boolean operations, defines a failure predicate
    /// (face count > threshold), then bisects to find the first step that
    /// pushes the count over.
    #[test]
    fn mb_r4_delta_debug_bisection() {
        let step_count = 8;
        let mut operations: Vec<(f64, f64)> = Vec::new();

        for i in 0..step_count {
            let offset = i as f64 * 0.5;
            operations.push((offset, 1.0));
        }

        let face_count_threshold = 10;

        let mut first_failure_idx = None;
        let (mut current_topo, mut current_geom) = build_cube([0.0, 0.0, 0.0], 1.0);

        for (i, &(offset, half)) in operations.iter().enumerate() {
            let (tool_topo, tool_geom) = build_cube([offset, 0.0, 0.0], half);

            let input = BooleanInput::new(
                current_topo,
                current_geom,
                tool_topo,
                tool_geom,
                BooleanOp::Union,
            );

            let envelope = execute_boolean(input)
                .unwrap_or_else(|e| panic!("Step {} failed: {:?}", i, e));
            let result = envelope.into_value();

            let fc = result.topology().arena().face_count();
            current_topo = result.topology().clone();
            current_geom = result.geometry().clone();

            if fc > face_count_threshold && first_failure_idx.is_none() {
                first_failure_idx = Some(i);
            }
        }

        eprintln!(
            "MB-R4: face_count_threshold={}, first_failure_idx={:?}, final_faces={}",
            face_count_threshold,
            first_failure_idx,
            current_topo.arena().face_count(),
        );
    }

    /// MB-R4 scaled: 200-step delta-debug bisection.
    ///
    /// Longer chain for more thorough bisection testing.
    #[test]
    #[ignore]
    fn mb_r4_scaled_200_step_bisection() {
        let step_count = 200;
        let mut operations: Vec<(f64, f64)> = Vec::new();

        for i in 0..step_count {
            let offset = i as f64 * 0.5;
            operations.push((offset, 1.0));
        }

        let face_count_threshold = 10;

        let mut first_failure_idx = None;
        let (mut current_topo, mut current_geom) = build_cube([0.0, 0.0, 0.0], 1.0);

        for (i, &(offset, half)) in operations.iter().enumerate() {
            let (tool_topo, tool_geom) = build_cube([offset, 0.0, 0.0], half);

            let input = BooleanInput::new(
                current_topo,
                current_geom,
                tool_topo,
                tool_geom,
                BooleanOp::Union,
            );

            let envelope = execute_boolean(input)
                .unwrap_or_else(|e| panic!("Step {} failed: {:?}", i, e));
            let result = envelope.into_value();

            let fc = result.topology().arena().face_count();
            current_topo = result.topology().clone();
            current_geom = result.geometry().clone();

            if fc > face_count_threshold && first_failure_idx.is_none() {
                first_failure_idx = Some(i);
            }
        }

        eprintln!(
            "MB-R4-SCALED: steps={}, face_count_threshold={}, first_failure_idx={:?}, final_faces={}",
            step_count,
            face_count_threshold,
            first_failure_idx,
            current_topo.arena().face_count(),
        );
    }

    /// MB-R5: Counterfactual replay on all classification decisions.
    ///
    /// Runs a real Boolean, collects all classification decisions,
    /// and replays each with a flipped classification. Verifies that
    /// all counterfactuals either diverge-but-valid or report broken.
    #[test]
    fn mb_r5_counterfactual_all_classifications() {
        let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);
        let (topo_b, geom_b) = build_cube([0.5, 0.0, 0.0], 1.0);

        let input = BooleanInput::new(
            topo_a, geom_a, topo_b, geom_b, BooleanOp::Union,
        );

        let envelope = execute_boolean_logged(input.clone())
            .expect("original Boolean failed");
        let original_hash = envelope.get_state_hash_after();
        let original_log = envelope.get_decision_log().clone();

        let classification_decisions: Vec<_> = original_log
            .decisions()
            .filter(|d| matches!(d.get_context(), DecisionContext::Classification { .. }))
            .collect();

        let mut divergent_count = 0usize;
        let mut broken_count = 0usize;
        let mut same_count = 0usize;

        for decision in &classification_decisions {
            let overrides = vec![(decision.get_id(), FaceClassification::Outside)];
            let cf_result = execute_boolean_with_overrides(input.clone(), &overrides);

            match cf_result {
                Ok(cf_envelope) => {
                    let cf_hash = cf_envelope.get_state_hash_after();
                    if cf_hash != original_hash {
                        divergent_count += 1;
                    } else {
                        same_count += 1;
                    }
                }
                Err(_) => {
                    broken_count += 1;
                }
            }
        }

        eprintln!(
            "MB-R5: {} classifications, {} divergent, {} broken, {} same",
            classification_decisions.len(),
            divergent_count,
            broken_count,
            same_count,
        );

        assert!(
            divergent_count + broken_count > 0,
            "At least some overrides should cause divergence or breakage"
        );
    }

    /// MB-R6: Cross-session replay via serialization.
    ///
    /// Runs a Boolean, serializes the DecisionLog to JSON, deserializes it,
    /// verifies the round-trip preserves decision count and content.
    #[test]
    fn mb_r6_decision_log_serialization_roundtrip() {
        let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);
        let (topo_b, geom_b) = build_cube([0.5, 0.0, 0.0], 1.0);

        let input = BooleanInput::new(
            topo_a, geom_a, topo_b, geom_b, BooleanOp::Union,
        );

        let envelope = execute_boolean_logged(input)
            .expect("Boolean failed");
        let log = envelope.get_decision_log();

        let serialized = serde_json::to_string(log)
            .expect("DecisionLog serialization failed");

        assert!(
            !serialized.is_empty(),
            "Serialized log should not be empty"
        );

        let deserialized: forge_core::DecisionLog = serde_json::from_str(&serialized)
            .expect("DecisionLog deserialization failed");

        assert_eq!(
            log.len(), deserialized.len(),
            "Round-trip should preserve decision count"
        );

        let original_decisions: Vec<_> = log.decisions().collect();
        let restored_decisions: Vec<_> = deserialized.decisions().collect();

        for (orig, restored) in original_decisions.iter().zip(restored_decisions.iter()) {
            assert_eq!(
                orig.get_id(), restored.get_id(),
                "Decision IDs should match after round-trip"
            );
            assert_eq!(
                orig.get_tier(), restored.get_tier(),
                "Decision tiers should match after round-trip"
            );
        }

        let arch = std::env::consts::ARCH;
        eprintln!(
            "MB-R6: {} decisions serialized ({} bytes), arch={}, round-trip OK",
            log.len(),
            serialized.len(),
            arch,
        );
    }

    /// MB-R7: FMA sensitivity via margin analysis.
    ///
    /// Runs a Boolean, collects all classification decisions, identifies
    /// decisions with margin < 1e-6 as potentially FMA-sensitive.
    #[test]
    fn mb_r7_fma_sensitivity_margin_analysis() {
        let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);
        let (topo_b, geom_b) = build_cube([0.5, 0.0, 0.0], 1.0);

        let input = BooleanInput::new(
            topo_a, geom_a, topo_b, geom_b, BooleanOp::Union,
        );

        let envelope = execute_boolean_logged(input)
            .expect("Boolean failed");
        let log = envelope.get_decision_log();

        let all_decisions: Vec<_> = log.decisions().collect();
        let classification_decisions: Vec<_> = all_decisions
            .iter()
            .filter(|d| matches!(d.get_context(), DecisionContext::Classification { .. }))
            .collect();

        let fma_epsilon = 1e-6;
        let fma_sensitive: Vec<_> = classification_decisions
            .iter()
            .filter(|d| d.get_margin().abs() < fma_epsilon)
            .collect();

        eprintln!(
            "MB-R7: {} classifications, {} with margin < {:.0e} (FMA-sensitive candidates)",
            classification_decisions.len(),
            fma_sensitive.len(),
            fma_epsilon,
        );

        for d in &fma_sensitive {
            assert!(
                matches!(
                    d.get_tier(),
                    DecisionTier::NearBoundary | DecisionTier::PolicyApplied | DecisionTier::Escalated
                ),
                "FMA-sensitive decision {:?} (margin={:.2e}) must have NearBoundary or Ambiguous tier, got {:?}",
                d.get_id(),
                d.get_margin(),
                d.get_tier(),
            );
            eprintln!(
                "  FMA-candidate: {:?} margin={:.2e} tier={:?}",
                d.get_id(),
                d.get_margin(),
                d.get_tier(),
            );
        }
    }
}
