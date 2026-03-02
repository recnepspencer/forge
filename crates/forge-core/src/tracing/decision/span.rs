//! Span protocol types for structured trace recording.
//!
//! DOMAIN: `SpanId` identifies logical phases within a `DecisionLog`.
//! `TraceEvent` is the protocol event that the log stores.

use std::fmt;

use serde::{Deserialize, Serialize};

use super::traced_decision::TracedDecision;

/// Sentinel `feature_scope` value for low-level Euler operator decisions.
///
/// Decisions tagged with this scope are filtered out in compact display
/// and only shown in verbose/full display mode.
pub const EULER_OP_FEATURE_SCOPE: u64 = u64::MAX;

/// Unique identifier for a trace span within a `DecisionLog`.
///
/// Monotonically increasing within a single log instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SpanId(pub u64);

impl fmt::Display for SpanId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "span-{}", self.0)
    }
}

/// A single event in the trace protocol.
///
/// The `DecisionLog` stores a flat `Vec<TraceEvent>`. Tree structure is
/// reconstructed on read by matching `StartSpan`/`EndSpan` pairs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TraceEvent {
    /// An atomic kernel decision.
    Decision(TracedDecision),
    /// Start of a named scope (logical phase).
    StartSpan {
        /// Unique span identifier.
        id: SpanId,
        /// Parent span, if nested.
        parent_id: Option<SpanId>,
        /// Human-readable phase name.
        name: String,
    },
    /// End of a named scope, with computed duration.
    EndSpan {
        /// Must match a previous `StartSpan.id`.
        id: SpanId,
        /// Wall-clock duration in microseconds.
        duration_micros: u64,
    },
}
