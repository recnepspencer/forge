//! # worth-topo
//!
//! Worth topology semantics layered over the Forge runtime stack.
//!
//! This crate is intended to own topology materialization and interpretation
//! for Worth without becoming a second truth runtime.

#![forbid(unsafe_code)]

mod bridge;
mod certification;
mod data;
mod diagnostics;
mod edit;
mod fixtures;
mod interpretation;
mod materialization;
mod parity;
mod query;
mod reader;
mod runtime_invariants;
mod validators;

pub mod facade;
