use super::super::{
    PrimitiveConstructionCorpusOutcomeDisposition, PrimitiveConstructionCorpusParameterRole,
};
use crate::construction::{
    PrimitiveConstructionBlockingBoundary, PrimitiveConstructionFamily,
    PrimitiveConstructionRejectionClass, PrimitiveConstructionRejectionLocality,
};
use std::collections::BTreeSet;
use worth_geom::facade::{
    PrimitiveRealizationExhaustionReason, PrimitiveRealizationStrategy, PrimitiveStabilityClass,
};

use super::support::siege_report;

#[test]
fn corpus_replay_siege_mixes_admitted_and_rejected_rows_across_family_ladder() {
    let report = siege_report("corpus-replay-siege");
    let rejected_ids = report
        .rows()
        .iter()
        .filter(|row| {
            row.outcome_disposition() == PrimitiveConstructionCorpusOutcomeDisposition::Rejected
        })
        .map(|row| row.scenario_id().to_string())
        .collect::<Vec<_>>();

    assert_eq!(
        rejected_ids,
        vec![
            "simplex_world_collapsed_threshold_rejected".to_string(),
            "simplex_world_collapsed_explicit_exhaustion".to_string(),
            "simplex_rejected".to_string(),
            "orthotope_threshold_rejected".to_string(),
            "orthotope_rejected".to_string(),
            "regular_prism_threshold_rejected".to_string(),
            "regular_prism_rejected".to_string(),
            "pyramid_threshold_rejected_neighbor".to_string(),
            "pyramid_semantic_exhaustion".to_string(),
            "regular_pyramid_rejected".to_string(),
            "wire_body_threshold_rejected".to_string(),
            "wire_body_rejected".to_string(),
            "shell_with_hole_threshold_rejected".to_string(),
            "shell_with_hole_rejected".to_string(),
        ]
    );
    assert_eq!(report.rows().len(), 38);
    assert_eq!(report.accepted_count(), 24);
    assert_eq!(report.rejected_count(), 14);
    assert_eq!(report.authoring_order_rows().len(), 4);
    assert!(report.authoring_order_parity_verified());
    assert_eq!(report.rejection_witness_rows().len(), 6);
    assert_ne!(
        report.report_digest(),
        report
            .row_for(
                PrimitiveConstructionFamily::SimplexSolid,
                PrimitiveConstructionCorpusParameterRole::ThresholdAdmitted,
            )
            .expect("simplex threshold admitted row")
            .row_digest()
    );
}

#[test]
fn corpus_replay_siege_admitted_rows_bind_real_breadth_and_birth_truth() {
    let report = siege_report("corpus-replay-siege.accepted");
    let row = report
        .row_for(
            PrimitiveConstructionFamily::ShellWithHole,
            PrimitiveConstructionCorpusParameterRole::StressAdmitted,
        )
        .expect("stress shell row");

    assert_eq!(
        row.outcome_disposition(),
        PrimitiveConstructionCorpusOutcomeDisposition::Admitted
    );
    let birth_digest = row.birth_digest().expect("admitted rows keep birth truth");
    assert!(row.construction_breadth() > 0);
    assert!(row.birth_attachment_breadth() > 0);
    assert!(row.certification_breadth() > 0);
    assert_ne!(birth_digest, row.row_digest());
    assert_ne!(birth_digest, row.direct_construction_digest());
    assert_eq!(
        row.realization_strategy(),
        Some(PrimitiveRealizationStrategy::DirectWorld)
    );
    assert_eq!(
        row.stability_class(),
        Some(PrimitiveStabilityClass::StableDirect)
    );
}

#[test]
fn corpus_replay_siege_rejected_rows_preserve_typed_rejection_and_zero_breadth() {
    let report = siege_report("corpus-replay-siege.rejected");
    let row = report
        .row_for(
            PrimitiveConstructionFamily::WireBody,
            PrimitiveConstructionCorpusParameterRole::ThresholdRejected,
        )
        .expect("rejected wire row");

    assert_eq!(
        row.outcome_disposition(),
        PrimitiveConstructionCorpusOutcomeDisposition::Rejected
    );
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
    assert!(row.birth_digest().is_none());
    assert_eq!(row.construction_breadth(), 0);
    assert_eq!(row.birth_attachment_breadth(), 0);
    assert_eq!(row.certification_breadth(), 0);
}

#[test]
fn corpus_replay_siege_row_digests_drift_across_parameter_roles() {
    let report = siege_report("corpus-replay-siege.digest-drift");
    let minimal = report
        .row_for(
            PrimitiveConstructionFamily::RegularPrism,
            PrimitiveConstructionCorpusParameterRole::MinimalAdmitted,
        )
        .expect("minimal prism row");
    let stress = report
        .row_for(
            PrimitiveConstructionFamily::RegularPrism,
            PrimitiveConstructionCorpusParameterRole::StressAdmitted,
        )
        .expect("stress prism row");

    assert_ne!(minimal.row_digest(), stress.row_digest());
    assert_ne!(
        minimal.direct_construction_digest(),
        stress.direct_construction_digest()
    );
}

#[test]
fn corpus_replay_siege_rows_expose_stable_scenario_identity() {
    let report = siege_report("corpus-replay-siege.scenario-id");
    let row = report
        .row_for(
            PrimitiveConstructionFamily::Orthotope,
            PrimitiveConstructionCorpusParameterRole::GenericAdmitted,
        )
        .expect("generic orthotope row");

    assert_eq!(row.scenario_id(), "orthotope_generic");
}

#[test]
fn corpus_replay_siege_authoring_order_rows_prove_multiple_real_sequence_shapes() {
    let report = siege_report("corpus-replay-siege.ordering");
    let lanes = report.authoring_order_rows();

    assert!(lanes.iter().all(|row| row.parity_verified()));
    assert_eq!(
        lanes.iter().map(|row| row.lane_name()).collect::<Vec<_>>(),
        vec!["canonical", "reversed", "rejected_first", "role_clustered"]
    );
    assert_eq!(
        lanes[0].normalized_matrix_digest(),
        lanes[1].normalized_matrix_digest()
    );
    assert_eq!(
        lanes[0].normalized_matrix_digest(),
        lanes[2].normalized_matrix_digest()
    );
    assert_eq!(
        lanes[0].normalized_matrix_digest(),
        lanes[3].normalized_matrix_digest()
    );
    assert_eq!(
        lanes
            .iter()
            .map(|row| row.lane_digest())
            .collect::<BTreeSet<_>>()
            .len(),
        lanes.len()
    );
}

#[test]
fn corpus_replay_siege_breadth_shapes_widen_monotonically_under_shell_pressure() {
    let report = siege_report("corpus-replay-siege.breadth");
    let minimal = report
        .row_for(
            PrimitiveConstructionFamily::ShellWithHole,
            PrimitiveConstructionCorpusParameterRole::MinimalAdmitted,
        )
        .expect("minimal shell row");
    let threshold = report
        .row_for(
            PrimitiveConstructionFamily::ShellWithHole,
            PrimitiveConstructionCorpusParameterRole::ThresholdAdmitted,
        )
        .expect("threshold shell row");
    let generic = report
        .row_for(
            PrimitiveConstructionFamily::ShellWithHole,
            PrimitiveConstructionCorpusParameterRole::GenericAdmitted,
        )
        .expect("generic shell row");
    let stress = report
        .row_for(
            PrimitiveConstructionFamily::ShellWithHole,
            PrimitiveConstructionCorpusParameterRole::StressAdmitted,
        )
        .expect("stress shell row");

    assert!(minimal.construction_breadth() < threshold.construction_breadth());
    assert!(threshold.construction_breadth() < generic.construction_breadth());
    assert!(generic.construction_breadth() < stress.construction_breadth());
    assert!(minimal.birth_attachment_breadth() < threshold.birth_attachment_breadth());
    assert!(threshold.birth_attachment_breadth() < generic.birth_attachment_breadth());
    assert!(generic.birth_attachment_breadth() < stress.birth_attachment_breadth());
    assert!(minimal.certification_breadth() < threshold.certification_breadth());
    assert!(threshold.certification_breadth() < generic.certification_breadth());
    assert!(generic.certification_breadth() < stress.certification_breadth());
}

#[test]
fn corpus_replay_siege_distinguishes_threshold_and_explicit_rejections_within_a_family() {
    let report = siege_report("corpus-replay-siege.threshold");
    let threshold = report
        .row_for(
            PrimitiveConstructionFamily::Orthotope,
            PrimitiveConstructionCorpusParameterRole::ThresholdRejected,
        )
        .expect("threshold orthotope row");
    let explicit = report
        .row_for(
            PrimitiveConstructionFamily::Orthotope,
            PrimitiveConstructionCorpusParameterRole::ExplicitRejected,
        )
        .expect("explicit orthotope row");

    assert_eq!(
        threshold.rejection_locality(),
        Some(PrimitiveConstructionRejectionLocality::Admission)
    );
    assert_eq!(
        explicit.rejection_locality(),
        Some(PrimitiveConstructionRejectionLocality::Admission)
    );
    assert_ne!(
        threshold.direct_construction_digest(),
        explicit.direct_construction_digest()
    );
    assert_ne!(threshold.row_digest(), explicit.row_digest());
}

#[test]
fn corpus_replay_siege_explicit_exhaustion_rows_preserve_realization_failure_truth() {
    let report = siege_report("corpus-replay-siege.explicit-exhaustion");
    let simplex = report
        .row_for(
            PrimitiveConstructionFamily::SimplexSolid,
            PrimitiveConstructionCorpusParameterRole::ExplicitExhaustion,
        )
        .expect("explicit exhaustion simplex row");
    let pyramid = report
        .rows()
        .iter()
        .find(|row| row.scenario_id() == "pyramid_semantic_exhaustion")
        .expect("explicit exhaustion pyramid row");

    assert_eq!(
        simplex.scenario_id(),
        "simplex_world_collapsed_explicit_exhaustion"
    );
    assert_eq!(
        simplex.outcome_disposition(),
        PrimitiveConstructionCorpusOutcomeDisposition::Rejected
    );
    assert_eq!(
        simplex.rejection_class(),
        Some(PrimitiveConstructionRejectionClass::ConditioningExhaustion)
    );
    assert_eq!(
        simplex.rejection_locality(),
        Some(PrimitiveConstructionRejectionLocality::Scaffold)
    );
    assert_eq!(
        simplex.blocking_boundary(),
        Some(PrimitiveConstructionBlockingBoundary::KernelIntent)
    );
    assert_eq!(
        simplex.attempted_realization_strategies(),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
    );
    assert_eq!(
        simplex.stability_class(),
        Some(PrimitiveStabilityClass::RejectedBelowConditioningFloor)
    );
    assert!(simplex.birth_digest().is_none());
    assert_eq!(simplex.construction_breadth(), 0);
    assert_eq!(simplex.birth_attachment_breadth(), 0);
    assert_eq!(
        pyramid.family(),
        PrimitiveConstructionFamily::RegularPyramid
    );
    assert_eq!(
        pyramid.parameter_role(),
        PrimitiveConstructionCorpusParameterRole::ExplicitExhaustion
    );
    assert_eq!(
        pyramid.rejection_class(),
        Some(PrimitiveConstructionRejectionClass::ConditioningExhaustion)
    );
    assert_eq!(
        pyramid.rejection_locality(),
        Some(PrimitiveConstructionRejectionLocality::Scaffold)
    );
    assert_eq!(
        pyramid.blocking_boundary(),
        Some(PrimitiveConstructionBlockingBoundary::KernelIntent)
    );
    assert_eq!(
        pyramid.attempted_realization_strategies(),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
    );
    assert_eq!(
        pyramid.stability_class(),
        Some(PrimitiveStabilityClass::RejectedBelowConditioningFloor)
    );
    assert_eq!(
        pyramid.exhaustion_reason(),
        Some(PrimitiveRealizationExhaustionReason::DegenerateSupportNormals)
    );
}
