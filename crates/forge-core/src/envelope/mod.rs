//! Operation result envelope for the Forge geometry kernel.
//!
//! DOMAIN: Universal return type wrapping every kernel operation's
//! result alongside decision logs, warnings, metrics, and lineage.
//!
//! STRUCTURE:
//!   facade.rs — Public API surface (§7)
//!   data/     — Type definitions (enums, structs)
//!   logic/    — Behavioral impls (Display, methods)

mod data;
mod logic;

pub mod facade;

#[cfg(test)]
mod tests;

pub use facade::*;
