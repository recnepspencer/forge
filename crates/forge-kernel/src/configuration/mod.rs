//! Kernel configuration domain component.
//!
//! DOMAIN: Unified configuration schema, override cascade, resolution,
//! provenance tracking, tolerance policies, and the `check_tolerance!` macro.
//!
//! INVARIANTS:
//! - Every configuration field has a named default co-located with its section.
//! - All sections can be validated independently; cross-section invariants
//!   are enforced by `ResolvedConfig::cross_validate()`.
//! - Provenance tracks the cascade source of every resolved field.
//!
//! DEPENDENCIES:
//! - `forge-core` (`KernelError`, `PolicyKind`, `ValidationCheckpoint`, `DecisionTier`)
//! - `serde` (serialization)

// ── Vertical slices ──────────────────────────────────────────────────
mod tolerance;
mod solver;
mod validation;
mod policy_rules;
mod precision;
mod diagnostics;

// ── Cross-cutting root ───────────────────────────────────────────────
mod kernel_config;
mod config_override;
mod resolve;
pub mod resolved;
mod provenance;

pub mod facade;
mod macros;

#[cfg(test)]
mod tests;
