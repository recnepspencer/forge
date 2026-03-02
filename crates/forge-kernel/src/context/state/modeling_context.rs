//! ModelingContext struct definition and lifecycle.
//!
//! DOMAIN: The core struct that holds all policy configuration and tracing state.
//! INVARIANTS: Default construction provides sensible defaults for all policies.

use forge_core::envelope::{KernelWarning, LineageDelta, OperationMetrics};
use forge_core::tracing::TraceAdjunctSet;
use forge_core::DecisionLog;

use crate::configuration::facade::KernelConfig;

/// The modeling context that governs all policy decisions.
///
/// Passed to operations that may encounter ambiguity. Records every
/// tolerance-driven decision for traceability (D2) and replay (D1).
///
/// # Example
/// ```
/// use forge_kernel::context::facade::ModelingContext;
///
/// let ctx = ModelingContext::default();
/// assert_eq!(ctx.get_decision_count(), 0);
/// ```
#[derive(Debug, Clone)]
pub struct ModelingContext {
    pub(crate) config: KernelConfig,
    pub(crate) decision_log: DecisionLog,
    /// Aggregated warnings absorbed from sub-operations that returned envelopes.
    pub(crate) sub_warnings: Vec<KernelWarning>,
    /// Aggregated metrics absorbed from sub-operations.
    pub(crate) sub_metrics: OperationMetrics,
    /// Aggregated lineage deltas absorbed from sub-operations.
    pub(crate) sub_lineage_delta: LineageDelta,
    /// Aggregated error budget consumed by absorbed sub-operations.
    pub(crate) sub_accumulated_error_budget: f64,
    /// Typed adjunct payloads produced alongside traced decisions.
    pub(crate) trace_adjuncts: TraceAdjunctSet,
    pub(crate) decision_counter: u64,
}

impl ModelingContext {
    /// Create a modeling context with default or inherited policies.
    pub fn new() -> Self {
        Self::from_config(KernelConfig::default())
    }

    /// Create a modeling context from an explicit base config.
    pub fn from_config(config: KernelConfig) -> Self {
        Self {
            config,
            decision_log: DecisionLog::new(),
            sub_warnings: Vec::new(),
            sub_metrics: OperationMetrics::default(),
            sub_lineage_delta: LineageDelta::default(),
            sub_accumulated_error_budget: 0.0,
            trace_adjuncts: TraceAdjunctSet::new(),
            decision_counter: 0,
        }
    }
}

impl Default for ModelingContext {
    fn default() -> Self {
        Self::new()
    }
}
