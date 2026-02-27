//! Counterfactual replay evaluation logic.
//!
//! DOMAIN: Re-executes a Boolean operation with forced classification
//! overrides to test "what would have happened if this decision went
//! the other way?" Uses the real Boolean pipeline, not hash perturbation.
//!
//! DEPENDENCIES: `schema` (CounterfactualResult, etc.),
//! `forge-core` (DecisionLog, DecisionId, TracedDecision, KernelError),
//! `forge-topo` (TopologyState, validate_topology, hashing),
//! `operations::boolean` (execute_boolean_with_overrides, FaceClassification)

use forge_core::{
    DecisionContext, DecisionId, DecisionLog, DecisionTier, KernelError, TracedDecision,
};
use forge_topo::hashing::compute_arena_topology_hash;
use forge_topo::validate::{validate_topology, ValidationLevel};

use crate::operations::boolean::{BooleanInput, FaceClassification};

use super::schema::{
    CounterfactualResult, CounterfactualValidation, DecisionOverride, EntityDelta,
};

/// Stub: deprecated pipeline removed from compilation.
fn execute_boolean_with_overrides(
    _input: BooleanInput,
    _overrides: &[(forge_core::DecisionId, FaceClassification)],
) -> forge_core::OperationResult<Result<crate::operations::boolean::BooleanResult, KernelError>> {
    forge_core::OperationResult::new(Err(KernelError::InternalError {
        message: "execute_boolean_with_overrides: deprecated pipeline removed".into(),
        context: None,
    }))
}

/// Replay a Boolean operation with a single classification override.
///
/// Clones the `BooleanInput`, injects the forced classification for
/// the specified decision, re-executes the full Boolean pipeline,
/// then compares the counterfactual topology against the original.
pub fn replay_decision(
    input: &BooleanInput,
    original_log: &DecisionLog,
    original_hash: u128,
    decision_override: &DecisionOverride,
) -> Result<CounterfactualResult, KernelError> {
    let target_id = decision_override.get_target_id();

    let original_decision = original_log
        .get_by_id(target_id)
        .ok_or_else(|| KernelError::InvalidInput {
            message: format!("Decision {:?} not found in log", target_id),
            context: None,
        })?
        .clone();

    let classification =
        parse_classification_from_context(&original_decision).ok_or_else(|| {
            KernelError::InvalidInput {
                message: format!("Decision {:?} is not a classification decision", target_id),
                context: None,
            }
        })?;

    let flipped = flip_classification(classification);
    let overrides = vec![(target_id, flipped)];

    let cloned_input = input.clone();
    let counterfactual_envelope = execute_boolean_with_overrides(cloned_input, &overrides);
    let counterfactual_decisions = counterfactual_envelope.get_decision_log().clone();

    let (counterfactual_hash, entity_delta, validation, cf_log) =
        match counterfactual_envelope.into_result() {
            Ok(cf_result) => {
                let cf_hash = compute_arena_topology_hash(cf_result.topology().arena());
                let cf_face_count = cf_result.topology().arena().face_count();

                let original_face_count = estimate_original_face_count(original_log);
                let face_diff =
                    (cf_face_count as i64 - original_face_count as i64).unsigned_abs() as usize;

                let valid = validate_topology(cf_result.topology().arena(), ValidationLevel::Full);
                let validation = match valid {
                    Ok(()) => {
                        if cf_hash != original_hash {
                            CounterfactualValidation::DivergentButValid
                        } else {
                            CounterfactualValidation::Valid
                        }
                    }
                    Err(e) => CounterfactualValidation::TopologyBroken {
                        errors: vec![format!("{e:?}")],
                    },
                };

                let delta = EntityDelta::new(
                    face_diff,
                    0,
                    0,
                    format!(
                        "Flipped {:?} from {} to {} — face count delta: {}",
                        target_id,
                        classification_label(classification),
                        classification_label(flipped),
                        face_diff,
                    ),
                );

                (cf_hash, delta, validation, counterfactual_decisions)
            }
            Err(e) => {
                let validation = CounterfactualValidation::TopologyBroken {
                    errors: vec![format!("Boolean re-execution failed: {e:?}")],
                };
                (
                    0,
                    EntityDelta::empty(),
                    validation,
                    counterfactual_decisions,
                )
            }
        };

    Ok(CounterfactualResult::new(
        original_decision,
        decision_override.clone(),
        original_hash,
        counterfactual_hash,
        entity_delta,
        validation,
        cf_log,
    ))
}

/// Replay all NearBoundary decisions and return counterfactual results.
///
/// For each classification decision at tier NearBoundary or higher,
/// flips the classification (Inside↔Outside, OnBoundary↔OppositeBoundary)
/// and re-executes the Boolean pipeline.
pub fn replay_all_near_boundary(
    input: &BooleanInput,
    original_log: &DecisionLog,
    original_hash: u128,
) -> Vec<Result<CounterfactualResult, KernelError>> {
    let interesting = original_log.interesting_only();

    interesting
        .into_iter()
        .filter(|d| is_classification_decision(d))
        .map(|decision| {
            let override_spec = DecisionOverride::new(
                decision.get_id(),
                decision.get_kind().clone(),
                decision.get_tier(),
                1.0 - decision.get_margin(),
            );
            replay_decision(input, original_log, original_hash, &override_spec)
        })
        .collect()
}

/// Parse the face classification from a decision's context string.
///
/// Classification decisions have context like "Target:Face#3 → Inside (seed)"
/// or "Tool:Face#1 → Outside (patch of seed #0)". This function extracts
/// the classification label from the result string.
fn parse_classification_from_context(decision: &TracedDecision) -> Option<FaceClassification> {
    let result_str = match decision.get_context() {
        DecisionContext::Classification { result, .. } => result.as_str(),
        _ => return None,
    };

    if result_str.contains("Inside") {
        return Some(FaceClassification::Inside);
    }
    if result_str.contains("Outside") {
        return Some(FaceClassification::Outside);
    }
    if result_str.contains("OppositeBoundary") {
        return Some(FaceClassification::OppositeBoundary);
    }
    if result_str.contains("OnBoundary") {
        return Some(FaceClassification::OnBoundary);
    }
    None
}

/// Whether a traced decision represents a classification decision.
fn is_classification_decision(decision: &TracedDecision) -> bool {
    parse_classification_from_context(decision).is_some()
}

/// Flip a face classification to its counterfactual opposite.
fn flip_classification(class: FaceClassification) -> FaceClassification {
    match class {
        FaceClassification::Inside => FaceClassification::Outside,
        FaceClassification::Outside => FaceClassification::Inside,
        FaceClassification::Ambiguous => FaceClassification::OnBoundary,
        FaceClassification::OnBoundary => FaceClassification::OppositeBoundary,
        FaceClassification::OppositeBoundary => FaceClassification::OnBoundary,
    }
}

/// Human-readable label for a classification.
fn classification_label(class: FaceClassification) -> &'static str {
    match class {
        FaceClassification::Inside => "Inside",
        FaceClassification::Outside => "Outside",
        FaceClassification::Ambiguous => "Ambiguous",
        FaceClassification::OnBoundary => "OnBoundary",
        FaceClassification::OppositeBoundary => "OppositeBoundary",
    }
}

/// Estimate the original face count from the decision log.
///
/// Counts unique classification decisions (each face gets one decision
/// in the classify phase).
fn estimate_original_face_count(log: &DecisionLog) -> usize {
    log.decisions()
        .filter(|d| is_classification_decision(d))
        .count()
}
