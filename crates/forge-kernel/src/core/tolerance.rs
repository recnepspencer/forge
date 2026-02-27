//! Tolerance policies and configuration.
//!
//! DOMAIN: All numeric thresholds and policy structs for geometry-layer computations.
//! INVARIANTS: All defaults are suitable for unit-scale CAD (meters).
//! DEPENDENCIES: serde (serialization)

use serde::{Deserialize, Serialize};
use super::config::defaults;

/// Spatial tolerance policy for coincidence detection.
#[derive(Debug, Clone)]
pub struct TolerancePolicy {
    spatial_tolerance: f64,
    angular_tolerance: f64,
}

impl TolerancePolicy {
    /// Create a tolerance policy with explicit values.
    pub fn new(spatial_tolerance: f64, angular_tolerance: f64) -> Self {
        Self {
            spatial_tolerance,
            angular_tolerance,
        }
    }

    /// Distance below which two points are considered coincident (meters).
    pub fn get_spatial_tolerance(&self) -> f64 {
        self.spatial_tolerance
    }

    /// Set the spatial coincidence tolerance.
    pub fn set_spatial_tolerance(&mut self, value: f64) {
        self.spatial_tolerance = value;
    }

    /// Angular tolerance for direction comparisons (radians).
    pub fn get_angular_tolerance(&self) -> f64 {
        self.angular_tolerance
    }

    /// Set the angular comparison tolerance.
    pub fn set_angular_tolerance(&mut self, value: f64) {
        self.angular_tolerance = value;
    }
}

impl Default for TolerancePolicy {
    fn default() -> Self {
        Self {
            spatial_tolerance: defaults::SPATIAL_TOLERANCE,
            angular_tolerance: defaults::ANGULAR_TOLERANCE,
        }
    }
}


/// Policy for handling near-tangent surface intersections.
#[derive(Debug, Clone)]
pub struct TangencyPolicy {
    min_transversal_angle: f64,
    max_tangent_gap: f64,
}

impl TangencyPolicy {
    /// Create a tangency policy with explicit values.
    pub fn new(min_transversal_angle: f64, max_tangent_gap: f64) -> Self {
        Self {
            min_transversal_angle,
            max_tangent_gap,
        }
    }

    /// Minimum angle between surfaces to classify as transversal (radians).
    pub fn get_min_transversal_angle(&self) -> f64 {
        self.min_transversal_angle
    }

    /// Set the minimum transversal angle.
    pub fn set_min_transversal_angle(&mut self, value: f64) {
        self.min_transversal_angle = value;
    }

    /// Maximum gap in near-tangent regions before escalating to policy decision.
    pub fn get_max_tangent_gap(&self) -> f64 {
        self.max_tangent_gap
    }

    /// Set the maximum tangent gap.
    pub fn set_max_tangent_gap(&mut self, value: f64) {
        self.max_tangent_gap = value;
    }
}

impl Default for TangencyPolicy {
    fn default() -> Self {
        Self {
            min_transversal_angle: defaults::MIN_TRANSVERSAL_ANGLE,
            max_tangent_gap: defaults::MAX_TANGENT_GAP,
        }
    }
}


/// Policy for sliver face detection and removal.
#[derive(Debug, Clone)]
pub struct SliverPolicy {
    min_face_area: f64,
    max_slivers_per_op: usize,
}

impl SliverPolicy {
    /// Create a sliver policy with explicit values.
    pub fn new(min_face_area: f64, max_slivers_per_op: usize) -> Self {
        Self {
            min_face_area,
            max_slivers_per_op,
        }
    }

    /// Minimum face area (m²) — faces below this are slivers.
    pub fn get_min_face_area(&self) -> f64 {
        self.min_face_area
    }

    /// Set the minimum face area.
    pub fn set_min_face_area(&mut self, value: f64) {
        self.min_face_area = value;
    }

    /// Maximum number of slivers an operation may create before requiring explicit waiver.
    pub fn get_max_slivers_per_op(&self) -> usize {
        self.max_slivers_per_op
    }

    /// Set the maximum slivers per operation.
    pub fn set_max_slivers_per_op(&mut self, value: usize) {
        self.max_slivers_per_op = value;
    }
}

impl Default for SliverPolicy {
    fn default() -> Self {
        Self {
            min_face_area: defaults::MIN_FACE_AREA,
            max_slivers_per_op: defaults::MAX_SLIVERS_PER_OP,
        }
    }
}


/// Policy for automatic gap closure during sewing.
#[derive(Debug, Clone)]
pub struct GapClosurePolicy {
    max_gap: f64,
}

impl GapClosurePolicy {
    /// Create a gap closure policy with explicit value.
    pub fn new(max_gap: f64) -> Self {
        Self { max_gap }
    }

    /// Maximum gap that will be automatically closed (meters).
    pub fn get_max_gap(&self) -> f64 {
        self.max_gap
    }

    /// Set the maximum gap for automatic closure.
    pub fn set_max_gap(&mut self, value: f64) {
        self.max_gap = value;
    }
}

impl Default for GapClosurePolicy {
    fn default() -> Self {
        Self {
            max_gap: defaults::GAP_CLOSURE_MAX,
        }
    }
}


/// Policy for precision escalation (Milestone 0.2.3).
#[derive(Debug, Clone)]
pub struct PrecisionEscalationPolicy {
    bit_length_threshold: u32,
}

impl PrecisionEscalationPolicy {
    /// Create a precision escalation policy with explicit value.
    pub fn new(bit_length_threshold: u32) -> Self {
        Self { bit_length_threshold }
    }

    /// Bit-length threshold before escalating.
    pub fn get_bit_length_threshold(&self) -> u32 {
        self.bit_length_threshold
    }

    /// Set the bit-length threshold.
    pub fn set_bit_length_threshold(&mut self, value: u32) {
        self.bit_length_threshold = value;
    }
}

impl Default for PrecisionEscalationPolicy {
    fn default() -> Self {
        Self {
            bit_length_threshold: defaults::BIT_LENGTH_THRESHOLD,
        }
    }
}


/// Hard floor — no tolerance may be tighter than this regardless of model scale.
///
/// Prevents floating-point underflow when the model bounding box diagonal is
/// sub-millimeter. IEEE-754 double precision loses meaningful digits below ~1e-15,
/// so 1e-13 gives a comfortable two-decade margin.
pub const ABSOLUTE_MINIMUM_TOLERANCE: f64 = 1e-13;

/// Configurable thresholds for geometry-layer computations.
///
/// These values are used by `forge-geom` functions that accept tolerance
/// parameters (plane intersection degeneracy, overconstrained residual, etc.).
/// Defaults are suitable for unit-scale CAD (meters). Adjust for different
/// model scales or import pipeline tolerance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToleranceConfig {
    /// Maximum acceptable residual for overconstrained vertex verification.
    residual: f64,
    /// Minimum acceptable |det| for 3-plane intersection degeneracy check.
    degeneracy: f64,
    /// Inward offset from face centroid along normal for point-in-solid sampling.
    sample_inward_offset: f64,
    /// Ray extent for point-in-solid classification.
    ray_extent: f64,
    /// Tolerance for coplanar plane normal parallelism (squared cross product magnitude).
    coplanar_angle_epsilon: f64,
    /// Tolerance for coplanar plane offset difference.
    coplanar_offset_epsilon: f64,
    /// Minimum denominator for edge-plane intersection (to avoid numeric instability).
    edge_split_degeneracy: f64,
    /// Minimum edge length to be considered non-degenerate.
    min_edge_length: f64,
    /// Tolerance for vertex collinearity check (dot product deviation from -1.0).
    collinearity_dot_tolerance: f64,
    /// AABB inflation margin for BVH overlap detection (meters).
    aabb_inflation: f64,
    /// Grid quantization scale for spatial hashing.
    ///
    /// Passed as-is to `forge_math::linalg::compute_spatial_hash`. A value of `1e6`
    /// gives 1-micrometer resolution on meter-scale models. Adjust with model scale.
    #[serde(default = "default_spatial_hash_grid_scale")]
    spatial_hash_grid_scale: f64,
    /// Diagonal of the model bounding box in mm, set once at import / build time.
    ///
    /// Drives scale-aware tolerance defaults per ISO 10303-42:
    /// `global_default = 1e-7 * max(model_scale_mm, 1.0)`.
    /// Zero (the default) causes the formula to use `max(0, 1.0) = 1.0`,
    /// giving the conservative fallback `1e-7`.
    #[serde(default)]
    model_scale_mm: f64,
    /// Maximum acceptable accumulated error across an operation chain (mm).
    ///
    /// When the sum of per-vertex tolerance deltas exceeds this threshold,
    /// `check_budget()` emits a `KernelWarning::ErrorBudgetExceeded`.
    /// Default is `f64::INFINITY` (budget warnings disabled).
    #[serde(default = "default_error_budget")]
    error_budget_mm: f64,
    /// Multiplier defining the ambiguity band around tolerance boundaries.
    ///
    /// When a surface classification measure falls in `tol..tol*factor`,
    /// `classify_surface_pair` returns `PolicyResult::Ambiguous` instead
    /// of making a crisp decision. Default is 10.0 (one decade).
    #[serde(default = "default_ambiguity_band_factor")]
    ambiguity_band_factor: f64,
}

impl ToleranceConfig {
    /// Create a tolerance config with explicit values.
    pub fn new(
        residual: f64,
        degeneracy: f64,
        sample_inward_offset: f64,
        ray_extent: f64,
        coplanar_angle_epsilon: f64,
        coplanar_offset_epsilon: f64,
        edge_split_degeneracy: f64,
        min_edge_length: f64,
        collinearity_dot_tolerance: f64,
    ) -> Self {
        Self {
            residual,
            degeneracy,
            sample_inward_offset,
            ray_extent,
            coplanar_angle_epsilon,
            coplanar_offset_epsilon,
            edge_split_degeneracy,
            min_edge_length,
            collinearity_dot_tolerance,
            aabb_inflation: defaults::AABB_INFLATION,
            spatial_hash_grid_scale: defaults::SPATIAL_HASH_GRID_SCALE,
            model_scale_mm: 0.0,
            error_budget_mm: f64::INFINITY,
            ambiguity_band_factor: defaults::AMBIGUITY_BAND_FACTOR,
        }
    }

    /// Scale-aware vertex tolerance following ISO 10303-42.
    ///
    /// Returns `1e-7 * max(model_scale_mm, 1.0)`, floored at
    /// `ABSOLUTE_MINIMUM_TOLERANCE` to prevent underflow on sub-mm models.
    pub fn scaled_vertex_tolerance(&self) -> f64 {
        let scale = self.model_scale_mm.max(1.0);
        (scale * 1e-7).max(ABSOLUTE_MINIMUM_TOLERANCE)
    }

    /// Diagonal of the model bounding box (mm).
    pub fn get_model_scale_mm(&self) -> f64 {
        self.model_scale_mm
    }

    /// Set the model bounding box diagonal (mm).
    ///
    /// Call once at import or build time so that `global_default()` and
    /// `scaled_vertex_tolerance()` reflect the actual model scale.
    pub fn set_model_scale_mm(&mut self, value: f64) {
        debug_assert!(value >= 0.0, "model_scale_mm must be non-negative");
        self.model_scale_mm = value;
    }

    /// Maximum acceptable accumulated error budget (mm).
    pub fn get_error_budget_mm(&self) -> f64 {
        self.error_budget_mm
    }

    /// Set the error budget threshold (mm). Use `f64::INFINITY` to disable.
    pub fn set_error_budget_mm(&mut self, value: f64) {
        self.error_budget_mm = value;
    }

    /// The residual tolerance for overconstrained verification.
    pub fn get_residual(&self) -> f64 {
        self.residual
    }

    /// Set the residual tolerance.
    pub fn set_residual(&mut self, value: f64) {
        self.residual = value;
    }

    /// The degeneracy threshold for plane intersection.
    pub fn get_degeneracy(&self) -> f64 {
        self.degeneracy
    }

    /// Set the degeneracy threshold.
    pub fn set_degeneracy(&mut self, value: f64) {
        self.degeneracy = value;
    }

    /// Inward offset from face centroid along normal for sampling.
    pub fn get_sample_inward_offset(&self) -> f64 {
        self.sample_inward_offset
    }

    /// Set the sample inward offset.
    pub fn set_sample_inward_offset(&mut self, value: f64) {
        self.sample_inward_offset = value;
    }

    /// Ray extent for point-in-solid classification.
    pub fn get_ray_extent(&self) -> f64 {
        self.ray_extent
    }

    /// Set the ray extent.
    pub fn set_ray_extent(&mut self, value: f64) {
        self.ray_extent = value;
    }

    /// Tolerance for coplanar plane normal parallelism.
    pub fn get_coplanar_angle_epsilon(&self) -> f64 {
        self.coplanar_angle_epsilon
    }

    /// Set tolerance for coplanar plane normal parallelism.
    pub fn set_coplanar_angle_epsilon(&mut self, value: f64) {
        self.coplanar_angle_epsilon = value;
    }

    /// Tolerance for coplanar plane offset difference.
    pub fn get_coplanar_offset_epsilon(&self) -> f64 {
        self.coplanar_offset_epsilon
    }

    /// Set tolerance for coplanar plane offset difference.
    pub fn set_coplanar_offset_epsilon(&mut self, value: f64) {
        self.coplanar_offset_epsilon = value;
    }

    /// Minimum denominator for edge-plane intersection.
    pub fn get_edge_split_degeneracy(&self) -> f64 {
        self.edge_split_degeneracy
    }

    /// Set edge split degeneracy threshold.
    pub fn set_edge_split_degeneracy(&mut self, value: f64) {
        self.edge_split_degeneracy = value;
    }

    /// Minimum edge length.
    pub fn get_min_edge_length(&self) -> f64 {
        self.min_edge_length
    }

    /// Set minimum edge length.
    pub fn set_min_edge_length(&mut self, value: f64) {
        self.min_edge_length = value;
    }

    /// Tolerance for collinearity (dot product).
    pub fn get_collinearity_dot_tolerance(&self) -> f64 {
        self.collinearity_dot_tolerance
    }

    /// Set collinearity tolerance.
    pub fn set_collinearity_dot_tolerance(&mut self, value: f64) {
        self.collinearity_dot_tolerance = value;
    }

    /// AABB inflation margin for BVH overlap detection.
    pub fn get_aabb_inflation(&self) -> f64 {
        self.aabb_inflation
    }

    /// Set AABB inflation margin.
    pub fn set_aabb_inflation(&mut self, value: f64) {
        self.aabb_inflation = value;
    }

    /// Grid quantization scale for deterministic spatial hashing.
    ///
    /// Pass this value to `forge_math::linalg::compute_spatial_hash`.
    /// Defaults to `1e6` (1-micrometer grid on meter-unit models).
    pub fn get_spatial_hash_grid_scale(&self) -> f64 {
        self.spatial_hash_grid_scale
    }

    /// Set the spatial hash grid scale.
    pub fn set_spatial_hash_grid_scale(&mut self, value: f64) {
        debug_assert!(value > 0.0, "spatial_hash_grid_scale must be positive");
        self.spatial_hash_grid_scale = value;
    }

    /// Multiplier for the ambiguity band around tolerance boundaries.
    ///
    /// Surface classification returns `Ambiguous` when a measure falls
    /// in `tol..tol*factor`. Default 10.0 (one decade).
    pub fn get_ambiguity_band_factor(&self) -> f64 {
        self.ambiguity_band_factor
    }

    /// Set the ambiguity band factor.
    pub fn set_ambiguity_band_factor(&mut self, value: f64) {
        debug_assert!(value > 1.0, "ambiguity_band_factor must be > 1.0");
        self.ambiguity_band_factor = value;
    }
}

impl Default for ToleranceConfig {
    fn default() -> Self {
        Self {
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
            spatial_hash_grid_scale: defaults::SPATIAL_HASH_GRID_SCALE,
            model_scale_mm: 0.0,
            error_budget_mm: f64::INFINITY,
            ambiguity_band_factor: defaults::AMBIGUITY_BAND_FACTOR,
        }
    }
}

/// Serde default helper for `error_budget_mm` (returns `f64::INFINITY`).
fn default_error_budget() -> f64 {
    f64::INFINITY
}


/// Serde default helper for `ambiguity_band_factor`.
fn default_ambiguity_band_factor() -> f64 {
    defaults::AMBIGUITY_BAND_FACTOR
}

/// Serde default helper for `spatial_hash_grid_scale`.
fn default_spatial_hash_grid_scale() -> f64 {
    defaults::SPATIAL_HASH_GRID_SCALE
}
