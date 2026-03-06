//! Typed subscriber outputs for feature lifecycle orchestration.

use crate::context::SubOperationMetadata;
use forge_core::tracing::{TraceAdjunctSet, TraceFingerprint};
use forge_core::DecisionLog;

/// Output produced by draining `ModelingContext` sinks.
#[derive(Debug, Clone)]
pub struct DecisionDrainOutput {
    pub decision_log: DecisionLog,
    pub trace_adjuncts: TraceAdjunctSet,
    pub sub_metadata: SubOperationMetadata,
}

/// Output produced by finalization assembly subscriber.
#[derive(Debug, Clone)]
pub struct FinalizationOutput {
    pub decision_log: DecisionLog,
    pub warnings: Vec<forge_core::envelope::KernelWarning>,
    pub metrics: forge_core::envelope::OperationMetrics,
    pub lineage_delta: forge_core::envelope::LineageDelta,
    pub accumulated_error_budget: f64,
    pub state_hash_before: u128,
    pub state_hash_after: u128,
    pub trace_fingerprint: TraceFingerprint,
    pub adjunct_count: usize,
    pub duration_micros: u64,
}

/// Final operation envelope metadata after audit policy processing.
#[derive(Debug, Clone)]
pub struct OperationEnvelopeOutput {
    pub decision_log: DecisionLog,
    pub warnings: Vec<forge_core::envelope::KernelWarning>,
    pub metrics: forge_core::envelope::OperationMetrics,
    pub lineage_delta: forge_core::envelope::LineageDelta,
    pub accumulated_error_budget: f64,
    pub state_hash_before: u128,
    pub state_hash_after: u128,
    pub extra_summaries: Vec<String>,
}
