//! Sparse tolerance configuration overrides.
//!
//! DOMAIN: Partial overrides for the tolerance section of the kernel configuration.

use serde::{Deserialize, Serialize};

use super::tolerance_section::UnitSystem;

/// Sparse overrides for `ToleranceSection`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToleranceOverride {
    pub unit_system: Option<UnitSystem>,
    pub spatial_tolerance: Option<f64>,
    pub angular_tolerance: Option<f64>,
    pub min_transversal_angle: Option<f64>,
    pub max_tangent_gap: Option<f64>,
    pub min_face_area: Option<f64>,
    pub max_slivers_per_op: Option<usize>,
    pub max_gap_closure: Option<f64>,
    pub residual: Option<f64>,
    pub degeneracy: Option<f64>,
    pub sample_inward_offset: Option<f64>,
    pub ray_extent: Option<f64>,
    pub coplanar_angle_epsilon: Option<f64>,
    pub coplanar_offset_epsilon: Option<f64>,
    pub edge_split_degeneracy: Option<f64>,
    pub min_edge_length: Option<f64>,
    pub collinearity_dot_tolerance: Option<f64>,
    pub aabb_inflation: Option<f64>,
    pub model_scale_mm: Option<f64>,
    pub error_budget_mm: Option<f64>,
    pub ambiguity_band_factor: Option<f64>,
}
