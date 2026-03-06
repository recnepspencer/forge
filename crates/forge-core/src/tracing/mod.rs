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
//!
//! PUBLIC API: All external access goes through `facade`. Internal modules
//! are `pub(crate)` — only the facade is `pub`.

pub(crate) mod decision;
pub(crate) mod decision_log;
pub mod facade;
pub(crate) mod payload;
pub mod replay;
pub mod sink;

#[cfg(test)]
mod tests;

// ── Public API — all re-exports routed through the facade ────────────────────
pub use facade::*;
