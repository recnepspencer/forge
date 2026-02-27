//! Decision logging and span management.
//!
//! DOMAIN: Recording tolerance decisions, precision escalations, and named spans.
//! INVARIANTS: Every decision gets a monotonically increasing ID.

use forge_core::envelope::OperationMetrics;
use forge_core::{
    DecisionContext, DecisionId, DecisionKind, DecisionLog, DecisionTier, TracedDecision,
};

use super::schema::ModelingContext;

impl ModelingContext {
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
            DecisionContext::Tolerance {
                measured: margin,
                threshold,
            },
        );
        if crate::core::tracing::KernelSpan::is_active() {
            crate::core::tracing::KernelSpan::record_decision(decision);
        } else {
            self.decision_log.record(decision);
        }
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
            if crate::core::tracing::KernelSpan::is_active() {
                crate::core::tracing::KernelSpan::record_decision(decision);
            } else {
                self.decision_log.record(decision);
            }
        }
    }

    /// Execute `f` within a named span, recording start/end events.
    pub fn scope<F, R>(&mut self, name: &'static str, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        let use_kernel_span = crate::core::tracing::KernelSpan::is_active();
        let span_id = if use_kernel_span {
            crate::core::tracing::KernelSpan::start_span(name)
        } else {
            self.decision_log.start_span(name)
        };
        let start = std::time::Instant::now();
        let result = f(self);
        let duration_micros = start.elapsed().as_micros() as u64;

        if use_kernel_span {
            crate::core::tracing::KernelSpan::end_span(span_id, duration_micros);
        } else {
            self.decision_log.end_span(span_id, duration_micros);
        }
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
        self.sub_lineage_delta = forge_core::envelope::LineageDelta::default();
        self.sub_accumulated_error_budget = 0.0;
        self.trace_adjuncts = forge_core::tracing::TraceAdjunctSet::new();
        self.decision_counter = 0;
        self.log_drained = false;
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
