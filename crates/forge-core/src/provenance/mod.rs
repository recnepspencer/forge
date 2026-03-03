//! Serializable provenance payloads for tracing and audit artifacts.
//!
//! STRUCTURE:
//!   facade.rs — Public API surface (§7)
//!   data/     — Type definitions (structs, enums)
//!   logic/    — Behavioral impls (hashing, validation)

mod data;
mod logic;

pub mod facade;

#[cfg(test)]
mod tests;

pub use facade::*;
