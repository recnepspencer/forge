//! `forge-runtime-bridge` owns the runtime bridge facade and its internal
//! subdomain boundaries.
//!
//! Milestone 1 currently includes canonical truth-envelope and snapshot input
//! contracts, build-time-frozen bridge mapping registration, and deterministic
//! routing plan / invalidation artifact lowering.

#![forbid(unsafe_code)]

mod adapter;
mod builder;
mod clone_budget;
mod delivery;
mod diagnostics;
mod error;
pub mod facade;
mod identity;
mod input;
mod mapping;
mod policy;
mod routing;
mod snapshot;

#[cfg(test)]
mod harness;

#[cfg(test)]
mod tests {}
