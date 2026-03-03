//! Universal operation result envelope.

use serde::{Deserialize, Serialize};

use crate::tracing::DecisionLog;

use super::kernel_warning::KernelWarning;
use super::lineage_delta::LineageDelta;
use super::operation_metrics::OperationMetrics;

/// Universal envelope wrapping every kernel operation's return value.
///
/// Carries the primary result alongside a queryable decision log,
/// warnings, performance metrics, lineage changes, and topology hashes.
/// An AI agent can reconstruct the full state transition from this
/// envelope alone.
///
/// # Example
/// ```
/// use forge_core::{OperationResult, OperationMetrics, LineageDelta};
///
/// let result: OperationResult<i32> = OperationResult::new(42);
/// assert_eq!(*result.get_value(), 42);
/// assert!(result.get_decision_log().is_empty());
/// assert!(result.get_decision_log().is_clean());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationResult<T> {
    /// The primary return value.
    pub(in crate::envelope) value: T,
    /// Non-fatal warnings emitted during the operation.
    pub(in crate::envelope) warnings: Vec<KernelWarning>,
    /// Full decision trace for this operation.
    pub(in crate::envelope) decision_log: DecisionLog,
    /// Performance and accounting metrics.
    pub(in crate::envelope) metrics: OperationMetrics,
    /// Summary of lineage changes.
    pub(in crate::envelope) lineage_delta: LineageDelta,
    /// Topology hash before the operation.
    pub(in crate::envelope) state_hash_before: u128,
    /// Topology hash after the operation.
    pub(in crate::envelope) state_hash_after: u128,
    /// Checkpoint validation results logged during this operation.
    pub(in crate::envelope) validation_results: Vec<String>,
    /// Extra summary lines to print during compact logging (e.g., replay or lineage stats).
    pub(in crate::envelope) extra_summaries: Vec<String>,
    /// Accumulated floating-point error budget consumed by this operation chain (mm).
    ///
    /// Increases by `max(new_vertex_tolerance) - global_default()` after each
    /// boolean phase that creates vertices. When this exceeds
    /// `ToleranceConfig::error_budget_mm`, `check_budget()` emits
    /// `KernelWarning::ErrorBudgetExceeded`. Defaults to `0.0`.
    #[serde(default)]
    pub(in crate::envelope) accumulated_error_budget: f64,
}

