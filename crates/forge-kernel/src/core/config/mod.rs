//! # Config — Unified kernel configuration
//!
//! DOMAIN: Centralized defaults, future home for KernelConfig, ConfigOverride,
//! ResolvedConfig, and cascade resolution.
//!
//! ## Modules
//!
//! - `defaults` — All named default constants (the one source of truth)

pub mod defaults;
pub mod overrides;
pub mod provenance;
pub mod resolve;
pub mod schema;

#[cfg(test)]
mod tests;

pub use schema::{
    ConfigSection, KernelConfig, PolicySection, PrecisionSection, SolverSection, ToleranceSection,
    UnitSystem, ValidationSection,
};

pub use overrides::{
    ConfigOverride, PolicyOverride, PrecisionOverride, SolverOverride, ToleranceOverride,
    ValidationOverride,
};
pub use provenance::{ConfigProvenance, ConfigScope, ConfigSource};
pub use resolve::{resolve_config, ResolvedConfig};
