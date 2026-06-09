use crate::construction::tests::support::compound_corpus::{
    expected_exhaustion_scenario_ids, expected_grazing_scenario_ids, expected_motion_scenario_ids,
    expected_required_scenario_ids, sorted_ids,
};
use crate::construction::tests::support::compound_lane_support::{
    compound_authoring_order_parity_verified, compound_lane_digest_uniqueness_verified,
    compound_lane_names, compound_normalized_matrix_digest, compound_report_digest,
    compound_required_lane_coverage_verified, compound_required_scenario_coverage_verified,
    compound_row_for, compound_scenario_ids, compound_scenario_stable_across_orders,
    compound_stable_scenario_coverage_verified,
};
use crate::construction::tests::support::compound_parity_support::{
    build_exhaustion_witness_parity_rows_from_siege, build_grazing_boundary_rows_from_siege,
    build_motion_parity_rows_from_siege,
};
use crate::construction::tests::support::compound_parity_view::{
    prepare_compound_parity_view, verify_compound_parity_view,
    PrimitiveConstructionCompoundParityVerificationMismatch,
};
use crate::construction::tests::support::compound_row_support::{
    exhaustion_reason, grazing_kind, motion_kind, realization_strategy, rejection_class,
    rejection_locality, row_digest, stability_class,
};
use crate::construction::tests::support::compound_runtime::{
    prepare_primitive_construction_compound_adversarial_lanes, PrimitiveConstructionCompoundRow,
};
use worth_geom::facade::PrimitiveRealizationExhaustionWitnessKind;

#[test]
fn compound_ordering_parity_report_requires_the_full_spec_order_matrix() {
    let report =
        prepare_primitive_construction_compound_adversarial_lanes().expect("compound siege report");

    assert!(compound_authoring_order_parity_verified(&report));
    assert!(compound_required_lane_coverage_verified(&report));
    assert!(compound_required_scenario_coverage_verified(&report));
    assert!(compound_lane_digest_uniqueness_verified(&report));
    assert!(compound_stable_scenario_coverage_verified(&report));
    assert_eq!(
        sorted_ids(compound_lane_names(&report)),
        vec![
            "canonical".to_string(),
            "escalation_clustered".to_string(),
            "family_clustered".to_string(),
            "rejected_first".to_string(),
            "reversed".to_string(),
        ]
    );
    assert_eq!(
        sorted_ids(compound_scenario_ids(&report)),
        expected_required_scenario_ids()
    );
    assert_scenario_stable_across_orders(&report, "sheet_patch_reorient_grazing_workplane");
    assert_scenario_stable_across_orders(&report, "simplex_world_collapsed_explicit_exhaustion");
    assert_scenario_stable_across_orders(&report, "pyramid_threshold_rejected_neighbor");
    assert_ne!(
        compound_normalized_matrix_digest(&report),
        compound_report_digest(&report)
    );
    assert_ne!(
        compound_report_digest(&report),
        row_digest(
            compound_row_for(&report, "orthotope_direct_stable").expect("orthotope scenario row"),
        )
    );
}

#[test]
fn compound_exhaustion_witness_parity_report_binds_kernel_rows_to_lower_layer_witnesses() {
    let report = prepare_compound_parity_view().expect("compound parity report");

    assert!(report.exhaustion_parity_verified());
    assert_eq!(
        sorted_ids(
            report
                .exhaustion_rows()
                .iter()
                .map(|row| row.scenario_id().to_string())
        ),
        expected_exhaustion_scenario_ids()
    );
    assert_eq!(
        report
            .exhaustion_row_for("pyramid_semantic_exhaustion")
            .expect("pyramid exhaustion row")
            .witness_kind(),
        PrimitiveRealizationExhaustionWitnessKind::ZeroRadiusPyramidSupportCollapse
    );
    assert_eq!(
        report
            .exhaustion_row_for("simplex_world_collapsed_explicit_exhaustion")
            .expect("simplex exhaustion row")
            .witness_kind(),
        PrimitiveRealizationExhaustionWitnessKind::AltitudeSqueezedSimplexSupportCollapse
    );
    assert!(report.exhaustion_siege_row_digest_uniqueness_verified());
    assert!(report.exhaustion_witness_row_digest_uniqueness_verified());
}

#[test]
fn compound_parity_report_bundles_ordering_motion_grazing_and_exhaustion_truth() {
    let report = prepare_compound_parity_view().expect("compound parity report");

    assert_eq!(
        compound_report_digest(report.siege()),
        compound_report_digest(report.siege())
    );
    assert!(report.motion_expected_inventory_coverage_verified());
    assert!(report.grazing_expected_inventory_coverage_verified());
    assert!(report.exhaustion_expected_inventory_coverage_verified());
    assert_eq!(
        sorted_ids(report.motion_scenario_ids()),
        expected_motion_scenario_ids()
    );
    assert_eq!(
        sorted_ids(report.grazing_scenario_ids()),
        expected_grazing_scenario_ids()
    );
    assert_eq!(
        sorted_ids(report.exhaustion_scenario_ids()),
        expected_exhaustion_scenario_ids()
    );
    assert_ne!(
        report.motion_report_digest(),
        report.grazing_report_digest()
    );
    assert_ne!(
        report.grazing_report_digest(),
        report.exhaustion_report_digest()
    );
}

#[test]
fn compound_ordering_parity_report_anchors_stability_on_named_canonical_lane_not_vector_position() {
    let report =
        prepare_primitive_construction_compound_adversarial_lanes().expect("compound siege report");
    let mut reordered_lanes = report.clone();
    reordered_lanes.rotate_left(2);
    let reordered = reordered_lanes;

    assert!(compound_authoring_order_parity_verified(&reordered));
    assert!(compound_required_lane_coverage_verified(&reordered));
    assert!(compound_required_scenario_coverage_verified(&reordered));
    assert_eq!(
        compound_normalized_matrix_digest(&reordered),
        compound_normalized_matrix_digest(&report)
    );
    assert_eq!(
        sorted_ids(compound_scenario_ids(&reordered)),
        sorted_ids(compound_scenario_ids(&report))
    );
    assert_scenario_stable_across_orders(&reordered, "pyramid_semantic_exhaustion");
}

#[test]
fn compound_parity_verification_failure_preserves_exact_mismatch_and_full_drift_context() {
    use crate::construction::tests::support::compound_runtime::PrimitiveConstructionCompoundMotionParityRow;

    let siege =
        prepare_primitive_construction_compound_adversarial_lanes().expect("compound siege report");
    let canonical_motion = build_motion_parity_rows_from_siege(&siege).expect("motion parity rows");
    let drifted_rows = canonical_motion
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
    let failure = verify_compound_parity_view(
        siege.clone(),
        drifted_rows,
        build_grazing_boundary_rows_from_siege(&siege).expect("grazing parity rows"),
        build_exhaustion_witness_parity_rows_from_siege(&siege).expect("exhaustion parity rows"),
    )
    .expect_err("drifted bundle should reject");

    assert_eq!(
        failure.mismatches(),
        &[PrimitiveConstructionCompoundParityVerificationMismatch::MotionProjectionDrift]
    );
    assert_eq!(
        compound_report_digest(failure.siege()),
        compound_report_digest(failure.siege())
    );
    assert!(compound_required_lane_coverage_verified(failure.siege()));
    assert!(compound_required_scenario_coverage_verified(
        failure.siege()
    ));
    assert_eq!(
        sorted_ids(failure.motion_scenario_ids()),
        sorted_ids(
            canonical_motion
                .iter()
                .map(|row| row.scenario_id().to_string())
        )
    );
    assert!(failure.grazing_expected_inventory_coverage_verified());
    assert!(failure.exhaustion_expected_inventory_coverage_verified());
}

fn assert_scenario_stable_across_orders(
    report: &crate::construction::tests::support::compound_runtime::PrimitiveConstructionCompoundAdversarialLanes,
    scenario_id: &str,
) {
    let canonical = compound_row_for(report, scenario_id).expect("canonical scenario row");
    let lane_rows = report
        .iter()
        .filter_map(|(_, rows)| rows.iter().find(|row| row.scenario_id() == scenario_id))
        .collect::<Vec<_>>();

    assert_eq!(lane_rows.len(), report.len());
    assert_all_rows_match(canonical, &lane_rows);
    assert!(compound_scenario_stable_across_orders(report, scenario_id));
}

fn assert_all_rows_match(
    canonical: &PrimitiveConstructionCompoundRow,
    lane_rows: &[&PrimitiveConstructionCompoundRow],
) {
    assert!(lane_rows
        .iter()
        .all(|row| row_digest(row) == row_digest(canonical)));
    assert!(lane_rows
        .iter()
        .all(|row| row.topology_class() == canonical.topology_class()));
    assert!(lane_rows
        .iter()
        .all(|row| row.row_class() == canonical.row_class()));
    assert!(lane_rows
        .iter()
        .all(|row| realization_strategy(row) == realization_strategy(canonical)));
    assert!(lane_rows
        .iter()
        .all(|row| stability_class(row) == stability_class(canonical)));
    assert!(lane_rows
        .iter()
        .all(|row| exhaustion_reason(row) == exhaustion_reason(canonical)));
    assert!(lane_rows
        .iter()
        .all(|row| rejection_class(row) == rejection_class(canonical)));
    assert!(lane_rows
        .iter()
        .all(|row| rejection_locality(row) == rejection_locality(canonical)));
    assert!(lane_rows
        .iter()
        .all(|row| motion_kind(row) == motion_kind(canonical)));
    assert!(lane_rows
        .iter()
        .all(|row| grazing_kind(row) == grazing_kind(canonical)));
}
