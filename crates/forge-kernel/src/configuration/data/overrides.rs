//! Sparse configuration overrides.
//!
//! DOMAIN: Data structures representing partial overrides of the kernel configuration.

use forge_core::{PolicyKind, ValidationCheckpoint};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use super::tolerance_section::UnitSystem;

/// An override container matching the structure of `KernelConfig`, but entirely sparse.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConfigOverride {
    pub tolerance: Option<ToleranceOverride>,
    pub solver: Option<SolverOverride>,
    pub validation: Option<ValidationOverride>,
    pub policy: Option<PolicyOverride>,
    pub precision: Option<PrecisionOverride>,
}

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

/// Sparse overrides for `SolverSection`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SolverOverride {
    pub max_iterations: Option<usize>,
    pub convergence_tolerance: Option<f64>,
    pub divergence_threshold: Option<f64>,
}

/// Sparse overrides for `ValidationSection`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ValidationOverride {
    pub checkpoints: Option<Vec<ValidationCheckpoint>>,
    pub include_geometric: Option<bool>,
    pub entity_limit: Option<usize>,
}

/// Sparse overrides for `PolicySection`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PolicyOverride {
    pub fallback_rules: Option<BTreeMap<PolicyKind, bool>>,
}

/// Sparse overrides for `PrecisionSection`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PrecisionOverride {
    pub bit_length_threshold: Option<u32>,
}
