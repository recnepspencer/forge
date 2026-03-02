//! Tolerance section of the unified configuration.
//!
//! DOMAIN: Spatial, angular, and tolerance settings.
//! INVARIANTS: spatial_tolerance > 0, ambiguity_band_factor > 1.0, model_scale_mm >= 0.

use forge_core::KernelError;
use serde::{Deserialize, Serialize};

use super::defaults;
use super::kernel_config::ConfigSection;

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
