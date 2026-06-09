use crate::construction::tests::support::corpus_cases::{
    simplex_world_collapsed_admitted_local_or_exact_intent,
    simplex_world_collapsed_explicit_exhaustion_intent,
    simplex_world_collapsed_threshold_rejected_intent,
};
use crate::construction::tests::support::corpus_replay_row::PrimitiveConstructionCorpusParameterRole;
use crate::construction::tests::support::corpus_replay_view::{
    row_attempted_realization_strategies, row_normalization_disposition, siege_report,
    siege_workspace,
};
use crate::construction::tests::support::corpus_simplex_registry::{
    required_simplex_exhaustion_witness_kinds, required_simplex_ladder_scenarios,
};
use crate::construction::tests::support::projection_consumption::{
    prepare_primitive_construction_query_projection_consumption_surface_digest,
    PrimitiveConstructionQueryProjectionConsumptionParityError,
};
use crate::construction::tests::support::realization::{
    prepare_primitive_construction_realization_exhaustion_report,
    prepare_primitive_construction_realization_exhaustion_witness_report,
    prepare_primitive_construction_realization_strategy_report,
    PrimitiveConstructionRealizationExhaustionStatus,
};
use crate::construction::tests::support::runtime_truth::{
    prepare_primitive_construction_certification_runtime_truth,
    PrimitiveConstructionCertificationRuntimeTruth,
};
use worth_geom::facade::{
    PrimitiveNormalizationDisposition, PrimitiveRealizationExhaustionReason,
    PrimitiveRealizationExhaustionWitnessKind, PrimitiveRealizationStrategy,
    PrimitiveStabilityClass,
};

#[test]
fn simplex_realization_strategy_ladder_inputs_bind_direct_query_and_corpus_truth() {
    let mut workspace = siege_workspace("corpus-simplex-ladder.rows");
    let report = siege_report("corpus-simplex-ladder.rows.corpus");
    let admitted_intent = simplex_world_collapsed_admitted_local_or_exact_intent();
    let rejected_intent = simplex_world_collapsed_threshold_rejected_intent();
    let exhausted_intent = simplex_world_collapsed_explicit_exhaustion_intent();
    let admitted = report
        .row_for(
            admitted_intent.family(),
            PrimitiveConstructionCorpusParameterRole::ThresholdAdmitted,
        )
        .expect("admitted simplex row");
    let rejected = report
        .row_for(
            rejected_intent.family(),
            PrimitiveConstructionCorpusParameterRole::ThresholdRejected,
        )
        .expect("threshold rejected simplex row");
    let exhausted = report
        .row_for(
            exhausted_intent.family(),
            PrimitiveConstructionCorpusParameterRole::ExplicitExhaustion,
        )
        .expect("exhausted simplex row");
    let admitted_strategy =
        prepare_primitive_construction_realization_strategy_report(admitted_intent.clone());
    let rejected_strategy =
        prepare_primitive_construction_realization_strategy_report(rejected_intent.clone());
    let exhausted_strategy =
        prepare_primitive_construction_realization_strategy_report(exhausted_intent.clone());
    let rejected_exhaustion =
        prepare_primitive_construction_realization_exhaustion_report(rejected_intent.clone());
    let exhausted_exhaustion =
        prepare_primitive_construction_realization_exhaustion_report(exhausted_intent.clone());
    let admitted_projection =
        prepare_primitive_construction_query_projection_consumption_surface_digest(
            &mut workspace,
            admitted_intent.clone(),
        )
        .expect("admitted projection digest");
    let rejected_projection =
        prepare_primitive_construction_query_projection_consumption_surface_digest(
            &mut workspace,
            rejected_intent,
        );
    let exhausted_projection =
        prepare_primitive_construction_query_projection_consumption_surface_digest(
            &mut workspace,
            exhausted_intent,
        );

    assert!(report.required_scenario_coverage_verified());
    assert_eq!(
        report
            .scenario_ids()
            .into_iter()
            .filter(|scenario_id| scenario_id.starts_with("simplex_world_collapsed_"))
            .collect::<Vec<_>>(),
        required_simplex_ladder_scenarios()
            .iter()
            .map(|scenario_id| (*scenario_id).to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        admitted.parameter_role(),
        PrimitiveConstructionCorpusParameterRole::ThresholdAdmitted
    );
    assert_eq!(
        admitted_strategy.selected_strategy(),
        Some(PrimitiveRealizationStrategy::ExactSupport)
    );
    assert_eq!(
        admitted_strategy.attempted_strategies(),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
    );
    assert_eq!(
        admitted_strategy.stability_class(),
        Some(PrimitiveStabilityClass::StableAfterEscalation)
    );
    let admitted_runtime_truth = match prepare_primitive_construction_certification_runtime_truth(
        admitted_intent.into_request(),
    ) {
        PrimitiveConstructionCertificationRuntimeTruth::Admitted(outcome) => outcome,
        PrimitiveConstructionCertificationRuntimeTruth::Rejected(_) => {
            panic!("admitted simplex intent should retain admitted runtime truth")
        }
    };
    assert_eq!(
        row_normalization_disposition(admitted)
            .expect("admitted simplex corpus row should preserve normalization"),
        PrimitiveNormalizationDisposition::LocalTransformationApplied
    );

    assert_eq!(
        rejected.parameter_role(),
        PrimitiveConstructionCorpusParameterRole::ThresholdRejected
    );
    assert_eq!(rejected_strategy.selected_strategy(), None);
    assert_eq!(
        rejected_exhaustion.status(),
        PrimitiveConstructionRealizationExhaustionStatus::NotApplicable
    );
    assert!(rejected_exhaustion.exhaustion_reason().is_none());
    assert!(matches!(
        rejected_projection,
        Err(PrimitiveConstructionQueryProjectionConsumptionParityError::RejectedOutcome { .. })
    ));

    assert_eq!(
        exhausted.parameter_role(),
        PrimitiveConstructionCorpusParameterRole::ExplicitExhaustion
    );
    assert_eq!(exhausted_strategy.selected_strategy(), None);
    assert_eq!(
        exhausted_exhaustion.status(),
        PrimitiveConstructionRealizationExhaustionStatus::Exhausted
    );
    assert_eq!(
        exhausted_exhaustion.exhaustion_reason(),
        Some(PrimitiveRealizationExhaustionReason::DegenerateSupportNormals)
    );
    assert_eq!(
        exhausted_strategy.stability_class(),
        Some(PrimitiveStabilityClass::RejectedBelowConditioningFloor)
    );
    assert!(report.row_digest_uniqueness_verified());
    assert_ne!(
        rejected_strategy.report_digest(),
        exhausted_strategy.report_digest()
    );
    assert_ne!(
        rejected_exhaustion.report_digest(),
        exhausted_exhaustion.report_digest()
    );
    assert_eq!(
        admitted_runtime_truth.read_surface(),
        topology::facade::TopologyConstructionQueryReadSurface::ProjectionConsumptionFromInspectionReceipt
    );
    assert_eq!(
        admitted_runtime_truth.inspection_surface(),
        topology::facade::TopologyConstructionQueryInspectionSurface::InspectReceipt
    );
    assert_eq!(
        admitted_runtime_truth.fact_provenance(),
        topology::facade::TopologyConstructionQueryFactProvenance::InspectionBackedProjectionConsumption
    );
    assert_ne!(
        admitted_projection,
        report
            .row_digest(admitted)
            .expect("admitted simplex row digest")
    );
    assert_ne!(
        report.report_digest(),
        report
            .row_digest(admitted)
            .expect("admitted simplex row digest")
    );
    assert!(matches!(
        exhausted_projection,
        Err(PrimitiveConstructionQueryProjectionConsumptionParityError::RejectedOutcome { .. })
    ));
}

#[test]
fn simplex_exhaustion_witness_inventory_comes_from_lower_realization_owner() {
    let report = prepare_primitive_construction_realization_exhaustion_witness_report();
    let simplex_rows = report
        .rows()
        .iter()
        .filter(|row| {
            row.family() == crate::construction::request::PrimitiveConstructionFamily::SimplexSolid
        })
        .collect::<Vec<_>>();
    let zero_scale = report
        .row_for(PrimitiveRealizationExhaustionWitnessKind::ZeroScaleSimplexSupportCollapse)
        .expect("zero-scale simplex witness");
    let altitude_squeezed = report
        .row_for(PrimitiveRealizationExhaustionWitnessKind::AltitudeSqueezedSimplexSupportCollapse)
        .expect("altitude-squeezed simplex witness");

    assert_eq!(
        simplex_rows
            .iter()
            .map(|row| row.witness_kind())
            .collect::<Vec<_>>(),
        required_simplex_exhaustion_witness_kinds().to_vec()
    );
    assert!(simplex_rows.iter().all(|row| row.exhaustion_reason()
        == PrimitiveRealizationExhaustionReason::DegenerateSupportNormals));
    assert!(simplex_rows.iter().all(|row| row.attempted_strategies()
        == &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]));
    assert_eq!(
        zero_scale.witness_kind(),
        PrimitiveRealizationExhaustionWitnessKind::ZeroScaleSimplexSupportCollapse
    );
    assert_eq!(
        altitude_squeezed.witness_kind(),
        PrimitiveRealizationExhaustionWitnessKind::AltitudeSqueezedSimplexSupportCollapse
    );
    assert_ne!(report.report_digest(), zero_scale.row_digest());
}

#[test]
fn simplex_drift_surface_reads_direct_threshold_and_lower_realization_truth() {
    let report = siege_report("corpus-simplex-ladder.boundary-drift");
    let exhaustion = prepare_primitive_construction_realization_exhaustion_witness_report();
    let simplex = report
        .row_for(
            crate::construction::request::PrimitiveConstructionFamily::SimplexSolid,
            PrimitiveConstructionCorpusParameterRole::ThresholdAdmitted,
        )
        .expect("simplex drift row");
    let simplex_witnesses = exhaustion
        .rows()
        .iter()
        .filter(|row| {
            row.family() == crate::construction::request::PrimitiveConstructionFamily::SimplexSolid
        })
        .collect::<Vec<_>>();

    assert_eq!(
        row_attempted_realization_strategies(simplex),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
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
    assert_ne!(
        report.report_digest(),
        report
            .row_digest(simplex)
            .expect("simplex drift row digest")
    );
}
