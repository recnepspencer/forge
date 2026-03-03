//! Tolerance sweep runner for parametric tolerance testing.
//!
//! DOMAIN: Runs the same test logic across a range of tolerance values
//! to catch tolerance-boundary bugs — operations that work at 1e-6
//! but fail at 1e-10 because a comparison flips.

use forge_core::KernelError;
use crate::configuration::facade::ResolvedConfig;

use super::builders::configs::config_with_spatial_tolerance;

/// Standard tolerance range for sweep tests.
pub const STANDARD_TOLERANCES: &[f64] = &[1e-14, 1e-10, 1e-6, 1e-3];

/// Tight-only tolerance range (for operations known to be robust).
pub const TIGHT_TOLERANCES: &[f64] = &[1e-14, 1e-12, 1e-10];

/// Run a test function across multiple tolerance values.
///
/// On failure, the panic message includes which tolerance caused it.
///
/// # Example
///
/// ```rust,ignore
/// tolerance_sweep(STANDARD_TOLERANCES, |config| {
///     let env = shapes::cube_with_config([0.;3], 1.0, config)?;
///     verify(&env).manifold().euler(2).pass();
///     Ok(())
/// });
/// ```
pub fn tolerance_sweep<F>(tolerances: &[f64], test_fn: F)
where
    F: Fn(&ResolvedConfig) -> Result<(), KernelError>,
{
    let mut failures = Vec::new();

    for &tol in tolerances {
        let config = config_with_spatial_tolerance(tol);
        if let Err(e) = test_fn(&config) {
            failures.push(format!("  tol={:.2e}: {:?}", tol, e));
        }
    }

    if !failures.is_empty() {
        panic!(
            "Tolerance sweep failed at {}/{} tolerances:\n{}",
            failures.len(),
            tolerances.len(),
            failures.join("\n")
        );
    }
}

/// Run a test function that can also panic (not just return Err).
///
/// Catches panics and reports them alongside the tolerance value.
pub fn tolerance_sweep_catch_panics<F>(tolerances: &[f64], test_fn: F)
where
    F: Fn(&ResolvedConfig) + std::panic::RefUnwindSafe,
{
    let mut failures = Vec::new();

    for &tol in tolerances {
        let config = config_with_spatial_tolerance(tol);
        let result = std::panic::catch_unwind(|| test_fn(&config));
        if let Err(e) = result {
            let msg = if let Some(s) = e.downcast_ref::<String>() {
                s.clone()
            } else if let Some(s) = e.downcast_ref::<&str>() {
                s.to_string()
            } else {
                "unknown panic".to_string()
            };
            failures.push(format!("  tol={:.2e}: {}", tol, msg));
        }
    }

    if !failures.is_empty() {
        panic!(
            "Tolerance sweep failed at {}/{} tolerances:\n{}",
            failures.len(),
            tolerances.len(),
            failures.join("\n")
        );
    }
}
