//! `forge-query` owns the typed query facade and canonical query artifact
//! authority.
//!
//! Milestone 1 establishes:
//!
//! - raw authored query and result-shape forms
//! - proof-carrying canonical query and result-shape artifacts
//! - canonical bundle construction with explicit compatibility checks
//! - a single public facade for ordinary consumers

#![forbid(unsafe_code)]

mod application;
mod authoring;
mod basis;
mod binding;
mod canonicalization;
mod collection;
mod composition;
mod correspondence;
mod correspondence_history;
mod correspondence_history_parity;
mod declarative_live;
mod diagnostics;
mod execution;
pub mod facade;
mod frontier_planning;
mod frontier_signal_adapter;
mod historical;
mod identity;
mod identity_evolution;
mod live;
mod live_performance;
mod memory_workspace;
mod planning;
mod preview;
mod query_context;
mod result_shape;
mod saved_query;
#[macro_use]
mod schema_macro;
mod schema_view;
mod typed;
mod validation;
mod view_shape;
mod view_shape_live;
mod workflow;

#[cfg(test)]
mod harness;
