//! # Topology
//!
//! Topology semantics layered over the Forge runtime stack.
//!
//! This crate is intended to own topology materialization and interpretation
//! without becoming a second truth runtime.

#![forbid(unsafe_code)]

mod brep;
pub mod certification;
mod compiled_product_family;
mod compiled_product_reuse_decision;
mod construction;
pub mod derived_invalidation_authority_inventory;
mod derived_invalidation_compiled_product_admission;
pub mod derived_invalidation_deletion_closeout;
pub mod derived_invalidation_execution;
pub mod derived_invalidation_family_catalog;
pub mod derived_invalidation_migrated_products;
pub mod derived_invalidation_milestone_ten_closeout;
pub mod derived_invalidation_operator_cutover;
pub mod derived_invalidation_route_input;
pub mod derived_invalidation_selected_plan;
pub mod derived_read_diagnostic_input;
mod derived_topology;
mod projection;
mod query_adoption;
pub mod query_domain;
pub mod query_native_runtime_boundary;
mod relational_aspect_boundary;
pub mod replay_family_catalog;
pub mod replay_undo_semantic_graph;
pub mod runtime_support;
mod selected_equivalence_family;
mod test_support;
mod topology_operators;
pub mod touched_graph_conflict;
pub mod undo_family_catalog;
mod validation;
pub mod validation_authority_inventory;
pub mod validator_invariant_catalog;
pub mod workload_platform;

pub mod facade;
