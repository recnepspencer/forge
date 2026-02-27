//! Tolerance query traits.
//!
//! DOMAIN: Named interfaces for tolerance values. Algorithms depend on
//! these traits instead of importing `ModelingContext` or `ToleranceConfig`
//! directly, following the Adapter Rule (§6).
//!
//! Each feature's `adapters.rs` can narrow these further into
//! domain-specific interfaces (e.g. `WeldToleranceProvider`).

use crate::core::ModelingContext;

/// Spatial coincidence and angular tolerances.
///
/// Used by: mesh builder (vertex dedup), ember boolean (mesh rebuild).
pub trait SpatialToleranceQuery {
    /// Distance below which two points are considered coincident (meters).
    fn spatial_tolerance(&self) -> f64;

    /// Angular tolerance for direction comparisons (radians).
    fn angular_tolerance(&self) -> f64;
}

impl SpatialToleranceQuery for ModelingContext {
    fn spatial_tolerance(&self) -> f64 {
        self.get_tolerance().get_spatial_tolerance()
    }

    fn angular_tolerance(&self) -> f64 {
        self.get_tolerance().get_angular_tolerance()
    }
}

/// Geometry-layer intersection and degeneracy tolerances.
///
/// Used by: boolean classify (plane intersection), boolean split (edge-plane).
pub trait GeometryToleranceQuery {
    /// Maximum acceptable residual for overconstrained vertex verification.
    fn residual_tolerance(&self) -> f64;

    /// Minimum acceptable |det| for 3-plane intersection degeneracy check.
    fn degeneracy_tolerance(&self) -> f64;

    /// Tolerance for coplanar plane normal parallelism.
    fn coplanar_angle_epsilon(&self) -> f64;

    /// Tolerance for coplanar plane offset difference.
    fn coplanar_offset_epsilon(&self) -> f64;

    /// Minimum denominator for edge-plane intersection.
    fn edge_split_degeneracy(&self) -> f64;

    /// Minimum edge length to be considered non-degenerate.
    fn min_edge_length(&self) -> f64;

    /// Multiplier for the ambiguity band around tolerance boundaries.
    fn ambiguity_band_factor(&self) -> f64;
}

impl GeometryToleranceQuery for ModelingContext {
    fn residual_tolerance(&self) -> f64 {
        self.get_tolerance_config().get_residual()
    }

    fn degeneracy_tolerance(&self) -> f64 {
        self.get_tolerance_config().get_degeneracy()
    }

    fn coplanar_angle_epsilon(&self) -> f64 {
        self.get_tolerance_config().get_coplanar_angle_epsilon()
    }

    fn coplanar_offset_epsilon(&self) -> f64 {
        self.get_tolerance_config().get_coplanar_offset_epsilon()
    }

    fn edge_split_degeneracy(&self) -> f64 {
        self.get_tolerance_config().get_edge_split_degeneracy()
    }

    fn min_edge_length(&self) -> f64 {
        self.get_tolerance_config().get_min_edge_length()
    }

    fn ambiguity_band_factor(&self) -> f64 {
        self.get_tolerance_config().get_ambiguity_band_factor()
    }
}

/// Sampling and ray-casting tolerances.
///
/// Used by: boolean classify (point-in-solid), boolean split (face sampling).
pub trait SamplingToleranceQuery {
    /// Inward offset from face centroid along normal for point-in-solid sampling.
    fn sample_inward_offset(&self) -> f64;

    /// Ray extent for point-in-solid classification.
    fn ray_extent(&self) -> f64;

    /// AABB inflation margin for BVH overlap detection (meters).
    fn aabb_inflation(&self) -> f64;
}

impl SamplingToleranceQuery for ModelingContext {
    fn sample_inward_offset(&self) -> f64 {
        self.get_tolerance_config().get_sample_inward_offset()
    }

    fn ray_extent(&self) -> f64 {
        self.get_tolerance_config().get_ray_extent()
    }

    fn aabb_inflation(&self) -> f64 {
        self.get_tolerance_config().get_aabb_inflation()
    }
}

/// Gap closure and weld tolerances.
///
/// Used by: boolean split (reconciliation), boolean assemble (stitching, weld floor).
pub trait GapToleranceQuery {
    /// Maximum gap that will be automatically closed (meters).
    fn max_gap_closure(&self) -> f64;
}

impl GapToleranceQuery for ModelingContext {
    fn max_gap_closure(&self) -> f64 {
        self.get_gap_closure().get_max_gap()
    }
}

/// Scale-aware tolerance for model-level decisions.
///
/// Used by: error budget tracking, import pipeline.
pub trait ScaleToleranceQuery {
    /// Diagonal of the model bounding box (mm).
    fn model_scale_mm(&self) -> f64;

    /// Scale-aware vertex tolerance (ISO 10303-42).
    fn scaled_vertex_tolerance(&self) -> f64;

    /// Maximum accumulated error budget (mm).
    fn error_budget_mm(&self) -> f64;
}

impl ScaleToleranceQuery for ModelingContext {
    fn model_scale_mm(&self) -> f64 {
        self.get_tolerance_config().get_model_scale_mm()
    }

    fn scaled_vertex_tolerance(&self) -> f64 {
        self.get_tolerance_config().scaled_vertex_tolerance()
    }

    fn error_budget_mm(&self) -> f64 {
        self.get_tolerance_config().get_error_budget_mm()
    }
}
