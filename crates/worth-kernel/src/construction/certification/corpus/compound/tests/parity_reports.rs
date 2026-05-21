use super::super::{
    prepare_primitive_construction_compound_exhaustion_witness_parity_report,
    prepare_primitive_construction_compound_ordering_parity_report,
    prepare_primitive_construction_compound_parity_report,
};
use super::support::{
    compound_workspace, expected_exhaustion_scenario_ids, expected_grazing_scenario_ids,
    expected_motion_scenario_ids, expected_required_scenario_ids, sorted_ids,
};
use std::collections::BTreeSet;
use worth_geom::facade::PrimitiveRealizationExhaustionWitnessKind;

#[test]
fn compound_ordering_parity_report_requires_the_full_spec_order_matrix() {
    let mut workspace = compound_workspace("worth-kernel.compound-ordering-parity");
    let report = prepare_primitive_construction_compound_ordering_parity_report(&mut workspace)
        .expect("ordering parity report");

    assert!(report.parity_verified());
    assert_eq!(
        sorted_ids(
            report
                .authoring_order_rows()
                .iter()
                .map(|row| row.lane_name().to_string()),
        ),
        vec![
            "canonical".to_string(),
            "escalation_clustered".to_string(),
            "family_clustered".to_string(),
            "rejected_first".to_string(),
            "reversed".to_string(),
        ]
    );
    let lane_digests = report
        .authoring_order_rows()
        .iter()
        .map(|row| row.lane_digest().to_string())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(lane_digests.len(), report.authoring_order_rows().len());
    assert_eq!(report.lane_reports().len(), 5);
    assert_eq!(
        sorted_ids(
            report
                .scenario_rows()
                .iter()
                .map(|row| row.scenario_id().to_string()),
        ),
        expected_required_scenario_ids()
    );
    let shell_graze = report
        .scenario_row_for("sheet_patch_reorient_grazing_workplane")
        .expect("shell grazing scenario row");
    assert!(shell_graze.stable_across_orders());
    assert!(shell_graze.grazing_kind_stable());
    let simplex_exhausted = report
        .scenario_row_for("simplex_world_collapsed_explicit_exhaustion")
        .expect("simplex exhausted scenario row");
    assert!(simplex_exhausted.stable_across_orders());
    assert!(simplex_exhausted.exhaustion_reason_stable());
    let rejected_pyramid = report
        .scenario_row_for("pyramid_threshold_rejected_neighbor")
        .expect("rejected pyramid scenario row");
    assert!(rejected_pyramid.rejection_class_stable());
    assert!(rejected_pyramid.rejection_locality_stable());
    assert_ne!(report.normalized_matrix_digest(), report.report_digest());
    assert_ne!(
        report.report_digest(),
        report
            .scenario_row_for("orthotope_direct_stable")
            .expect("orthotope scenario row")
            .row_digest()
    );
}

#[test]
fn compound_exhaustion_witness_parity_report_binds_kernel_rows_to_lower_layer_witnesses() {
    let mut workspace = compound_workspace("worth-kernel.compound-exhaustion-parity");
    let report =
        prepare_primitive_construction_compound_exhaustion_witness_parity_report(&mut workspace)
            .expect("exhaustion parity report");
    let truth = prepare_primitive_construction_compound_parity_report(&mut compound_workspace(
        "worth-kernel.compound-exhaustion-truth",
    ))
    .expect("compound parity report");

    assert!(report.parity_verified());
    assert_eq!(
        sorted_ids(
            report
                .rows()
                .iter()
                .map(|row| row.scenario_id().to_string())
        ),
        expected_exhaustion_scenario_ids(truth.truth())
    );
    assert_eq!(
        report
            .row_for("pyramid_semantic_exhaustion")
            .expect("pyramid exhaustion row")
            .witness_kind(),
        PrimitiveRealizationExhaustionWitnessKind::ZeroRadiusPyramidSupportCollapse
    );
    assert_eq!(
        report
            .row_for("simplex_world_collapsed_explicit_exhaustion")
            .expect("simplex exhaustion row")
            .witness_kind(),
        PrimitiveRealizationExhaustionWitnessKind::AltitudeSqueezedSimplexSupportCollapse
    );
    assert_eq!(
        report
            .rows()
            .iter()
            .map(|row| row.siege_row_digest())
            .collect::<BTreeSet<_>>()
            .len(),
        report.rows().len()
    );
    assert_eq!(
        report
            .rows()
            .iter()
            .map(|row| row.witness_row_digest())
            .collect::<BTreeSet<_>>()
            .len(),
        report.rows().len()
    );
}

#[test]
fn compound_parity_report_bundles_ordering_motion_grazing_and_exhaustion_truth() {
    let mut workspace = compound_workspace("worth-kernel.compound-parity");
    let report = prepare_primitive_construction_compound_parity_report(&mut workspace)
        .expect("compound parity report");
    let truth = report.truth().clone();

    assert_eq!(
        report.truth().siege().report_digest(),
        report.siege().report_digest()
    );
    assert_eq!(report.motion().rows().len(), 3);
    assert_eq!(report.grazing().rows().len(), 2);
    assert_eq!(report.exhaustion().rows().len(), 2);
    assert_eq!(
        sorted_ids(
            report
                .motion()
                .rows()
                .iter()
                .map(|row| row.scenario_id().to_string()),
        ),
        expected_motion_scenario_ids(&truth)
    );
    assert_eq!(
        sorted_ids(
            report
                .grazing()
                .rows()
                .iter()
                .map(|row| row.scenario_id().to_string()),
        ),
        expected_grazing_scenario_ids(&truth)
    );
    assert_eq!(
        sorted_ids(
            report
                .exhaustion()
                .rows()
                .iter()
                .map(|row| row.scenario_id().to_string()),
        ),
        expected_exhaustion_scenario_ids(&truth)
    );
    assert_ne!(
        report.motion().report_digest(),
        report.grazing().report_digest()
    );
    assert_ne!(
        report.grazing().report_digest(),
        report.exhaustion().report_digest()
    );
}

#[test]
fn compound_ordering_parity_report_anchors_stability_on_named_canonical_lane_not_vector_position() {
    let mut workspace = compound_workspace("worth-kernel.compound-ordering-canonical-lane");
    let report = prepare_primitive_construction_compound_ordering_parity_report(&mut workspace)
        .expect("ordering parity report");
    let mut reordered_lanes = report.lane_reports().to_vec();
    reordered_lanes.rotate_left(2);
    let reordered =
        super::super::PrimitiveConstructionCompoundOrderingParityReport::new(reordered_lanes);

    assert!(reordered.parity_verified());
    assert_eq!(
        reordered.normalized_matrix_digest(),
        report.normalized_matrix_digest()
    );
    assert_eq!(
        sorted_ids(
            reordered
                .scenario_rows()
                .iter()
                .map(|row| row.scenario_id().to_string()),
        ),
        sorted_ids(
            report
                .scenario_rows()
                .iter()
                .map(|row| row.scenario_id().to_string()),
        )
    );
    assert!(reordered
        .scenario_row_for("pyramid_semantic_exhaustion")
        .expect("pyramid scenario row")
        .stable_across_orders());
}

#[test]
fn compound_parity_verification_failure_preserves_exact_mismatch_and_full_drift_context() {
    use super::super::builder::{
        build_exhaustion_witness_parity_report_from_siege,
        build_grazing_boundary_report_from_siege, build_motion_parity_report_from_siege,
        prepare_primitive_construction_compound_adversarial_siege_report,
        PrimitiveConstructionCompoundAdversarialSiegeError,
    };
    use super::super::ordering_report::PrimitiveConstructionCompoundOrderingParityReport;
    use super::super::parity::{
        verify_bundle, PrimitiveConstructionCompoundParityReportBundle,
        PrimitiveConstructionCompoundParityVerificationMismatch,
    };
    use super::super::rows::PrimitiveConstructionCompoundMotionParityRow;
    use super::super::PrimitiveConstructionCompoundMotionParityReport;

    let mut workspace = compound_workspace("worth-kernel.compound-parity-verification-failure");
    let siege = prepare_primitive_construction_compound_adversarial_siege_report(&mut workspace)
        .expect("compound siege report");
    let ordering =
        PrimitiveConstructionCompoundOrderingParityReport::new(siege.lane_reports().to_vec());
    let canonical_motion = build_motion_parity_report_from_siege(&siege).expect("motion report");
    let drifted_rows = canonical_motion
        .rows()
        .iter()
        .enumerate()
        .map(|(index, row)| {
            let digest = if index == 0 {
                format!("{}-drift", row.motion_digest())
            } else {
                row.motion_digest().to_string()
            };
            PrimitiveConstructionCompoundMotionParityRow::new(
                row.scenario_id().to_string(),
                row.motion_kind(),
                digest,
            )
        })
        .collect::<Vec<_>>();
    let drifted_motion =
        PrimitiveConstructionCompoundMotionParityReport::new(drifted_rows, &ordering);
    let bundle = PrimitiveConstructionCompoundParityReportBundle::new(
        siege.clone(),
        ordering,
        drifted_motion,
        build_grazing_boundary_report_from_siege(&siege).expect("grazing report"),
        build_exhaustion_witness_parity_report_from_siege(&siege).expect("exhaustion report"),
    );

    let error = verify_bundle(bundle).expect_err("drifted bundle should reject");
    let failure = match error {
        PrimitiveConstructionCompoundAdversarialSiegeError::Verification(failure) => failure,
        other => panic!("expected verification error, got {other:?}"),
    };

    assert_eq!(
        failure.mismatches(),
        &[PrimitiveConstructionCompoundParityVerificationMismatch::MotionProjectionDrift]
    );
    assert_eq!(
        failure.truth().siege().report_digest(),
        failure.siege().report_digest()
    );
    assert_eq!(failure.ordering().lane_reports().len(), 5);
    assert_eq!(failure.motion().rows().len(), 3);
    assert_eq!(failure.grazing().rows().len(), 2);
    assert_eq!(failure.exhaustion().rows().len(), 2);
}
