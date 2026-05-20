use worth_geom::facade::{
    PrimitiveFeatureConditioningClass, PrimitiveNormalizationDisposition,
    PrimitiveRealizationStrategy, PrimitiveSupportNormalClass,
};

use super::super::PrimitiveConstructionCorpusParameterRole;
use super::support::siege_report;
use crate::construction::PrimitiveConstructionFamily;

#[test]
fn corpus_replay_siege_distinguishes_direct_and_escalated_conditioning_classes() {
    let report = siege_report("corpus-replay-siege.conditioning-classes");
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
        prism.normalization_disposition(),
        Some(PrimitiveNormalizationDisposition::WorldSpaceSufficient)
    );
    assert_eq!(
        prism.support_normal_class(),
        Some(PrimitiveSupportNormalClass::Robust)
    );
    assert_eq!(
        pyramid.normalization_disposition(),
        Some(PrimitiveNormalizationDisposition::LocalTransformationApplied)
    );
    assert_eq!(
        pyramid.realization_strategy(),
        Some(PrimitiveRealizationStrategy::ExactSupport)
    );
    assert_eq!(
        pyramid.attempted_realization_strategies(),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
    );
    assert_eq!(
        pyramid.support_normal_class(),
        Some(PrimitiveSupportNormalClass::Degenerate)
    );
    assert_eq!(
        pyramid.feature_conditioning_class(),
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
        row.realization_strategy(),
        Some(PrimitiveRealizationStrategy::ExactSupport)
    );
    assert_eq!(
        row.attempted_realization_strategies(),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
    );
    assert_eq!(
        row.normalization_disposition(),
        Some(PrimitiveNormalizationDisposition::LocalTransformationApplied)
    );
    assert_eq!(
        row.feature_conditioning_class(),
        Some(PrimitiveFeatureConditioningClass::Healthy)
    );
}
