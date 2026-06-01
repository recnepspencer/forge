//! # Topology
//!
//! Topology semantics layered over the Forge runtime stack.
//!
//! This crate is intended to own topology materialization and interpretation
//! without becoming a second truth runtime.

#![forbid(unsafe_code)]

mod brep;
mod certification;
mod committed_artifact;
mod construction;
mod derived_topology;
mod projection;
pub mod query_domain;
mod relational_aspect_boundary;
pub mod runtime_support;
mod test_support;
mod topology_operators;
mod validation;

pub mod facade;
