//! Config and policy accessors for ModelingContext.
//!
//! DOMAIN: Getters and setters for tolerance, tangency, sliver, gap, precision,
//! validation, and operation space configuration.

use crate::analysis::proof_validation::checkpoint::ValidationConfig;
use crate::core::tolerance::{
    TolerancePolicy, TangencyPolicy, SliverPolicy,
    GapClosurePolicy, PrecisionEscalationPolicy, ToleranceConfig,
};

use super::schema::ModelingContext;

impl ModelingContext {
    /// Get the spatial tolerance policy.
    pub fn get_tolerance(&self) -> TolerancePolicy {
        TolerancePolicy::new(
            self.config.tolerance.spatial_tolerance,
            self.config.tolerance.angular_tolerance,
        )
    }

    /// Set the spatial tolerance policy.
    pub fn set_tolerance(&mut self, policy: TolerancePolicy) {
        self.config.tolerance.spatial_tolerance = policy.get_spatial_tolerance();
        self.config.tolerance.angular_tolerance = policy.get_angular_tolerance();
    }

    /// Get the tangency handling policy.
    pub fn get_tangency(&self) -> TangencyPolicy {
        TangencyPolicy::new(
            self.config.tolerance.min_transversal_angle,
            self.config.tolerance.max_tangent_gap,
        )
    }

    /// Set the tangency handling policy.
    pub fn set_tangency(&mut self, policy: TangencyPolicy) {
        self.config.tolerance.min_transversal_angle = policy.get_min_transversal_angle();
        self.config.tolerance.max_tangent_gap = policy.get_max_tangent_gap();
    }

    /// Get the sliver face policy.
    pub fn get_sliver(&self) -> SliverPolicy {
        SliverPolicy::new(
            self.config.tolerance.min_face_area,
            self.config.tolerance.max_slivers_per_op,
        )
    }

    /// Set the sliver face policy.
    pub fn set_sliver(&mut self, policy: SliverPolicy) {
        self.config.tolerance.min_face_area = policy.get_min_face_area();
        self.config.tolerance.max_slivers_per_op = policy.get_max_slivers_per_op();
    }

    /// Get the gap closure policy.
    pub fn get_gap_closure(&self) -> GapClosurePolicy {
        GapClosurePolicy::new(self.config.tolerance.max_gap_closure)
    }

    /// Set the gap closure policy.
    pub fn set_gap_closure(&mut self, policy: GapClosurePolicy) {
        self.config.tolerance.max_gap_closure = policy.get_max_gap();
    }

    /// Get the precision escalation policy.
    pub fn get_precision(&self) -> PrecisionEscalationPolicy {
        PrecisionEscalationPolicy::new(self.config.precision.bit_length_threshold)
    }

    /// Set the precision escalation policy.
    pub fn set_precision(&mut self, policy: PrecisionEscalationPolicy) {
        self.config.precision.bit_length_threshold = policy.get_bit_length_threshold();
    }

    /// Get the geometry-layer tolerance configuration.
    pub fn get_tolerance_config(&self) -> ToleranceConfig {
        let mut t = ToleranceConfig::new(
            self.config.tolerance.residual,
            self.config.tolerance.degeneracy,
            self.config.tolerance.sample_inward_offset,
            self.config.tolerance.ray_extent,
            self.config.tolerance.coplanar_angle_epsilon,
            self.config.tolerance.coplanar_offset_epsilon,
            self.config.tolerance.edge_split_degeneracy,
            self.config.tolerance.min_edge_length,
            self.config.tolerance.collinearity_dot_tolerance,
        );
        t.set_aabb_inflation(self.config.tolerance.aabb_inflation);
        t.set_model_scale_mm(self.config.tolerance.model_scale_mm);
        t.set_error_budget_mm(self.config.tolerance.error_budget_mm);
        t.set_ambiguity_band_factor(self.config.tolerance.ambiguity_band_factor);
        t
    }

    /// Set the geometry-layer tolerance configuration.
    pub fn set_tolerance_config(&mut self, t: ToleranceConfig) {
        self.config.tolerance.residual = t.get_residual();
        self.config.tolerance.degeneracy = t.get_degeneracy();
        self.config.tolerance.sample_inward_offset = t.get_sample_inward_offset();
        self.config.tolerance.ray_extent = t.get_ray_extent();
        self.config.tolerance.coplanar_angle_epsilon = t.get_coplanar_angle_epsilon();
        self.config.tolerance.coplanar_offset_epsilon = t.get_coplanar_offset_epsilon();
        self.config.tolerance.edge_split_degeneracy = t.get_edge_split_degeneracy();
        self.config.tolerance.min_edge_length = t.get_min_edge_length();
        self.config.tolerance.collinearity_dot_tolerance = t.get_collinearity_dot_tolerance();
        self.config.tolerance.aabb_inflation = t.get_aabb_inflation();
        self.config.tolerance.model_scale_mm = t.get_model_scale_mm();
        self.config.tolerance.error_budget_mm = t.get_error_budget_mm();
        self.config.tolerance.ambiguity_band_factor = t.get_ambiguity_band_factor();
    }

    /// Get the validation checkpoint configuration.
    pub fn get_validation_config(&self) -> ValidationConfig {
        ValidationConfig {
            checkpoints: self.config.validation.checkpoints.clone(),
            include_geometric: self.config.validation.include_geometric,
            entity_limit: self.config.validation.entity_limit,
        }
    }

    /// Set the validation checkpoint configuration.
    pub fn set_validation_config(&mut self, config: ValidationConfig) {
        self.config.validation.checkpoints = config.checkpoints;
        self.config.validation.include_geometric = config.include_geometric;
        self.config.validation.entity_limit = config.entity_limit;
    }

    /// Get the operation space for the current feature evaluation.
    ///
    /// Steps use this to read world-space coordinates in local frame
    /// via `op_space().to_local(point)` — geometry stays immutable.
    pub fn get_operation_space(&self) -> &super::super::OperationSpace {
        &self.operation_space
    }

    /// Set the operation space for the current feature evaluation.
    ///
    /// Called by the feature pipeline executor after analyzing input
    /// geometry scale. Resets to `OperationSpace::identity()` between features.
    pub fn set_operation_space(&mut self, space: super::super::OperationSpace) {
        self.operation_space = space;
    }
}
