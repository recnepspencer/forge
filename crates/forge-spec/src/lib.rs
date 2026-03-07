//! # forge-spec
//!
//! Typed specification graph truth runtime for Forge.
//! The specification graph is the source of truth. Topology, geometry,
//! audit products, and other runtime products are derived projections.
//!
//! `forge-spec` owns:
//! - immutable committed spec snapshots
//! - mutable spec drafts
//! - typed node/relation schema
//! - graph-native replay, lineage, and naming records
//! - deterministic serialization and hashing boundaries
//!
//! `forge-spec` does not own:
//! - reactive scheduling
//! - B-Rep projection storage
//! - geometry numerics
//! - UI-specific state projection

#![forbid(unsafe_code)]

mod data;
mod logic;
mod presentation;

pub mod facade;

#[cfg(test)]
mod tests;
