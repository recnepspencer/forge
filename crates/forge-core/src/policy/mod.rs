//! Policy types for the Forge geometry kernel.
//!
//! DOMAIN: Three-state return type (`PolicyResult<T>`) and structured
//! policy queries for Doctrine D2 (Ambiguity Escalation). When a
//! geometry solver can't decide, it returns `PolicyResult::Ambiguous`
//! and the kernel layer applies the appropriate policy.
//!
//! DEPENDENCIES: serde
//!
//! STRUCTURE:
//!   data/  — Type definitions (enums, structs)
//!   logic/ — Behavioral impls (methods, conversions)

pub(crate) mod data;
mod logic;

#[cfg(test)]
mod tests;

pub use data::{PolicyKind, PolicyQuery, PolicyResult, ValidationCheckpoint};
