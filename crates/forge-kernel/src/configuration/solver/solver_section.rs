//! Solver section of the unified configuration.
//!
//! DOMAIN: Configuration for iterative numeric solvers and default constants.
//! INVARIANTS: max_iterations > 0, convergence_tolerance > 0.

use forge_core::KernelError;
use serde::{Deserialize, Serialize};

use super::super::kernel_config::ConfigSection;

// ── Default constants ────────────────────────────────────────────────

/// Maximum iterations for iterative numeric solvers (e.g. Newton-Raphson).
pub const MAX_ITERATIONS: usize = 50;

/// Target numeric residual for solver convergence.
pub const CONVERGENCE_TOLERANCE: f64 = 1e-10;

/// Bail-out threshold if an iterative solver is diverging.
pub const DIVERGENCE_THRESHOLD: f64 = 1e2;

// ── Solver section ───────────────────────────────────────────────────

/// Configuration for iterative numeric solvers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolverSection {
    pub max_iterations: usize,
    pub convergence_tolerance: f64,
    pub divergence_threshold: f64,
}

impl ConfigSection for SolverSection {
    fn defaults() -> Self {
        Self {
            max_iterations: MAX_ITERATIONS,
            convergence_tolerance: CONVERGENCE_TOLERANCE,
            divergence_threshold: DIVERGENCE_THRESHOLD,
        }
    }

    fn validate(&self) -> Result<(), KernelError> {
        if self.max_iterations == 0 {
            return Err(KernelError::InvalidConfig {
                field: "max_iterations".into(),
                reason: "must be > 0".into(),
            });
        }
        if self.convergence_tolerance <= 0.0 {
            return Err(KernelError::InvalidConfig {
                field: "convergence_tolerance".into(),
                reason: "must be positive".into(),
            });
        }
        Ok(())
    }
}

impl Default for SolverSection {
    fn default() -> Self {
        Self::defaults()
    }
}
