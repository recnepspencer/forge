//! # worth-topo
//!
//! Worth topology semantics layered over the Forge runtime stack.
//!
//! This crate is intended to own topology materialization and interpretation
//! for Worth without becoming a second truth runtime.

#![forbid(unsafe_code)]

mod certification;
mod bridge;
mod data;
mod interpretation;
mod materialization;
mod reader;
mod runtime_invariants;
mod validators;

pub mod facade;
