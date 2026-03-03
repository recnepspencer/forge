//! Envelope data shapes — OperationResult, metrics, warnings, lineage.

pub mod kernel_warning;
pub mod lineage_delta;
pub mod operation_metrics;
pub mod operation_result;

pub use kernel_warning::KernelWarning;
pub use lineage_delta::LineageDelta;
pub use operation_metrics::OperationMetrics;
pub use operation_result::OperationResult;
