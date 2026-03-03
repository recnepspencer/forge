//! Transactional key-value storage primitives.
//!
//! DOMAIN: Generic infrastructure for mapping typed keys to values
//! with transactional commit/rollback semantics.
//!
//! STRUCTURE:
//!   facade.rs — Public API surface (§7)
//!   data/     — Type definitions (structs)
//!   logic/    — Behavioral impls (methods, Default)

mod data;
mod logic;

pub mod facade;

#[cfg(test)]
mod tests;

pub use facade::*;
