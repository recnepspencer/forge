//! # forge-signal
//!
//! Generic, deterministic reactive computation runtime for host-managed state graphs.
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
//! - **In-place transactions with hard rewind** ([`facade::SignalRuntime`]):
//!   graph writes happen in-place and are restored from sparse patches on failure
//! - **Multi-aspect versioning** ([`facade::AspectVersion`]):
//!   User-defined aspect slots carry independent version counters
//! - **Push phase** ([`facade::mark_dirty`]):
//!   Synchronous dirty propagation with cycle detection
//! - **Pull phase**:
//!   Planner-backed prepared precompute plus deterministic serial apply
//! - **Condition-gated evaluation** ([`facade::EvaluationCondition`]):
//!   on-demand, aspect-filtered, threshold, debounce, and custom evaluation policies,
//!   exposed through readable builder helpers on [`facade::NodeBuilder`]
//! - **Partition-aware subscriptions** ([`facade::PartitionSubscription`]):
//!   downstream nodes can subscribe to one partition or one partition/detail pair
//!   instead of invalidating on every change to a large artifact
//! - **Parallel safety** ([`facade::ExecutionReadView`]):
//!   Immutable execution snapshot for stage-local precompute, not thread-local mutation
//! - **Diagnostics-first observability**:
//!   production summaries, diffs, inspectors, failure diagnostics, and causal flow reporting
//! - **Productized runtime surface**:
//!   [`facade::SignalRuntime::builder`] and [`easy::ReactiveGraph`]
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
pub mod diagnostics;
pub mod easy;
mod logic;
mod presentation;
mod state;

pub mod facade;

#[cfg(test)]
mod tests;
