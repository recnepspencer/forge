//! Tracing infrastructure for the Forge geometry kernel.
//!
//! DOMAIN: Span-based decision tracing protocol. Every kernel decision
//! is recorded as a `TracedDecision` within a `DecisionLog`, organized
//! into `TraceEvent` spans. The log is queryable, serializable, diffable,
//! and auto-persistable to disk.
//!
//! DEPENDENCIES: serde, serde_json (persistence)

mod schema;
mod decision_log;
mod persistence;
mod logging;

#[cfg(test)]
mod tests;

pub use schema::{
    EntityRef, SpanId, DecisionTier, TraceEvent,
    DecisionKind, DecisionContext, DecisionId, TracedDecision,
    EULER_OP_FEATURE_SCOPE,
};

pub use decision_log::{
    DecisionLog, DecisionSummary, SpanSummaryEntry,
    TraceSummary, TraceDiff,
};

pub use persistence::{resolve_trace_dir, write_trace_file};

pub use logging::{LogLevel, log_level, log_result, log_decision_log};
