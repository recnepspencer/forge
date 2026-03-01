//! Public API for configuration.

pub use super::overrides::{
    ConfigOverride, PolicyOverride, PrecisionOverride, SolverOverride, ToleranceOverride,
    ValidationOverride,
};
pub use super::provenance::{ConfigProvenance, ConfigScope, ConfigSource};
pub use super::resolve::{resolve_config, ResolvedConfig};
pub use super::schema::{
    ConfigSection, KernelConfig, PolicySection, PrecisionSection, SolverSection, ToleranceSection,
    UnitSystem, ValidationSection,
};
