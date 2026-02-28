//! Tracing infrastructure for the Forge geometry kernel.
//!
//! DOMAIN: Span-based decision tracing protocol. Every kernel decision
//! is recorded as a `TracedDecision` within a `DecisionLog`, organized
//! into `TraceEvent` spans. The log is queryable, serializable, and diffable.
//!
//! - `checkpoint_diff`: Checkpoint diffing for causal replay (P3.1)
//! - `delta_debug`: Binary search for minimal failure-inducing step (P3.2)
//!
//! DEPENDENCIES: serde, tracing

pub mod adjunct;
pub mod checkpoint_diff;
mod decision_log;
pub mod delta_debug;
pub mod divergence;
pub mod fingerprint;
mod logging;
pub mod policy_trace;
pub mod reidentification_trace;
pub mod resolution_trace;
mod schema;

#[cfg(test)]
mod tests;

pub use schema::{
    DecisionContext, DecisionId, DecisionKind, DecisionTier, EntityKind, EntityRef, SpanId,
    TopologyDelta, TraceEvent, TracedDecision, EULER_OP_FEATURE_SCOPE,
};

pub use decision_log::{DecisionLog, DecisionSummary, SpanSummaryEntry, TraceDiff, TraceSummary};

pub use checkpoint_diff::{diff_decision_logs, CheckpointLog, DecisionChange, DecisionDelta};

pub use divergence::{scan_for_divergences, DivergenceDetail, DivergenceReport};

pub use adjunct::{
    TraceAdjunctRecord, TraceAdjunctSet, POLICY_DECISION_TRACE_PAYLOAD_KIND,
    POLICY_DECISION_TRACE_PAYLOAD_VERSION, REIDENTIFICATION_TRACE_PAYLOAD_KIND,
    REIDENTIFICATION_TRACE_PAYLOAD_VERSION, RESOLUTION_TRACE_PAYLOAD_KIND,
    RESOLUTION_TRACE_PAYLOAD_VERSION,
};
pub use fingerprint::{compute_trace_fingerprint, TraceFingerprint};
pub use logging::{log_decision_log, log_error, log_result};
pub use policy_trace::{
    CandidateValueSummary, PolicyDecisionTracePayload, PolicyResolutionOutcome,
    PolicyResolutionScopeRef, PolicyResolutionSource, PolicyTraceConsistencyError,
};
pub use reidentification_trace::{
    ReidentificationCompatibilitySummary, ReidentificationFailureCauseSummary,
    ReidentificationModeSummary, ReidentificationOriginKindSummary, ReidentificationOutcome,
    ReidentificationTraceConsistencyError, ReidentificationTracePayload,
};
pub use resolution_trace::{
    ResolutionCandidateSummary, ResolutionMatchKind, ResolutionOutcome, ResolutionQuerySummary,
    ResolutionRoute, ResolutionTraceConsistencyError, ResolutionTracePayload,
};
