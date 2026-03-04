//! Decision logging, span management, and DecisionSink impl.
//!
//! DOMAIN: Recording tolerance decisions, precision escalations, and named spans.
//! INVARIANTS: Every decision gets a monotonically increasing ID.
//! All decisions flow through self.decision_log — no ambient state.

use forge_core::envelope::OperationMetrics;
use forge_core::tracing::sink::DecisionSink;
use forge_core::{
    DecisionContext, DecisionId, DecisionKind, DecisionLog, DecisionTier, TracedDecision,
};

use crate::context::state::ModelingContext;

// ── DecisionSink implementation ─────────────────────────────────────────────

impl DecisionSink for ModelingContext {
    fn record_tolerance_snap(
        &mut self,
        entity_index: u32,
        gap: f64,
        threshold: f64,
        tier: DecisionTier,
    ) {
        self.decision_counter += 1;
        let mut decision = TracedDecision::new(
            DecisionId(self.decision_counter),
            DecisionKind::PolicyApplied {
                policy: forge_core::PolicyKind::CoincidentGeometry,
                default_used: true,
            },
            tier,
            gap,
            DecisionContext::Tolerance { measured: gap, threshold },
        );
        decision.set_entity_scope(forge_core::EntityRef::new(
            forge_core::EntityKind::Vertex,
            entity_index,
            0,
        ));
        self.decision_log.record(decision);
    }

    fn record_near_boundary(
        &mut self,
        entity_index: u32,
        margin: f64,
        threshold: f64,
    ) {
        self.decision_counter += 1;
        let mut decision = TracedDecision::new(
            DecisionId(self.decision_counter),
            DecisionKind::NearBoundary { threshold },
            DecisionTier::NearBoundary,
            margin,
            DecisionContext::Tolerance { measured: margin, threshold },
        );
        decision.set_entity_scope(forge_core::EntityRef::new(
            forge_core::EntityKind::Vertex,
            entity_index,
            0,
        ));
        self.decision_log.record(decision);
    }

    fn record_classification(
        &mut self,
        entity_index: u32,
        result_label: &str,
        tier: DecisionTier,
    ) {
        self.decision_counter += 1;
        let mut decision = TracedDecision::new(
            DecisionId(self.decision_counter),
            DecisionKind::Exact,
            tier,
            1.0,
            DecisionContext::Classification {
                point: [0.0; 3],
                result: result_label.to_string(),
            },
        );
        decision.set_entity_scope(forge_core::EntityRef::new(
            forge_core::EntityKind::Face,
            entity_index,
            0,
        ));
        self.decision_log.record(decision);
    }

    fn record_escalation(
        &mut self,
        entity_index: u32,
        escalation: &forge_math::arithmetic::precision::PrecisionEscalation,
    ) {
        if escalation.resolved_at > forge_math::arithmetic::precision::PrecisionMode::Float64 {
            self.decision_counter += 1;
            let mut decision = TracedDecision::new(
                DecisionId(self.decision_counter),
                DecisionKind::Exact,
                DecisionTier::Escalated,
                escalation.disagreement_magnitude.unwrap_or(0.0),
                DecisionContext::PrecisionEscalation { escalation: escalation.clone() },
            );
            decision.set_entity_scope(forge_core::EntityRef::new(
                forge_core::EntityKind::Vertex,
                entity_index,
                0,
            ));
            self.decision_log.record(decision);
        }
    }

    fn record_policy_applied(
        &mut self,
        policy: forge_core::PolicyKind,
        margin: f64,
        default_used: bool,
        _description: Option<&str>,
    ) {
        self.decision_counter += 1;
        let decision = TracedDecision::new(
            DecisionId(self.decision_counter),
            DecisionKind::PolicyApplied { policy, default_used },
            DecisionTier::PolicyApplied,
            margin,
            DecisionContext::Tolerance { measured: margin, threshold: 0.0 },
        );
        self.decision_log.record(decision);
    }

    fn record_ambiguous(
        &mut self,
        fallback_description: &str,
        margin: f64,
    ) {
        self.decision_counter += 1;
        let decision = TracedDecision::new(
            DecisionId(self.decision_counter),
            DecisionKind::Ambiguous { fallback_applied: fallback_description.to_string() },
            DecisionTier::Escalated,
            margin,
            DecisionContext::Degeneracy { description: fallback_description.to_string() },
        );
        self.decision_log.record(decision);
    }

    fn record_forced(
        &mut self,
        reason: &str,
        entity_index: u32,
        margin: f64,
    ) {
        self.decision_counter += 1;
        let mut decision = TracedDecision::new(
            DecisionId(self.decision_counter),
            DecisionKind::Forced { reason: reason.to_string() },
            DecisionTier::Escalated,
            margin,
            DecisionContext::Degeneracy { description: format!("forced: {}", reason) },
        );
        decision.set_entity_scope(forge_core::EntityRef::new(
            forge_core::EntityKind::Vertex,
            entity_index,
            0,
        ));
        self.decision_log.record(decision);
    }

    fn start_span(&mut self, name: &'static str) -> forge_core::SpanId {
        self.decision_log.start_span(name)
    }

    fn end_span(&mut self, id: forge_core::SpanId, duration_micros: u64) {
        self.decision_log.end_span(id, duration_micros);
    }

    fn record_raw(&mut self, decision: TracedDecision) {
        self.decision_log.record(decision);
    }
}

// ── Legacy methods (preserved for backward compat during migration) ─────────

impl ModelingContext {
    /// Record a tolerance decision, auto-assigning a unique ID.
    ///
    /// Prefer `sink.record_tolerance_snap()` or `sink.record_near_boundary()`
    /// for new code.
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
            DecisionContext::Tolerance {
                measured: margin,
                threshold,
            },
        );
        self.decision_log.record(decision);
    }

    /// Record a precision escalation decision, auto-assigning a unique ID.
    ///
    /// Prefer `sink.record_escalation()` for new code.
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
    pub fn clear_decision_log_only(&mut self) {
        self.decision_log = DecisionLog::new();
    }

    /// Reset per-operation trace/metadata state while preserving configuration.
    pub fn reset_for_new_operation(&mut self) {
        self.decision_log = DecisionLog::new();
        self.sub_warnings.clear();
        self.sub_metrics = OperationMetrics::default();
        self.sub_lineage_delta = forge_core::envelope::LineageDelta::default();
        self.sub_accumulated_error_budget = 0.0;
        self.trace_adjuncts = forge_core::tracing::TraceAdjunctSet::new();
        self.decision_counter = 0;
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

    /// Generate a divergence report from the current decision log (P2.3).
    pub fn generate_divergence_report(&self) -> forge_core::DivergenceReport {
        forge_core::scan_for_divergences(&self.decision_log)
    }

    /// Check whether the accumulated error budget has been exceeded.
    pub fn check_budget(&self, accumulated: f64) -> Option<forge_core::KernelWarning> {
        let threshold = self.config.tolerance.error_budget_mm;
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
