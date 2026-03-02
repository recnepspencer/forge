use forge_core::envelope::{KernelWarning, LineageDelta, OperationMetrics};

/// Aggregated metadata absorbed from sub-operation envelopes.
#[derive(Debug, Clone, Default)]
pub struct SubOperationMetadata {
    pub warnings: Vec<KernelWarning>,
    pub metrics: OperationMetrics,
    pub lineage_delta: LineageDelta,
    pub accumulated_error_budget: f64,
}
