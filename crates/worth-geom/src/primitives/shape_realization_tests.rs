use super::{
    primitive_realization_exhaustion_witness_rows, realize_prism_support, realize_pyramid_support,
    realize_tetrahedron_support, realize_tetrahedron_support_with_altitude_component,
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
fn tiny_simplex_escalates_out_of_direct_world_realization() {
    let error = realize_tetrahedron_support([0.0, 0.0, 0.0], 1.0e-200).expect_err("tiny simplex");
    let report = match error {
        super::PrimitiveRealizationError::Exhausted(report) => report,
        super::PrimitiveRealizationError::Geometry(other) => {
            panic!("expected exhausted realization report, got geometry error: {other}")
        }
    };
    assert_eq!(
        report.attempted_strategies(),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
    );
    assert_eq!(
        report.exhaustion_reason(),
        PrimitiveRealizationExhaustionReason::DegenerateSupportNormals
    );
    assert_eq!(
        report.stability_class(),
        PrimitiveStabilityClass::RejectedBelowConditioningFloor
    );
    assert_eq!(
        report.conditioning_witness().normalization_disposition(),
        PrimitiveNormalizationDisposition::LocalTransformationApplied
    );
}

#[test]
fn world_collapsed_simplex_uses_local_normalized_realization() {
    let error =
        realize_tetrahedron_support([1.0e308, -1.0e308, 1.0e308], 1.0).expect_err("simplex");
    let report = match error {
        super::PrimitiveRealizationError::Exhausted(report) => report,
        super::PrimitiveRealizationError::Geometry(other) => {
            panic!("expected exhausted realization report, got geometry error: {other}")
        }
    };
    assert_eq!(
        report.exhaustion_reason(),
        PrimitiveRealizationExhaustionReason::DegenerateSupportNormals
    );
    assert_eq!(
        report.attempted_strategies(),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
    );
    assert_eq!(
        report.stability_class(),
        PrimitiveStabilityClass::RejectedBelowConditioningFloor
    );
    assert_eq!(
        report.conditioning_witness().normalization_disposition(),
        PrimitiveNormalizationDisposition::LocalTransformationApplied
    );
}

#[test]
fn altitude_squeezed_world_collapsed_simplex_stays_admitted_under_exact_support() {
    let realization = realize_tetrahedron_support_with_altitude_component(
        [2f64.powi(548), -2f64.powi(548), 2f64.powi(548)],
        1.0e-200,
        1.0e-220,
    )
    .expect("altitude-squeezed simplex");

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
fn altitude_squeezed_world_collapsed_simplex_can_exhaust_exact_support() {
    let error = realize_tetrahedron_support_with_altitude_component(
        [2f64.powi(548), -2f64.powi(548), 2f64.powi(548)],
        1.0e-240,
        1.0e-280,
    )
    .expect_err("altitude-squeezed simplex exhaustion");
    let report = match error {
        super::PrimitiveRealizationError::Exhausted(report) => report,
        super::PrimitiveRealizationError::Geometry(other) => {
            panic!("expected exhausted realization report, got geometry error: {other}")
        }
    };

    assert_eq!(
        report.attempted_strategies(),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
    );
    assert_eq!(
        report.exhaustion_reason(),
        PrimitiveRealizationExhaustionReason::DegenerateSupportNormals
    );
    assert_eq!(
        report.stability_class(),
        PrimitiveStabilityClass::RejectedBelowConditioningFloor
    );
    assert_eq!(
        report.conditioning_witness().normalization_disposition(),
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

#[test]
fn zero_scale_simplex_support_collapse_becomes_a_real_exhaustion_witness() {
    let rows = primitive_realization_exhaustion_witness_rows();
    let row = rows
        .iter()
        .find(|row| {
            row.witness_kind()
                == PrimitiveRealizationExhaustionWitnessKind::ZeroScaleSimplexSupportCollapse
        })
        .expect("zero-scale simplex witness");

    assert_eq!(row.family(), "simplex_solid");
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

#[test]
fn altitude_squeezed_simplex_support_collapse_becomes_a_real_exhaustion_witness() {
    let rows = primitive_realization_exhaustion_witness_rows();
    let row = rows
        .iter()
        .find(|row| {
            row.witness_kind()
                == PrimitiveRealizationExhaustionWitnessKind::AltitudeSqueezedSimplexSupportCollapse
        })
        .expect("altitude-squeezed simplex witness");

    assert_eq!(row.family(), "simplex_solid");
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
