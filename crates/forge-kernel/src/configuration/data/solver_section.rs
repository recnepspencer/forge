//! Solver section of the unified configuration.
//!
//! DOMAIN: Configuration for iterative numeric solvers.
//! INVARIANTS: max_iterations > 0, convergence_tolerance > 0.

use forge_core::KernelError;
use serde::{Deserialize, Serialize};

use super::defaults;
use super::kernel_config::ConfigSection;

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
            max_iterations: defaults::MAX_ITERATIONS,
            convergence_tolerance: defaults::CONVERGENCE_TOLERANCE,
            divergence_threshold: defaults::DIVERGENCE_THRESHOLD,
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
