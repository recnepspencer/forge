//! Method implementations for `OperationResult<T>`.

use crate::envelope::data::{
    KernelWarning, LineageDelta, OperationMetrics, OperationResult,
};
use crate::tracing::DecisionLog;

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
    pub fn into_value(self) -> T {
        self.value
    }

    /// Mutable reference to the inner value.
    ///
    /// Used by the pipeline to perform coordinate restoration on the
    /// output `SolidEnvelope`'s geometry after feature execution.
    pub fn get_value_mut(&mut self) -> &mut T {
        &mut self.value
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
        self.metrics.accumulate(&other_metrics);

        let other_lineage = std::mem::take(&mut other.lineage_delta);
        self.lineage_delta.accumulate(&other_lineage);

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
}

impl<T> OperationResult<Result<T, crate::KernelError>> {
    /// Extract the inner `Result`, logging the trace summary or error.
    ///
    /// This is the primary extraction point for the always-envelope architecture.
    /// - On `Ok`: emits `display_interesting()` via `tracing::info!`
    /// - On `Err`: emits the error via `tracing::error!`
    ///
    /// Every kernel operation should return `OperationResult<Result<T, KernelError>>`,
    /// and callers should use `into_result()` to extract.
    pub fn into_result(self) -> Result<T, crate::KernelError> {
        match &self.value {
            Ok(_) => {
                crate::log_result("pipeline", &self);
            }
            Err(e) => {
                crate::log_error("pipeline", e);
            }
        }
        self.value
    }
}
