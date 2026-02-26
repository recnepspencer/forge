//! Policy resolution — the cascade engine for ambiguous queries.
//!
//! DOMAIN: Resolves policy queries against the 5-level override cascade
//! (defaults → session → model → feature → operation).
//! INVARIANTS: Missing policies fail closed (ForcedSafeFallback).

use std::collections::BTreeMap;

use forge_core::{
    TracedDecision, DecisionKind, DecisionContext, DecisionId, DecisionTier,
    KernelError, PolicyKind, PolicyQuery,
};
use forge_core::errors::AmbiguousResult;
use forge_core::tracing::{
    CandidateValueSummary, PolicyDecisionTracePayload, PolicyResolutionOutcome,
    PolicyResolutionScopeRef, PolicyResolutionSource, TraceAdjunctRecord,
};

use super::schema::ModelingContext;

/// Value source metadata chosen by policy precedence resolution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedPolicySource {
    pub source: PolicyResolutionSource,
    pub source_scope: Option<PolicyResolutionScopeRef>,
    pub default_used: bool,
}

/// Resolved policy decision for an ambiguous query.
#[derive(Debug, Clone)]
pub struct ResolvedPolicyDecision {
    pub accept_potential_value: bool,
    pub source: ResolvedPolicySource,
    pub decision_id: DecisionId,
    pub adjunct: TraceAdjunctRecord,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopedPolicyValue {
    pub(crate) value: bool,
    pub(crate) scope: Option<PolicyResolutionScopeRef>,
}

/// Immutable snapshot of policy registry layers visible to an operation.
#[derive(Debug, Clone)]
pub struct PolicyRegistrySnapshot {
    pub defaults: BTreeMap<PolicyKind, bool>,
    pub session_overrides: BTreeMap<PolicyKind, ScopedPolicyValue>,
    pub model_overrides: BTreeMap<String, BTreeMap<PolicyKind, bool>>,
    pub feature_overrides: BTreeMap<String, BTreeMap<PolicyKind, bool>>,
    pub operation_overrides: BTreeMap<String, BTreeMap<PolicyKind, bool>>,
    pub active_model_scope: Option<String>,
    pub active_feature_scope: Option<String>,
    pub active_operation_scope: Option<String>,
}

pub(crate) fn default_policy_registry() -> BTreeMap<PolicyKind, bool> {
    let mut defaults = BTreeMap::new();
    // Epic A/B current product semantics: weakly-simple coplanar region boundaries
    // are accepted by default, but the decision must be traced.
    defaults.insert(PolicyKind::CoincidentGeometry, true);
    defaults
}

impl PolicyRegistrySnapshot {
    /// Check if any policy scope has a configured rule for the given kind.
    ///
    /// Returns `true` if defaults, session, model, feature, or operation
    /// scopes contain a rule — meaning `resolve_policy_query` will NOT
    /// fall through to `ForcedSafeFallback`.
    pub fn has_any_rule_for(&self, kind: &PolicyKind) -> bool {
        self.defaults.contains_key(kind)
            || self.session_overrides.contains_key(kind)
            || self.active_model_scope.as_ref()
                .and_then(|s| self.model_overrides.get(s))
                .is_some_and(|m| m.contains_key(kind))
            || self.active_feature_scope.as_ref()
                .and_then(|s| self.feature_overrides.get(s))
                .is_some_and(|m| m.contains_key(kind))
            || self.active_operation_scope.as_ref()
                .and_then(|s| self.operation_overrides.get(s))
                .is_some_and(|m| m.contains_key(kind))
    }
}

impl ModelingContext {
    /// Get an immutable snapshot of the policy registry visible to this context.
    pub fn policy_registry_snapshot(&self) -> PolicyRegistrySnapshot {
        PolicyRegistrySnapshot {
            defaults: self.policy_defaults.clone(),
            session_overrides: self.policy_session_overrides.clone(),
            model_overrides: self.policy_model_overrides.clone(),
            feature_overrides: self.policy_feature_overrides.clone(),
            operation_overrides: self.policy_operation_overrides.clone(),
            active_model_scope: self.active_model_policy_scope.clone(),
            active_feature_scope: self.active_feature_policy_scope.clone(),
            active_operation_scope: self.active_operation_policy_scope.clone(),
        }
    }

    /// Verify that a policy kind has a configured resolution strategy.
    ///
    /// Returns `Ok(())` if any scope has a configuration for this kind.
    /// Returns `Err` if no scope covers it — a fail-fast pre-check so the
    /// pipeline rejects misconfigured features before execution starts.
    pub fn validate_policy_configured(&self, kind: &PolicyKind) -> Result<(), KernelError> {
        let snapshot = self.policy_registry_snapshot();
        if snapshot.has_any_rule_for(kind) {
            Ok(())
        } else {
            Err(KernelError::InvalidInput {
                message: format!(
                    "Policy {:?} is not configured in any scope (default/session/model/feature/operation)",
                    kind
                ),
                context: None,
            })
        }
    }

    pub fn set_policy_default(&mut self, kind: PolicyKind, accept_potential_value: bool) {
        self.policy_defaults.insert(kind, accept_potential_value);
    }

    pub fn clear_policy_default(&mut self, kind: PolicyKind) {
        self.policy_defaults.remove(&kind);
    }

    pub fn set_session_policy_override(
        &mut self,
        kind: PolicyKind,
        accept_potential_value: bool,
        scope_id: Option<String>,
    ) {
        self.policy_session_overrides.insert(
            kind,
            ScopedPolicyValue {
                value: accept_potential_value,
                scope: Some(PolicyResolutionScopeRef::SessionUser { scope_id }),
            },
        );
    }

    pub fn clear_session_policy_override(&mut self, kind: PolicyKind) {
        self.policy_session_overrides.remove(&kind);
    }

    pub fn set_active_model_policy_scope(&mut self, scope: Option<String>) {
        self.active_model_policy_scope = scope;
    }

    pub fn set_active_feature_policy_scope(&mut self, scope: Option<String>) {
        self.active_feature_policy_scope = scope;
    }

    pub fn set_active_operation_policy_scope(&mut self, scope: Option<String>) {
        self.active_operation_policy_scope = scope;
    }

    pub fn get_active_operation_policy_scope(&self) -> Option<&str> {
        self.active_operation_policy_scope.as_deref()
    }

    pub fn set_model_policy_override(
        &mut self,
        model_policy_key: impl Into<String>,
        kind: PolicyKind,
        accept_potential_value: bool,
    ) {
        self.policy_model_overrides
            .entry(model_policy_key.into())
            .or_default()
            .insert(kind, accept_potential_value);
    }

    pub fn set_feature_policy_override(
        &mut self,
        feature_id: impl Into<String>,
        kind: PolicyKind,
        accept_potential_value: bool,
    ) {
        self.policy_feature_overrides
            .entry(feature_id.into())
            .or_default()
            .insert(kind, accept_potential_value);
    }

    pub fn set_operation_policy_override(
        &mut self,
        operation_id: impl Into<String>,
        kind: PolicyKind,
        accept_potential_value: bool,
    ) {
        self.policy_operation_overrides
            .entry(operation_id.into())
            .or_default()
            .insert(kind, accept_potential_value);
    }

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

        if let Some(op_id) = self.active_operation_policy_scope.as_ref() {
            if let Some(value) = self
                .policy_operation_overrides
                .get(op_id)
                .and_then(|m| m.get(&query.kind))
            {
                return Some((
                    *value,
                    ResolvedPolicySource {
                        source: PolicyResolutionSource::OperationOverride,
                        source_scope: Some(PolicyResolutionScopeRef::Operation {
                            operation_id: op_id.clone(),
                        }),
                        default_used: false,
                    },
                ));
            }
        }

        if let Some(feature_id) = self.active_feature_policy_scope.as_ref() {
            if let Some(value) = self
                .policy_feature_overrides
                .get(feature_id)
                .and_then(|m| m.get(&query.kind))
            {
                return Some((
                    *value,
                    ResolvedPolicySource {
                        source: PolicyResolutionSource::FeatureOverride,
                        source_scope: Some(PolicyResolutionScopeRef::Feature {
                            feature_id: feature_id.clone(),
                        }),
                        default_used: false,
                    },
                ));
            }
        }

        if let Some(model_key) = self.active_model_policy_scope.as_ref() {
            if let Some(value) = self
                .policy_model_overrides
                .get(model_key)
                .and_then(|m| m.get(&query.kind))
            {
                return Some((
                    *value,
                    ResolvedPolicySource {
                        source: PolicyResolutionSource::ModelSpecOverride,
                        source_scope: Some(PolicyResolutionScopeRef::ModelSpec {
                            policy_key: model_key.clone(),
                        }),
                        default_used: false,
                    },
                ));
            }
        }

        if let Some(scoped) = self.policy_session_overrides.get(&query.kind) {
            return Some((
                scoped.value,
                ResolvedPolicySource {
                    source: PolicyResolutionSource::SessionUserOverride,
                    source_scope: scoped.scope.clone(),
                    default_used: false,
                },
            ));
        }

        if let Some(value) = self.policy_defaults.get(&query.kind) {
            return Some((
                *value,
                ResolvedPolicySource {
                    source: PolicyResolutionSource::DefaultPolicy,
                    source_scope: None,
                    default_used: true,
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
                        operation_scope_id: self.active_operation_policy_scope.clone(),
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

        let decision = TracedDecision::new(
            decision_id,
            decision_kind,
            decision_tier,
            margin,
            context,
        );
        self.decision_log.record(decision);

        let payload = PolicyDecisionTracePayload {
            decision_id,
            policy_kind: query.kind.clone(),
            operation_scope_id: self.active_operation_policy_scope.clone(),
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
}
