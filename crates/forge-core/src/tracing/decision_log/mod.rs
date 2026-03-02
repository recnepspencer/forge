//! Decision log, querying, and output.
//!
//! DOMAIN: The `DecisionLog` is the queryable collection that stores all
//! `TraceEvent`s. This slice also includes fingerprinting (deterministic
//! hashing for comparison) and logging adapters (structured output).

mod decision_log;
mod fingerprint;
mod logging;

pub use decision_log::{DecisionLog, DecisionSummary, SpanSummaryEntry, TraceDiff, TraceSummary};
pub use fingerprint::{compute_trace_fingerprint, TraceFingerprint};
pub use logging::{log_decision_log, log_error, log_result};
