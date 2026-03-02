//! Output data from a completed span.

use crate::configuration::facade::ResolvedConfig;
use forge_core::envelope::{KernelWarning, LineageDelta};
use forge_core::{DecisionLog, OperationMetrics};

/// Output collected from a completed `KernelSpan`.
#[derive(Debug, Clone)]
pub struct SpanOutput {
    pub decision_log: DecisionLog,
    pub warnings: Vec<KernelWarning>,
    pub metrics: OperationMetrics,
    pub lineage_delta: LineageDelta,
    pub config_snapshot: Option<ResolvedConfig>,
}
