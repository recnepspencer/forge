//! # Config — Unified kernel configuration
//!
//! DOMAIN: Centralized defaults, future home for KernelConfig, ConfigOverride,
//! ResolvedConfig, and cascade resolution.
//!
//! ## Modules
//!
//! - `defaults` — All named default constants (the one source of truth)

pub mod defaults;
pub mod schema;
pub mod overrides;
pub mod provenance;
pub mod resolve;

#[cfg(test)]
mod tests;

pub use schema::{
    KernelConfig, ConfigSection, UnitSystem, ToleranceSection,
    SolverSection, ValidationSection, PolicySection, PrecisionSection,
};

pub use overrides::{
    ConfigOverride, ToleranceOverride, SolverOverride,
    ValidationOverride, PolicyOverride, PrecisionOverride,
};
pub use provenance::{ConfigScope, ConfigSource, ConfigProvenance};
pub use resolve::{ResolvedConfig, resolve_config};
