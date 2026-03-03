//! Policy types for the Forge geometry kernel.
//!
//! DOMAIN: Three-state return type (`PolicyResult<T>`) and structured
//! policy queries for Doctrine D2 (Ambiguity Escalation).
//!
//! STRUCTURE:
//!   facade.rs — Public API surface (§7)
//!   data/     — Type definitions (enums, structs)
//!   logic/    — Behavioral impls (methods, conversions)

mod data;
mod logic;

pub mod facade;

#[cfg(test)]
mod tests;

pub use facade::*;
