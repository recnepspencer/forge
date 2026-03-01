//! Kernel configuration domain component.
//!
//! DOMAIN: Unified configuration schema, override cascade, resolution,
//! provenance tracking, tolerance policies, and the `check_tolerance!` macro.
//!
//! INVARIANTS:
//! - Every configuration field has a named default in `defaults`.
//! - All sections can be validated independently; cross-section invariants
//!   are enforced by `ResolvedConfig::cross_validate()`.
//! - Provenance tracks the cascade source of every resolved field.
//!
//! DEPENDENCIES:
//! - `forge-core` (`KernelError`, `PolicyKind`, `ValidationCheckpoint`, `DecisionTier`)
//! - `serde` (serialization)

pub mod defaults;
pub mod facade;
mod macros;
pub mod overrides;
pub mod policy;
pub mod provenance;
pub mod resolve;
pub mod resolved;
pub mod schema;

#[cfg(test)]
mod tests;
