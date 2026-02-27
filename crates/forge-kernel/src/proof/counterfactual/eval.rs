//! Generic counterfactual replay evaluation logic.
//!
//! DOMAIN: Re-executes any operation with forced classification overrides
//! to test "what would have happened if this decision went the other way?"
//!
//! The replay mechanism is operation-agnostic: each feature (boolean,
//! fillet, extrude) provides its own `ReplayFn` closure and
//! `ClassificationCodec`. This module provides the generic comparison
//! and validation infrastructure.
//!
//! DEPENDENCIES: `schema` (CounterfactualResult, etc.),
//! `forge-core` (DecisionLog, DecisionId, TracedDecision, KernelError),
//! `forge-topo` (TopologyState, validate_topology, hashing)

use forge_core::{DecisionContext, DecisionId, DecisionLog, KernelError, TracedDecision};
use forge_topo::hashing::compute_arena_topology_hash;
use forge_topo::state::TopologyState;
use forge_topo::validate::{validate_topology, ValidationLevel};

use super::schema::{
    CounterfactualResult, CounterfactualValidation, DecisionOverride, EntityDelta,
};

/// Outcome of an operation replay, containing the topology result
/// and the decision log produced during replay.
///
/// Each operation constructs this from its own result type.
pub struct ReplayOutcome {
    /// The resulting topology state after replay.
    topology: TopologyState,
    /// Decision log produced during the replay execution.
    decision_log: DecisionLog,
}

impl ReplayOutcome {
    /// Construct a replay outcome from a topology state and decision log.
    pub fn new(topology: TopologyState, decision_log: DecisionLog) -> Self {
        Self {
            topology,
            decision_log,
        }
    }

    /// The replayed topology.
    pub fn get_topology(&self) -> &TopologyState {
        &self.topology
    }

    /// The decision log from the replay.
    pub fn get_decision_log(&self) -> &DecisionLog {
        &self.decision_log
    }
}

/// Strategy for parsing and flipping classifications from decision contexts.
///
/// Each operation type implements this to teach the counterfactual engine
/// how to interpret and invert its domain-specific classification labels.
///
/// For booleans, the labels are "Inside", "Outside", "OnBoundary", etc.
/// For fillets, they might be "Convex", "Concave", "Smooth".
/// For extrusions, they might be "Cap", "Side", "Base".
pub trait ClassificationCodec {
    /// Parse a classification label from a decision's context string.
    /// Returns None if this decision is not a classification decision.
    fn parse_classification(&self, decision: &TracedDecision) -> Option<String>;

    /// Flip a classification label to its counterfactual opposite.
    /// Returns the opposite label as a string.
    fn flip_classification(&self, label: &str) -> String;

    /// Human-readable name of this classification scheme (e.g., "boolean", "fillet").
    fn scheme_name(&self) -> &str;
}

/// The replay function signature.
///
/// Given a list of `(DecisionId, forced_label)` overrides, re-execute
/// the operation and return the outcome. The function captures the
/// operation's input by closure.
///
/// Each operation provides its own implementation. For example, boolean
/// captures `BooleanInput` and calls `execute_boolean_with_overrides`.
pub type ReplayFn<'a> =
    dyn Fn(&[(DecisionId, String)]) -> Result<ReplayOutcome, KernelError> + 'a;

/// Replay an operation with a single classification override.
///
/// Uses the provided `replay_fn` to re-execute the operation with the
/// forced classification for the specified decision, then compares the
/// counterfactual topology against the original hash.
pub fn replay_decision(
    replay_fn: &ReplayFn<'_>,
    codec: &dyn ClassificationCodec,
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

    let original_label =
        codec
            .parse_classification(&original_decision)
            .ok_or_else(|| KernelError::InvalidInput {
                message: format!(
                    "Decision {:?} is not a {} classification decision",
                    target_id,
                    codec.scheme_name()
                ),
                context: None,
            })?;

    let flipped_label = codec.flip_classification(&original_label);
    let overrides = vec![(target_id, flipped_label.clone())];

    let (counterfactual_hash, entity_delta, validation, cf_log) = match replay_fn(&overrides) {
        Ok(outcome) => {
            let cf_hash = compute_arena_topology_hash(outcome.get_topology().arena());
            let cf_face_count = outcome.get_topology().arena().face_count();

            let original_face_count = count_classification_decisions(original_log, codec);
            let face_diff =
                (cf_face_count as i64 - original_face_count as i64).unsigned_abs() as usize;

            let valid =
                validate_topology(outcome.get_topology().arena(), ValidationLevel::Full);
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
                    target_id, original_label, flipped_label, face_diff,
                ),
            );

            (
                cf_hash,
                delta,
                validation,
                outcome.get_decision_log().clone(),
            )
        }
        Err(e) => {
            let validation = CounterfactualValidation::TopologyBroken {
                errors: vec![format!("Operation re-execution failed: {e:?}")],
            };
            (0, EntityDelta::empty(), validation, DecisionLog::new())
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
/// flips the classification and re-executes the operation.
pub fn replay_all_near_boundary(
    replay_fn: &ReplayFn<'_>,
    codec: &dyn ClassificationCodec,
    original_log: &DecisionLog,
    original_hash: u128,
) -> Vec<Result<CounterfactualResult, KernelError>> {
    let interesting = original_log.interesting_only();

    interesting
        .into_iter()
        .filter(|d| codec.parse_classification(d).is_some())
        .map(|decision| {
            let override_spec = DecisionOverride::new(
                decision.get_id(),
                decision.get_kind().clone(),
                decision.get_tier(),
                1.0 - decision.get_margin(),
            );
            replay_decision(replay_fn, codec, original_log, original_hash, &override_spec)
        })
        .collect()
}

/// Count the classification decisions in a log (used to estimate original entity counts).
fn count_classification_decisions(
    log: &DecisionLog,
    codec: &dyn ClassificationCodec,
) -> usize {
    log.decisions()
        .filter(|d| codec.parse_classification(d).is_some())
        .count()
}
