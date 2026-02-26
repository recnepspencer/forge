//! Modeling context — the policy engine for kernel operations.
//!
//! DOMAIN: Main entry point for kernel policy decisions and tracing.
//! INVARIANTS: Every tolerance decision is logged (Doctrine D2).
//! DEPENDENCIES: `forge-core` (DecisionLog, TracedDecision), `tolerance` (policy structs)
//!
//! # Arena Snapshots
//!
//! `ArenaSnapshot` captures the slot counts of a `TopologyArena` at a point
//! in time. By comparing two snapshots, `compute_topology_delta` produces a
//! `TopologyDelta` listing every entity created between them.
//!
//! # Architecture (Doctrine D2)
//!
//! The `ModelingContext` carries all policy configuration and records every
//! tolerance-driven decision. When the kernel encounters ambiguity in curved
//! geometry (Phase 3+), it either:
//! - Applies the configured policy and logs a `TracedDecision`, or
//! - Returns `KernelError::PolicyRequired` for the caller to decide
//!
//! Silent heuristic decisions are forbidden. Every judgment call is traceable.

use std::collections::BTreeMap;

use forge_core::{
    TracedDecision, DecisionKind, DecisionContext, DecisionId, DecisionLog, DecisionTier,
    TopologyDelta, KernelError, PolicyKind, PolicyQuery,
};
use forge_core::errors::AmbiguousResult;
use forge_core::envelope::{KernelWarning, LineageDelta, OperationMetrics, OperationResult};
use forge_core::tracing::{
    CandidateValueSummary, PolicyDecisionTracePayload, PolicyResolutionOutcome,
    PolicyResolutionScopeRef, PolicyResolutionSource, TraceAdjunctRecord, TraceAdjunctSet,
};
use forge_topo::arena::TopologyArena;

use crate::operations::boolean::FaceClassification;

use crate::analysis::proof_validation::checkpoint::ValidationConfig;

use super::tolerance::{
    TolerancePolicy, TangencyPolicy, SliverPolicy,
    GapClosurePolicy, PrecisionEscalationPolicy, ToleranceConfig,
};

/// Aggregated metadata absorbed from sub-operation envelopes.
#[derive(Debug, Clone, Default)]
pub struct SubOperationMetadata {
    pub warnings: Vec<KernelWarning>,
    pub metrics: OperationMetrics,
    pub lineage_delta: LineageDelta,
    pub accumulated_error_budget: f64,
}

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
    value: bool,
    scope: Option<PolicyResolutionScopeRef>,
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

fn default_policy_registry() -> BTreeMap<PolicyKind, bool> {
    let mut defaults = BTreeMap::new();
    // Epic A/B current product semantics: weakly-simple coplanar region boundaries
    // are accepted by default, but the decision must be traced.
    defaults.insert(PolicyKind::CoincidentGeometry, true);
    defaults
}

/// The modeling context that governs all policy decisions.
///
/// Passed to operations that may encounter ambiguity. Records every
/// tolerance-driven decision for traceability (D2) and replay (D1).
///
/// # Example
/// ```
/// use forge_kernel::core::ModelingContext;
///
/// let ctx = ModelingContext::default();
/// assert_eq!(ctx.get_decision_count(), 0);
/// ```
#[derive(Debug, Clone)]
pub struct ModelingContext {
    tolerance: TolerancePolicy,
    tangency: TangencyPolicy,
    sliver: SliverPolicy,
    gap_closure: GapClosurePolicy,
    precision: PrecisionEscalationPolicy,
    tolerance_config: ToleranceConfig,
    validation_config: ValidationConfig,
    decision_log: DecisionLog,
    /// Aggregated warnings absorbed from sub-operations that returned envelopes.
    sub_warnings: Vec<KernelWarning>,
    /// Aggregated metrics absorbed from sub-operations.
    sub_metrics: OperationMetrics,
    /// Aggregated lineage deltas absorbed from sub-operations.
    sub_lineage_delta: LineageDelta,
    /// Aggregated error budget consumed by absorbed sub-operations.
    sub_accumulated_error_budget: f64,
    /// Typed adjunct payloads produced alongside traced decisions.
    trace_adjuncts: TraceAdjunctSet,
    decision_counter: u64,
    /// When true, Drop persists the DecisionLog as an error trace
    /// if take_decision_log() was never called (i.e. the operation failed).
    auto_persist: bool,
    /// Set by take_decision_log() to indicate the success path was taken.
    log_drained: bool,
    /// Forced classification overrides for counterfactual replay.
    ///
    /// Keyed by `DecisionId` raw value (face index). When the classify
    /// phase encounters a matching decision, it uses the forced
    /// `FaceClassification` instead of the computed result.
    classification_overrides: BTreeMap<u64, FaceClassification>,
    policy_defaults: BTreeMap<PolicyKind, bool>,
    policy_session_overrides: BTreeMap<PolicyKind, ScopedPolicyValue>,
    policy_model_overrides: BTreeMap<String, BTreeMap<PolicyKind, bool>>,
    policy_feature_overrides: BTreeMap<String, BTreeMap<PolicyKind, bool>>,
    policy_operation_overrides: BTreeMap<String, BTreeMap<PolicyKind, bool>>,
    active_model_policy_scope: Option<String>,
    active_feature_policy_scope: Option<String>,
    active_operation_policy_scope: Option<String>,
}

impl ModelingContext {
    /// Create a modeling context with default policies.
    pub fn new() -> Self {
        Self {
            tolerance: TolerancePolicy::default(),
            tangency: TangencyPolicy::default(),
            sliver: SliverPolicy::default(),
            gap_closure: GapClosurePolicy::default(),
            precision: PrecisionEscalationPolicy::default(),
            tolerance_config: ToleranceConfig::default(),
            validation_config: ValidationConfig::default(),
            decision_log: DecisionLog::new(),
            sub_warnings: Vec::new(),
            sub_metrics: OperationMetrics::default(),
            sub_lineage_delta: LineageDelta::default(),
            sub_accumulated_error_budget: 0.0,
            trace_adjuncts: TraceAdjunctSet::new(),
            decision_counter: 0,
            auto_persist: false,
            log_drained: false,
            classification_overrides: BTreeMap::new(),
            policy_defaults: default_policy_registry(),
            policy_session_overrides: BTreeMap::new(),
            policy_model_overrides: BTreeMap::new(),
            policy_feature_overrides: BTreeMap::new(),
            policy_operation_overrides: BTreeMap::new(),
            active_model_policy_scope: None,
            active_feature_policy_scope: None,
            active_operation_policy_scope: None,
        }
    }

    /// Enable auto-persist on Drop. Call this on contexts used by
    /// top-level operations (e.g. `execute_boolean`) so that error
    /// traces are captured when the operation fails.
    pub fn enable_auto_persist(&mut self) {
        self.auto_persist = true;
    }

    /// Get the spatial tolerance policy.
    pub fn get_tolerance(&self) -> &TolerancePolicy {
        &self.tolerance
    }

    /// Set the spatial tolerance policy.
    pub fn set_tolerance(&mut self, policy: TolerancePolicy) {
        self.tolerance = policy;
    }

    /// Get the tangency handling policy.
    pub fn get_tangency(&self) -> &TangencyPolicy {
        &self.tangency
    }

    /// Set the tangency handling policy.
    pub fn set_tangency(&mut self, policy: TangencyPolicy) {
        self.tangency = policy;
    }

    /// Get the sliver face policy.
    pub fn get_sliver(&self) -> &SliverPolicy {
        &self.sliver
    }

    /// Set the sliver face policy.
    pub fn set_sliver(&mut self, policy: SliverPolicy) {
        self.sliver = policy;
    }

    /// Get the gap closure policy.
    pub fn get_gap_closure(&self) -> &GapClosurePolicy {
        &self.gap_closure
    }

    /// Set the gap closure policy.
    pub fn set_gap_closure(&mut self, policy: GapClosurePolicy) {
        self.gap_closure = policy;
    }

    /// Get the precision escalation policy.
    pub fn get_precision(&self) -> &PrecisionEscalationPolicy {
        &self.precision
    }

    /// Set the precision escalation policy.
    pub fn set_precision(&mut self, policy: PrecisionEscalationPolicy) {
        self.precision = policy;
    }

    /// Get the geometry-layer tolerance configuration.
    pub fn get_tolerance_config(&self) -> &ToleranceConfig {
        &self.tolerance_config
    }

    /// Set the geometry-layer tolerance configuration.
    pub fn set_tolerance_config(&mut self, config: ToleranceConfig) {
        self.tolerance_config = config;
    }

    /// Get the validation checkpoint configuration.
    pub fn get_validation_config(&self) -> &ValidationConfig {
        &self.validation_config
    }

    /// Set the validation checkpoint configuration.
    pub fn set_validation_config(&mut self, config: ValidationConfig) {
        self.validation_config = config;
    }

    /// Record a tolerance decision, auto-assigning a unique ID.
    pub fn log_decision(
        &mut self,
        kind: DecisionKind,
        tier: DecisionTier,
        _location: [f64; 3],
        margin: f64,
        threshold: f64,
    ) {
        self.decision_counter += 1;
        let decision = TracedDecision::new(
            DecisionId(self.decision_counter),
            kind,
            tier,
            margin,
            DecisionContext::Tolerance { measured: margin, threshold },
        );
        self.decision_log.record(decision);
    }

    /// Record a precision escalation decision, auto-assigning a unique ID.
    pub fn log_escalation(
        &mut self,
        escalation: forge_math::arithmetic::precision::PrecisionEscalation,
    ) {
        if escalation.resolved_at > forge_math::arithmetic::precision::PrecisionMode::Float64 {
            self.decision_counter += 1;
            let decision = TracedDecision::new(
                DecisionId(self.decision_counter),
                DecisionKind::Exact,
                DecisionTier::Escalated,
                escalation.disagreement_magnitude.unwrap_or(0.0),
                DecisionContext::PrecisionEscalation { escalation },
            );
            self.decision_log.record(decision);
        }
    }

    /// Execute `f` within a named span, recording start/end events.
    pub fn scope<F, R>(&mut self, name: &'static str, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        let span_id = self.decision_log.start_span(name);
        let start = std::time::Instant::now();
        let result = f(self);
        let duration_micros = start.elapsed().as_micros() as u64;
        self.decision_log.end_span(span_id, duration_micros);
        result
    }

    /// Clear only the decision log events, preserving counters and sub-op sinks.
    ///
    /// This is a low-level utility for tests/debug tooling. It is NOT a full
    /// operation-boundary reset.
    pub fn clear_decision_log_only(&mut self) {
        self.decision_log = DecisionLog::new();
    }

    /// Reset per-operation trace/metadata state while preserving configuration.
    ///
    /// Resets:
    /// - decision log
    /// - decision ID counter (next decision restarts at 1)
    /// - success-path drain flag (`log_drained`)
    /// - absorbed sub-operation metadata sink
    ///
    /// Preserves:
    /// - policy/tolerance/validation configuration
    /// - auto-persist capability flag
    /// - classification overrides (counterfactual replay control)
    pub fn reset_for_new_operation(&mut self) {
        self.decision_log = DecisionLog::new();
        self.sub_warnings.clear();
        self.sub_metrics = OperationMetrics::default();
        self.sub_lineage_delta = LineageDelta::default();
        self.sub_accumulated_error_budget = 0.0;
        self.trace_adjuncts = TraceAdjunctSet::new();
        self.decision_counter = 0;
        self.log_drained = false;
        self.policy_operation_overrides.clear();
        self.active_operation_policy_scope = None;
    }

    /// Returns the number of tolerance decisions made.
    pub fn get_decision_count(&self) -> usize {
        self.decision_log.len()
    }

    /// Get the decision log.
    pub fn get_decision_log(&self) -> &DecisionLog {
        &self.decision_log
    }

    /// Get mutable access to the decision log.
    pub fn get_decision_log_mut(&mut self) -> &mut DecisionLog {
        &mut self.decision_log
    }

    /// Warnings absorbed from sub-operation envelopes.
    pub fn get_sub_warnings(&self) -> &[KernelWarning] {
        &self.sub_warnings
    }

    /// Aggregated metrics absorbed from sub-operation envelopes.
    pub fn get_sub_metrics(&self) -> &OperationMetrics {
        &self.sub_metrics
    }

    /// Aggregated lineage delta absorbed from sub-operation envelopes.
    pub fn get_sub_lineage_delta(&self) -> &LineageDelta {
        &self.sub_lineage_delta
    }

    /// Aggregated error budget absorbed from sub-operation envelopes.
    pub fn get_sub_accumulated_error_budget(&self) -> f64 {
        self.sub_accumulated_error_budget
    }

    /// Typed adjunct payloads accumulated alongside the decision log.
    pub fn get_trace_adjuncts(&self) -> &TraceAdjunctSet {
        &self.trace_adjuncts
    }

    /// Append one adjunct payload to the context's typed trace adjunct sink.
    pub fn push_trace_adjunct(&mut self, record: TraceAdjunctRecord) {
        self.trace_adjuncts.insert(record);
    }

    /// Drain typed adjunct payloads and reset the adjunct sink.
    pub fn take_trace_adjuncts(&mut self) -> TraceAdjunctSet {
        std::mem::take(&mut self.trace_adjuncts)
    }

    /// Drain all aggregated sub-operation metadata and reset the sink.
    pub fn take_sub_metadata(&mut self) -> SubOperationMetadata {
        SubOperationMetadata {
            warnings: std::mem::take(&mut self.sub_warnings),
            metrics: std::mem::take(&mut self.sub_metrics),
            lineage_delta: std::mem::take(&mut self.sub_lineage_delta),
            accumulated_error_budget: std::mem::take(&mut self.sub_accumulated_error_budget),
        }
    }

    /// Pull all metadata from an `OperationResult` sub-operation into this context.
    ///
    /// Use this when the current function returns a plain value/result (not an
    /// `OperationResult`) but must preserve the sub-operation's audit trail.
    ///
    /// This is a true drain: absorbed metadata is removed from `op` to prevent
    /// accidental double-counting if the caller reuses the child envelope.
    pub fn absorb_sub_result<U>(&mut self, op: &mut OperationResult<U>) {
        self.decision_log.merge(op.take_decision_log());
        self.sub_warnings.extend(op.take_warnings());

        let metrics = op.take_metrics();
        self.sub_metrics.duration += metrics.duration;
        self.sub_metrics.entities_created += metrics.entities_created;
        self.sub_metrics.entities_deleted += metrics.entities_deleted;
        self.sub_metrics.entities_modified += metrics.entities_modified;
        self.sub_metrics.exact_predicate_calls += metrics.exact_predicate_calls;
        self.sub_metrics.policy_decisions_made += metrics.policy_decisions_made;

        let lineage = op.take_lineage_delta();
        self.sub_lineage_delta.faces_created += lineage.faces_created;
        self.sub_lineage_delta.faces_deleted += lineage.faces_deleted;
        self.sub_lineage_delta.half_edges_created += lineage.half_edges_created;
        self.sub_lineage_delta.half_edges_deleted += lineage.half_edges_deleted;
        self.sub_lineage_delta.vertices_created += lineage.vertices_created;
        self.sub_lineage_delta.vertices_deleted += lineage.vertices_deleted;
        self.sub_lineage_delta.loops_created += lineage.loops_created;
        self.sub_lineage_delta.loops_deleted += lineage.loops_deleted;
        self.sub_lineage_delta.edges_created += lineage.edges_created;
        self.sub_lineage_delta.edges_deleted += lineage.edges_deleted;
        self.sub_lineage_delta.shells_created += lineage.shells_created;
        self.sub_lineage_delta.shells_deleted += lineage.shells_deleted;
        self.sub_lineage_delta.solids_created += lineage.solids_created;
        self.sub_lineage_delta.solids_deleted += lineage.solids_deleted;

        self.sub_accumulated_error_budget += op.take_accumulated_budget();
    }

    /// Take ownership of the decision log, replacing it with an empty one.
    ///
    /// Marks the log as drained so `Drop` knows the success path was taken.
    pub fn take_decision_log(&mut self) -> DecisionLog {
        self.log_drained = true;
        std::mem::take(&mut self.decision_log)
    }

    /// Set a forced classification override for counterfactual replay.
    ///
    /// When the classify phase encounters a decision with this ID,
    /// it uses the forced `FaceClassification` instead of computing
    /// the result from ray-casting. This enables re-executing the
    /// Boolean pipeline with different classification outcomes.
    pub fn set_classification_override(
        &mut self,
        decision_id: DecisionId,
        classification: FaceClassification,
    ) {
        self.classification_overrides.insert(decision_id.0, classification);
    }

    /// Check if a classification override exists for a decision ID.
    ///
    /// Returns the forced `FaceClassification` if one was set via
    /// `set_classification_override`, or `None` for normal execution.
    pub fn get_classification_override(
        &self,
        decision_id: DecisionId,
    ) -> Option<FaceClassification> {
        self.classification_overrides.get(&decision_id.0).copied()
    }

    /// Remove all classification overrides.
    pub fn clear_classification_overrides(&mut self) {
        self.classification_overrides.clear();
    }

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

    /// Generate a divergence report from the current decision log (P2.3).
    ///
    /// Scans all decisions for cases where the f64 fast-path disagreed
    /// with the higher-precision answer. Returns a structured report
    /// with divergence rate, topology impact, and per-decision details.
    pub fn generate_divergence_report(&self) -> forge_core::DivergenceReport {
        forge_core::scan_for_divergences(&self.decision_log)
    }

    /// Check whether the accumulated error budget has been exceeded.
    ///
    /// Returns `Some(KernelWarning::ErrorBudgetExceeded)` when `accumulated`
    /// exceeds `ToleranceConfig::error_budget_mm`. Call this after each
    /// boolean pipeline phase and push the warning into the `OperationResult`
    /// envelope via `add_warning`. Set `error_budget_mm = f64::INFINITY` to
    /// disable budget checks entirely (the default).
    pub fn check_budget(&self, accumulated: f64) -> Option<forge_core::KernelWarning> {
        let threshold = self.tolerance_config.get_error_budget_mm();
        if accumulated > threshold {
            Some(forge_core::KernelWarning::ErrorBudgetExceeded {
                accumulated_mm: accumulated,
                threshold_mm: threshold,
            })
        } else {
            None
        }
    }
}

// ── Arena Snapshot for topology delta capture ──────────────────────────────

/// A lightweight snapshot of arena slot counts at a point in time.
///
/// Used to compute `TopologyDelta` — the set of entities created
/// between two snapshots (pre-op and post-op).
#[derive(Debug, Clone)]
pub struct ArenaSnapshot {
    face_slots: usize,
    half_edge_slots: usize,
    vertex_slots: usize,
}

impl ArenaSnapshot {
    /// Capture the current slot counts of an arena.
    pub fn capture(arena: &TopologyArena) -> Self {
        Self {
            face_slots: arena.face_slot_count(),
            half_edge_slots: arena.half_edge_slot_count(),
            vertex_slots: arena.vertex_slot_count(),
        }
    }
}

/// Compute the topology delta between a pre-operation snapshot and the
/// current arena state.
///
/// Any slot indices in `[snapshot.X_slots .. arena.X_slot_count())` are
/// entities created since the snapshot was taken.
pub fn compute_topology_delta(snapshot: &ArenaSnapshot, arena: &TopologyArena) -> TopologyDelta {
    let created_faces: Vec<u32> = (snapshot.face_slots..arena.face_slot_count())
        .filter(|&i| arena.face_generation(i).is_some())
        .map(|i| i as u32)
        .collect();

    let created_halfedges: Vec<u32> = (snapshot.half_edge_slots..arena.half_edge_slot_count())
        .filter(|&i| arena.half_edge_generation(i).is_some())
        .map(|i| i as u32)
        .collect();

    let created_vertices: Vec<u32> = (snapshot.vertex_slots..arena.vertex_slot_count())
        .filter(|&i| arena.vertex_generation(i).is_some())
        .map(|i| i as u32)
        .collect();

    TopologyDelta {
        created_faces,
        created_halfedges,
        created_vertices,
        deleted_faces: Vec::new(),
        deleted_halfedges: Vec::new(),
        deleted_vertices: Vec::new(),
    }
}

impl Drop for ModelingContext {
    /// Auto-persist the DecisionLog on error or panic.
    ///
    /// Only fires when `auto_persist` is enabled (top-level operations)
    /// AND `take_decision_log()` was never called (the error path).
    fn drop(&mut self) {
        if !self.auto_persist || self.log_drained || self.decision_log.is_empty() {
            return;
        }

        let dir = match forge_core::resolve_trace_dir() {
            Some(d) => d,
            None => return,
        };

        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            forge_core::write_trace_file(&dir, &self.decision_log, 0, "error");
        }));
    }
}

impl Default for ModelingContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::check_tolerance;
    use forge_core::envelope::OperationResult;
    use forge_core::tracing::{CandidateValueSummary, PolicyResolutionOutcome, PolicyResolutionSource};

    fn sample_policy_query(kind: PolicyKind, overridable: bool) -> PolicyQuery {
        PolicyQuery {
            kind,
            location: [1.0, 2.0, 3.0],
            margin: 2.5e-6,
            overridable,
        }
    }

    fn sample_candidate_summary() -> CandidateValueSummary {
        CandidateValueSummary::EnumTag {
            type_name: "WeakSimpleCertificate".into(),
            variant: "WeaklySimple".into(),
        }
    }

    #[test]
    fn default_context_has_no_decisions() {
        let ctx = ModelingContext::new();
        assert_eq!(ctx.get_decision_count(), 0);
    }

    #[test]
    fn decisions_are_recorded() {
        let mut ctx = ModelingContext::new();
        ctx.log_decision(
            DecisionKind::NearBoundary { threshold: 1e-6 },
            DecisionTier::NearBoundary,
            [1.0, 2.0, 3.0],
            1e-8,
            1e-6,
        );
        assert_eq!(ctx.get_decision_count(), 1);
        let decisions: Vec<_> = ctx.get_decision_log().decisions().collect();
        assert_eq!(decisions[0].get_id(), DecisionId(1));
    }

    #[test]
    fn check_tolerance_macro_logs_when_within() {
        let mut ctx = ModelingContext::new();
        let distance = 1e-8;
        let threshold = 1e-6;
        let location = [0.0, 0.0, 0.0];

        let within = check_tolerance!(ctx, threshold, distance, location, DecisionKind::NearBoundary { threshold });

        assert!(within);
        assert_eq!(ctx.get_decision_count(), 1);
    }

    #[test]
    fn check_tolerance_macro_does_not_log_when_outside() {
        let mut ctx = ModelingContext::new();
        let distance = 1e-3;
        let threshold = 1e-6;
        let location = [0.0, 0.0, 0.0];

        let within = check_tolerance!(ctx, threshold, distance, location, DecisionKind::NearBoundary { threshold });

        assert!(!within);
        assert_eq!(ctx.get_decision_count(), 0);
    }

    #[test]
    fn take_decision_log_drains() {
        let mut ctx = ModelingContext::new();
        ctx.log_decision(
            DecisionKind::Exact,
            DecisionTier::Deterministic,
            [0.0, 0.0, 0.0],
            0.0,
            1e-6,
        );
        assert_eq!(ctx.get_decision_count(), 1);

        let log = ctx.take_decision_log();
        assert_eq!(log.len(), 1);
        assert_eq!(ctx.get_decision_count(), 0);
    }

    #[test]
    fn reset_for_new_operation_restarts_decision_ids_at_one() {
        let mut ctx = ModelingContext::new();

        ctx.log_decision(
            DecisionKind::Exact,
            DecisionTier::Deterministic,
            [0.0, 0.0, 0.0],
            0.0,
            1.0,
        );
        let _ = ctx.take_decision_log();

        ctx.reset_for_new_operation();
        ctx.log_decision(
            DecisionKind::Exact,
            DecisionTier::Deterministic,
            [0.0, 0.0, 0.0],
            0.0,
            1.0,
        );

        let ids: Vec<_> = ctx.get_decision_log().decisions().map(|d| d.get_id().0).collect();
        assert_eq!(ids, vec![1], "full operation reset must restore deterministic ID sequence");
    }

    #[test]
    fn reset_for_new_operation_clears_log_drained_and_sub_metadata_sink() {
        let mut ctx = ModelingContext::new();
        ctx.enable_auto_persist();

        ctx.log_decision(
            DecisionKind::Exact,
            DecisionTier::Deterministic,
            [0.0, 0.0, 0.0],
            0.0,
            1.0,
        );
        let _ = ctx.take_decision_log();
        assert!(ctx.log_drained, "take_decision_log sets success-path drain flag");

        let mut sub = OperationResult::new(());
        sub.add_warning(forge_core::KernelWarning::AutoDecision { decision_id: DecisionId(99) });
        let mut metrics = forge_core::envelope::OperationMetrics::default();
        metrics.entities_deleted = 4;
        sub.set_metrics(metrics);
        ctx.absorb_sub_result(&mut sub);
        assert_eq!(ctx.get_sub_warnings().len(), 1);
        assert_eq!(ctx.get_sub_metrics().entities_deleted, 4);

        ctx.reset_for_new_operation();

        assert!(!ctx.log_drained, "new operation must not inherit prior success-path drain state");
        assert_eq!(ctx.get_decision_count(), 0);
        assert!(ctx.get_sub_warnings().is_empty());
        assert_eq!(ctx.get_sub_metrics().entities_deleted, 0);
        assert_eq!(ctx.get_sub_accumulated_error_budget(), 0.0);
        assert_eq!(ctx.get_tolerance_config().get_error_budget_mm(), f64::INFINITY);
    }

    #[test]
    fn clear_decision_log_only_preserves_counter_and_sub_metadata() {
        let mut ctx = ModelingContext::new();
        ctx.log_decision(
            DecisionKind::Exact,
            DecisionTier::Deterministic,
            [0.0, 0.0, 0.0],
            0.0,
            1.0,
        );
        let mut sub = OperationResult::new(());
        let mut metrics = forge_core::envelope::OperationMetrics::default();
        metrics.entities_created = 7;
        sub.set_metrics(metrics);
        ctx.absorb_sub_result(&mut sub);

        ctx.clear_decision_log_only();
        assert_eq!(ctx.get_decision_count(), 0);
        assert_eq!(ctx.get_sub_metrics().entities_created, 7, "log-only clear must not wipe metadata sink");

        ctx.log_decision(
            DecisionKind::Exact,
            DecisionTier::Deterministic,
            [0.0, 0.0, 0.0],
            0.0,
            1.0,
        );
        let ids: Vec<_> = ctx.get_decision_log().decisions().map(|d| d.get_id().0).collect();
        assert_eq!(ids, vec![2], "log-only clear preserves monotonically increasing IDs");
    }

    #[test]
    fn absorb_sub_result_accumulates_full_metadata() {
        let mut ctx = ModelingContext::new();
        let mut sub = OperationResult::new(());

        sub.get_decision_log_mut().record(TracedDecision::new(
            DecisionId(42),
            DecisionKind::Exact,
            DecisionTier::Deterministic,
            1.0,
            DecisionContext::Degeneracy { description: "sub".into() },
        ));
        sub.add_warning(forge_core::KernelWarning::AutoDecision { decision_id: DecisionId(42) });

        let mut metrics = forge_core::envelope::OperationMetrics::default();
        metrics.entities_modified = 5;
        metrics.policy_decisions_made = 1;
        sub.set_metrics(metrics);

        let mut lineage = forge_core::envelope::LineageDelta::default();
        lineage.faces_deleted = 2;
        sub.set_lineage_delta(lineage);
        sub.consume_budget(2e-6);

        ctx.absorb_sub_result(&mut sub);

        assert_eq!(ctx.get_decision_count(), 1);
        assert_eq!(ctx.get_sub_warnings().len(), 1);
        assert_eq!(ctx.get_sub_metrics().entities_modified, 5);
        assert_eq!(ctx.get_sub_metrics().policy_decisions_made, 1);
        assert_eq!(ctx.get_sub_lineage_delta().faces_deleted, 2);
        assert!(ctx.get_sub_accumulated_error_budget() > 0.0);
        assert!(sub.get_decision_log().is_empty());
        assert!(sub.get_warnings().is_empty());
        assert_eq!(sub.get_metrics().entities_modified, 0);
        assert_eq!(sub.get_metrics().policy_decisions_made, 0);
        assert_eq!(sub.get_lineage_delta().faces_deleted, 0);
        assert_eq!(sub.get_accumulated_budget(), 0.0);
    }

    #[test]
    fn absorb_sub_result_is_idempotent_for_child_envelope_metadata() {
        let mut ctx = ModelingContext::new();
        let mut sub = OperationResult::new(());

        sub.add_warning(forge_core::KernelWarning::AutoDecision { decision_id: DecisionId(11) });
        let mut metrics = forge_core::envelope::OperationMetrics::default();
        metrics.entities_created = 2;
        metrics.policy_decisions_made = 1;
        sub.set_metrics(metrics);
        let mut lineage = forge_core::envelope::LineageDelta::default();
        lineage.edges_deleted = 3;
        sub.set_lineage_delta(lineage);
        sub.consume_budget(9.0e-7);

        ctx.absorb_sub_result(&mut sub);
        ctx.absorb_sub_result(&mut sub);

        assert_eq!(ctx.get_decision_count(), 0, "no decisions were added in this fixture");
        assert_eq!(ctx.get_sub_warnings().len(), 1, "warnings must not double-count");
        assert_eq!(ctx.get_sub_metrics().entities_created, 2, "metrics must drain from child");
        assert_eq!(ctx.get_sub_metrics().policy_decisions_made, 1);
        assert_eq!(ctx.get_sub_lineage_delta().edges_deleted, 3, "lineage must drain from child");
        assert!((ctx.get_sub_accumulated_error_budget() - 9.0e-7).abs() < f64::EPSILON);
    }

    #[test]
    fn absorb_sub_result_accumulates_then_take_sub_metadata_drains() {
        let mut ctx = ModelingContext::new();
        let mut sub = OperationResult::new(());

        sub.add_warning(forge_core::KernelWarning::AutoDecision { decision_id: DecisionId(7) });
        let mut metrics = forge_core::envelope::OperationMetrics::default();
        metrics.entities_created = 3;
        metrics.policy_decisions_made = 2;
        sub.set_metrics(metrics);

        let mut lineage = forge_core::envelope::LineageDelta::default();
        lineage.vertices_created = 4;
        sub.set_lineage_delta(lineage);
        sub.consume_budget(1.25e-6);

        ctx.absorb_sub_result(&mut sub);
        let drained = ctx.take_sub_metadata();

        assert_eq!(drained.warnings.len(), 1);
        assert_eq!(drained.metrics.entities_created, 3);
        assert_eq!(drained.metrics.policy_decisions_made, 2);
        assert_eq!(drained.lineage_delta.vertices_created, 4);
        assert!((drained.accumulated_error_budget - 1.25e-6).abs() < f64::EPSILON);

        assert!(ctx.get_sub_warnings().is_empty());
        assert_eq!(ctx.get_sub_metrics().entities_created, 0);
        assert_eq!(ctx.get_sub_lineage_delta().vertices_created, 0);
        assert_eq!(ctx.get_sub_accumulated_error_budget(), 0.0);
    }

    #[test]
    fn take_sub_metadata_is_idempotent_after_reset() {
        let mut ctx = ModelingContext::new();

        let first = ctx.take_sub_metadata();
        assert!(first.warnings.is_empty());
        assert_eq!(first.metrics.entities_created, 0);
        assert_eq!(first.accumulated_error_budget, 0.0);

        let second = ctx.take_sub_metadata();
        assert!(second.warnings.is_empty());
        assert_eq!(second.metrics.entities_created, 0);
        assert_eq!(second.accumulated_error_budget, 0.0);
    }

    #[test]
    fn policy_registry_precedence_resolves_deterministically_across_all_layers() {
        let mut ctx = ModelingContext::new();
        let query = sample_policy_query(PolicyKind::CoincidentGeometry, true);

        ctx.set_session_policy_override(PolicyKind::CoincidentGeometry, false, Some("user-1".into()));
        ctx.set_active_model_policy_scope(Some("model.merge".into()));
        ctx.set_model_policy_override("model.merge", PolicyKind::CoincidentGeometry, true);
        ctx.set_active_feature_policy_scope(Some("feature-7".into()));
        ctx.set_feature_policy_override("feature-7", PolicyKind::CoincidentGeometry, false);
        ctx.set_active_operation_policy_scope(Some("sheet_region_merge".into()));
        ctx.set_operation_policy_override("sheet_region_merge", PolicyKind::CoincidentGeometry, true);

        let d1 = ctx.resolve_policy_query(
            DecisionId(1001),
            &query,
            Some(1.0e-5),
            sample_candidate_summary(),
        ).expect("op override must resolve");
        assert!(d1.accept_potential_value);
        assert_eq!(d1.source.source, PolicyResolutionSource::OperationOverride);

        ctx.policy_operation_overrides.clear();
        let d2 = ctx.resolve_policy_query(
            DecisionId(1002),
            &query,
            Some(1.0e-5),
            sample_candidate_summary(),
        ).expect("feature override must resolve");
        assert!(!d2.accept_potential_value);
        assert_eq!(d2.source.source, PolicyResolutionSource::FeatureOverride);

        ctx.set_active_feature_policy_scope(None);
        let d3 = ctx.resolve_policy_query(
            DecisionId(1003),
            &query,
            Some(1.0e-5),
            sample_candidate_summary(),
        ).expect("model override must resolve");
        assert!(d3.accept_potential_value);
        assert_eq!(d3.source.source, PolicyResolutionSource::ModelSpecOverride);

        ctx.set_active_model_policy_scope(None);
        let d4 = ctx.resolve_policy_query(
            DecisionId(1004),
            &query,
            Some(1.0e-5),
            sample_candidate_summary(),
        ).expect("session override must resolve");
        assert!(!d4.accept_potential_value);
        assert_eq!(d4.source.source, PolicyResolutionSource::SessionUserOverride);

        ctx.clear_session_policy_override(PolicyKind::CoincidentGeometry);
        let d5 = ctx.resolve_policy_query(
            DecisionId(1005),
            &query,
            Some(1.0e-5),
            sample_candidate_summary(),
        ).expect("default policy must resolve");
        assert!(d5.accept_potential_value);
        assert_eq!(d5.source.source, PolicyResolutionSource::DefaultPolicy);
    }

    #[test]
    fn policy_operation_override_does_not_leak_across_reset() {
        let mut ctx = ModelingContext::new();
        let query = sample_policy_query(PolicyKind::CoincidentGeometry, true);

        ctx.set_active_operation_policy_scope(Some("op-A".into()));
        ctx.set_operation_policy_override("op-A", PolicyKind::CoincidentGeometry, false);
        let before_reset = ctx.resolve_policy_query(
            DecisionId(1101),
            &query,
            Some(1.0e-5),
            sample_candidate_summary(),
        ).expect("per-op override resolves before reset");
        assert!(!before_reset.accept_potential_value);
        assert_eq!(before_reset.source.source, PolicyResolutionSource::OperationOverride);

        ctx.reset_for_new_operation();
        let after_reset = ctx.resolve_policy_query(
            DecisionId(1102),
            &query,
            Some(1.0e-5),
            sample_candidate_summary(),
        ).expect("reset should fall back to default policy");
        assert!(after_reset.accept_potential_value);
        assert_eq!(after_reset.source.source, PolicyResolutionSource::DefaultPolicy);
        assert!(ctx.active_operation_policy_scope.is_none(), "reset must clear active op scope");
    }

    #[test]
    fn policy_missing_fails_closed_and_emits_typed_adjunct() {
        let mut ctx = ModelingContext::new();
        let query = sample_policy_query(PolicyKind::NearTangency, true);

        let err = ctx.resolve_policy_query(
            DecisionId(1201),
            &query,
            Some(5.0e-5),
            sample_candidate_summary(),
        ).expect_err("missing policy must fail closed");
        assert!(matches!(err, KernelError::AmbiguousResult { .. }));

        let decisions: Vec<_> = ctx.get_decision_log().decisions().collect();
        assert_eq!(decisions.len(), 1, "missing policy path must still emit a traced decision");
        assert!(matches!(decisions[0].get_kind(), DecisionKind::Ambiguous { .. }));

        let adjuncts = ctx.get_trace_adjuncts();
        assert_eq!(adjuncts.records().len(), 1, "missing policy path must emit typed policy adjunct");
        let payload = adjuncts.records()[0]
            .as_policy_payload()
            .expect("policy adjunct kind")
            .expect("decode policy payload");
        assert_eq!(payload.outcome, PolicyResolutionOutcome::EscalatedError);
        assert_eq!(payload.source, PolicyResolutionSource::ForcedSafeFallback);
        assert!(payload.source_scope.is_none());
        assert_eq!(payload.decision_id, DecisionId(1201));
    }

    #[test]
    fn sub_metadata_can_be_folded_into_operation_result_without_double_counting() {
        let mut ctx = ModelingContext::new();
        let mut sub = OperationResult::new(());

        let mut metrics = forge_core::envelope::OperationMetrics::default();
        metrics.entities_modified = 8;
        sub.set_metrics(metrics);

        let mut lineage = forge_core::envelope::LineageDelta::default();
        lineage.faces_created = 2;
        sub.set_lineage_delta(lineage);
        sub.consume_budget(3.0e-6);

        ctx.absorb_sub_result(&mut sub);
        let drained = ctx.take_sub_metadata();

        let mut parent = OperationResult::new("ok");
        for warning in drained.warnings {
            parent.add_warning(warning);
        }
        parent.set_metrics(drained.metrics.clone());
        parent.set_lineage_delta(drained.lineage_delta.clone());
        parent.consume_budget(drained.accumulated_error_budget);

        assert_eq!(parent.get_metrics().entities_modified, 8);
        assert_eq!(parent.get_lineage_delta().faces_created, 2);
        assert!((parent.get_accumulated_budget() - 3.0e-6).abs() < f64::EPSILON);

        let drained_again = ctx.take_sub_metadata();
        assert_eq!(drained_again.metrics.entities_modified, 0);
        assert_eq!(drained_again.lineage_delta.faces_created, 0);
        assert_eq!(drained_again.accumulated_error_budget, 0.0);
    }
}
