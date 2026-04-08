//! `forge-runtime-bridge` owns the runtime bridge facade and its internal
//! subdomain boundaries.
//!
//! Milestone 1 currently includes canonical truth-envelope and snapshot input
//! contracts, build-time-frozen bridge mapping registration, and deterministic
//! routing plan / invalidation artifact lowering.
//!
//! Milestone 5 bulk planning begins by composing canonical single-route plans
//! into a replay-safe batch workload identity and summary without reintroducing
//! scalar orchestration at the public boundary.

#![forbid(unsafe_code)]

mod adapter;
mod builder;
mod clone_budget;
mod continuity;
mod delivery;
mod diagnostics;
mod error;
pub mod facade;
mod historical;
mod identity;
mod input;
mod mapping;
mod policy;
mod routing;
mod snapshot;
mod stream;

#[cfg(test)]
mod harness;

#[cfg(test)]
mod tests {}
