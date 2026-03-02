//! Public façade for `forge-core` tracing infrastructure.
//!
//! This file documents the authoritative public boundary for the tracing module.
//! All external crates should import via `forge_core::tracing::{...}` —
//! this façade enumerates exactly what is public.
//!
//! DOMAIN STANDARDS: Each component exposes a single public façade file.
//! Internal complexity (sink.rs, decision.rs, decision_log.rs, etc.) remains hidden.

// Re-exported at forge_core::tracing::{...} via mod.rs
// Listed here as documentation of the authoritative public surface.

// Decision types
pub use super::decision::{
    DecisionContext, DecisionId, DecisionKind, DecisionTier,
    EntityKind, EntityRef, SpanId, TopologyDelta, TraceEvent, TracedDecision,
    EULER_OP_FEATURE_SCOPE,
};

// Decision log
pub use super::decision_log::{
    compute_trace_fingerprint, log_decision_log, log_error, log_result,
    DecisionLog, DecisionSummary, SpanSummaryEntry, TraceDiff, TraceFingerprint, TraceSummary,
};

// Sinks (production only — NullSink/TestSink banned)
pub use super::sink::{DecisionSink, DecisionSinkHandle};

// Replay/diff tooling
pub use super::replay::{
    delta_debug, diff_decision_logs, scan_for_divergences,
    CheckpointLog, DecisionChange, DecisionDelta, DeltaDebugResult,
    DivergenceDetail, DivergenceReport,
};

// Trace payloads
pub use super::payload::{
    CandidateValueSummary, PolicyDecisionTracePayload, PolicyResolutionOutcome,
    PolicyResolutionScopeRef, PolicyResolutionSource, PolicyTraceConsistencyError,
    ReidentificationCompatibilitySummary, ReidentificationFailureCauseSummary,
    ReidentificationModeSummary, ReidentificationOriginKindSummary, ReidentificationOutcome,
    ReidentificationTraceConsistencyError, ReidentificationTracePayload,
    ResolutionCandidateSummary, ResolutionMatchKind, ResolutionOutcome, ResolutionQuerySummary,
    ResolutionRoute, ResolutionTraceConsistencyError, ResolutionTracePayload,
    TraceAdjunctRecord, TraceAdjunctSet,
    POLICY_DECISION_TRACE_PAYLOAD_KIND, POLICY_DECISION_TRACE_PAYLOAD_VERSION,
    REIDENTIFICATION_TRACE_PAYLOAD_KIND, REIDENTIFICATION_TRACE_PAYLOAD_VERSION,
    RESOLUTION_TRACE_PAYLOAD_KIND, RESOLUTION_TRACE_PAYLOAD_VERSION,
};

