//! KernelConfig and ConfigSection trait.
//!
//! DOMAIN: The unified configuration root for all kernel operations.
//! INVARIANTS: Each section can be validated independently. Cross-section
//! consistency is enforced by `ResolvedConfig::cross_validate()`.

use forge_core::KernelError;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::tolerance::ToleranceSection;
use super::solver::SolverSection;
use super::validation::ValidationSection;
use super::policy_rules::PolicySection;
use super::precision::PrecisionSection;

/// Trait for a sub-section of the unified configuration.
///
/// Guarantees every section can produce its own defaults
/// and validate its invariants independently.
pub trait ConfigSection: Default + Serialize + DeserializeOwned {
    /// Named defaults (same as Default::default but explicit).
    fn defaults() -> Self;

    /// Validate invariants *within this section* (e.g., spatial_tolerance > 0).
    /// Called by `KernelConfig::validate()`.
    ///
    /// Note: Cross-section invariants (like gap closure vs ambiguity band)
    /// are checked later in `KernelConfig::cross_validate()`. Keeping this
    /// isolated ensures each section remains independently testable.
    fn validate(&self) -> Result<(), KernelError>;
}

/// The unified configuration root for all kernel operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelConfig {
    pub tolerance: ToleranceSection,
    pub solver: SolverSection,
    pub validation: ValidationSection,
    pub policy: PolicySection,
    pub precision: PrecisionSection,
}

impl Default for KernelConfig {
    fn default() -> Self {
        Self {
            tolerance: ToleranceSection::defaults(),
            solver: SolverSection::defaults(),
            validation: ValidationSection::defaults(),
            policy: PolicySection::defaults(),
            precision: PrecisionSection::defaults(),
        }
    }
}

impl KernelConfig {
    /// Validates all sections individually.
    ///
    /// This calls `validate()` on every sub-section to ensure field-level invariants
    /// are met. Full validation including cross-section invariants will be
    /// handled by `cross_validate()` on `ResolvedConfig` (implemented later).
    pub fn validate(&self) -> Result<(), KernelError> {
        self.tolerance.validate()?;
        self.solver.validate()?;
        self.validation.validate()?;
        self.policy.validate()?;
        self.precision.validate()?;
        Ok(())
    }
}
