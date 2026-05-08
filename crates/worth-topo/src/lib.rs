//! # Topology
//!
//! Topology semantics layered over the Forge runtime stack.
//!
//! This crate is intended to own topology materialization and interpretation
//! without becoming a second truth runtime.

#![forbid(unsafe_code)]

mod brep;
mod certification;
mod derived_topology;
mod projection;
mod test_support;
mod topology_operators;
mod validation;

pub mod facade;
