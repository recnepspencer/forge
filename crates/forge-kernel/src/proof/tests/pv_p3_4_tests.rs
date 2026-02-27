//! P3.4: Witness-Based Replay acceptance tests.
//!
//! These tests exercise real Boolean operations, extract real DecisionLogs,
//! then use the counterfactual replay infrastructure to re-execute the
//! Boolean pipeline with forced classification overrides.
//!
//! PV-39: Single classification override → verify topology diverges or breaks
//! PV-40: Deterministic classification override → verify via replay_decision
//! PV-40.5: replay_all_near_boundary on real log

#[cfg(test)]
mod tests {
    use forge_core::{DecisionId, DecisionKind, DecisionTier};
    use forge_topo::hashing::compute_arena_topology_hash;

    use crate::proof::counterfactual::{
        replay_all_near_boundary, replay_decision, DecisionOverride,
    };
    use crate::operations::boolean::test_helpers::{
        build_cube, euler_audit, execute_boolean_logged,
    };
    use crate::operations::boolean::{
        execute_boolean_with_overrides, BooleanInput, BooleanOp, FaceClassification,
    };

    /// PV-39: Override a single classification decision and verify the
    /// topology either diverges from the original OR the pipeline fails.
    ///
    /// PROOF: Re-executing a Boolean with a flipped classification produces
    /// genuinely different behavior — either a different topology hash or a
    /// pipeline failure from inconsistent face selections.
    #[test]
    fn pv_39_single_override_produces_divergent_behavior() {
        let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);
        let (topo_b, geom_b) = build_cube([0.5, 0.0, 0.0], 1.0);

        let input = BooleanInput::new(topo_a, geom_a, topo_b, geom_b, BooleanOp::Union);

        let envelope = execute_boolean_logged(input.clone());
        let original_hash = envelope.get_state_hash_after();
        let original_log = envelope.get_decision_log().clone();

        let classification_decisions: Vec<_> = original_log
            .decisions()
            .filter(|d| {
                matches!(
                    d.get_context(),
                    forge_core::DecisionContext::Classification { .. }
                )
            })
            .collect();

        assert!(
            !classification_decisions.is_empty(),
            "Boolean should produce classification decisions"
        );

        let mut any_diverged = false;

        for decision in &classification_decisions {
            let flipped = match decision.get_context() {
                forge_core::DecisionContext::Classification { result, .. } => {
                    if result.contains("Inside") {
                        FaceClassification::Outside
                    } else if result.contains("Outside") {
                        FaceClassification::Inside
                    } else {
                        FaceClassification::Outside
                    }
                }
                _ => FaceClassification::Outside,
            };

            let overrides = vec![(decision.get_id(), flipped)];
            let cf_envelope = execute_boolean_with_overrides(input.clone(), &overrides);
            let cf_hash = cf_envelope.get_state_hash_after();

            match cf_envelope.into_result() {
                Ok(_) => {
                    if cf_hash != original_hash {
                        any_diverged = true;
                    }
                }
                Err(_) => {
                    any_diverged = true;
                }
            }

            if any_diverged {
                break;
            }
        }

        assert!(
            any_diverged,
            "At least one flipped classification should cause divergence or pipeline failure"
        );
    }

    /// PV-40: Override a classification and verify replay_decision reports it.
    ///
    /// PROOF: The replay_decision function correctly re-executes the
    /// Boolean with the override and reports divergence or broken topology.
    #[test]
    fn pv_40_replay_decision_reports_divergence_or_broken() {
        let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);
        let (topo_b, geom_b) = build_cube([0.5, 0.0, 0.0], 1.0);

        let input = BooleanInput::new(topo_a, geom_a, topo_b, geom_b, BooleanOp::Intersection);

        let envelope = execute_boolean_logged(input.clone());
        let original_hash = envelope.get_state_hash_after();
        let original_log = envelope.get_decision_log().clone();

        let classification_decisions: Vec<_> = original_log
            .decisions()
            .filter(|d| {
                matches!(
                    d.get_context(),
                    forge_core::DecisionContext::Classification { .. }
                )
            })
            .collect();

        assert!(
            !classification_decisions.is_empty(),
            "Boolean should produce classification decisions"
        );

        let mut any_effect = false;

        for target in &classification_decisions {
            let override_spec = DecisionOverride::new(
                target.get_id(),
                target.get_kind().clone(),
                target.get_tier(),
                target.get_margin(),
            );

            let cf_result = replay_decision(&input, &original_log, original_hash, &override_spec);

            match cf_result {
                Ok(cf) => {
                    if cf.has_diverged() || cf.get_validation().is_broken() {
                        any_effect = true;
                        break;
                    }
                }
                Err(_) => {
                    any_effect = true;
                    break;
                }
            }
        }

        assert!(
            any_effect,
            "At least one classification replay should cause divergence or pipeline failure"
        );
    }

    /// PV-40.5: replay_all_near_boundary on a real Boolean log.
    ///
    /// PROOF: All NearBoundary classification decisions can be
    /// counterfactually replayed. Each result reports valid/divergent
    /// or broken topology — no panics or hangs.
    #[test]
    fn pv_40_5_replay_all_near_boundary_completes() {
        let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);
        let (topo_b, geom_b) = build_cube([0.5, 0.0, 0.0], 1.0);

        let input = BooleanInput::new(topo_a, geom_a, topo_b, geom_b, BooleanOp::Union);

        let envelope = execute_boolean_logged(input.clone());
        let original_hash = envelope.get_state_hash_after();
        let original_log = envelope.get_decision_log().clone();

        let results = replay_all_near_boundary(&input, &original_log, original_hash);

        for result in &results {
            match result {
                Ok(cf) => {
                    assert!(
                        cf.get_validation().is_valid() || cf.get_validation().is_broken(),
                        "Each counterfactual should produce a definitive validation"
                    );
                }
                Err(e) => {
                    panic!("replay_all_near_boundary failed on a decision: {:?}", e);
                }
            }
        }

        let total_classification = original_log
            .decisions()
            .filter(|d| {
                matches!(
                    d.get_context(),
                    forge_core::DecisionContext::Classification { .. }
                )
            })
            .count();

        eprintln!(
            "PV-40.5: {} classification decisions, {} near-boundary replayed",
            total_classification,
            results.len(),
        );
    }
}
