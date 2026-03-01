//! Frozen, fully-resolved configuration output.
//!
//! DOMAIN: The immutable result of cascade resolution — carries both the
//! effective `KernelConfig` values and the `ConfigProvenance` audit trail.

use forge_core::{KernelError, PolicyKind};

use super::defaults;
use super::policy::ToleranceConfig;
use super::provenance::{ConfigProvenance, ConfigSource};
use super::schema::{KernelConfig, UnitSystem};

/// Hard floor — no tolerance may be tighter than this regardless of model scale.
///
/// Prevents floating-point underflow when the model bounding box diagonal is
/// sub-millimeter. IEEE-754 double precision loses meaningful digits below ~1e-15,
/// so 1e-13 gives a comfortable two-decade margin.
pub const ABSOLUTE_MINIMUM_TOLERANCE: f64 = 1e-13;

/// Frozen, fully-resolved configuration for the current scope.
#[derive(Debug, Clone)]
pub struct ResolvedConfig {
    /// The effective values after cascade resolution.
    pub(super) config: KernelConfig,
    /// Which scope provided each value (for tracing/audit).
    pub(super) provenance: ConfigProvenance,
}

impl ResolvedConfig {
    /// Access the resolved configuration.
    pub fn config(&self) -> &KernelConfig {
        &self.config
    }

    /// Access the provenance tracker.
    pub fn provenance(&self) -> &ConfigProvenance {
        &self.provenance
    }

    /// Query which scope set a given field, including the specific entity.
    pub fn source_of(&self, field: &str) -> Option<&ConfigSource> {
        self.provenance.source_of(field)
    }

    /// Multiplier to convert values from the configured unit system into meters.
    pub fn scale_factor(&self) -> f64 {
        match self.config.tolerance.unit_system {
            UnitSystem::Meters => 1.0,
            UnitSystem::Millimeters => 0.001,
            UnitSystem::Inches => 0.0254,
        }
    }

    /// Scale-aware vertex tolerance following ISO 10303-42.
    ///
    /// Returns `1e-7 * max(model_scale_mm, 1.0)`, floored at
    /// `ABSOLUTE_MINIMUM_TOLERANCE` to prevent underflow on sub-mm models.
    pub fn scaled_vertex_tolerance(&self) -> f64 {
        let scale = self.config.tolerance.model_scale_mm.max(1.0);
        (scale * 1e-7).max(ABSOLUTE_MINIMUM_TOLERANCE)
    }

    /// Build geometry-layer tolerance settings from this resolved config.
    pub fn tolerance_config(&self) -> ToleranceConfig {
        let tol = &self.config.tolerance;
        let mut t = ToleranceConfig::new(
            tol.residual,
            tol.degeneracy,
            tol.sample_inward_offset,
            tol.ray_extent,
            tol.coplanar_angle_epsilon,
            tol.coplanar_offset_epsilon,
            tol.edge_split_degeneracy,
            tol.min_edge_length,
            tol.collinearity_dot_tolerance,
        );
        t.set_aabb_inflation(tol.aabb_inflation);
        t.set_model_scale_mm(tol.model_scale_mm);
        t.set_error_budget_mm(tol.error_budget_mm);
        t.set_ambiguity_band_factor(tol.ambiguity_band_factor);
        t
    }

    /// Spatial coincidence tolerance.
    pub fn spatial_tolerance(&self) -> f64 {
        self.config.tolerance.spatial_tolerance
    }

    /// Validate that a policy kind has a configured fallback rule.
    pub fn validate_policy_configured(&self, kind: &PolicyKind) -> Result<(), KernelError> {
        if self.config.policy.fallback_rules.contains_key(kind) {
            Ok(())
        } else {
            Err(KernelError::InvalidConfig {
                field: format!("policy.fallback_rules.{:?}", kind),
                reason: format!("No configured policy found for {:?}", kind),
            })
        }
    }

    /// Perform cross-section invariant validation.
    ///
    /// Validates individual sections, then checks multi-field invariants
    /// (e.g., gap closure vs spatial tolerance * ambiguity band).
    pub fn cross_validate(&self) -> Result<(), KernelError> {
        self.config.validate()?;

        let tol = &self.config.tolerance;
        let ambiguity_limit = tol.spatial_tolerance * tol.ambiguity_band_factor;

        let relaxed_limit = ambiguity_limit * defaults::GAP_CLOSURE_RELAXATION_FACTOR;
        if tol.max_gap_closure > relaxed_limit {
            return Err(KernelError::InvalidConfig {
                field: "max_gap_closure".into(),
                reason: format!(
                    "max_gap_closure ({}) cannot exceed spatial_tolerance * ambiguity_band_factor * GAP_CLOSURE_RELAXATION_FACTOR ({})",
                    tol.max_gap_closure, relaxed_limit
                ),
            });
        }

        Ok(())
    }
}
