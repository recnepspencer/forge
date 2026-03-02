//! Core decision data types for the tracing protocol.
//!
//! DOMAIN: Every type in this slice defines the shape of a kernel decision:
//! what was decided, how it was resolved, how significant it was, and which
//! entities were affected.

mod decision_kind;
mod decision_tier;
mod entity_ref;
mod span;
mod traced_decision;

pub use decision_kind::{DecisionContext, DecisionKind};
pub use decision_tier::DecisionTier;
pub use entity_ref::{EntityKind, EntityRef};
pub use span::{SpanId, TraceEvent, EULER_OP_FEATURE_SCOPE};
pub use traced_decision::{DecisionId, TopologyDelta, TracedDecision};
