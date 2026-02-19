//! Envelope data shapes — OperationResult, metrics, warnings, lineage.

use std::fmt;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::tracing::{
    DecisionId, DecisionLog,
    resolve_trace_dir, write_trace_file,
};

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
}

impl fmt::Display for KernelWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KernelWarning::SliverFaceCreated { face_index, area, threshold } => {
                write!(f, "Sliver face {} (area {:.2e}, threshold {:.2e})", face_index, area, threshold)
            }
            KernelWarning::ShortEdgeCreated { halfedge_index, length, threshold } => {
                write!(f, "Short edge {} (length {:.2e}, threshold {:.2e})", halfedge_index, length, threshold)
            }
            KernelWarning::AutoDecision { decision_id } => {
                write!(f, "Automatic tolerance decision: {}", decision_id)
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
        Self { value, warnings, decision_log, metrics, lineage_delta, state_hash_before, state_hash_after }
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

    /// Summary of lineage changes from the operation.
    pub fn get_lineage_delta(&self) -> &LineageDelta {
        &self.lineage_delta
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
        }
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
        let dir = match resolve_trace_dir() {
            Some(d) => d,
            None => return,
        };

        if self.decision_log.is_empty() {
            return;
        }

        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            write_trace_file(&dir, &self.decision_log, self.state_hash_after, "ok");
        }));
    }
}
