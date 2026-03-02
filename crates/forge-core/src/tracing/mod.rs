//! Tracing infrastructure for the Forge geometry kernel.
//!
//! DOMAIN: Span-based decision tracing protocol. Every kernel decision
//! is recorded as a `TracedDecision` within a `DecisionLog`, organized
//! into `TraceEvent` spans. The log is queryable, serializable, and diffable.
//!
//! Vertical slices:
//! - `decision`:     Core data types (TracedDecision, DecisionKind, etc.)
//! - `decision_log`: Queryable log collection + fingerprinting + output
//! - `payload`:      Typed trace payloads (policy, resolution, reidentification)
//! - `replay`:       Causal replay & diagnosis tooling (P3)
//!
//! DEPENDENCIES: serde, tracing

pub mod decision;
pub mod decision_log;
pub mod payload;
pub mod replay;
pub mod sink;

#[cfg(test)]
mod tests;

// ── Stable re-exports (preserves all existing public API paths) ──────────

pub use decision::{
    DecisionContext, DecisionId, DecisionKind, DecisionTier, EntityKind, EntityRef, SpanId,
    TopologyDelta, TraceEvent, TracedDecision, EULER_OP_FEATURE_SCOPE,
};

pub use decision_log::{
    DecisionLog, DecisionSummary, SpanSummaryEntry, TraceDiff, TraceSummary,
    compute_trace_fingerprint, TraceFingerprint,
    log_decision_log, log_error, log_result,
};

pub use payload::{
    TraceAdjunctRecord, TraceAdjunctSet, POLICY_DECISION_TRACE_PAYLOAD_KIND,
    POLICY_DECISION_TRACE_PAYLOAD_VERSION, REIDENTIFICATION_TRACE_PAYLOAD_KIND,
    REIDENTIFICATION_TRACE_PAYLOAD_VERSION, RESOLUTION_TRACE_PAYLOAD_KIND,
    RESOLUTION_TRACE_PAYLOAD_VERSION,
    CandidateValueSummary, PolicyDecisionTracePayload, PolicyResolutionOutcome,
    PolicyResolutionScopeRef, PolicyResolutionSource, PolicyTraceConsistencyError,
    ReidentificationCompatibilitySummary, ReidentificationFailureCauseSummary,
    ReidentificationModeSummary, ReidentificationOriginKindSummary, ReidentificationOutcome,
    ReidentificationTraceConsistencyError, ReidentificationTracePayload,
    ResolutionCandidateSummary, ResolutionMatchKind, ResolutionOutcome, ResolutionQuerySummary,
    ResolutionRoute, ResolutionTraceConsistencyError, ResolutionTracePayload,
};

pub use replay::{
    diff_decision_logs, CheckpointLog, DecisionChange, DecisionDelta,
    delta_debug, DeltaDebugResult,
    scan_for_divergences, DivergenceDetail, DivergenceReport,
};

pub use sink::{
    DecisionSink, DecisionSinkHandle, NullSink, TestSink,
};
