use crate::construction::certification::corpus::replay_siege_report::{
    row_attempted_realization_strategies, row_blocking_boundary, row_normalization_disposition,
    row_realization_strategy, row_rejection_locality, row_stability_class, siege_report,
};
use crate::construction::outcome::PrimitiveConstructionRejectionLocality;
use crate::construction::request::{PrimitiveConstructionFamily, PRIMITIVE_CONSTRUCTION_FAMILIES};
use crate::construction::tests::support::blocking_boundary::PrimitiveConstructionBlockingBoundary;
use crate::construction::tests::support::corpus_replay_row::PrimitiveConstructionCorpusParameterRole;
use crate::construction::tests::support::realization::prepare_primitive_construction_realization_exhaustion_witness_report;
use worth_geom::facade::{
    PrimitiveNormalizationDisposition, PrimitiveRealizationExhaustionReason,
    PrimitiveRealizationExhaustionWitnessKind, PrimitiveRealizationStrategy,
    PrimitiveStabilityClass,
};

#[test]
fn threshold_corpus_rows_cover_every_family_boundary_pair() {
    let report = siege_report("corpus-family-boundaries.threshold-pairs");

    assert!(report.required_scenario_coverage_verified());
    assert!(report.row_digest_uniqueness_verified());
    assert_eq!(
        PRIMITIVE_CONSTRUCTION_FAMILIES
            .iter()
            .flat_map(|family| {
                [
                    (
                        *family,
                        PrimitiveConstructionCorpusParameterRole::ThresholdAdmitted,
                    ),
                    (
                        *family,
                        PrimitiveConstructionCorpusParameterRole::ThresholdRejected,
                    ),
                ]
            })
            .all(|(family, role)| report.row_for(family, role).is_some()),
        true
    );
    assert_ne!(
        report.report_digest(),
        report
            .row_digest(
                report
                    .row_for(
                        PrimitiveConstructionFamily::SimplexSolid,
                        PrimitiveConstructionCorpusParameterRole::ThresholdAdmitted,
                    )
                    .expect("simplex row"),
            )
            .expect("simplex row digest")
    );
}

#[test]
fn threshold_corpus_rows_distinguish_direct_and_escalated_boundary_truth() {
    let report = siege_report("corpus-family-boundaries.transition-classes");
    let simplex = report
        .row_for(
            PrimitiveConstructionFamily::SimplexSolid,
            PrimitiveConstructionCorpusParameterRole::ThresholdAdmitted,
        )
        .expect("simplex row");
    let prism = report
        .row_for(
            PrimitiveConstructionFamily::RegularPrism,
            PrimitiveConstructionCorpusParameterRole::ThresholdAdmitted,
        )
        .expect("prism row");
    let pyramid = report
        .row_for(
            PrimitiveConstructionFamily::RegularPyramid,
            PrimitiveConstructionCorpusParameterRole::ThresholdAdmitted,
        )
        .expect("pyramid row");

    assert_eq!(
        row_realization_strategy(prism),
        Some(PrimitiveRealizationStrategy::DirectWorld)
    );
    assert_eq!(
        row_stability_class(prism),
        Some(PrimitiveStabilityClass::StableDirect)
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
        row_stability_class(simplex),
        Some(PrimitiveStabilityClass::StableAfterEscalation)
    );
    assert_eq!(
        row_normalization_disposition(simplex),
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
        row_stability_class(pyramid),
        Some(PrimitiveStabilityClass::StableAfterEscalation)
    );
    assert_eq!(
        row_normalization_disposition(pyramid),
        Some(PrimitiveNormalizationDisposition::LocalTransformationApplied)
    );
}

#[test]
fn threshold_corpus_rows_bind_lower_layer_exhaustion_truth_for_pyramid_floor() {
    let report = siege_report("corpus-family-boundaries.lower-layer-exhaustion");
    let exhaustion = prepare_primitive_construction_realization_exhaustion_witness_report();
    let simplex = report
        .row_for(
            PrimitiveConstructionFamily::SimplexSolid,
            PrimitiveConstructionCorpusParameterRole::ThresholdRejected,
        )
        .expect("simplex threshold rejected row");
    let pyramid = report
        .row_for(
            PrimitiveConstructionFamily::RegularPyramid,
            PrimitiveConstructionCorpusParameterRole::ThresholdRejected,
        )
        .expect("pyramid threshold rejected row");
    let _orthotope = report
        .row_for(
            PrimitiveConstructionFamily::Orthotope,
            PrimitiveConstructionCorpusParameterRole::ThresholdRejected,
        )
        .expect("orthotope threshold rejected row");
    let pyramid_witnesses = exhaustion
        .rows()
        .iter()
        .filter(|row| row.family() == PrimitiveConstructionFamily::RegularPyramid)
        .collect::<Vec<_>>();
    let orthotope_witnesses = exhaustion
        .rows()
        .iter()
        .filter(|row| row.family() == PrimitiveConstructionFamily::Orthotope)
        .collect::<Vec<_>>();
    let simplex_witnesses = exhaustion
        .rows()
        .iter()
        .filter(|row| row.family() == PrimitiveConstructionFamily::SimplexSolid)
        .collect::<Vec<_>>();

    assert_eq!(
        row_rejection_locality(pyramid),
        Some(PrimitiveConstructionRejectionLocality::Admission)
    );
    assert_eq!(
        row_blocking_boundary(pyramid),
        Some(PrimitiveConstructionBlockingBoundary::PrimitiveClassAdmission)
    );
    assert_eq!(
        pyramid_witnesses
            .iter()
            .map(|witness| witness.witness_kind())
            .collect::<Vec<_>>(),
        vec![PrimitiveRealizationExhaustionWitnessKind::ZeroRadiusPyramidSupportCollapse]
    );
    assert_eq!(pyramid_witnesses.len(), 1);
    assert_eq!(
        pyramid_witnesses[0].exhaustion_reason(),
        PrimitiveRealizationExhaustionReason::DegenerateSupportNormals
    );
    assert_eq!(
        pyramid_witnesses[0].attempted_strategies(),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
    );
    assert!(!pyramid_witnesses[0].row_digest().is_empty());
    assert_eq!(orthotope_witnesses.len(), 0);
    assert_eq!(
        row_rejection_locality(simplex),
        Some(PrimitiveConstructionRejectionLocality::Admission)
    );
    assert_eq!(simplex_witnesses.len(), 2);
    assert_eq!(
        simplex_witnesses
            .iter()
            .map(|witness| witness.witness_kind())
            .collect::<Vec<_>>(),
        vec![
            PrimitiveRealizationExhaustionWitnessKind::ZeroScaleSimplexSupportCollapse,
            PrimitiveRealizationExhaustionWitnessKind::AltitudeSqueezedSimplexSupportCollapse,
        ]
    );
    assert!(simplex_witnesses
        .iter()
        .all(|witness| witness.exhaustion_reason()
            == PrimitiveRealizationExhaustionReason::DegenerateSupportNormals));
    assert!(simplex_witnesses
        .iter()
        .all(|witness| witness.attempted_strategies()
            == &[
                PrimitiveRealizationStrategy::DirectWorld,
                PrimitiveRealizationStrategy::ExactSupport,
            ]));
}
