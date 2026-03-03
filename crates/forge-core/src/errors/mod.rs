//! Error taxonomy for the Forge geometry kernel.
//!
//! DOMAIN: Shared error types that every Forge crate speaks.
//!
//! STRUCTURE:
//!   facade.rs — Public API surface (§7)
//!   data/     — Type definitions (enums, structs)
//!   logic/    — Behavioral impls (Display, From, methods)
//!   summary/  — Serializable audit-artifact projections

mod data;
mod logic;
mod summary;

pub mod facade;

#[cfg(test)]
mod tests;

pub use facade::*;
