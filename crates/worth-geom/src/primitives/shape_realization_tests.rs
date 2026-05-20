use super::{
    primitive_realization_exhaustion_witness_rows, realize_prism_support, realize_pyramid_support,
    PrimitiveFeatureConditioningClass, PrimitiveNormalizationDisposition,
    PrimitiveRealizationExhaustionReason, PrimitiveRealizationExhaustionWitnessKind,
    PrimitiveRealizationStrategy, PrimitiveStabilityClass, PrimitiveSupportNormalClass,
};

#[test]
fn prism_threshold_case_remains_direct_stable() {
    let realization = realize_prism_support([0.0, 0.0, 0.0], 3, 1.0e-150, 1.0e-150).expect("prism");
    assert_eq!(
        realization.report().strategy(),
        PrimitiveRealizationStrategy::DirectWorld
    );
    assert_eq!(
        realization.report().stability_class(),
        PrimitiveStabilityClass::StableDirect
    );
    assert_eq!(
        realization
            .report()
            .conditioning_witness()
            .normalization_disposition(),
        PrimitiveNormalizationDisposition::WorldSpaceSufficient
    );
}

#[test]
fn tiny_pyramid_escalates_to_exact_support_realization() {
    let realization =
        realize_pyramid_support([0.0, 0.0, 0.0], 3, 1.0e-200, 1.0e-200).expect("pyramid");
    assert_eq!(
        realization.report().strategy(),
        PrimitiveRealizationStrategy::ExactSupport
    );
    assert_eq!(
        realization.report().stability_class(),
        PrimitiveStabilityClass::StableAfterEscalation
    );
    assert_eq!(
        realization
            .report()
            .conditioning_witness()
            .normalization_disposition(),
        PrimitiveNormalizationDisposition::LocalTransformationApplied
    );
    assert_eq!(
        realization
            .report()
            .conditioning_witness()
            .support_normal_class(),
        PrimitiveSupportNormalClass::Degenerate
    );
    assert_eq!(
        realization
            .report()
            .conditioning_witness()
            .feature_conditioning_class(),
        PrimitiveFeatureConditioningClass::Healthy
    );
    assert_eq!(realization.planes().len(), 4);
}

#[test]
fn large_coordinate_pyramid_uses_local_normalized_realization() {
    let realization = realize_pyramid_support(
        [2f64.powi(548), -2f64.powi(548), 2f64.powi(548)],
        4,
        2f64.powi(500),
        2f64.powi(501),
    )
    .expect("large-coordinate pyramid");
    assert_eq!(
        realization.report().strategy(),
        PrimitiveRealizationStrategy::LocalNormalized
    );
    assert_eq!(
        realization.report().stability_class(),
        PrimitiveStabilityClass::StableAfterEscalation
    );
    assert_eq!(
        realization
            .report()
            .conditioning_witness()
            .normalization_disposition(),
        PrimitiveNormalizationDisposition::LocalTransformationApplied
    );
}

#[test]
fn world_collapsed_pyramid_is_salvaged_by_exact_support() {
    let realization =
        realize_pyramid_support([1.0e308, 1.0e308, 1.0e308], 3, 1.0, 1.0).expect("pyramid");

    assert_eq!(
        realization.report().strategy(),
        PrimitiveRealizationStrategy::ExactSupport
    );
    assert_eq!(
        realization.report().attempted_strategies(),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
    );
    assert_eq!(
        realization.report().stability_class(),
        PrimitiveStabilityClass::StableAfterEscalation
    );
    assert_eq!(
        realization
            .report()
            .conditioning_witness()
            .normalization_disposition(),
        PrimitiveNormalizationDisposition::LocalTransformationApplied
    );
}

#[test]
fn zero_radius_pyramid_support_collapse_becomes_a_real_exhaustion_witness() {
    let rows = primitive_realization_exhaustion_witness_rows();
    let row = rows
        .iter()
        .find(|row| {
            row.witness_kind()
                == PrimitiveRealizationExhaustionWitnessKind::ZeroRadiusPyramidSupportCollapse
        })
        .expect("zero-radius pyramid witness");

    assert_eq!(row.family(), "regular_pyramid");
    assert_eq!(
        row.attempted_strategies(),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
    );
    assert_eq!(
        row.exhaustion_reason(),
        PrimitiveRealizationExhaustionReason::DegenerateSupportNormals
    );
    assert_eq!(
        row.exhaustion_report().stability_class(),
        PrimitiveStabilityClass::RejectedBelowConditioningFloor
    );
    assert_eq!(
        row.exhaustion_report()
            .conditioning_witness()
            .normalization_disposition(),
        PrimitiveNormalizationDisposition::LocalTransformationApplied
    );
    assert!(!row.row_digest().is_empty());
}
