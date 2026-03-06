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
mod diagnostics;
mod policy_rules;
mod precision;
mod solver;
mod tolerance;
mod validation;

// ── Cross-cutting root ───────────────────────────────────────────────
mod config_override;
mod kernel_config;
mod provenance;
mod resolve;
pub mod resolved;

pub mod facade;
mod macros;

#[cfg(test)]
mod tests;
