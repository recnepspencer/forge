//! Derived-cache policy and dirty-state framework.
//!
//! DOMAIN: Shared orchestration primitives for deterministic cache
//! invalidation/refresh across topology, geometry, and spatial crates.

mod data;

pub mod facade;

#[cfg(test)]
mod tests;

pub use facade::*;
