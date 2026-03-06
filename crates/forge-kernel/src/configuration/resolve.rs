//! Configuration resolution logic.
//!
//! DOMAIN: The cascade logic that flattens multiple layers of configuration
//! overrides into a final `ResolvedConfig`, computing `ConfigProvenance` along the way.

use forge_core::KernelError;

use super::config_override::ConfigOverride;
use super::kernel_config::KernelConfig;
use super::provenance::{ConfigProvenance, ConfigScope, ConfigSource};
use super::resolved::ResolvedConfig;

/// Helper macro to reduce boilerplate when cascading configuration overrides.
macro_rules! cascade {
    (
        $resolved:ident,
        $provenance:ident,
        $field_path_str:literal,
        $section_ident:ident . $field_ident:ident,
        $op:expr,
        $feat:expr,
        $model:expr
    ) => {
        if let Some(val) = $op
            .as_ref()
            .and_then(|(o, _)| o.$section_ident.as_ref())
            .and_then(|s| s.$field_ident.clone())
        {
            $resolved.$section_ident.$field_ident = val;
            $provenance.record(
                $field_path_str,
                ConfigSource::new(
                    ConfigScope::OperationOverride,
                    $op.as_ref().unwrap().1.clone(),
                ),
            );
        } else if let Some(val) = $feat
            .as_ref()
            .and_then(|(o, _)| o.$section_ident.as_ref())
            .and_then(|s| s.$field_ident.clone())
        {
            $resolved.$section_ident.$field_ident = val;
            $provenance.record(
                $field_path_str,
                ConfigSource::new(
                    ConfigScope::FeatureOverride,
                    $feat.as_ref().unwrap().1.clone(),
                ),
            );
        } else if let Some(val) = $model
            .as_ref()
            .and_then(|(o, _)| o.$section_ident.as_ref())
            .and_then(|s| s.$field_ident.clone())
        {
            $resolved.$section_ident.$field_ident = val;
            $provenance.record(
                $field_path_str,
                ConfigSource::new(
                    ConfigScope::ModelOverride,
                    $model.as_ref().unwrap().1.clone(),
                ),
            );
        } else {
            $provenance.record($field_path_str, ConfigSource::session());
        }
    };
}

/// Resolves a configuration cascade into a final, validated `ResolvedConfig`.
///
/// Order of precedence (highest to lowest):
/// 1. Operation override
/// 2. Feature override
/// 3. Model override
/// 4. Session defaults
pub fn resolve_config(
    session: &KernelConfig,
    model: Option<(&ConfigOverride, Option<String>)>,
    feature: Option<(&ConfigOverride, Option<String>)>,
    operation: Option<(&ConfigOverride, Option<String>)>,
) -> Result<ResolvedConfig, KernelError> {
    let mut resolved = session.clone();
    let mut provenance = ConfigProvenance::new();

    let mo = &model;
    let fo = &feature;
    let oo = &operation;

    // Tolerance fields
    cascade!(
        resolved,
        provenance,
        "tolerance.unit_system",
        tolerance.unit_system,
        oo,
        fo,
        mo
    );
    cascade!(
        resolved,
        provenance,
        "tolerance.spatial_tolerance",
        tolerance.spatial_tolerance,
        oo,
        fo,
        mo
    );
    cascade!(
        resolved,
        provenance,
        "tolerance.angular_tolerance",
        tolerance.angular_tolerance,
        oo,
        fo,
        mo
    );
    cascade!(
        resolved,
        provenance,
        "tolerance.min_transversal_angle",
        tolerance.min_transversal_angle,
        oo,
        fo,
        mo
    );
    cascade!(
        resolved,
        provenance,
        "tolerance.max_tangent_gap",
        tolerance.max_tangent_gap,
        oo,
        fo,
        mo
    );
    cascade!(
        resolved,
        provenance,
        "tolerance.min_face_area",
        tolerance.min_face_area,
        oo,
        fo,
        mo
    );
    cascade!(
        resolved,
        provenance,
        "tolerance.max_slivers_per_op",
        tolerance.max_slivers_per_op,
        oo,
        fo,
        mo
    );
    cascade!(
        resolved,
        provenance,
        "tolerance.max_gap_closure",
        tolerance.max_gap_closure,
        oo,
        fo,
        mo
    );
    cascade!(
        resolved,
        provenance,
        "tolerance.residual",
        tolerance.residual,
        oo,
        fo,
        mo
    );
    cascade!(
        resolved,
        provenance,
        "tolerance.degeneracy",
        tolerance.degeneracy,
        oo,
        fo,
        mo
    );
    cascade!(
        resolved,
        provenance,
        "tolerance.sample_inward_offset",
        tolerance.sample_inward_offset,
        oo,
        fo,
        mo
    );
    cascade!(
        resolved,
        provenance,
        "tolerance.ray_extent",
        tolerance.ray_extent,
        oo,
        fo,
        mo
    );
    cascade!(
        resolved,
        provenance,
        "tolerance.coplanar_angle_epsilon",
        tolerance.coplanar_angle_epsilon,
        oo,
        fo,
        mo
    );
    cascade!(
        resolved,
        provenance,
        "tolerance.coplanar_offset_epsilon",
        tolerance.coplanar_offset_epsilon,
        oo,
        fo,
        mo
    );
    cascade!(
        resolved,
        provenance,
        "tolerance.edge_split_degeneracy",
        tolerance.edge_split_degeneracy,
        oo,
        fo,
        mo
    );
    cascade!(
        resolved,
        provenance,
        "tolerance.min_edge_length",
        tolerance.min_edge_length,
        oo,
        fo,
        mo
    );
    cascade!(
        resolved,
        provenance,
        "tolerance.collinearity_dot_tolerance",
        tolerance.collinearity_dot_tolerance,
        oo,
        fo,
        mo
    );
    cascade!(
        resolved,
        provenance,
        "tolerance.aabb_inflation",
        tolerance.aabb_inflation,
        oo,
        fo,
        mo
    );
    cascade!(
        resolved,
        provenance,
        "tolerance.model_scale_mm",
        tolerance.model_scale_mm,
        oo,
        fo,
        mo
    );
    cascade!(
        resolved,
        provenance,
        "tolerance.error_budget_mm",
        tolerance.error_budget_mm,
        oo,
        fo,
        mo
    );
    cascade!(
        resolved,
        provenance,
        "tolerance.ambiguity_band_factor",
        tolerance.ambiguity_band_factor,
        oo,
        fo,
        mo
    );

    // Solver fields
    cascade!(
        resolved,
        provenance,
        "solver.max_iterations",
        solver.max_iterations,
        oo,
        fo,
        mo
    );
    cascade!(
        resolved,
        provenance,
        "solver.convergence_tolerance",
        solver.convergence_tolerance,
        oo,
        fo,
        mo
    );
    cascade!(
        resolved,
        provenance,
        "solver.divergence_threshold",
        solver.divergence_threshold,
        oo,
        fo,
        mo
    );

    // Validation fields
    cascade!(
        resolved,
        provenance,
        "validation.checkpoints",
        validation.checkpoints,
        oo,
        fo,
        mo
    );
    cascade!(
        resolved,
        provenance,
        "validation.include_geometric",
        validation.include_geometric,
        oo,
        fo,
        mo
    );
    cascade!(
        resolved,
        provenance,
        "validation.entity_limit",
        validation.entity_limit,
        oo,
        fo,
        mo
    );

    // Policy fields
    cascade!(
        resolved,
        provenance,
        "policy.fallback_rules",
        policy.fallback_rules,
        oo,
        fo,
        mo
    );

    // Precision fields
    cascade!(
        resolved,
        provenance,
        "precision.bit_length_threshold",
        precision.bit_length_threshold,
        oo,
        fo,
        mo
    );

    let rc = ResolvedConfig {
        config: resolved,
        provenance,
    };

    rc.cross_validate()?;

    Ok(rc)
}
