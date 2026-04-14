//! Tolerance section of the unified configuration.
//!
//! DOMAIN: Spatial, angular, and tolerance settings with their default constants.
//! INVARIANTS: spatial_tolerance > 0, ambiguity_band_factor > 1.0, model_scale_mm >= 0.

use forge_core::KernelError;
use serde::{Deserialize, Serialize};

use super::super::kernel_config::ConfigSection;

// ── Default constants ────────────────────────────────────────────────

/// Spatial coincidence tolerance: 1 micron.
pub const SPATIAL_TOLERANCE: f64 = 1e-6;

/// Angular tolerance for direction comparisons: ~0.00006 degrees.
pub const ANGULAR_TOLERANCE: f64 = 1e-6;

/// Minimum angle between surfaces to classify as transversal: ~0.06 degrees.
pub const MIN_TRANSVERSAL_ANGLE: f64 = 1e-3;

/// Maximum gap in near-tangent regions before policy escalation: 0.1mm.
pub const MAX_TANGENT_GAP: f64 = 1e-4;

/// Minimum face area (m²) — faces below this are slivers.
pub const MIN_FACE_AREA: f64 = 1e-10;

/// Maximum number of slivers an operation may create.
pub const MAX_SLIVERS_PER_OP: usize = 3;

/// Maximum gap for automatic gap closure: 0.1mm.
pub const GAP_CLOSURE_MAX: f64 = 1e-4;

/// Residual tolerance for intersection computations.
pub const RESIDUAL_TOLERANCE: f64 = 1e-8;

/// Degeneracy threshold for near-zero denominators.
pub const DEGENERACY_THRESHOLD: f64 = 1e-12;

/// Inward offset for classification sample points.
pub const SAMPLE_INWARD_OFFSET: f64 = 1e-6;

/// Maximum ray extent for ray-casting operations.
pub const RAY_EXTENT: f64 = 1e6;

/// Angular epsilon for coplanar plane detection.
pub const COPLANAR_ANGLE_EPSILON: f64 = 1e-20;

/// Offset epsilon for coplanar plane detection.
pub const COPLANAR_OFFSET_EPSILON: f64 = 1e-12;

/// Degeneracy threshold for edge splitting.
pub const EDGE_SPLIT_DEGENERACY: f64 = 1e-30;

/// Minimum edge length — edges shorter than this are degenerate.
pub const MIN_EDGE_LENGTH: f64 = 1e-9;

/// Dot-product tolerance for collinearity detection.
pub const COLLINEARITY_DOT_TOLERANCE: f64 = 1e-8;

/// AABB inflation factor for bounding box overlap tests.
pub const AABB_INFLATION: f64 = 1e-7;

/// Multiplier applied to spatial_tolerance to define the ambiguity band.
pub const AMBIGUITY_BAND_FACTOR: f64 = 10.0;

/// Grid scale for spatial hashing (quantization resolution).
///
/// `1e6` = 1-micrometer grid cells on a meter-unit model. Drives
/// `worth_math::linalg::compute_spatial_hash` via `ToleranceConfig::get_spatial_hash_grid_scale`.
pub const SPATIAL_HASH_GRID_SCALE: f64 = 1e6;

/// Relaxation multiplier for cross-section gap closure validation.
///
/// When checking that `max_gap_closure ≤ spatial_tolerance × ambiguity_band_factor`,
/// we relax by this factor to account for floating-point rounding in the legacy
/// default combination (GAP_CLOSURE_MAX=1e-4, ambiguity_limit=1e-5).
pub const GAP_CLOSURE_RELAXATION_FACTOR: f64 = 10.1;

// ── Unit system ──────────────────────────────────────────────────────

/// Unit system for linear measurements.
/// Attached to ToleranceSection to prevent silent scale mismatches
/// when importing bodies from different CAD systems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnitSystem {
    Meters,
    Millimeters,
    Inches,
}

impl UnitSystem {
    /// Multiplier to convert values from this unit system into meters.
    pub fn scale_factor(self) -> f64 {
        match self {
            Self::Meters => 1.0,
            Self::Millimeters => 0.001,
            Self::Inches => 0.0254,
        }
    }
}

impl Default for UnitSystem {
    fn default() -> Self {
        Self::Meters
    }
}

// ── Tolerance section ────────────────────────────────────────────────

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
            spatial_tolerance: SPATIAL_TOLERANCE,
            angular_tolerance: ANGULAR_TOLERANCE,
            min_transversal_angle: MIN_TRANSVERSAL_ANGLE,
            max_tangent_gap: MAX_TANGENT_GAP,
            min_face_area: MIN_FACE_AREA,
            max_slivers_per_op: MAX_SLIVERS_PER_OP,
            max_gap_closure: GAP_CLOSURE_MAX,
            residual: RESIDUAL_TOLERANCE,
            degeneracy: DEGENERACY_THRESHOLD,
            sample_inward_offset: SAMPLE_INWARD_OFFSET,
            ray_extent: RAY_EXTENT,
            coplanar_angle_epsilon: COPLANAR_ANGLE_EPSILON,
            coplanar_offset_epsilon: COPLANAR_OFFSET_EPSILON,
            edge_split_degeneracy: EDGE_SPLIT_DEGENERACY,
            min_edge_length: MIN_EDGE_LENGTH,
            collinearity_dot_tolerance: COLLINEARITY_DOT_TOLERANCE,
            aabb_inflation: AABB_INFLATION,
            model_scale_mm: 0.0,
            error_budget_mm: f64::INFINITY,
            ambiguity_band_factor: AMBIGUITY_BAND_FACTOR,
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
