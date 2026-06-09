use crate::construction::outcome::{
    PrimitiveConstructionRejectionClass, PrimitiveConstructionRejectionLocality,
};
use crate::construction::request::PrimitiveConstructionFamily;
use crate::construction::tests::support::blocking_boundary::PrimitiveConstructionBlockingBoundary;
use crate::construction::tests::support::corpus_replay_row::PrimitiveConstructionCorpusParameterRole;
use crate::construction::tests::support::corpus_replay_view::{
    row_attempted_realization_strategies, row_blocking_boundary, row_construction_breadth,
    row_exhaustion_reason, row_realization_strategy, row_rejection_class, row_rejection_locality,
    row_stability_class, siege_report,
};
use worth_geom::facade::{PrimitiveRealizationStrategy, PrimitiveStabilityClass};

#[test]
fn corpus_replay_siege_pyramid_threshold_rejection_stays_at_admission_boundary() {
    let report = siege_report("corpus-replay-siege.pyramid-threshold-rejection");
    let row = report
        .row_for(
            PrimitiveConstructionFamily::RegularPyramid,
            PrimitiveConstructionCorpusParameterRole::ThresholdRejected,
        )
        .expect("threshold rejected pyramid row");

    assert_eq!(
        row_rejection_class(row),
        Some(PrimitiveConstructionRejectionClass::InvalidRequest)
    );
    assert_eq!(
        row_rejection_locality(row),
        Some(PrimitiveConstructionRejectionLocality::Admission)
    );
    assert_eq!(
        row_blocking_boundary(row),
        Some(PrimitiveConstructionBlockingBoundary::PrimitiveClassAdmission)
    );
    assert!(row_attempted_realization_strategies(row).is_empty());
    assert_eq!(row_attempted_realization_strategies(row).len(), 0);
    assert_eq!(row_realization_strategy(row), None);
    assert_eq!(row_stability_class(row), None);
    assert_eq!(row_exhaustion_reason(row), None);
    assert_eq!(row_construction_breadth(row), 0);
}

#[test]
fn corpus_replay_siege_exposes_escalated_stability_for_tiny_pyramid_threshold_case() {
    let report = siege_report("corpus-replay-siege.realization");
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
        row_realization_strategy(prism),
        Some(PrimitiveRealizationStrategy::DirectWorld)
    );
    assert_eq!(
        row_stability_class(prism),
        Some(PrimitiveStabilityClass::StableDirect)
    );
    assert_eq!(
        row_realization_strategy(pyramid),
        Some(PrimitiveRealizationStrategy::ExactSupport)
    );
    assert_eq!(
        row_stability_class(pyramid),
        Some(PrimitiveStabilityClass::StableAfterEscalation)
    );
    assert_ne!(
        report.row_digest(prism).expect("threshold prism digest"),
        report
            .row_digest(pyramid)
            .expect("threshold pyramid digest")
    );
}
