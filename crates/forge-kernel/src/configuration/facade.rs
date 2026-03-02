//! Public API for the configuration domain.

pub use super::kernel_config::{ConfigSection, KernelConfig};
pub use super::config_override::ConfigOverride;
pub use super::tolerance::{
    ToleranceSection, ToleranceOverride, UnitSystem,
    TolerancePolicy, TangencyPolicy, SliverPolicy, GapClosurePolicy, ToleranceConfig,
};
pub use super::tolerance::tolerance_section;
pub use super::solver::{SolverSection, SolverOverride};
pub use super::validation::{ValidationSection, ValidationOverride};
pub use super::policy_rules::{PolicySection, PolicyOverride};
pub use super::precision::{PrecisionSection, PrecisionOverride, PrecisionEscalationPolicy};
pub use super::provenance::{ConfigProvenance, ConfigScope, ConfigSource};
pub use super::resolve::resolve_config;
pub use super::resolved::{ResolvedConfig, ABSOLUTE_MINIMUM_TOLERANCE};
