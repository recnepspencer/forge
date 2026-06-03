//! # worth-spatial
//!
//! `worth-spatial` owns authored spatial vocabulary and spatial semantics.
//! `forge-query` owns runtime-facing declaration, inspection, and workflow
//! lifecycle.
//!
//! Use the namespaced [`facade`] modules as the public entry surface:
//!
//! - [`facade::refs`] for authored reference vocabulary
//! - [`facade::witness_catalog`] and [`facade::witness_resolution`] for
//!   witness meaning
//! - [`facade::frames`], [`facade::placement`], [`facade::motion`], and
//!   [`facade::constraints`] for semantic admission and application
//! - [`facade::lowering`] for explicit Query declaration handoff
//! - [`facade::arbitration`] for conflict, preview, continuity, and policy
//! - [`facade::bindings`] for primitive-birth planning and consequence meaning

#![forbid(unsafe_code)]

mod bindings;
mod spatial_intent;
#[cfg(test)]
mod structure_guard;
#[cfg(test)]
mod test_support;

pub mod facade;
