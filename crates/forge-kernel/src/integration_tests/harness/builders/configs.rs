//! Config fixtures for integration tests.
//!
//! DOMAIN: Provides pre-configured `ResolvedConfig` instances with
//! different tolerance values. Equivalent to Laravel's
//! `config(['timezone' => ...])` for per-test environment control.

use crate::configuration::facade::{
    resolve_config, ConfigOverride, KernelConfig, ResolvedConfig, ToleranceOverride,
};

/// Default test configuration.
pub fn test_config() -> ResolvedConfig {
    resolve_config(&KernelConfig::default(), None, None, None).unwrap()
}

/// Tight tolerance config — catches near-degenerate geometry.
pub fn config_tight() -> ResolvedConfig {
    config_with_spatial_tolerance(1e-10)
}

/// Loose tolerance config — for stress-testing robustness.
pub fn config_loose() -> ResolvedConfig {
    config_with_spatial_tolerance(1e-3)
}

/// Config with a specific spatial tolerance value.
///
/// Uses a `ConfigOverride` at the operation level to override only
/// the `spatial_tolerance` field, preserving all other defaults.
pub fn config_with_spatial_tolerance(tol: f64) -> ResolvedConfig {
    let base = KernelConfig::default();
    let tol_override = ConfigOverride {
        tolerance: Some(ToleranceOverride {
            spatial_tolerance: Some(tol),
            ..Default::default()
        }),
        ..Default::default()
    };
    resolve_config(&base, None, None, Some((&tol_override, None))).unwrap()
}
