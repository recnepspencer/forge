//! Finalization summary data types.
//!
//! DOMAIN: Pure data structures for finalization output — status enums,
//! hash boundaries, drained metadata counts, and the collected bundle.
//! No logic here; logic lives in `logic/finalizer.rs`.

use forge_core::envelope::{LineageDelta, OperationMetrics};
use forge_core::tracing::{TraceAdjunctSet, TraceFingerprint};

/// Finalization path status (typed; avoids stringly status flags in callers).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FinalizationStatus {
    Success,
    Error,
}

impl FinalizationStatus {
    pub(crate) fn as_trace_status(self) -> &'static str {
        match self {
            FinalizationStatus::Success => "ok",
            FinalizationStatus::Error => "error",
        }
    }
}

/// Topology hash boundary values for finalization.
///
/// Phase 2 explicitly treats these as topology-state hashes (not a composite
/// kernel-state hash) until a `KernelState` fingerprint contract exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TopologyHashBoundary {
    pub before: Option<u128>,
    pub after: Option<u128>,
}

/// Aggregate counts/drained summaries captured during finalization.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DrainedMetadataCounts {
    pub warnings: usize,
    pub validation_results: usize,
    pub extra_summaries: usize,
}

/// Deterministic finalization summary (collect phase output).
#[derive(Debug, Clone)]
pub struct FinalizationSummary {
    pub status: FinalizationStatus,
    pub trace_fingerprint: TraceFingerprint,
    pub adjunct_count: usize,
    pub topology_state_hash_before: Option<u128>,
    pub topology_state_hash_after: Option<u128>,
    pub drained_metadata_counts: DrainedMetadataCounts,
    pub drained_metrics: OperationMetrics,
    pub drained_lineage_delta: LineageDelta,
    pub drained_accumulated_error_budget: f64,
    pub trace_emitted: bool,
}

/// Deterministic collected finalization artifact bundle (pre-emit).
#[derive(Debug, Clone)]
pub struct CollectedFinalization {
    pub summary: FinalizationSummary,
    pub(crate) decision_log: forge_core::DecisionLog,
    pub(crate) adjuncts: TraceAdjunctSet,
}

/// Finalizer reuse error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FinalizationError {
    AlreadyFinalized,
}
