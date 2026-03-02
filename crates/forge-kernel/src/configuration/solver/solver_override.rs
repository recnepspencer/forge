//! Sparse solver configuration overrides.
//!
//! DOMAIN: Partial overrides for the solver section of the kernel configuration.

use serde::{Deserialize, Serialize};

/// Sparse overrides for `SolverSection`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SolverOverride {
    pub max_iterations: Option<usize>,
    pub convergence_tolerance: Option<f64>,
    pub divergence_threshold: Option<f64>,
}
