//! Boolean tolerance adapters.
//!
//! DOMAIN: Extract the specific tolerance values that boolean operations
//! need from `ResolvedConfig`.
//! This decouples boolean algorithms from broader execution context state.
//!
//! Each boolean phase takes `&BooleanTolerances` instead of
//! config/context bags, making tolerance access explicit and testable.

use crate::configuration::facade::ResolvedConfig;

/// Tolerance values used by boolean operations.
///
/// Extracted from `ModelingContext` at the boolean entry point and
/// threaded through all phases. Each field documents which phase
/// uses it and why.
#[derive(Debug, Clone)]
pub struct BooleanTolerances {
    // ── Split phase ──────────────────────────────────────
    /// Maximum gap that the reconciliation pass will try to close.
    /// `split/eval.rs` uses this as the base for reconcile_tol computation.
    pub max_gap: f64,

    /// Minimum edge length below which edges are considered degenerate.
    /// Used by `OperationSpace::analyze_binary` and split edge validation.
    pub min_edge_length: f64,

    /// Minimum denominator for edge-plane intersection.
    /// Avoids numeric instability in the splitter.
    pub edge_split_degeneracy: f64,

    // ── Classify phase ───────────────────────────────────
    /// Inward offset from face centroid along normal for sampling.
    pub sample_inward_offset: f64,

    /// Ray extent for point-in-solid classification.
    pub ray_extent: f64,

    /// Maximum residual for overconstrained vertex verification.
    pub residual: f64,

    /// Degeneracy threshold for 3-plane intersection.
    pub degeneracy: f64,

    /// AABB inflation margin for BVH overlap detection.
    pub aabb_inflation: f64,

    // ── Assemble phase ───────────────────────────────────
    /// Weld floor for vertex stitching: `max_gap * 4.0`.
    /// Used by `assemble_result` to determine weld tolerance.
    pub weld_floor: f64,

    // ── Postprocess phase ────────────────────────────────
    /// Tolerance for coplanar plane normal parallelism.
    pub coplanar_angle_epsilon: f64,

    /// Tolerance for coplanar plane offset difference.
    pub coplanar_offset_epsilon: f64,

    /// Tolerance for vertex collinearity check.
    pub collinearity_dot_tolerance: f64,

    // ── Scale-aware ──────────────────────────────────────
    /// Model bounding box diagonal in mm.
    pub model_scale_mm: f64,

    /// Scale-aware vertex tolerance (ISO 10303-42).
    pub scaled_vertex_tolerance: f64,

    /// Spatial tolerance for coincidence detection.
    pub spatial_tolerance: f64,

    /// Ambiguity band factor (multiplier on tolerance for gray zone).
    pub ambiguity_band_factor: f64,
}

impl BooleanTolerances {
    /// Extract tolerances from the resolved operation config.
    pub fn from_config(config: &ResolvedConfig) -> Self {
        let tc = config.tolerance_config();
        let gap = config.config().tolerance.max_gap_closure;

        Self {
            max_gap: gap,
            min_edge_length: tc.get_min_edge_length(),
            edge_split_degeneracy: tc.get_edge_split_degeneracy(),
            sample_inward_offset: tc.get_sample_inward_offset(),
            ray_extent: tc.get_ray_extent(),
            residual: tc.get_residual(),
            degeneracy: tc.get_degeneracy(),
            aabb_inflation: tc.get_aabb_inflation(),
            weld_floor: gap * 4.0,
            coplanar_angle_epsilon: tc.get_coplanar_angle_epsilon(),
            coplanar_offset_epsilon: tc.get_coplanar_offset_epsilon(),
            collinearity_dot_tolerance: tc.get_collinearity_dot_tolerance(),
            model_scale_mm: tc.get_model_scale_mm(),
            scaled_vertex_tolerance: tc.scaled_vertex_tolerance(),
            spatial_tolerance: config.spatial_tolerance(),
            ambiguity_band_factor: tc.get_ambiguity_band_factor(),
        }
    }
}
