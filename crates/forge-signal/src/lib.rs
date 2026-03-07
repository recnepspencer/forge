//! # forge-signal
//!
//! Reactive signal graph for the Forge geometry kernel.
//! `forge-signal` is domain-free infrastructure and does not own host
//! structural graphs.
//!
//! ## Architecture
//!
//! Two graph kinds must remain separate:
//! - **Evaluation dependency graph (owned here):** DAG only.
//! - **Structural host graph (external):** may be cyclic and is opaque input.
//!
//! See `crates/forge-signal/BOUNDARY_CONTRACT.md` for normative integration
//! boundaries.
//!
//! ## Core Concepts
//!
//! - **Three-state invalidation** ([`facade::NodeState`]):
//!   `Clean(Version)` / `MaybeStale` / `Dirty`
//! - **Multi-aspect versioning** ([`facade::AspectVersion`]):
//!   Topology and geometry versions are independent
//! - **Push phase** ([`facade::mark_dirty`]):
//!   Synchronous dirty propagation with cycle detection
//! - **Pull phase** ([`facade::evaluate`]):
//!   Lazy recomputation with version-gated skip
//! - **Parallel safety** ([`facade::EvaluationContext`]):
//!   Explicit context object, not thread-local (Doctrine D8)
//!
//! ## Dependencies
//!
//! - `serde`: snapshot-friendly data structures
//! - `tracing`: runtime instrumentation hooks
//!
//! ## Dependents
//!
//! - Any crate that needs deterministic reactive DAG evaluation

#![forbid(unsafe_code)]

mod data;
mod logic;
mod presentation;

pub mod facade;

#[cfg(test)]
mod tests;
