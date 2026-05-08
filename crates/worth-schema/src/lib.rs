//! # -schema
//!
//!  truth vocabulary built for the Forge runtime stack.
//!
//! This crate defines the domain names  uses at the truth boundary:
//!
//! - entity kinds
//! - relation kinds
//! - aspect vocabulary
//! - invariant groups
//!
//! It does not execute mutations, own topology materialization, or schedule
//! derived computation.

#![forbid(unsafe_code)]

mod data;
mod topology_authoring;

pub mod facade;
