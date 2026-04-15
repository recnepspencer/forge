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

mod authoring;
mod basis;
mod binding;
mod canonicalization;
mod collection;
mod diagnostics;
mod execution;
pub mod facade;
mod identity;
mod planning;
mod result_shape;
#[macro_use]
mod schema_macro;
mod schema_view;
mod typed;
mod validation;

#[cfg(test)]
mod harness;
