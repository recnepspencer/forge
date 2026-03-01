//! Policy resolution — the cascade engine for ambiguous queries.
//!
//! DOMAIN: Resolves policy queries against the 5-level override cascade
//! (defaults → session → model → feature → operation).
//! INVARIANTS: Missing policies fail closed (ForcedSafeFallback).

use forge_core::errors::AmbiguousResult;
use forge_core::tracing::{
    CandidateValueSummary, PolicyDecisionTracePayload, PolicyResolutionOutcome,
    PolicyResolutionScopeRef, PolicyResolutionSource, TraceAdjunctRecord,
};
use forge_core::{
    DecisionContext, DecisionId, DecisionKind, DecisionTier, KernelError, PolicyKind, PolicyQuery,
    TracedDecision,
};

use crate::configuration::facade::ConfigScope;
use crate::context::data::{ModelingContext, ResolvedPolicyDecision, ResolvedPolicySource};
use crate::observability::facade::KernelSpan;

impl ModelingContext {
    fn resolve_policy_source_for_query(
        &self,
        query: &PolicyQuery,
    ) -> Option<(bool, ResolvedPolicySource)> {
        if !query.overridable {
            return Some((
                false,
                ResolvedPolicySource {
                    source: PolicyResolutionSource::NonOverridableRule,
                    source_scope: None,
                    default_used: false,
                },
            ));
        }

        let (config, config_source, default_used) = if KernelSpan::is_active() {
            if let Some(snapshot) = KernelSpan::get_config_snapshot() {
                let fallback_path = format!("policy.fallback_rules.{:?}", query.kind);
                let source = snapshot.source_of(&fallback_path).cloned();

                let default_used = if let Some(src) = &source {
                    matches!(src.scope, ConfigScope::SessionDefault)
                } else {
                    true
                };

                (snapshot.config().clone(), source, default_used)
            } else {
                (self.config.clone(), None, true)
            }
        } else {
            (self.config.clone(), None, true)
        };

        if let Some(value) = config.policy.fallback_rules.get(&query.kind) {
            let (res_source, res_scope) = match config_source {
                Some(src) => {
                    let res_src = match src.scope {
                        ConfigScope::SessionDefault => PolicyResolutionSource::SessionUserOverride,
                        ConfigScope::ModelOverride => PolicyResolutionSource::ModelSpecOverride,
                        ConfigScope::FeatureOverride => PolicyResolutionSource::FeatureOverride,
                        ConfigScope::OperationOverride => PolicyResolutionSource::OperationOverride,
                    };

                    let res_scp = match src.scope {
                        ConfigScope::SessionDefault => None,
                        ConfigScope::ModelOverride => src
                            .origin
                            .as_deref()
                            .map(|id| PolicyResolutionScopeRef::ModelSpec {
                                policy_key: id.to_string(),
                            }),
                        ConfigScope::FeatureOverride => src.origin.as_deref().map(|id| {
                            PolicyResolutionScopeRef::Feature {
                                feature_id: id.to_string(),
                            }
                        }),
                        ConfigScope::OperationOverride => src.origin.as_deref().map(|id| {
                            PolicyResolutionScopeRef::Operation {
                                operation_id: id.to_string(),
                            }
                        }),
                    };
                    (res_src, res_scp)
                }
                None => (PolicyResolutionSource::DefaultPolicy, None), // It came from the base config defaults
            };

            return Some((
                *value,
                ResolvedPolicySource {
                    source: res_source,
                    source_scope: res_scope,
                    default_used,
                },
            ));
        }

        None
    }

    /// Resolve an ambiguous `PolicyQuery` using the context's policy registry.
    ///
    /// Returns a typed resolution record and logs a traced decision. The returned
    /// adjunct must be attached to the operation's finalization adjunct set.
    pub fn resolve_policy_query(
        &mut self,
        decision_id: DecisionId,
        query: &PolicyQuery,
        threshold: Option<f64>,
        candidate_summary: CandidateValueSummary,
    ) -> Result<ResolvedPolicyDecision, KernelError> {
        let margin = query.margin;
        let context = match threshold {
            Some(t) => DecisionContext::Tolerance {
                measured: margin,
                threshold: t,
            },
            None => DecisionContext::Degeneracy {
                description: format!("Policy query {:?} at {:?}", query.kind, query.location),
            },
        };

        let (accept, resolved_source, outcome, decision_kind, decision_tier) =
            match self.resolve_policy_source_for_query(query) {
                Some((accept, source)) => {
                    if source.source == PolicyResolutionSource::NonOverridableRule {
                        (
                            false,
                            source,
                            PolicyResolutionOutcome::RejectedPotentialValue,
                            DecisionKind::Forced {
                                reason: format!("NonOverridablePolicy({:?})", query.kind),
                            },
                            DecisionTier::Escalated,
                        )
                    } else if accept {
                        (
                            true,
                            source.clone(),
                            PolicyResolutionOutcome::AcceptedPotentialValue,
                            DecisionKind::PolicyApplied {
                                policy: query.kind.clone(),
                                default_used: source.default_used,
                            },
                            DecisionTier::PolicyApplied,
                        )
                    } else {
                        (
                            false,
                            source,
                            PolicyResolutionOutcome::RejectedPotentialValue,
                            DecisionKind::Ambiguous {
                                fallback_applied: "policy_rejected_candidate".to_string(),
                            },
                            DecisionTier::Escalated,
                        )
                    }
                }
                None => {
                    let source = ResolvedPolicySource {
                        source: PolicyResolutionSource::ForcedSafeFallback,
                        source_scope: None,
                        default_used: false,
                    };
                    let decision = TracedDecision::new(
                        decision_id,
                        DecisionKind::Ambiguous {
                            fallback_applied: "policy_missing_escalated_error".to_string(),
                        },
                        DecisionTier::Escalated,
                        margin,
                        context.clone(),
                    );
                    self.decision_log.record(decision);
                    let payload = PolicyDecisionTracePayload {
                        decision_id,
                        policy_kind: query.kind.clone(),
                        operation_scope_id: None,
                        query_location: query.location,
                        measured_margin: margin,
                        threshold,
                        overridable: query.overridable,
                        candidate_summary,
                        outcome: PolicyResolutionOutcome::EscalatedError,
                        source: source.source,
                        source_scope: source.source_scope.clone(),
                        default_used: false,
                    };
                    self.push_trace_adjunct(TraceAdjunctRecord::from_policy_payload(&payload));
                    return Err(KernelError::AmbiguousResult {
                        result: AmbiguousResult {
                            location: query.location,
                            residual: query.margin,
                            context: format!("No configured policy for {:?}", query.kind),
                        },
                        context: None,
                    });
                }
            };

        let decision =
            TracedDecision::new(decision_id, decision_kind, decision_tier, margin, context);
        self.decision_log.record(decision);

        let payload = PolicyDecisionTracePayload {
            decision_id,
            policy_kind: query.kind.clone(),
            operation_scope_id: None,
            query_location: query.location,
            measured_margin: margin,
            threshold,
            overridable: query.overridable,
            candidate_summary,
            outcome,
            source: resolved_source.source,
            source_scope: resolved_source.source_scope.clone(),
            default_used: resolved_source.default_used,
        };
        let adjunct = TraceAdjunctRecord::from_policy_payload(&payload);
        self.push_trace_adjunct(adjunct.clone());

        Ok(ResolvedPolicyDecision {
            accept_potential_value: accept,
            source: resolved_source,
            decision_id,
            adjunct,
        })
    }

    /// Verify that a policy kind has a configured resolution strategy
    /// (default, session override, model override, or operation override).
    /// Returns Ok(()) if any scope has a configuration for this kind.
    /// Returns Err(KernelError::InvalidConfig) if no scope covers it.
    ///
    /// This is a fail-fast pre-check — it does NOT resolve the policy,
    /// it only verifies that resolution won't hit ForcedSafeFallback
    /// due to total absence of configuration.
    pub fn validate_policy_configured(&self, kind: &PolicyKind) -> Result<(), KernelError> {
        // We use the same configuration snapshot logic as resolve_policy_source_for_query
        let config = if KernelSpan::is_active() {
            if let Some(snapshot) = KernelSpan::get_config_snapshot() {
                snapshot.config().clone()
            } else {
                self.config.clone()
            }
        } else {
            self.config.clone()
        };

        if config.policy.fallback_rules.contains_key(kind) {
            Ok(())
        } else {
            Err(KernelError::InvalidConfig {
                field: format!("policy.fallback_rules.{:?}", kind),
                reason: format!("No configured policy found for {:?}", kind),
            })
        }
    }
}
