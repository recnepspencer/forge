//! Typed policy-resolution trace payloads (side-channel to `TracedDecision`).
//!
//! This module provides machine-readable policy resolution details that are not
//! fully expressible in `DecisionKind` + `DecisionContext` alone. It is designed
//! to be stored alongside decision logs and audit artifacts, keyed by `DecisionId`.

use serde::{Deserialize, Serialize};

use crate::policy::PolicyKind;

use super::{DecisionContext, DecisionId, DecisionKind, DecisionTier, TracedDecision};

/// Where the policy resolution came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyResolutionSource {
    DefaultPolicy,
    UserOverride,
    ForcedSafeFallback,
    NonOverridableRule,
}

/// The outcome of resolving an ambiguous policy query.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyResolutionOutcome {
    AcceptedPotentialValue,
    RejectedPotentialValue,
    EscalatedError,
}

/// Compact typed summary of the candidate value carried by a `PolicyResult::Ambiguous`.
///
/// This is intentionally a summary, not a generic serialized `T`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CandidateValueSummary {
    BooleanFlag { value: bool },
    EnumTag { type_name: String, variant: String },
    Opaque { type_name: String },
}

/// Typed policy-resolution payload to accompany a `TracedDecision`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PolicyDecisionTracePayload {
    pub decision_id: DecisionId,
    pub policy_kind: PolicyKind,
    pub query_location: [f64; 3],
    pub measured_margin: f64,
    pub threshold: Option<f64>,
    pub overridable: bool,
    pub candidate_summary: CandidateValueSummary,
    pub outcome: PolicyResolutionOutcome,
    pub source: PolicyResolutionSource,
    pub default_used: bool,
}

/// Semantic mismatch between a `PolicyDecisionTracePayload` and a `TracedDecision`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyTraceConsistencyError {
    DecisionIdMismatch { payload: u64, decision: u64 },
    DecisionKindMismatch,
    DecisionTierMismatch,
    PolicyKindMismatch,
    SourceDefaultMismatch,
    NonFiniteQueryLocation { axis: u8 },
    NonFiniteMargin,
    NonFiniteThreshold,
    MarginMismatch,
    ThresholdMismatch,
    OverridableMismatch,
}

impl PolicyDecisionTracePayload {
    /// Validate that this payload is consistent with the paired traced decision.
    ///
    /// This validator is intentionally strict for numeric values:
    /// - all payload numerics must be finite
    /// - `measured_margin` and `threshold` (when present) must match the decision
    ///   exactly (bitwise via `to_bits()`)
    ///
    /// Phase 2 may add a domain-tolerant comparison mode for replay/interop use,
    /// but this base-layer validator is the "no silent drift" contract.
    pub fn validate_against_decision(
        &self,
        decision: &TracedDecision,
    ) -> Result<(), PolicyTraceConsistencyError> {
        if self.decision_id != decision.get_id() {
            return Err(PolicyTraceConsistencyError::DecisionIdMismatch {
                payload: self.decision_id.0,
                decision: decision.get_id().0,
            });
        }

        if self.overridable != decision.is_overridable() {
            return Err(PolicyTraceConsistencyError::OverridableMismatch);
        }

        for (axis, coord) in self.query_location.iter().enumerate() {
            if !coord.is_finite() {
                return Err(PolicyTraceConsistencyError::NonFiniteQueryLocation {
                    axis: axis as u8,
                });
            }
        }
        if !self.measured_margin.is_finite() {
            return Err(PolicyTraceConsistencyError::NonFiniteMargin);
        }
        if let Some(t) = self.threshold {
            if !t.is_finite() {
                return Err(PolicyTraceConsistencyError::NonFiniteThreshold);
            }
        }

        match self.source {
            PolicyResolutionSource::DefaultPolicy if !self.default_used => {
                return Err(PolicyTraceConsistencyError::SourceDefaultMismatch);
            }
            PolicyResolutionSource::UserOverride if self.default_used => {
                return Err(PolicyTraceConsistencyError::SourceDefaultMismatch);
            }
            PolicyResolutionSource::ForcedSafeFallback | PolicyResolutionSource::NonOverridableRule => {
                // `default_used` may be false or true depending on how the caller models
                // forced/non-overridable handling. Phase 2 will tighten this once the
                // policy registry + operation finalization contract is in place.
            }
            _ => {}
        }

        if self.measured_margin.to_bits() != decision.get_margin().to_bits() {
            return Err(PolicyTraceConsistencyError::MarginMismatch);
        }

        match decision.get_context() {
            DecisionContext::Tolerance { measured, threshold } => {
                if self.measured_margin.to_bits() != measured.to_bits() {
                    return Err(PolicyTraceConsistencyError::MarginMismatch);
                }
                match self.threshold {
                    Some(t) if t.to_bits() == threshold.to_bits() => {}
                    _ => return Err(PolicyTraceConsistencyError::ThresholdMismatch),
                }
            }
            _ => {
                if self.threshold.is_some() {
                    return Err(PolicyTraceConsistencyError::ThresholdMismatch);
                }
            }
        }

        match self.outcome {
            PolicyResolutionOutcome::AcceptedPotentialValue => {
                match decision.get_kind() {
                    DecisionKind::PolicyApplied { policy, default_used } => {
                        if *policy != self.policy_kind || *default_used != self.default_used {
                            return Err(PolicyTraceConsistencyError::PolicyKindMismatch);
                        }
                    }
                    _ => return Err(PolicyTraceConsistencyError::DecisionKindMismatch),
                }
                if decision.get_tier() != DecisionTier::PolicyApplied {
                    return Err(PolicyTraceConsistencyError::DecisionTierMismatch);
                }
            }
            PolicyResolutionOutcome::RejectedPotentialValue | PolicyResolutionOutcome::EscalatedError => {
                match decision.get_kind() {
                    DecisionKind::Ambiguous { .. } | DecisionKind::Forced { .. } => {}
                    _ => return Err(PolicyTraceConsistencyError::DecisionKindMismatch),
                }
                if decision.get_tier() != DecisionTier::Escalated {
                    return Err(PolicyTraceConsistencyError::DecisionTierMismatch);
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy::PolicyKind;
    use crate::tracing::{DecisionContext, DecisionId, DecisionKind, DecisionTier, TracedDecision};

    fn sample_payload(source: PolicyResolutionSource) -> PolicyDecisionTracePayload {
        PolicyDecisionTracePayload {
            decision_id: DecisionId(7),
            policy_kind: PolicyKind::CoincidentGeometry,
            query_location: [1.0, 2.0, 3.0],
            measured_margin: 1e-6,
            threshold: Some(1e-5),
            overridable: true,
            candidate_summary: CandidateValueSummary::EnumTag {
                type_name: "WeakSimpleCertificate".to_string(),
                variant: "WeaklySimple".to_string(),
            },
            outcome: PolicyResolutionOutcome::AcceptedPotentialValue,
            source,
            default_used: matches!(source, PolicyResolutionSource::DefaultPolicy),
        }
    }

    fn sample_decision(default_used: bool) -> TracedDecision {
        TracedDecision::new(
            DecisionId(7),
            DecisionKind::PolicyApplied {
                policy: PolicyKind::CoincidentGeometry,
                default_used,
            },
            DecisionTier::PolicyApplied,
            1e-6,
            DecisionContext::Tolerance {
                measured: 1e-6,
                threshold: 1e-5,
            },
        )
    }

    #[test]
    fn policy_trace_payload_round_trips_json() {
        let payload = sample_payload(PolicyResolutionSource::DefaultPolicy);
        let json = serde_json::to_string(&payload).expect("serialize policy trace payload");
        let restored: PolicyDecisionTracePayload =
            serde_json::from_str(&json).expect("deserialize policy trace payload");
        assert_eq!(restored, payload);
    }

    #[test]
    fn policy_trace_payload_captures_default_vs_user_source() {
        let default_payload = sample_payload(PolicyResolutionSource::DefaultPolicy);
        let user_payload = sample_payload(PolicyResolutionSource::UserOverride);

        assert_ne!(default_payload.source, user_payload.source);
        assert!(default_payload.default_used);
        assert!(!user_payload.default_used);
    }

    #[test]
    fn policy_trace_payload_outcome_matches_decision_kind_and_tier() {
        let payload = sample_payload(PolicyResolutionSource::DefaultPolicy);
        let decision = sample_decision(true);

        assert_eq!(payload.validate_against_decision(&decision), Ok(()));
    }

    #[test]
    fn policy_trace_payload_detects_kind_tier_mismatch() {
        let mut payload = sample_payload(PolicyResolutionSource::DefaultPolicy);
        payload.outcome = PolicyResolutionOutcome::EscalatedError;

        let decision = sample_decision(true);
        assert_eq!(
            payload.validate_against_decision(&decision),
            Err(PolicyTraceConsistencyError::DecisionKindMismatch)
        );
    }

    #[test]
    fn policy_trace_payload_detects_source_default_mismatch() {
        let mut payload = sample_payload(PolicyResolutionSource::UserOverride);
        payload.default_used = true;

        let mut decision = sample_decision(true);
        decision.set_overridable(true);

        assert_eq!(
            payload.validate_against_decision(&decision),
            Err(PolicyTraceConsistencyError::SourceDefaultMismatch)
        );
    }

    #[test]
    fn policy_trace_payload_rejects_non_finite_numeric_fields() {
        let decision = sample_decision(true);

        let mut bad_margin = sample_payload(PolicyResolutionSource::DefaultPolicy);
        bad_margin.measured_margin = f64::NAN;
        assert_eq!(
            bad_margin.validate_against_decision(&decision),
            Err(PolicyTraceConsistencyError::NonFiniteMargin)
        );

        let mut bad_threshold = sample_payload(PolicyResolutionSource::DefaultPolicy);
        bad_threshold.threshold = Some(f64::INFINITY);
        assert_eq!(
            bad_threshold.validate_against_decision(&decision),
            Err(PolicyTraceConsistencyError::NonFiniteThreshold)
        );

        let mut bad_loc = sample_payload(PolicyResolutionSource::DefaultPolicy);
        bad_loc.query_location[2] = f64::NEG_INFINITY;
        assert_eq!(
            bad_loc.validate_against_decision(&decision),
            Err(PolicyTraceConsistencyError::NonFiniteQueryLocation { axis: 2 })
        );
    }

    #[test]
    fn policy_trace_payload_uses_strict_float_matching_for_signed_zero() {
        let mut payload = sample_payload(PolicyResolutionSource::DefaultPolicy);
        payload.measured_margin = -0.0;

        let decision = TracedDecision::new(
            DecisionId(7),
            DecisionKind::PolicyApplied {
                policy: PolicyKind::CoincidentGeometry,
                default_used: true,
            },
            DecisionTier::PolicyApplied,
            0.0,
            DecisionContext::Tolerance {
                measured: 0.0,
                threshold: 1e-5,
            },
        );

        assert_eq!(
            payload.validate_against_decision(&decision),
            Err(PolicyTraceConsistencyError::MarginMismatch),
            "strict validator must detect signed-zero drift"
        );
    }

    #[test]
    fn policy_trace_payload_detects_tiny_margin_delta_under_strict_matching() {
        let mut payload = sample_payload(PolicyResolutionSource::DefaultPolicy);
        payload.measured_margin = 1e-6 + f64::EPSILON;

        let decision = sample_decision(true);
        assert_eq!(
            payload.validate_against_decision(&decision),
            Err(PolicyTraceConsistencyError::MarginMismatch)
        );
    }
}
