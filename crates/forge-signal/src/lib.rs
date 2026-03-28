//! # forge-signal
//!
//! `forge-signal` is a deterministic incremental runtime for derived work.
//!
//! Your app owns the real state.
//! `forge-signal` owns dependency tracking, invalidation, recompute, rollback,
//! and diagnostics.
//!
//! The main import path is:
//!
//! ```rust
//! use forge_signal::facade::*;
//! ```
//!
//! Most days, the center of gravity is:
//!
//! - [`facade::SignalGraph`]
//! - [`facade::SignalRuntime`]
//! - `runtime.transaction(...)`
//! - `runtime.diagnostics()`
//!
//! ## Fast Mental Model
//!
//! - Build a dependency graph.
//! - Tell the runtime what changed.
//! - Read the derived node you care about.
//! - Ask diagnostics why something ran when it should not have.
//!
//! `forge-signal` is domain-free on purpose. The same runtime shape works for:
//!
//! - web backends and reactive views
//! - finance and risk pipelines
//! - ML feature and scoring flows
//! - geometry or compiler-style partial recompute
//!
//! ## Small Example
//!
//! ```no_run
//! use forge_signal::facade::*;
//! const PRICE: Aspect = Aspect::new(0);
//! const TOTAL: Aspect = Aspect::new(1);
//!
//! #[derive(Default)]
//! struct CheckoutState {
//!     price_version: u64,
//!     total_version: u64,
//! }
//!
//! let mut graph = SignalGraph::new();
//! let price = graph.node().build();
//! let total = graph.node().on_demand().build();
//!
//! graph.set_dependencies(total, [DependencyEdge::new(price, PRICE)])?;
//!
//! let mut runtime = SignalRuntime::build_for::<CheckoutState>(graph);
//!
//! let mut state = CheckoutState {
//!     price_version: 2,
//!     total_version: 5,
//! };
//!
//! let evaluate = |view: &mut EvaluationContext<'_, CheckoutState>| {
//!     let result = if view.node() == price {
//!         view.finish(NodeEvaluationResult::from_version(
//!             AspectVersion::from_updates([(PRICE, view.domain().price_version)]),
//!         ))
//!     } else {
//!         let _upstream = view.read_aspect_version(price, PRICE)?;
//!         view.finish(NodeEvaluationResult::from_version(
//!             AspectVersion::from_updates([(TOTAL, view.domain().total_version)]),
//!         ))
//!     };
//!     Ok::<_, SignalError>(result)
//! };
//!
//! runtime.transaction(&mut state, |tx| {
//!     tx.mark_changed(price, PRICE)?;
//!     tx.target(total).read(&evaluate)?;
//!     Ok(())
//! })?;
//!
//! let version = runtime.target(total).read(&state, &evaluate)?;
//! assert_eq!(version.get(TOTAL), 5);
//! # Ok::<(), SignalError>(())
//! ```
//!
//! ## Docs
//!
//! Start with:
//!
//! - `crates/forge-signal/docs/QUICKSTART.md`
//! - `crates/forge-signal/docs/DAILY_WORKFLOWS.md`
//! - `crates/forge-signal/docs/API_OVERVIEW.md`
//! - `crates/forge-signal/docs/DIAGNOSTICS.md`
//!
//! Examples live in:
//!
//! - `crates/forge-signal/examples/web_live_search.rs`
//! - `crates/forge-signal/examples/finance_risk_refresh.rs`
//! - `crates/forge-signal/examples/ml_feature_pipeline.rs`

#![forbid(unsafe_code)]

mod data;
pub mod diagnostics;
pub mod easy;
mod logic;
#[cfg(not(test))]
mod presentation;
#[cfg(test)]
pub mod presentation;
mod state;

pub mod facade;

#[cfg(test)]
mod tests;
