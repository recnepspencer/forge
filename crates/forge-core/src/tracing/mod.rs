//! Tracing infrastructure for the Forge geometry kernel.
//!
//! DOMAIN: Span-based decision tracing protocol. Every kernel decision
//! is recorded as a `TracedDecision` within a `DecisionLog`, organized
//! into `TraceEvent` spans. The log is queryable, serializable, diffable,
//! and auto-persistable to disk.
//!
//! - `checkpoint_diff`: Checkpoint diffing for causal replay (P3.1)
//! - `delta_debug`: Binary search for minimal failure-inducing step (P3.2)
//!
//! DEPENDENCIES: serde, serde_json (persistence)

mod schema;
mod decision_log;
pub mod checkpoint_diff;
pub mod delta_debug;
pub mod divergence;
mod persistence;
mod logging;

#[cfg(test)]
mod tests;

pub use schema::{
    EntityRef, EntityKind, SpanId, DecisionTier, TraceEvent,
    DecisionKind, DecisionContext, DecisionId, TracedDecision,
    TopologyDelta,
    EULER_OP_FEATURE_SCOPE,
};

pub use decision_log::{
    DecisionLog, DecisionSummary, SpanSummaryEntry,
    TraceSummary, TraceDiff,
};

pub use checkpoint_diff::{
    DecisionDelta, DecisionChange, CheckpointLog, diff_decision_logs,
};

pub use divergence::{
    DivergenceReport, DivergenceDetail, scan_for_divergences,
};

pub use persistence::{resolve_trace_dir, write_trace_file};

pub use logging::{LogLevel, log_level, log_result, log_decision_log, log_error};
