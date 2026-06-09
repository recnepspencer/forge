//! # worth-spatial
//!
//! `worth-spatial` owns authored spatial vocabulary and spatial semantics.
//! `forge-query` owns runtime-facing declaration, inspection, and workflow
//! lifecycle.
//!
//! Use the namespaced [`facade`] modules as the public entry surface:
//!
//! - [`facade::refs`] for authored reference vocabulary
//! - [`facade::refs`] for witness catalog vocabulary
//! - [`facade::anchor_selection`] for Query-native anchor and witness selection
//!   declaration entry and fact-delivery surfaces
//! - [`facade::anchor_binding`] for Query-native anchor binding declaration,
//!   evidence, identity, and projection surfaces
//! - [`facade::binding`] for Query-native primitive binding declaration,
//!   evidence, identity, and projection surfaces
//! - [`facade::rebinding`] for Query-native rebinding declaration, evidence,
//!   projection, and runtime-posture surfaces
//! - [`facade::placement`] for declarative placement vocabulary consumed by
//!   Query-native families
//! - [`facade::bindings`] for shared binding-site and anchor-carrier vocabulary
//! - [`facade::neighborhood`] for grouped local-neighborhood and topology
//!   replacement workflow surfaces
//! - [`facade::continuation`] for rebinding signal-compatibility and
//!   continuation workflow surfaces
//! - [`facade::inspection`] for retained historical, branch-local, and replay
//!   geometry surfaces
//! - [`facade::projection`] for receipt-backed geometry projection consumption
//! - [`facade::recovery`] for typed geometry recovery actions
//! - [`facade::support`] for public family inventory and applicability posture
//! - [`facade::tolerance`] for tolerance and precision certification families
//!
//! Certification and test harnesses should use the same family-owned
//! declaration and fact surfaces as ordinary runtime code rather than a
//! parallel support namespace.

#![forbid(unsafe_code)]

mod anchor_selection;
mod authored_refs;
mod bindings;
pub mod certification;
mod placement;
#[cfg(test)]
mod structure_guard;
#[cfg(test)]
mod test_support;
mod witness_resolution;

pub mod facade;
