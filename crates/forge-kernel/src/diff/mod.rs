//! Diff domain component.
//!
//! DOMAIN: Computes "what changed?" snapshots across topology mutations.

pub mod facade;
mod topology_delta;

pub use topology_delta::{ArenaSnapshot, compute_topology_delta};
