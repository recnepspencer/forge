//! Kernel configuration domain component.
//!
//! DOMAIN: Unified configuration schema, overrides, resolution, and provenance.

pub mod defaults;
pub mod facade;
pub mod overrides;
pub mod provenance;
pub mod resolve;
pub mod schema;

#[cfg(test)]
mod tests;

pub use overrides::{
    ConfigOverride, PolicyOverride, PrecisionOverride, SolverOverride, ToleranceOverride,
    ValidationOverride,
};
pub use provenance::{ConfigProvenance, ConfigScope, ConfigSource};
pub use resolve::{resolve_config, ResolvedConfig};
pub use schema::{
    ConfigSection, KernelConfig, PolicySection, PrecisionSection, SolverSection, ToleranceSection,
    UnitSystem, ValidationSection,
};
