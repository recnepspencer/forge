//! Public API for the configuration domain.

pub use super::data::{
    ConfigOverride, ConfigSection, KernelConfig, PolicyOverride, PolicySection, PrecisionOverride,
    PrecisionSection, SolverOverride, SolverSection, ToleranceOverride, ToleranceSection,
    UnitSystem, ValidationOverride, ValidationSection,
};
pub use super::data::defaults;
pub use super::logic::provenance::{ConfigProvenance, ConfigScope, ConfigSource};
pub use super::logic::resolve::resolve_config;
pub use super::logic::resolved::{ResolvedConfig, ABSOLUTE_MINIMUM_TOLERANCE};
pub use super::policy::{
    GapClosurePolicy, PrecisionEscalationPolicy, SliverPolicy, TangencyPolicy, ToleranceConfig,
    TolerancePolicy,
};
