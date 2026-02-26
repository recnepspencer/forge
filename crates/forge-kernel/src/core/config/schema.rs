//! # Unified Kernel Configuration
//!
//! DOMAIN: The schema for `KernelConfig` and its constituent sections.
//! INVARIANTS: Each section can be validated independently. Cross-section
//! consistency is enforced by `KernelConfig::cross_validate()`.

use std::collections::BTreeMap;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use forge_core::{KernelError, PolicyKind};
use crate::analysis::proof_validation::checkpoint::ValidationCheckpoint;

use super::defaults;

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

/// Unit system for linear measurements.
/// Attached to ToleranceSection to prevent silent scale mismatches
/// when importing bodies from different CAD systems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnitSystem {
    Meters,
    Millimeters,
    Inches,
}

impl Default for UnitSystem {
    fn default() -> Self {
        Self::Meters
    }
}

/// Spatial, angular, and tolerance settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToleranceSection {
    pub unit_system: UnitSystem,
    pub spatial_tolerance: f64,
    pub angular_tolerance: f64,
    pub min_transversal_angle: f64,
    pub max_tangent_gap: f64,
    pub min_face_area: f64,
    pub max_slivers_per_op: usize,
    pub max_gap_closure: f64,
    pub residual: f64,
    pub degeneracy: f64,
    pub sample_inward_offset: f64,
    pub ray_extent: f64,
    pub coplanar_angle_epsilon: f64,
    pub coplanar_offset_epsilon: f64,
    pub edge_split_degeneracy: f64,
    pub min_edge_length: f64,
    pub collinearity_dot_tolerance: f64,
    pub aabb_inflation: f64,
    pub model_scale_mm: f64,
    pub error_budget_mm: f64,
    pub ambiguity_band_factor: f64,
}

impl ConfigSection for ToleranceSection {
    fn defaults() -> Self {
        Self {
            unit_system: UnitSystem::default(),
            spatial_tolerance: defaults::SPATIAL_TOLERANCE,
            angular_tolerance: defaults::ANGULAR_TOLERANCE,
            min_transversal_angle: defaults::MIN_TRANSVERSAL_ANGLE,
            max_tangent_gap: defaults::MAX_TANGENT_GAP,
            min_face_area: defaults::MIN_FACE_AREA,
            max_slivers_per_op: defaults::MAX_SLIVERS_PER_OP,
            max_gap_closure: defaults::GAP_CLOSURE_MAX,
            residual: defaults::RESIDUAL_TOLERANCE,
            degeneracy: defaults::DEGENERACY_THRESHOLD,
            sample_inward_offset: defaults::SAMPLE_INWARD_OFFSET,
            ray_extent: defaults::RAY_EXTENT,
            coplanar_angle_epsilon: defaults::COPLANAR_ANGLE_EPSILON,
            coplanar_offset_epsilon: defaults::COPLANAR_OFFSET_EPSILON,
            edge_split_degeneracy: defaults::EDGE_SPLIT_DEGENERACY,
            min_edge_length: defaults::MIN_EDGE_LENGTH,
            collinearity_dot_tolerance: defaults::COLLINEARITY_DOT_TOLERANCE,
            aabb_inflation: defaults::AABB_INFLATION,
            model_scale_mm: 0.0,
            error_budget_mm: f64::INFINITY,
            ambiguity_band_factor: defaults::AMBIGUITY_BAND_FACTOR,
        }
    }

    fn validate(&self) -> Result<(), KernelError> {
        if self.spatial_tolerance <= 0.0 {
            return Err(KernelError::InvalidConfig {
                field: "spatial_tolerance".into(),
                reason: "must be positive".into(),
            });
        }
        if self.ambiguity_band_factor <= 1.0 {
            return Err(KernelError::InvalidConfig {
                field: "ambiguity_band_factor".into(),
                reason: "must be > 1.0".into(),
            });
        }
        if self.model_scale_mm < 0.0 {
            return Err(KernelError::InvalidConfig {
                field: "model_scale_mm".into(),
                reason: "must be non-negative".into(),
            });
        }
        Ok(())
    }
}

impl Default for ToleranceSection {
    fn default() -> Self {
        Self::defaults()
    }
}

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

/// Invariant validation checkpoints and settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationSection {
    pub checkpoints: Vec<ValidationCheckpoint>,
    pub include_geometric: bool,
    pub entity_limit: usize,
}

impl ConfigSection for ValidationSection {
    fn defaults() -> Self {
        if cfg!(debug_assertions) {
            Self {
                checkpoints: vec![
                    ValidationCheckpoint::PostCommit,
                    ValidationCheckpoint::PostBoolean,
                    ValidationCheckpoint::PostFeature,
                    ValidationCheckpoint::PostImport,
                ],
                include_geometric: true,
                entity_limit: 0,
            }
        } else {
            Self {
                checkpoints: vec![
                    ValidationCheckpoint::PostBoolean,
                    ValidationCheckpoint::PostImport,
                ],
                include_geometric: false,
                entity_limit: 50_000,
            }
        }
    }

    fn validate(&self) -> Result<(), KernelError> {
        Ok(())
    }
}

impl Default for ValidationSection {
    fn default() -> Self {
        Self::defaults()
    }
}

/// Default rules for handling policy ambiguity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicySection {
    pub fallback_rules: BTreeMap<PolicyKind, bool>,
}

impl ConfigSection for PolicySection {
    fn defaults() -> Self {
        let mut fallback_rules = BTreeMap::new();
        // Epic A/B current product semantics: weakly-simple coplanar region boundaries
        // are accepted by default, but the decision must be traced.
        fallback_rules.insert(PolicyKind::CoincidentGeometry, true);

        Self { fallback_rules }
    }

    fn validate(&self) -> Result<(), KernelError> {
        Ok(())
    }
}

impl Default for PolicySection {
    fn default() -> Self {
        Self::defaults()
    }
}

/// Settings for automatic scaling to arbitrary precision.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrecisionSection {
    pub bit_length_threshold: u32,
}

impl ConfigSection for PrecisionSection {
    fn defaults() -> Self {
        Self {
            bit_length_threshold: defaults::BIT_LENGTH_THRESHOLD,
        }
    }

    fn validate(&self) -> Result<(), KernelError> {
        Ok(())
    }
}

impl Default for PrecisionSection {
    fn default() -> Self {
        Self::defaults()
    }
}
