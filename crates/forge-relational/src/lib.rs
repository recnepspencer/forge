//! # forge-relational
//!
//! Deterministic truth-state runtime infrastructure for high-consequence graph
//! domains such as geometry kernels, chip-design systems, and other workloads
//! that require durable identity, transactional mutation, replay, lineage, and
//! audit-grade diagnostics.
//!
//! The crate is intentionally shaped around the Forge domain standards:
//!
//! - component-oriented structure
//! - explicit `presentation` / `logic` / `data` layers
//! - a single public facade boundary
//! - contracts that preserve serialized authority and immutable read semantics
//!
//! The initial scaffold is implementation-light and contract-heavy on purpose.
//! For this runtime, getting the boundaries right early is materially more
//! important than racing toward a shallow feature-complete prototype.
//!
//! Local developer note for Milestone 2 aspect semantics:
//! `src/aspect_truth_flow.md`

#![forbid(unsafe_code)]
#![deny(unreachable_patterns)]

mod authority;
mod capabilities;
mod commit_strategies;
mod config;
mod diagnostics;
mod durability;
mod errors;
mod grouped_truth;
mod history;
mod identity;
mod indexes;
mod inspection;
mod lineage;
mod logic;
mod merge;
mod payloads;
mod performance;
mod presentation;
mod publication;
mod query;
mod replay;
mod schema;
mod simulation;
mod snapshots;
mod storage;
mod symbols;
mod transactions;
mod validation;
mod visibility;

pub mod facade;

#[cfg(test)]
mod testing;

#[cfg(test)]
mod tests;
