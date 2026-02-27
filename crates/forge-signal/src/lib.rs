//! # forge-signal
//!
//! Reactive signal graph for the Forge geometry kernel.
//!
//! ## Architecture
//!
//! Every CAD feature (Extrude, Boolean, Fillet) is a signal node in
//! a dependency graph. Euler operators remain procedural inside
//! `MutableDraft` transactions — signals operate at feature granularity.
//!
//! ## Core Concepts
//!
//! - **Three-state invalidation** ([`schema::NodeState`]):
//!   `Clean(Version)` / `MaybeStale` / `Dirty`
//! - **Multi-aspect versioning** ([`schema::AspectVersion`]):
//!   Topology and geometry versions are independent
//! - **Push phase** ([`eval::mark_dirty`]):
//!   Synchronous dirty propagation with cycle detection
//! - **Pull phase** ([`eval::evaluate`]):
//!   Lazy recomputation with version-gated skip
//! - **Parallel safety** ([`context::EvaluationContext`]):
//!   Explicit context object, not thread-local (Doctrine D8)
//!
//! ## Dependencies
//!
//! - `forge-core`: `KernelError` for structured errors
//!
//! ## Dependents
//!
//! - `forge-kernel`: Features register as signal nodes

#![forbid(unsafe_code)]

pub mod evaluation;
pub mod graph;
pub mod handles;
pub mod prelude;
pub mod schema;

pub use evaluation::{evaluate, mark_dirty, EvaluationContext};
pub use graph::SignalGraph;
pub use handles::NodeId;
pub use schema::{Aspect, DependencyEdge};

#[cfg(test)]
mod tests;
