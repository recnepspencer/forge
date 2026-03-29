//! # forge-signal
//!
//! `forge-signal` is a deterministic incremental runtime for derived work.
//!
//! Your app owns the real state.
//! `forge-signal` owns dependency tracking, invalidation, recompute, rollback,
//! diagnostics, replay, and history.
//!
//! There are two normal entry paths:
//!
//! ```rust
//! use forge_signal::easy::*;
//! use forge_signal::facade::*;
//! ```
//!
//! Use `easy` for the shortest path.
//! Use `facade` when you want the broader runtime surface from the start.
//!
//! The important line is this:
//!
//! - not "reactive graph plus some debug helpers"
//! - not "incremental cache plus a separate audit layer"
//! - not "rerun less work and figure out the rest later"
//!
//! `forge-signal` keeps change propagation, transactions, diagnostics, and
//! history in one runtime.
//!
//! Most days, the center of gravity is:
//!
//! - [`facade::SignalGraph`]
//! - [`facade::SignalRuntime`]
//! - `runtime.transaction(...)`
//! - `runtime.diagnostics()`
//! - `runtime.history()`
//!
//! ## Fast Mental Model
//!
//! - Build a dependency graph.
//! - Tell the runtime what changed.
//! - Read the derived node you care about.
//! - Ask diagnostics why something ran when it should not have.
//! - Use history when you need the trail, not just the latest answer.
//!
//! `forge-signal` is domain-free on purpose. The same runtime shape works for:
//!
//! - web backends and reactive views
//! - finance and risk pipelines
//! - ML feature and scoring flows
//! - geometry or compiler-style partial recompute
//!
//! The flagship story looks like this:
//!
//! - a source file changes
//! - a transaction lands the update
//! - only the right downstream targets rerun
//! - diagnostics explain why the bundle moved
//! - replay keeps the trail
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
//! - `crates/forge-signal/docs/GETTING_STARTED.md`
//! - `crates/forge-signal/docs/API_OVERVIEW.md`
//! - `crates/forge-signal/docs/walkthroughs/compiler-targeted-rebuild.md`
//! - `crates/forge-signal/docs/guides/running-the-runtime.md`
//! - `crates/forge-signal/docs/guides/debugging-and-diagnostics.md`
//!
//! Examples live in:
//!
//! - `crates/forge-signal/examples/easy_task_board.rs`
//! - `crates/forge-signal/examples/compiler_targeted_rebuild.rs`
//! - `crates/forge-signal/examples/geometry_partial_recompute.rs`

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
