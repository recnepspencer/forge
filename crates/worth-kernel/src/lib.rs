//! `worth-kernel` no longer ships a public primitive-construction runtime or
//! authoring facade.
//!
//! The kernel does not export a public geometry runtime facade, a public
//! certification bucket, a public query-proof / realization-proof report
//! warehouse, public replay / branch-preview-runtime / hostility-suite proof
//! products, a public primitive-construction lane, a public `facade` namespace,
//! or a second local runtime.

#![forbid(unsafe_code)]

extern crate self as worth_kernel;

pub mod docs_closeout;
pub mod graph_read_access_declarations;
pub mod graph_read_access_inventory;
pub mod graph_read_access_plan_adoption;
pub mod query_adoption;
mod query_authoring_helpers;
pub mod query_graph_authority_gate;
#[doc(hidden)]
pub mod query_obligation_selection;
pub mod replay_undo_consumer_cutover;
pub mod replay_undo_family_catalog;
pub mod replay_undo_inventory;
pub mod replay_undo_transaction_boundary;
pub mod workload_composition;

#[cfg(test)]
mod binding;
mod construction;
