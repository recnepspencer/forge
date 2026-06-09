use crate::construction::outcome::{
    PrimitiveConstructionRejectionClass, PrimitiveConstructionRejectionLocality,
};
use crate::construction::request::PrimitiveConstructionFamily;
use crate::construction::tests::support::blocking_boundary::PrimitiveConstructionBlockingBoundary;
use crate::construction::tests::support::corpus_replay_row::PrimitiveConstructionCorpusParameterRole;
use crate::construction::tests::support::runtime_truth::PrimitiveConstructionCertificationRuntimeTruth;
use worth_geom::facade::{
    PrimitiveRealizationExhaustionReason, PrimitiveRealizationStrategy, PrimitiveStabilityClass,
};

use crate::construction::tests::support::corpus_replay_view::{
    row_attempted_realization_strategies, row_birth_digest, row_blocking_boundary,
    row_construction_breadth, row_exhaustion_reason, row_realization_strategy, row_rejection_class,
    row_rejection_locality, row_stability_class, siege_report,
};

#[test]
fn corpus_replay_siege_mixes_admitted_and_rejected_rows_across_family_ladder() {
    let report = siege_report("corpus-replay-siege");
    let rejected_ids = report
        .rows()
        .iter()
        .filter(|row| {
            matches!(
                row.runtime_truth(),
                PrimitiveConstructionCertificationRuntimeTruth::Rejected(_)
            )
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
    assert!(report.required_scenario_coverage_verified());
    assert!(report.row_digest_uniqueness_verified());
    assert_eq!(report.accepted_count(), 24);
    assert_eq!(report.rejected_count(), 14);
    assert!(report.authoring_order_lane_coverage_verified());
    assert!(report.authoring_order_parity_verified());
    assert!(report.authoring_order_digest_uniqueness_verified());
    assert_ne!(
        report.report_digest(),
        report
            .row_digest(
                report
                    .row_for(
                        PrimitiveConstructionFamily::SimplexSolid,
                        PrimitiveConstructionCorpusParameterRole::ThresholdAdmitted,
                    )
                    .expect("simplex threshold admitted row"),
            )
            .expect("simplex threshold admitted digest")
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

    assert!(matches!(
        row.runtime_truth(),
        PrimitiveConstructionCertificationRuntimeTruth::Admitted(_)
    ));
    let birth_digest = row_birth_digest(row).expect("admitted rows keep birth truth");
    assert!(row_construction_breadth(row) > 0);
    assert_ne!(
        birth_digest,
        report.row_digest(row).expect("stress shell row digest")
    );
    assert_ne!(birth_digest, row.outcome_digest());
    assert_eq!(
        row_realization_strategy(row),
        Some(PrimitiveRealizationStrategy::DirectWorld)
    );
    assert_eq!(
        row_stability_class(row),
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

    assert!(matches!(
        row.runtime_truth(),
        PrimitiveConstructionCertificationRuntimeTruth::Rejected(_)
    ));
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
    assert!(row_birth_digest(row).is_none());
    assert_eq!(row_construction_breadth(row), 0);
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

    assert_ne!(
        report.row_digest(minimal).expect("minimal prism digest"),
        report.row_digest(stress).expect("stress prism digest")
    );
    assert_ne!(minimal.outcome_digest(), stress.outcome_digest());
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
    assert!(report.authoring_order_parity_verified());
    assert!(report.authoring_order_matrix_stability_verified());
    assert_eq!(
        report.lane_names(),
        vec!["canonical", "reversed", "rejected_first", "role_clustered"]
    );
    assert!(report.authoring_order_digest_uniqueness_verified());
}

#[test]
fn corpus_replay_siege_anchors_public_rows_on_named_canonical_lane_not_vector_position() {
    let report = siege_report("corpus-replay-siege.named-canonical-lane");
    let mut reordered_lanes = report.authoring_order_lanes().to_vec();
    reordered_lanes.rotate_left(2);
    let reordered = crate::construction::tests::support::corpus_replay_view::PrimitiveConstructionCorpusReplaySiegeView::new(
        report.rows().to_vec(),
        report
            .rows()
            .iter()
            .map(|row| {
                report
                    .row_digest(row)
                    .expect("reordered digest")
                    .to_string()
            })
            .collect(),
        reordered_lanes,
    );

    assert!(reordered.authoring_order_lane_coverage_verified());
    assert!(reordered.authoring_order_matrix_stability_verified());
    assert_eq!(reordered.scenario_ids(), report.scenario_ids());
    assert_eq!(
        reordered
            .row_digest(
                reordered
                    .row_for(
                        PrimitiveConstructionFamily::Orthotope,
                        PrimitiveConstructionCorpusParameterRole::GenericAdmitted,
                    )
                    .expect("generic orthotope row"),
            )
            .expect("reordered orthotope digest"),
        report
            .row_digest(
                report
                    .row_for(
                        PrimitiveConstructionFamily::Orthotope,
                        PrimitiveConstructionCorpusParameterRole::GenericAdmitted,
                    )
                    .expect("generic orthotope row"),
            )
            .expect("canonical orthotope digest")
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

    assert!(row_construction_breadth(minimal) < row_construction_breadth(threshold));
    assert!(row_construction_breadth(threshold) < row_construction_breadth(generic));
    assert!(row_construction_breadth(generic) < row_construction_breadth(stress));
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
        row_rejection_locality(threshold),
        Some(PrimitiveConstructionRejectionLocality::Admission)
    );
    assert_eq!(
        row_rejection_locality(explicit),
        Some(PrimitiveConstructionRejectionLocality::Scaffold)
    );
    assert_eq!(
        row_blocking_boundary(threshold),
        Some(PrimitiveConstructionBlockingBoundary::PrimitiveClassAdmission)
    );
    assert_eq!(
        row_blocking_boundary(explicit),
        Some(PrimitiveConstructionBlockingBoundary::KernelIntent)
    );
    assert_ne!(threshold.outcome_digest(), explicit.outcome_digest());
    assert_ne!(
        report
            .row_digest(threshold)
            .expect("threshold orthotope digest"),
        report
            .row_digest(explicit)
            .expect("explicit orthotope digest")
    );
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
    assert!(report
        .scenario_ids()
        .into_iter()
        .any(|scenario_id| scenario_id == "pyramid_semantic_exhaustion"));
    let pyramid = report
        .rows()
        .iter()
        .find(|row| row.scenario_id() == "pyramid_semantic_exhaustion")
        .expect("explicit exhaustion pyramid row");

    assert_eq!(
        simplex.scenario_id(),
        "simplex_world_collapsed_explicit_exhaustion"
    );
    assert!(matches!(
        simplex.runtime_truth(),
        PrimitiveConstructionCertificationRuntimeTruth::Rejected(_)
    ));
    assert_eq!(
        row_rejection_class(simplex),
        Some(PrimitiveConstructionRejectionClass::ConditioningExhaustion)
    );
    assert_eq!(
        row_rejection_locality(simplex),
        Some(PrimitiveConstructionRejectionLocality::Scaffold)
    );
    assert_eq!(
        row_blocking_boundary(simplex),
        Some(PrimitiveConstructionBlockingBoundary::KernelIntent)
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
        Some(PrimitiveStabilityClass::RejectedBelowConditioningFloor)
    );
    assert!(row_birth_digest(simplex).is_none());
    assert_eq!(row_construction_breadth(simplex), 0);
    assert_eq!(
        pyramid.family(),
        PrimitiveConstructionFamily::RegularPyramid
    );
    assert_eq!(
        pyramid.parameter_role(),
        PrimitiveConstructionCorpusParameterRole::ExplicitExhaustion
    );
    assert_eq!(
        row_rejection_class(pyramid),
        Some(PrimitiveConstructionRejectionClass::ConditioningExhaustion)
    );
    assert_eq!(
        row_rejection_locality(pyramid),
        Some(PrimitiveConstructionRejectionLocality::Scaffold)
    );
    assert_eq!(
        row_blocking_boundary(pyramid),
        Some(PrimitiveConstructionBlockingBoundary::KernelIntent)
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
        Some(PrimitiveStabilityClass::RejectedBelowConditioningFloor)
    );
    assert_eq!(
        row_exhaustion_reason(pyramid),
        Some(PrimitiveRealizationExhaustionReason::DegenerateSupportNormals)
    );
}
