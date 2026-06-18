use worth_geom::facade::{
    PrimitiveFeatureConditioningClass, PrimitiveNormalizationDisposition,
    PrimitiveRealizationExhaustionReason, PrimitiveRealizationStrategy, PrimitiveStabilityClass,
    PrimitiveSupportNormalClass,
};

use crate::construction::certification::corpus::replay_siege_report::{
    row_attempted_realization_strategies, row_exhaustion_reason, row_feature_conditioning_class,
    row_normalization_disposition, row_realization_strategy, row_stability_class,
    row_support_normal_class, siege_report,
};
use crate::construction::request::PrimitiveConstructionFamily;
use crate::construction::tests::support::corpus_replay_row::PrimitiveConstructionCorpusParameterRole;

#[test]
fn corpus_replay_siege_distinguishes_direct_and_escalated_conditioning_classes() {
    let report = siege_report("corpus-replay-siege.conditioning-classes");
    let simplex = report
        .row_for(
            PrimitiveConstructionFamily::SimplexSolid,
            PrimitiveConstructionCorpusParameterRole::ThresholdAdmitted,
        )
        .expect("threshold simplex row");
    let prism = report
        .row_for(
            PrimitiveConstructionFamily::RegularPrism,
            PrimitiveConstructionCorpusParameterRole::ThresholdAdmitted,
        )
        .expect("threshold prism row");
    let pyramid = report
        .row_for(
            PrimitiveConstructionFamily::RegularPyramid,
            PrimitiveConstructionCorpusParameterRole::ThresholdAdmitted,
        )
        .expect("threshold pyramid row");

    assert_eq!(
        row_normalization_disposition(prism),
        Some(PrimitiveNormalizationDisposition::WorldSpaceSufficient)
    );
    assert_eq!(
        row_support_normal_class(prism),
        Some(PrimitiveSupportNormalClass::Robust)
    );
    assert_eq!(
        row_normalization_disposition(simplex),
        Some(PrimitiveNormalizationDisposition::LocalTransformationApplied)
    );
    assert_eq!(
        simplex.scenario_id(),
        "simplex_world_collapsed_admitted_local_or_exact"
    );
    assert_eq!(
        row_realization_strategy(simplex),
        Some(PrimitiveRealizationStrategy::ExactSupport)
    );
    assert_eq!(
        row_attempted_realization_strategies(simplex),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
    );
    assert_eq!(
        row_normalization_disposition(pyramid),
        Some(PrimitiveNormalizationDisposition::LocalTransformationApplied)
    );
    assert_eq!(
        row_realization_strategy(pyramid),
        Some(PrimitiveRealizationStrategy::ExactSupport)
    );
    assert_eq!(
        row_attempted_realization_strategies(pyramid),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
    );
    assert_eq!(
        row_support_normal_class(pyramid),
        Some(PrimitiveSupportNormalClass::Degenerate)
    );
    assert_eq!(
        row_feature_conditioning_class(pyramid),
        Some(PrimitiveFeatureConditioningClass::Healthy)
    );
}

#[test]
fn corpus_replay_siege_large_coordinate_pyramid_row_carries_local_normalized_truth() {
    let report = siege_report("corpus-replay-siege.large-coordinate-pyramid");
    let row = report
        .row_for(
            PrimitiveConstructionFamily::RegularPyramid,
            PrimitiveConstructionCorpusParameterRole::StressAdmitted,
        )
        .expect("stress pyramid row");

    assert_eq!(
        row_realization_strategy(row),
        Some(PrimitiveRealizationStrategy::ExactSupport)
    );
    assert_eq!(
        row_attempted_realization_strategies(row),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
    );
    assert_eq!(
        row_normalization_disposition(row),
        Some(PrimitiveNormalizationDisposition::LocalTransformationApplied)
    );
    assert_eq!(
        row_feature_conditioning_class(row),
        Some(PrimitiveFeatureConditioningClass::Healthy)
    );
}

#[test]
fn corpus_replay_siege_explicit_exhaustion_simplex_row_preserves_exhaustion_truth() {
    let report = siege_report("corpus-replay-siege.simplex-explicit-exhaustion");
    let row = report
        .row_for(
            PrimitiveConstructionFamily::SimplexSolid,
            PrimitiveConstructionCorpusParameterRole::ExplicitExhaustion,
        )
        .expect("explicit exhaustion simplex row");

    assert_eq!(row_realization_strategy(row), None);
    assert_eq!(
        row_attempted_realization_strategies(row),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
    );
    assert_eq!(
        row_stability_class(row),
        Some(PrimitiveStabilityClass::RejectedBelowConditioningFloor)
    );
    assert_eq!(
        row_normalization_disposition(row),
        Some(PrimitiveNormalizationDisposition::LocalTransformationApplied)
    );
    assert_eq!(
        row_exhaustion_reason(row),
        Some(PrimitiveRealizationExhaustionReason::DegenerateSupportNormals)
    );
}
