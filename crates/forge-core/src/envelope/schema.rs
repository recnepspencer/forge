//! Envelope data shapes — OperationResult, metrics, warnings, lineage.

use std::fmt;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::tracing::{resolve_trace_dir, write_trace_file, DecisionId, DecisionLog};

// =========================================================================
// KERNEL WARNING
// =========================================================================

/// Non-fatal warning emitted during an operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KernelWarning {
    /// A sliver face was created (area below threshold).
    SliverFaceCreated {
        /// Index of the sliver face.
        face_index: u32,
        /// Computed area of the face.
        area: f64,
        /// Threshold below which a face is a sliver.
        threshold: f64,
    },
    /// A near-degenerate edge was created (length below threshold).
    ShortEdgeCreated {
        /// Index of the short edge.
        halfedge_index: u32,
        /// Computed length.
        length: f64,
        /// Minimum acceptable length threshold.
        threshold: f64,
    },
    /// A tolerance decision was made automatically.
    AutoDecision {
        /// The decision that was auto-applied.
        decision_id: DecisionId,
    },
    /// The cumulative floating-point error budget has been exhausted.
    ///
    /// Accumulated from per-vertex tolerance deltas across the operation chain.
    /// Non-fatal: the operation still succeeds; the caller decides whether to abort.
    ErrorBudgetExceeded {
        /// Total accumulated error (mm) across all operations so far.
        accumulated_mm: f64,
        /// Configured budget threshold (mm) that was exceeded.
        threshold_mm: f64,
    },
    /// A healed vertex with large tolerance participated in a tight-tolerance operation.
    ///
    /// Emitted when a vertex with `ToleranceRegime::Healed` enters an operation
    /// whose `global_default()` is tighter than the vertex's healing tolerance.
    RegimeMismatch {
        /// The healing tolerance of the incoming vertex (mm).
        healing_tolerance_mm: f64,
        /// The operation's `global_default()` tolerance (mm).
        operation_tolerance: f64,
    },
}

impl fmt::Display for KernelWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KernelWarning::SliverFaceCreated {
                face_index,
                area,
                threshold,
            } => {
                write!(
                    f,
                    "Sliver face {} (area {:.2e}, threshold {:.2e})",
                    face_index, area, threshold
                )
            }
            KernelWarning::ShortEdgeCreated {
                halfedge_index,
                length,
                threshold,
            } => {
                write!(
                    f,
                    "Short edge {} (length {:.2e}, threshold {:.2e})",
                    halfedge_index, length, threshold
                )
            }
            KernelWarning::AutoDecision { decision_id } => {
                write!(f, "Automatic tolerance decision: {}", decision_id)
            }
            KernelWarning::ErrorBudgetExceeded {
                accumulated_mm,
                threshold_mm,
            } => {
                write!(
                    f,
                    "Error budget exceeded: {:.2e} mm accumulated (threshold {:.2e} mm)",
                    accumulated_mm, threshold_mm
                )
            }
            KernelWarning::RegimeMismatch {
                healing_tolerance_mm,
                operation_tolerance,
            } => {
                write!(
                    f,
                    "Regime mismatch: healed vertex tol={:.2e} mm in op with tol={:.2e} mm",
                    healing_tolerance_mm, operation_tolerance
                )
            }
        }
    }
}

// =========================================================================
// OPERATION METRICS
// =========================================================================

/// Performance and accounting metrics for an operation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OperationMetrics {
    /// Wall-clock duration of the operation.
    pub duration: Duration,
    /// Number of entities created during the operation.
    pub entities_created: u32,
    /// Number of entities deleted during the operation.
    pub entities_deleted: u32,
    /// Number of entities modified during the operation.
    pub entities_modified: u32,
    /// Number of exact predicate evaluations.
    pub exact_predicate_calls: u64,
    /// Number of policy-driven decisions made.
    pub policy_decisions_made: u32,
}

impl OperationMetrics {
    /// Accumulate another metrics record into this one.
    ///
    /// Adds all counters field-by-field. Used by `absorb_sub_result`
    /// and `OperationFinalizer` to roll up sub-operation metrics
    /// without manual per-field addition.
    pub fn accumulate(&mut self, other: &Self) {
        self.duration += other.duration;
        self.entities_created += other.entities_created;
        self.entities_deleted += other.entities_deleted;
        self.entities_modified += other.entities_modified;
        self.exact_predicate_calls += other.exact_predicate_calls;
        self.policy_decisions_made += other.policy_decisions_made;
    }
}

// =========================================================================
// LINEAGE DELTA
// =========================================================================

/// Summary of lineage changes from an operation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LineageDelta {
    /// Number of faces created.
    pub faces_created: u32,
    /// Number of faces deleted.
    pub faces_deleted: u32,
    /// Number of half-edges created.
    pub half_edges_created: u32,
    /// Number of half-edges deleted.
    pub half_edges_deleted: u32,
    /// Number of vertices created.
    pub vertices_created: u32,
    /// Number of vertices deleted.
    pub vertices_deleted: u32,
    /// Number of loops created.
    pub loops_created: u32,
    /// Number of loops deleted.
    pub loops_deleted: u32,
    /// Number of edges created.
    pub edges_created: u32,
    /// Number of edges deleted.
    pub edges_deleted: u32,
    /// Number of shells created.
    pub shells_created: u32,
    /// Number of shells deleted.
    pub shells_deleted: u32,
    /// Number of solids created.
    pub solids_created: u32,
    /// Number of solids deleted.
    pub solids_deleted: u32,
}

impl LineageDelta {
    /// Accumulate another lineage delta into this one.
    ///
    /// Adds all counters field-by-field. Used by `absorb_sub_result`
    /// and `OperationFinalizer` to roll up sub-operation lineage deltas
    /// without manual per-field addition.
    pub fn accumulate(&mut self, other: &Self) {
        self.faces_created += other.faces_created;
        self.faces_deleted += other.faces_deleted;
        self.half_edges_created += other.half_edges_created;
        self.half_edges_deleted += other.half_edges_deleted;
        self.vertices_created += other.vertices_created;
        self.vertices_deleted += other.vertices_deleted;
        self.loops_created += other.loops_created;
        self.loops_deleted += other.loops_deleted;
        self.edges_created += other.edges_created;
        self.edges_deleted += other.edges_deleted;
        self.shells_created += other.shells_created;
        self.shells_deleted += other.shells_deleted;
        self.solids_created += other.solids_created;
        self.solids_deleted += other.solids_deleted;
    }
}

// =========================================================================
// OPERATION RESULT (Universal Envelope)
// =========================================================================

/// Universal envelope wrapping every kernel operation's return value.
///
/// Carries the primary result alongside a queryable decision log,
/// warnings, performance metrics, lineage changes, and topology hashes.
/// An AI agent can reconstruct the full state transition from this
/// envelope alone.
///
/// # Example
/// ```
/// use forge_core::envelope::{OperationResult, OperationMetrics, LineageDelta};
///
/// let result: OperationResult<i32> = OperationResult::new(42);
/// assert_eq!(*result.get_value(), 42);
/// assert!(result.get_decision_log().is_empty());
/// assert!(result.get_decision_log().is_clean());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationResult<T> {
    /// The primary return value.
    value: T,
    /// Non-fatal warnings emitted during the operation.
    warnings: Vec<KernelWarning>,
    /// Full decision trace for this operation.
    decision_log: DecisionLog,
    /// Performance and accounting metrics.
    metrics: OperationMetrics,
    /// Summary of lineage changes.
    lineage_delta: LineageDelta,
    /// Topology hash before the operation.
    state_hash_before: u128,
    /// Topology hash after the operation.
    state_hash_after: u128,
    /// Checkpoint validation results logged during this operation.
    validation_results: Vec<String>,
    /// Extra summary lines to print during compact logging (e.g., replay or lineage stats).
    extra_summaries: Vec<String>,
    /// Accumulated floating-point error budget consumed by this operation chain (mm).
    ///
    /// Increases by `max(new_vertex_tolerance) - global_default()` after each
    /// boolean phase that creates vertices. When this exceeds
    /// `ToleranceConfig::error_budget_mm`, `check_budget()` emits
    /// `KernelWarning::ErrorBudgetExceeded`. Defaults to `0.0`.
    #[serde(default)]
    accumulated_error_budget: f64,
}

impl<T> OperationResult<T> {
    /// Create a new operation result with empty metadata.
    pub fn new(value: T) -> Self {
        Self {
            value,
            warnings: Vec::new(),
            decision_log: DecisionLog::new(),
            metrics: OperationMetrics::default(),
            lineage_delta: LineageDelta::default(),
            state_hash_before: 0,
            state_hash_after: 0,
            validation_results: Vec::new(),
            extra_summaries: Vec::new(),
            accumulated_error_budget: 0.0,
        }
    }

    /// Create an operation result with full metadata.
    pub fn with_metadata(
        value: T,
        warnings: Vec<KernelWarning>,
        decision_log: DecisionLog,
        metrics: OperationMetrics,
        lineage_delta: LineageDelta,
        state_hash_before: u128,
        state_hash_after: u128,
    ) -> Self {
        Self {
            value,
            warnings,
            decision_log,
            metrics,
            lineage_delta,
            state_hash_before,
            state_hash_after,
            validation_results: Vec::new(),
            extra_summaries: Vec::new(),
            accumulated_error_budget: 0.0,
        }
    }

    /// The primary return value of the operation.
    pub fn get_value(&self) -> &T {
        &self.value
    }

    /// Consume the result and return the inner value.
    ///
    /// If `FORGE_TRACE_DIR` is set, automatically persists the decision
    /// log as a JSON trace file before returning. This is the universal
    /// hook — every kernel operation that produces an `OperationResult`
    /// gets traced with zero wiring.
    pub fn into_value(self) -> T {
        self.maybe_persist_trace();
        self.value
    }

    /// Non-fatal warnings emitted during the operation.
    pub fn get_warnings(&self) -> &[KernelWarning] {
        &self.warnings
    }

    /// Take ownership of warnings, replacing them with an empty list.
    pub fn take_warnings(&mut self) -> Vec<KernelWarning> {
        std::mem::take(&mut self.warnings)
    }

    /// The full decision log.
    pub fn get_decision_log(&self) -> &DecisionLog {
        &self.decision_log
    }

    /// Mutable access to the decision log (for populating during execution).
    pub fn get_decision_log_mut(&mut self) -> &mut DecisionLog {
        &mut self.decision_log
    }

    /// Performance metrics for the operation.
    pub fn get_metrics(&self) -> &OperationMetrics {
        &self.metrics
    }

    /// Take ownership of metrics, replacing with defaults.
    pub fn take_metrics(&mut self) -> OperationMetrics {
        std::mem::take(&mut self.metrics)
    }

    /// Summary of lineage changes from the operation.
    pub fn get_lineage_delta(&self) -> &LineageDelta {
        &self.lineage_delta
    }

    /// Take ownership of lineage delta, replacing with defaults.
    pub fn take_lineage_delta(&mut self) -> LineageDelta {
        std::mem::take(&mut self.lineage_delta)
    }

    /// Topology hash before the operation.
    pub fn get_state_hash_before(&self) -> u128 {
        self.state_hash_before
    }

    /// Topology hash after the operation.
    pub fn get_state_hash_after(&self) -> u128 {
        self.state_hash_after
    }

    /// Whether any warnings were emitted.
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    /// Whether any decisions were recorded.
    pub fn has_decisions(&self) -> bool {
        !self.decision_log.is_empty()
    }

    /// Set the operation metrics.
    pub fn set_metrics(&mut self, metrics: OperationMetrics) {
        self.metrics = metrics;
    }

    /// Set the lineage delta.
    pub fn set_lineage_delta(&mut self, delta: LineageDelta) {
        self.lineage_delta = delta;
    }

    /// Set the topology hash before the operation.
    pub fn set_state_hash_before(&mut self, hash: u128) {
        self.state_hash_before = hash;
    }

    /// Set the topology hash after the operation.
    pub fn set_state_hash_after(&mut self, hash: u128) {
        self.state_hash_after = hash;
    }

    /// Set the decision log.
    pub fn set_decision_log(&mut self, log: DecisionLog) {
        self.decision_log = log;
    }

    /// Take ownership of the decision log, replacing it with an empty one.
    pub fn take_decision_log(&mut self) -> DecisionLog {
        std::mem::take(&mut self.decision_log)
    }

    /// Add a warning.
    pub fn add_warning(&mut self, warning: KernelWarning) {
        self.warnings.push(warning);
    }

    /// Absorb metadata from another envelope, discarding its value.
    ///
    /// This is for nested operation composition: preserve the parent `value`
    /// while merging sub-operation audit data (decisions, warnings, metrics,
    /// lineage deltas, validation summaries, and accumulated error budget).
    ///
    /// State hashes are intentionally NOT merged because they describe the
    /// parent operation boundary, not arbitrary nested sub-operations.
    pub fn absorb_metadata<U>(&mut self, other: &mut OperationResult<U>) {
        self.decision_log.merge(other.take_decision_log());
        self.warnings.extend(std::mem::take(&mut other.warnings));

        let other_metrics = std::mem::take(&mut other.metrics);
        self.metrics.duration += other_metrics.duration;
        self.metrics.entities_created += other_metrics.entities_created;
        self.metrics.entities_deleted += other_metrics.entities_deleted;
        self.metrics.entities_modified += other_metrics.entities_modified;
        self.metrics.exact_predicate_calls += other_metrics.exact_predicate_calls;
        self.metrics.policy_decisions_made += other_metrics.policy_decisions_made;

        let other_lineage = std::mem::take(&mut other.lineage_delta);
        self.lineage_delta.faces_created += other_lineage.faces_created;
        self.lineage_delta.faces_deleted += other_lineage.faces_deleted;
        self.lineage_delta.half_edges_created += other_lineage.half_edges_created;
        self.lineage_delta.half_edges_deleted += other_lineage.half_edges_deleted;
        self.lineage_delta.vertices_created += other_lineage.vertices_created;
        self.lineage_delta.vertices_deleted += other_lineage.vertices_deleted;
        self.lineage_delta.loops_created += other_lineage.loops_created;
        self.lineage_delta.loops_deleted += other_lineage.loops_deleted;
        self.lineage_delta.edges_created += other_lineage.edges_created;
        self.lineage_delta.edges_deleted += other_lineage.edges_deleted;
        self.lineage_delta.shells_created += other_lineage.shells_created;
        self.lineage_delta.shells_deleted += other_lineage.shells_deleted;
        self.lineage_delta.solids_created += other_lineage.solids_created;
        self.lineage_delta.solids_deleted += other_lineage.solids_deleted;

        self.validation_results
            .extend(std::mem::take(&mut other.validation_results));
        self.extra_summaries
            .extend(std::mem::take(&mut other.extra_summaries));
        self.accumulated_error_budget += std::mem::take(&mut other.accumulated_error_budget);
    }

    /// Transform the inner value while preserving all metadata.
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> OperationResult<U> {
        OperationResult {
            value: f(self.value),
            warnings: self.warnings,
            decision_log: self.decision_log,
            metrics: self.metrics,
            lineage_delta: self.lineage_delta,
            state_hash_before: self.state_hash_before,
            state_hash_after: self.state_hash_after,
            validation_results: self.validation_results,
            extra_summaries: self.extra_summaries,
            accumulated_error_budget: self.accumulated_error_budget,
        }
    }

    /// Accumulate floating-point error into the budget tracker.
    ///
    /// `delta` is the additional error (mm) this operation contributes.
    /// Typically `max(new_vertex_tolerance) - global_default()` per phase.
    pub fn consume_budget(&mut self, delta: f64) {
        self.accumulated_error_budget += delta;
    }

    /// Total accumulated error budget consumed so far (mm).
    pub fn get_accumulated_budget(&self) -> f64 {
        self.accumulated_error_budget
    }

    /// Take and reset the accumulated error budget tracker.
    pub fn take_accumulated_budget(&mut self) -> f64 {
        std::mem::take(&mut self.accumulated_error_budget)
    }

    /// Checkpoint validation results from this operation.
    pub fn get_validation_results(&self) -> &[String] {
        &self.validation_results
    }

    /// Add a validation result summary string.
    pub fn add_validation_result(&mut self, result: String) {
        self.validation_results.push(result);
    }

    /// Checkpoint extra summaries.
    pub fn get_extra_summaries(&self) -> &[String] {
        &self.extra_summaries
    }

    /// Add an extra summary line.
    pub fn add_extra_summary(&mut self, summary: String) {
        self.extra_summaries.push(summary);
    }

    /// Persist the trace to disk if `FORGE_TRACE_DIR` is set.
    ///
    /// Uses `OnceLock` to check the env var exactly once per process.
    /// When the dir is set, writes a JSON file compatible with
    /// `forge_view::trace_store::TraceFile`.
    ///
    /// In debug/test builds, falls back to `workspace_root/traces/`
    /// (relative to this crate's compile-time `CARGO_MANIFEST_DIR`).
    fn maybe_persist_trace(&self) {
        self.persist_trace_with_status("ok");
    }

    /// Persist the trace to disk with a status label ("ok" or "error").
    fn persist_trace_with_status(&self, status: &str) {
        let dir = match resolve_trace_dir() {
            Some(d) => d,
            None => return,
        };

        if self.decision_log.is_empty() {
            return;
        }

        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            write_trace_file(&dir, &self.decision_log, self.state_hash_after, status);
        }));
    }
}

// =========================================================================
// RESULT-VALUED ENVELOPE (Always-Envelope Architecture)
// =========================================================================

impl<T> OperationResult<Result<T, crate::KernelError>> {
    /// Extract the inner `Result`, auto-persisting the trace and logging errors.
    ///
    /// This is the primary extraction point for the always-envelope architecture.
    /// Unlike `into_value()`, this method:
    /// - Persists the trace on BOTH success and failure
    /// - Calls `log_error` when the inner value is `Err`
    /// - Calls `log_result`-equivalent when the inner value is `Ok`
    ///
    /// Every kernel operation should return `OperationResult<Result<T, KernelError>>`,
    /// and callers should use `into_result()` to extract.
    pub fn into_result(self) -> Result<T, crate::KernelError> {
        match &self.value {
            Ok(_) => {
                self.persist_trace_with_status("ok");
                crate::log_result("pipeline", &self);
            }
            Err(e) => {
                self.persist_trace_with_status("error");
                crate::log_error("pipeline", e);
            }
        }
        self.value
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn budget_tracking_accumulates_correctly() {
        let mut envelope = OperationResult::new(42);
        assert_eq!(envelope.get_accumulated_budget(), 0.0);

        envelope.consume_budget(1e-7);
        assert_eq!(envelope.get_accumulated_budget(), 1e-7);

        envelope.consume_budget(3e-8);
        assert!((envelope.get_accumulated_budget() - 1.3e-7).abs() < 1e-15);

        let warning = KernelWarning::ErrorBudgetExceeded {
            accumulated_mm: envelope.get_accumulated_budget(),
            threshold_mm: 1e-7,
        };
        envelope.add_warning(warning);
        assert_eq!(envelope.get_warnings().len(), 1);
        assert!(matches!(
            envelope.get_warnings()[0],
            KernelWarning::ErrorBudgetExceeded { .. }
        ));
    }

    #[test]
    fn absorb_metadata_merges_suboperation_audit_data() {
        let mut parent = OperationResult::new("parent");
        let mut child = OperationResult::new("child");

        child.add_warning(KernelWarning::AutoDecision {
            decision_id: DecisionId(7),
        });
        child
            .get_decision_log_mut()
            .record(crate::TracedDecision::new(
                DecisionId(1),
                crate::DecisionKind::Exact,
                crate::DecisionTier::Deterministic,
                1.0,
                crate::DecisionContext::Degeneracy {
                    description: "sub-op".into(),
                },
            ));
        child.metrics.entities_modified = 3;
        child.metrics.policy_decisions_made = 1;
        child.lineage_delta.faces_deleted = 2;
        child.add_validation_result("ok".into());
        child.add_extra_summary("summary".into());
        child.consume_budget(1e-6);

        parent.absorb_metadata(&mut child);

        assert_eq!(parent.get_warnings().len(), 1);
        assert_eq!(parent.get_decision_log().len(), 1);
        assert_eq!(parent.get_metrics().entities_modified, 3);
        assert_eq!(parent.get_metrics().policy_decisions_made, 1);
        assert_eq!(parent.get_lineage_delta().faces_deleted, 2);
        assert_eq!(parent.get_validation_results().len(), 1);
        assert_eq!(parent.get_extra_summaries().len(), 1);
        assert!(parent.get_accumulated_budget() > 0.0);

        assert!(child.get_decision_log().is_empty());
        assert!(child.get_warnings().is_empty());
        assert_eq!(child.get_metrics().entities_modified, 0);
        assert_eq!(child.get_metrics().policy_decisions_made, 0);
        assert_eq!(child.get_lineage_delta().faces_deleted, 0);
        assert_eq!(child.get_accumulated_budget(), 0.0);
        assert!(child.get_validation_results().is_empty());
        assert!(child.get_extra_summaries().is_empty());
    }
}
