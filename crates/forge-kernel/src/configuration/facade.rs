//! Public API for the configuration domain.

pub use super::overrides::{
    ConfigOverride, PolicyOverride, PrecisionOverride, SolverOverride, ToleranceOverride,
    ValidationOverride,
};
pub use super::policy::{
    GapClosurePolicy, PrecisionEscalationPolicy, SliverPolicy, TangencyPolicy,
    ToleranceConfig, TolerancePolicy,
};
pub use super::provenance::{ConfigProvenance, ConfigScope, ConfigSource};
pub use super::resolve::resolve_config;
pub use super::resolved::{ResolvedConfig, ABSOLUTE_MINIMUM_TOLERANCE};
pub use super::schema::{
    ConfigSection, KernelConfig, PolicySection, PrecisionSection, SolverSection,
    ToleranceSection, UnitSystem, ValidationSection,
};
