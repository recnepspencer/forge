#[cfg(test)]
mod cascade_tests {
    use super::super::{
        KernelConfig, ConfigOverride, ToleranceOverride, ConfigScope,
        resolve_config,
    };

    #[test]
    fn resolve_passes_through_base_when_no_overrides() {
        let base = KernelConfig::default();
        let resolved = resolve_config(&base, None, None, None).unwrap();

        assert_eq!(
            resolved.config().tolerance.spatial_tolerance,
            base.tolerance.spatial_tolerance
        );
        let src = resolved.source_of("tolerance.spatial_tolerance").unwrap();
        assert_eq!(src.scope, ConfigScope::SessionDefault);
        assert_eq!(src.origin, None);
    }

    #[test]
    fn resolve_merges_partial_overrides_respecting_precedence() {
        let base = KernelConfig::default();

        let mut model = ConfigOverride::default();
        model.tolerance = Some(ToleranceOverride {
            spatial_tolerance: Some(1e-4),
            angular_tolerance: Some(0.01),
            ..Default::default()
        });

        let mut feature = ConfigOverride::default();
        feature.tolerance = Some(ToleranceOverride {
            angular_tolerance: Some(0.05), // Overrides model
            min_face_area: Some(1e-8),     // New override at feature level
            ..Default::default()
        });

        let mut operation = ConfigOverride::default();
        operation.tolerance = Some(ToleranceOverride {
            spatial_tolerance: Some(1e-3), // Overrides model
            ..Default::default()
        });

        let resolved = resolve_config(
            &base,
            Some((&model, Some("ModelA".into()))),
            Some((&feature, Some("FeatureB".into()))),
            Some((&operation, Some("OpC".into()))),
        ).unwrap();

        let tol = &resolved.config().tolerance;
        assert_eq!(tol.spatial_tolerance, 1e-3);
        assert_eq!(tol.angular_tolerance, 0.05);
        assert_eq!(tol.min_face_area, 1e-8);
        assert_eq!(tol.max_slivers_per_op, base.tolerance.max_slivers_per_op);

        // Check provenance
        let src_spatial = resolved.source_of("tolerance.spatial_tolerance").unwrap();
        assert_eq!(src_spatial.scope, ConfigScope::OperationOverride);
        assert_eq!(src_spatial.origin.as_deref(), Some("OpC"));

        let src_angular = resolved.source_of("tolerance.angular_tolerance").unwrap();
        assert_eq!(src_angular.scope, ConfigScope::FeatureOverride);
        assert_eq!(src_angular.origin.as_deref(), Some("FeatureB"));

        let src_area = resolved.source_of("tolerance.min_face_area").unwrap();
        assert_eq!(src_area.scope, ConfigScope::FeatureOverride);
        assert_eq!(src_area.origin.as_deref(), Some("FeatureB"));

        let src_slivers = resolved.source_of("tolerance.max_slivers_per_op").unwrap();
        assert_eq!(src_slivers.scope, ConfigScope::SessionDefault);
        assert_eq!(src_slivers.origin, None);
    }

    #[test]
    fn cross_validate_catches_gap_closure_violation() {
        let base = KernelConfig::default();

        let mut operation = ConfigOverride::default();
        operation.tolerance = Some(ToleranceOverride {
            spatial_tolerance: Some(1e-6),
            ambiguity_band_factor: Some(10.0), // limit is 1e-6 * 10 * 10 = 1e-4
            max_gap_closure: Some(1.1e-4),     // Violates relaxed limit
            ..Default::default()
        });

        let result = resolve_config(&base, None, None, Some((&operation, None)));
        assert!(result.is_err());
        let err = result.unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("max_gap_closure"), "Error should mention max_gap_closure: {}", msg);
    }
}
