//! Public API for the configuration domain.

pub use super::config_override::ConfigOverride;
pub use super::diagnostics::{
    DiagnosticsSection, FingerprintDetail, GeometryValidationDepth, TraceVerbosity,
};
pub use super::kernel_config::{ConfigSection, KernelConfig};
pub use super::policy_rules::{PolicyOverride, PolicySection};
pub use super::precision::{PrecisionEscalationPolicy, PrecisionOverride, PrecisionSection};
pub use super::provenance::{ConfigProvenance, ConfigScope, ConfigSource};
pub use super::resolve::resolve_config;
pub use super::resolved::{ResolvedConfig, ABSOLUTE_MINIMUM_TOLERANCE};
pub use super::solver::{SolverOverride, SolverSection};
pub use super::tolerance::tolerance_section;
pub use super::tolerance::{
    GapClosurePolicy, SliverPolicy, TangencyPolicy, ToleranceConfig, ToleranceOverride,
    TolerancePolicy, ToleranceSection, UnitSystem,
};
pub use super::validation::{ValidationOverride, ValidationSection};
