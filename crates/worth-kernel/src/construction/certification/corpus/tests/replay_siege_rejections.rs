use super::super::PrimitiveConstructionCorpusParameterRole;
use super::support::siege_report;
use crate::construction::diagnostics::PrimitiveConstructionBlockingBoundary;
use crate::construction::outcome::{
    PrimitiveConstructionRejectionClass, PrimitiveConstructionRejectionLocality,
};
use crate::construction::PrimitiveConstructionFamily;
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
        row.rejection_class(),
        Some(PrimitiveConstructionRejectionClass::InvalidRequest)
    );
    assert_eq!(
        row.rejection_locality(),
        Some(PrimitiveConstructionRejectionLocality::Admission)
    );
    assert_eq!(
        row.blocking_boundary(),
        Some(PrimitiveConstructionBlockingBoundary::PrimitiveClassAdmission)
    );
    assert!(row.attempted_realization_strategies().is_empty());
    assert_eq!(row.attempted_realization_strategy_count(), 0);
    assert_eq!(row.realization_strategy(), None);
    assert_eq!(row.stability_class(), None);
    assert_eq!(row.exhaustion_reason(), None);
    assert_eq!(row.construction_breadth(), 0);
    assert_eq!(row.birth_attachment_breadth(), 0);
    assert_eq!(row.certification_breadth(), 0);
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
        prism.realization_strategy(),
        Some(PrimitiveRealizationStrategy::DirectWorld)
    );
    assert_eq!(
        prism.stability_class(),
        Some(PrimitiveStabilityClass::StableDirect)
    );
    assert_eq!(
        pyramid.realization_strategy(),
        Some(PrimitiveRealizationStrategy::ExactSupport)
    );
    assert_eq!(
        pyramid.stability_class(),
        Some(PrimitiveStabilityClass::StableAfterEscalation)
    );
    assert_ne!(prism.row_digest(), pyramid.row_digest());
}

#[test]
fn corpus_replay_siege_rejection_witnesses_cover_every_major_failure_boundary() {
    let report = siege_report("corpus-replay-siege.witnesses");
    let boundaries = report
        .rejection_witness_rows()
        .iter()
        .map(|row| row.blocking_boundary())
        .collect::<Vec<_>>();

    assert!(boundaries.contains(&PrimitiveConstructionBlockingBoundary::PrimitiveClassAdmission));
    assert!(boundaries.contains(&PrimitiveConstructionBlockingBoundary::KernelIntent));
    assert!(boundaries.contains(&PrimitiveConstructionBlockingBoundary::SpatialBirth));
    assert!(boundaries.contains(&PrimitiveConstructionBlockingBoundary::TopologyLegality));
    assert_ne!(
        report.rejection_witness_rows()[0].row_digest(),
        report.rejection_witness_rows()[1].row_digest()
    );
}
