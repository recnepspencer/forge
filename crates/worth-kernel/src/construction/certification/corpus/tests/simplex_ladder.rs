use super::support::siege_workspace;
use crate::construction::certification::corpus::family_boundary_drift_report::prepare_primitive_construction_family_boundary_drift_report;
use crate::construction::certification::corpus::simplex_exhaustion_witness_report::prepare_primitive_construction_simplex_realization_exhaustion_witness_report;
use crate::construction::certification::corpus::simplex_ladder_report::{
    prepare_primitive_construction_simplex_realization_strategy_ladder_report,
    PrimitiveConstructionSimplexQuerySurfaceStatus,
};
use crate::construction::certification::corpus::PrimitiveConstructionCorpusParameterRole;
use crate::construction::certification::realization::PrimitiveConstructionRealizationExhaustionStatus;
use std::collections::BTreeSet;
use worth_geom::facade::{
    PrimitiveNormalizationDisposition, PrimitiveRealizationExhaustionReason,
    PrimitiveRealizationExhaustionWitnessKind, PrimitiveRealizationStrategy,
    PrimitiveStabilityClass,
};

#[test]
fn simplex_realization_strategy_ladder_report_binds_direct_query_and_corpus_truth() {
    let mut workspace = siege_workspace("corpus-simplex-ladder.rows");
    let report =
        prepare_primitive_construction_simplex_realization_strategy_ladder_report(&mut workspace)
            .expect("simplex ladder report");
    let admitted = report
        .row_for("simplex_world_collapsed_admitted_local_or_exact")
        .expect("admitted simplex row");
    let rejected = report
        .row_for("simplex_world_collapsed_threshold_rejected")
        .expect("threshold rejected simplex row");
    let exhausted = report
        .row_for("simplex_world_collapsed_explicit_exhaustion")
        .expect("exhausted simplex row");

    assert_eq!(report.rows().len(), 3);
    assert_eq!(
        admitted.parameter_role(),
        PrimitiveConstructionCorpusParameterRole::ThresholdAdmitted
    );
    assert_eq!(
        admitted.direct_selected_strategy(),
        Some(PrimitiveRealizationStrategy::ExactSupport)
    );
    assert_eq!(
        admitted.attempted_strategies(),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
    );
    assert_eq!(
        admitted.stability_class(),
        Some(PrimitiveStabilityClass::StableAfterEscalation)
    );
    assert_eq!(
        admitted.normalization_disposition(),
        Some(PrimitiveNormalizationDisposition::LocalTransformationApplied)
    );

    assert_eq!(
        rejected.parameter_role(),
        PrimitiveConstructionCorpusParameterRole::ThresholdRejected
    );
    assert_eq!(rejected.direct_selected_strategy(), None);
    assert_eq!(
        rejected.exhaustion_status(),
        PrimitiveConstructionRealizationExhaustionStatus::NotApplicable
    );
    assert!(rejected.exhaustion_reason().is_none());
    assert_eq!(
        rejected.query_surface_status(),
        PrimitiveConstructionSimplexQuerySurfaceStatus::UnavailableByTypedAdmissionRejection
    );
    assert!(rejected.inspection_digest().is_none());
    assert!(rejected.projection_consumption_digest().is_none());

    assert_eq!(
        exhausted.parameter_role(),
        PrimitiveConstructionCorpusParameterRole::ExplicitExhaustion
    );
    assert_eq!(exhausted.direct_selected_strategy(), None);
    assert_eq!(
        exhausted.exhaustion_status(),
        PrimitiveConstructionRealizationExhaustionStatus::Exhausted
    );
    assert_eq!(
        exhausted.exhaustion_reason(),
        Some(PrimitiveRealizationExhaustionReason::DegenerateSupportNormals)
    );
    assert_eq!(
        exhausted.stability_class(),
        Some(PrimitiveStabilityClass::RejectedBelowConditioningFloor)
    );
    assert_eq!(
        exhausted.query_surface_status(),
        PrimitiveConstructionSimplexQuerySurfaceStatus::UnavailableByRealizationExhaustion
    );
    assert!(exhausted.inspection_digest().is_none());
    assert!(exhausted.projection_consumption_digest().is_none());
    assert_eq!(
        report
            .rows()
            .iter()
            .map(|row| row.row_digest())
            .collect::<BTreeSet<_>>()
            .len(),
        report.rows().len()
    );
    assert_eq!(
        report
            .rows()
            .iter()
            .map(|row| row.replay_digest())
            .collect::<BTreeSet<_>>()
            .len(),
        report.rows().len()
    );
    assert_eq!(
        report
            .rows()
            .iter()
            .map(|row| row.branch_local_digest())
            .collect::<BTreeSet<_>>()
            .len(),
        report.rows().len()
    );
    assert_eq!(
        report
            .rows()
            .iter()
            .map(|row| row.corpus_row_digest())
            .collect::<BTreeSet<_>>()
            .len(),
        report.rows().len()
    );
    assert_ne!(
        rejected.direct_strategy_digest(),
        exhausted.direct_strategy_digest()
    );
    assert_ne!(
        rejected.direct_exhaustion_digest(),
        exhausted.direct_exhaustion_digest()
    );
    let admitted_inspection_digest = admitted
        .inspection_digest()
        .expect("admitted simplex row should open inspection");
    let admitted_projection_digest = admitted
        .projection_consumption_digest()
        .expect("admitted simplex row should open projection");
    assert_ne!(admitted_inspection_digest, admitted.row_digest());
    assert_ne!(admitted_projection_digest, admitted.row_digest());
    for row in report.rows() {
        match row.query_surface_status() {
            PrimitiveConstructionSimplexQuerySurfaceStatus::Available => {
                assert!(row.inspection_digest().is_some());
                assert!(row.projection_consumption_digest().is_some());
            }
            PrimitiveConstructionSimplexQuerySurfaceStatus::UnavailableByTypedAdmissionRejection => {
                assert!(row.inspection_digest().is_none());
                assert!(row.projection_consumption_digest().is_none());
            }
            PrimitiveConstructionSimplexQuerySurfaceStatus::UnavailableByRealizationExhaustion => {
                assert!(row.inspection_digest().is_none());
                assert!(row.projection_consumption_digest().is_none());
            }
        }
    }
    assert_ne!(report.report_digest(), admitted.row_digest());
}

#[test]
fn simplex_exhaustion_witness_report_binds_lower_layer_simplex_witness_inventory() {
    let report = prepare_primitive_construction_simplex_realization_exhaustion_witness_report();
    let zero_scale = report
        .row_for(PrimitiveRealizationExhaustionWitnessKind::ZeroScaleSimplexSupportCollapse)
        .expect("zero-scale simplex witness");
    let altitude_squeezed = report
        .row_for(PrimitiveRealizationExhaustionWitnessKind::AltitudeSqueezedSimplexSupportCollapse)
        .expect("altitude-squeezed simplex witness");

    assert_eq!(report.rows().len(), 2);
    assert_eq!(
        zero_scale.linked_parameter_role(),
        PrimitiveConstructionCorpusParameterRole::ThresholdRejected
    );
    assert_eq!(
        altitude_squeezed.linked_parameter_role(),
        PrimitiveConstructionCorpusParameterRole::ExplicitExhaustion
    );
    assert!(report.rows().iter().all(|row| row.exhaustion_reason()
        == PrimitiveRealizationExhaustionReason::DegenerateSupportNormals));
    assert!(report.rows().iter().all(|row| row.attempted_strategies()
        == &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]));
    assert_eq!(
        report
            .rows()
            .iter()
            .map(|row| row.lower_layer_row_digest())
            .collect::<BTreeSet<_>>()
            .len(),
        report.rows().len()
    );
    assert_ne!(report.report_digest(), zero_scale.lower_layer_row_digest());
}

#[test]
fn family_boundary_drift_report_exposes_the_named_simplex_drift_surface() {
    let mut workspace = siege_workspace("corpus-simplex-ladder.boundary-drift");
    let report = prepare_primitive_construction_family_boundary_drift_report(&mut workspace)
        .expect("family boundary drift report");
    let simplex = report
        .row_for(crate::construction::PrimitiveConstructionFamily::SimplexSolid)
        .expect("simplex drift row");

    assert_eq!(
        simplex.admitted_attempted_strategies(),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
    );
    assert_eq!(simplex.lower_layer_exhaustion_witnesses().len(), 2);
    assert_eq!(
        simplex
            .lower_layer_exhaustion_witnesses()
            .iter()
            .map(|witness| witness.witness_kind())
            .collect::<Vec<_>>(),
        vec![
            PrimitiveRealizationExhaustionWitnessKind::ZeroScaleSimplexSupportCollapse,
            PrimitiveRealizationExhaustionWitnessKind::AltitudeSqueezedSimplexSupportCollapse,
        ]
    );
    assert_ne!(report.report_digest(), simplex.row_digest());
}
