//! # worth-schema
//!
//! Truth vocabulary built for the Forge runtime stack.
//!
//! This crate defines the domain names Worth uses at the truth boundary:
//!
//! - platform aspect catalogs
//! - platform entity catalogs
//! - platform relation catalogs
//! - platform authority vocabulary
//! - query-facing schema vocabulary
//!
//! It does not own the public runtime workflow for topology execution,
//! inspection, or recovery. Query-backed runtime behavior belongs in
//! `forge-query`.

#![forbid(unsafe_code)]

mod data;
mod topology_authoring;

pub mod facade;
