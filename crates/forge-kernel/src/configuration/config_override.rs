//! Root override compositor.
//!
//! DOMAIN: Aggregates all section-level overrides into a single override container
//! matching the structure of `KernelConfig`.

use serde::{Deserialize, Serialize};

use super::tolerance::ToleranceOverride;
use super::solver::SolverOverride;
use super::validation::ValidationOverride;
use super::policy_rules::PolicyOverride;
use super::precision::PrecisionOverride;

/// An override container matching the structure of `KernelConfig`, but entirely sparse.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConfigOverride {
    pub tolerance: Option<ToleranceOverride>,
    pub solver: Option<SolverOverride>,
    pub validation: Option<ValidationOverride>,
    pub policy: Option<PolicyOverride>,
    pub precision: Option<PrecisionOverride>,
}
