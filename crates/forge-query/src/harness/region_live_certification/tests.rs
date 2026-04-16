use super::row_catalog::{DigestRelation, CANONICAL_ROW_SPECS, REJECTION_ROW_SPECS};
use super::MilestoneFivePointOneLiveCertificationAdapter;
use crate::harness::certification::{milestone_five_point_one_requirements, unmet_required_rows};
use crate::harness::live_certification::{
    LiveCertificationMatrix, LiveCertificationRow, LiveFailureClass, LivePerturbationClass,
    LiveRejectionRow,
};

#[test]
fn region_live_adapter_emits_named_matrix() {
    let matrix = MilestoneFivePointOneLiveCertificationAdapter::
        region_scoped_live_narrowing_and_stream_contract_test();

    assert_eq!(
        matrix.suite_name,
        "Region-Scoped Live Narrowing And Stream Contract Test"
    );
    assert_named_rows_present(
        &matrix,
        CANONICAL_ROW_SPECS.iter().map(|spec| spec.row_name),
        REJECTION_ROW_SPECS.iter().map(|spec| spec.row_name),
    );
}

#[test]
fn region_live_matrix_meets_required_rows() {
    let matrix = MilestoneFivePointOneLiveCertificationAdapter::
        region_scoped_live_narrowing_and_stream_contract_test();
    let requirements = milestone_five_point_one_requirements();
    let missing = unmet_required_rows(
        &matrix,
        requirements.required_canonical_rows,
        requirements.required_rejection_rows,
    );

    assert!(
        missing.is_empty(),
        "missing milestone 5.1 rows: {missing:?}"
    );
}

#[test]
fn region_live_artifact_is_offline_ready() {
    let artifact = MilestoneFivePointOneLiveCertificationAdapter::
        region_scoped_live_narrowing_and_stream_contract_certification_artifact();

    assert_eq!(
        artifact.suite_name,
        "Region-Scoped Live Narrowing And Stream Contract Test"
    );
    assert!(!artifact.certification_bundle_digest.is_empty());
    assert!(!artifact.coverage_matrix_digest.is_empty());
    assert!(artifact.counter_snapshot.locality_region_match_count() > 0);
    assert!(
        artifact
            .counter_snapshot
            .locality_off_region_suppression_count()
            > 0
    );
    assert!(
        artifact
            .counter_snapshot
            .locality_breadth_budget_cross_count()
            > 0
    );
    assert!(artifact.counter_snapshot.locality_widening_denial_count() > 0);
    assert!(
        artifact
            .counter_snapshot
            .locality_widening_budget_cross_count()
            > 0
    );
    assert!(
        artifact
            .counter_snapshot
            .locality_bridge_slice_incompatibility_count()
            > 0
    );
    assert!(artifact.counter_snapshot.stream_contract_admission_count() > 0);
    assert!(artifact.counter_snapshot.stream_contract_denial_count() > 0);
    assert!(artifact.counter_snapshot.stream_lowered_delivery_count() > 0);
    assert!(artifact.counter_snapshot.stream_lowered_delivery_width() > 0);
    assert!(
        artifact
            .counter_snapshot
            .stream_member_width_budget_cross_count()
            > 0
    );
    assert!(
        artifact
            .counter_snapshot
            .locality_work_avoided_by_region_narrowing_count()
            > 0
    );
    assert!(
        artifact
            .counter_snapshot
            .locality_work_avoided_vs_broad_control_count()
            > 0
    );
    assert!(
        artifact
            .counter_snapshot
            .locality_unsupported_predicate_rejection_count()
            > 0
    );
    assert_eq!(
        artifact
            .counter_snapshot
            .locality_executor_rediscovery_count(),
        0
    );
    assert!(artifact
        .matrix
        .rows
        .iter()
        .all(|row| row.has_required_outputs()));
    assert!(artifact
        .matrix
        .rejection_rows
        .iter()
        .all(|row| row.has_required_outputs()));
    assert!(artifact
        .matrix
        .rows
        .iter()
        .any(|row| row.perturbation_class == LivePerturbationClass::RegionScopedConvergenceParity));
    assert!(artifact
        .matrix
        .rows
        .iter()
        .any(|row| row.perturbation_class == LivePerturbationClass::CollectionPartitionParity));
    assert!(artifact
        .matrix
        .rows
        .iter()
        .any(|row| row.perturbation_class
            == LivePerturbationClass::BoundedMaterializationRegionParity));
    assert!(artifact
        .matrix
        .rows
        .iter()
        .any(|row| row.perturbation_class == LivePerturbationClass::StreamContractParity));
    assert!(artifact
        .matrix
        .rows
        .iter()
        .any(|row| row.perturbation_class == LivePerturbationClass::CdcStreamLoweredParity));
    assert!(artifact.matrix.rows.iter().any(
        |row| row.perturbation_class == LivePerturbationClass::LocalityBreadthBudgetEnforcement
    ));
    assert!(artifact
        .matrix
        .rows
        .iter()
        .any(|row| row.perturbation_class
            == LivePerturbationClass::StreamMemberWidthBudgetEnforcement));
    assert!(artifact
        .matrix
        .rejection_rows
        .iter()
        .any(|row| row.perturbation_class
            == LivePerturbationClass::ForbiddenLocalityWideningRejection));
    assert!(artifact
        .matrix
        .rejection_rows
        .iter()
        .any(|row| row.perturbation_class
            == LivePerturbationClass::ForbiddenBroadSuccessLaneRejection));
    assert!(artifact
        .matrix
        .rejection_rows
        .iter()
        .any(|row| row.perturbation_class
            == LivePerturbationClass::ForbiddenStreamWidthOverflowSuccessRejection));
    assert!(artifact
        .matrix
        .rejection_rows
        .iter()
        .any(|row| row.perturbation_class
            == LivePerturbationClass::RawStreamMemberForbiddenRejection));
}

#[test]
fn region_live_canonical_rows_preserve_counter_parity() {
    let matrix = MilestoneFivePointOneLiveCertificationAdapter::
        region_scoped_live_narrowing_and_stream_contract_test();

    for row in &matrix.rows {
        if row.row_name == "broad-vs-region-narrowing-control" {
            continue;
        }
        assert_eq!(
            row.control_lane.counter_snapshot, row.hostile_lane.counter_snapshot,
            "canonical row {} drifted between control and hostile counters",
            row.row_name
        );
        assert_eq!(
            row.control_lane.counter_snapshot, row.parity_lane.counter_snapshot,
            "canonical row {} drifted between control and parity counters",
            row.row_name
        );
    }

    let broad_vs_region = canonical_row(&matrix, "broad-vs-region-narrowing-control");
    assert_eq!(
        broad_vs_region
            .control_lane
            .counter_snapshot
            .locality_region_match_count(),
        0
    );
    assert_eq!(
        broad_vs_region
            .control_lane
            .counter_snapshot
            .locality_work_avoided_by_region_narrowing_count(),
        0
    );
    assert_eq!(
        broad_vs_region
            .hostile_lane
            .counter_snapshot
            .locality_region_match_count(),
        1
    );
    assert_eq!(
        broad_vs_region
            .hostile_lane
            .counter_snapshot
            .locality_work_avoided_by_region_narrowing_count(),
        1
    );
    assert_eq!(
        broad_vs_region.hostile_lane.counter_snapshot,
        broad_vs_region.parity_lane.counter_snapshot,
        "broad-vs-region row should preserve exact locality counters between hostile and parity lanes"
    );
}

#[test]
fn region_live_rejection_rows_preserve_control_and_taxonomy() {
    let matrix = MilestoneFivePointOneLiveCertificationAdapter::
        region_scoped_live_narrowing_and_stream_contract_test();

    for row in &matrix.rejection_rows {
        assert_eq!(
            row.control_lane.counter_snapshot, row.parity_lane.counter_snapshot,
            "rejection row {} drifted between control and parity counters",
            row.row_name
        );
    }

    for spec in REJECTION_ROW_SPECS {
        assert_rejection_failure_class(&matrix, spec.row_name, spec.failure_class);
    }
}

#[test]
fn region_live_concrete_rows_enforce_exact_counter_shapes() {
    let matrix = MilestoneFivePointOneLiveCertificationAdapter::
        region_scoped_live_narrowing_and_stream_contract_test();

    for expectation in CANONICAL_ROW_SPECS {
        assert_canonical_expectation(&matrix, expectation);
    }

    for expectation in REJECTION_ROW_SPECS {
        assert_rejection_expectation(&matrix, expectation);
    }
}

fn canonical_row<'a>(
    matrix: &'a LiveCertificationMatrix,
    row_name: &str,
) -> &'a LiveCertificationRow {
    matrix
        .rows
        .iter()
        .find(|row| row.row_name == row_name)
        .unwrap_or_else(|| panic!("missing canonical row {row_name}"))
}

fn rejection_row<'a>(matrix: &'a LiveCertificationMatrix, row_name: &str) -> &'a LiveRejectionRow {
    matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == row_name)
        .unwrap_or_else(|| panic!("missing rejection row {row_name}"))
}

fn assert_rejection_failure_class(
    matrix: &LiveCertificationMatrix,
    row_name: &str,
    expected_failure_class: LiveFailureClass,
) {
    let row = rejection_row(matrix, row_name);
    assert_eq!(
        row.hostile_lane.failure_class, expected_failure_class,
        "rejection row {row_name} drifted to the wrong failure class"
    );
    assert!(
        !row.hostile_lane.failure_digest.is_empty(),
        "rejection row {row_name} should carry a typed failure digest"
    );
}

fn assert_canonical_expectation(
    matrix: &LiveCertificationMatrix,
    expectation: &super::row_catalog::CanonicalRowSpec,
) {
    let row = canonical_row(matrix, expectation.row_name);
    assert_eq!(row.control_lane.family, expectation.family);
    assert_eq!(row.control_lane.outcome_kind, expectation.outcome_kind);
    match expectation.digest_relation {
        DigestRelation::MatchesDeliveryDigest => {
            assert_eq!(
                row.control_lane.outcome_digest, row.control_lane.delivery_digest,
                "canonical row {} should keep outcome digest aligned with delivery digest",
                expectation.row_name
            );
        }
        DigestRelation::DiffersFromDeliveryDigest => {
            assert_ne!(
                row.control_lane.outcome_digest, row.control_lane.delivery_digest,
                "canonical row {} should keep stream contract digest distinct from delivery digest",
                expectation.row_name
            );
        }
    }

    match expectation.row_name {
        "region-live-convergence" => {
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .locality_region_match_count(),
                1
            );
            assert_eq!(row.control_lane.counter_snapshot.live_patch_count(), 1);
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .live_patch_field_delta_count(),
                1
            );
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .locality_work_avoided_by_region_narrowing_count(),
                1
            );
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .stream_contract_admission_count(),
                0
            );
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .locality_widening_denial_count(),
                0
            );
        }
        "off-region-suppression-parity" => {
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .locality_off_region_suppression_count(),
                1
            );
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .live_suppressed_update_count(),
                1
            );
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .locality_work_avoided_vs_broad_control_count(),
                1
            );
            assert_eq!(row.control_lane.counter_snapshot.live_patch_count(), 0);
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .stream_contract_admission_count(),
                0
            );
        }
        "collection-partition-hit" => {
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .locality_partition_match_count(),
                1
            );
            assert_eq!(row.control_lane.counter_snapshot.live_patch_count(), 1);
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .live_patch_field_delta_count(),
                1
            );
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .stream_contract_admission_count(),
                0
            );
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .locality_widening_denial_count(),
                0
            );
        }
        "bounded-materialization-region-hit" => {
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .locality_region_match_count(),
                1
            );
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .live_materialization_patch_count(),
                1
            );
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .live_work_avoided_by_scope_proof_count(),
                1
            );
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .stream_contract_admission_count(),
                0
            );
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .locality_widening_denial_count(),
                0
            );
        }
        "broad-vs-region-narrowing-control" => {
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .locality_region_match_count(),
                0
            );
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .locality_work_avoided_by_region_narrowing_count(),
                0
            );
            assert_eq!(
                row.hostile_lane
                    .counter_snapshot
                    .locality_region_match_count(),
                1
            );
            assert_eq!(
                row.hostile_lane
                    .counter_snapshot
                    .locality_work_avoided_by_region_narrowing_count(),
                1
            );
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .stream_contract_admission_count(),
                0
            );
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .locality_widening_denial_count(),
                0
            );
        }
        "cdc-stream-lowered-parity" => {
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .stream_contract_admission_count(),
                1
            );
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .stream_lowered_delivery_count(),
                1
            );
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .stream_lowered_delivery_width(),
                2
            );
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .locality_widening_denial_count(),
                0
            );
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .stream_member_width_budget_cross_count(),
                0
            );
        }
        "stream-contract-delivery-parity" => {
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .stream_contract_admission_count(),
                1
            );
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .stream_lowered_delivery_count(),
                1
            );
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .stream_lowered_delivery_width(),
                1
            );
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .locality_widening_denial_count(),
                0
            );
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .stream_member_width_budget_cross_count(),
                0
            );
        }
        "locality-breadth-budget-enforcement" => {
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .locality_region_match_count(),
                1
            );
            assert_eq!(row.control_lane.counter_snapshot.live_patch_count(), 1);
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .locality_breadth_budget_cross_count(),
                0
            );
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .stream_contract_admission_count(),
                0
            );
        }
        "stream-member-width-budget-enforcement" => {
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .stream_contract_admission_count(),
                1
            );
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .stream_lowered_delivery_count(),
                1
            );
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .stream_lowered_delivery_width(),
                2
            );
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .stream_member_width_budget_cross_count(),
                0
            );
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .locality_widening_denial_count(),
                0
            );
        }
        "locality-work-avoided-counter-parity" => {
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .locality_off_region_suppression_count(),
                1
            );
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .locality_work_avoided_by_region_narrowing_count(),
                1
            );
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .locality_work_avoided_vs_broad_control_count(),
                1
            );
            assert_eq!(
                row.control_lane
                    .counter_snapshot
                    .stream_contract_admission_count(),
                0
            );
        }
        _ => panic!(
            "missing canonical row expectation handler for {}",
            expectation.row_name
        ),
    }
}

fn assert_rejection_expectation(
    matrix: &LiveCertificationMatrix,
    expectation: &super::row_catalog::RejectionRowSpec,
) {
    let row = rejection_row(matrix, expectation.row_name);
    assert_eq!(row.hostile_lane.failure_class, expectation.failure_class);
    assert_eq!(row.control_lane.family, expectation.control_family);
    assert!(
        row.hostile_lane
            .failure_digest
            .contains(expectation.failure_digest_fragment),
        "rejection row {} should carry failure digest fragment {}",
        expectation.row_name,
        expectation.failure_digest_fragment
    );

    match expectation.row_name {
        "collection-cross-partition-denied" | "forbidden-locality-widening" => {
            assert_eq!(
                row.hostile_lane
                    .counter_snapshot
                    .locality_widening_denial_count(),
                1
            );
            assert_eq!(
                row.hostile_lane
                    .counter_snapshot
                    .locality_widening_budget_cross_count(),
                1
            );
        }
        "unsupported-locality-family" => {
            assert_eq!(
                row.hostile_lane
                    .counter_snapshot
                    .locality_unsupported_family_rejection_count(),
                1
            );
            assert_eq!(
                row.hostile_lane
                    .counter_snapshot
                    .live_invalid_promotion_rejection_count(),
                0
            );
        }
        "unsupported-locality-predicate" => {
            assert_eq!(
                row.hostile_lane
                    .counter_snapshot
                    .locality_unsupported_predicate_rejection_count(),
                1
            );
        }
        "unsupported-stream-consumer-contract" => {
            assert_eq!(
                row.hostile_lane
                    .counter_snapshot
                    .stream_contract_denial_count(),
                1
            );
            assert_eq!(
                row.hostile_lane
                    .counter_snapshot
                    .stream_member_width_budget_cross_count(),
                0
            );
        }
        "raw-partition-event-leakage-forbidden" => {
            assert_eq!(
                row.hostile_lane
                    .counter_snapshot
                    .locality_widening_denial_count(),
                1
            );
            assert_eq!(
                row.hostile_lane
                    .counter_snapshot
                    .locality_widening_budget_cross_count(),
                1
            );
        }
        "raw-stream-member-forbidden" | "raw-stream-member-leakage-forbidden" => {
            assert_eq!(
                row.hostile_lane
                    .counter_snapshot
                    .stream_contract_denial_count(),
                1
            );
            assert_eq!(
                row.hostile_lane
                    .counter_snapshot
                    .stream_member_width_budget_cross_count(),
                0
            );
        }
        "forbidden-broad-success-lane" => {
            assert_eq!(
                row.hostile_lane
                    .counter_snapshot
                    .locality_breadth_budget_cross_count(),
                1
            );
        }
        "forbidden-stream-width-overflow-success" => {
            assert_eq!(
                row.hostile_lane
                    .counter_snapshot
                    .stream_contract_denial_count(),
                1
            );
            assert_eq!(
                row.hostile_lane
                    .counter_snapshot
                    .stream_member_width_budget_cross_count(),
                1
            );
        }
        "bridge-slice-incompatibility-denied" => {
            assert_eq!(
                row.hostile_lane
                    .counter_snapshot
                    .locality_bridge_slice_incompatibility_count(),
                1
            );
        }
        _ => panic!(
            "missing rejection row expectation handler for {}",
            expectation.row_name
        ),
    }
}

fn assert_named_rows_present(
    matrix: &LiveCertificationMatrix,
    canonical_row_names: impl IntoIterator<Item = &'static str>,
    rejection_row_names: impl IntoIterator<Item = &'static str>,
) {
    for row_name in canonical_row_names {
        assert!(
            matrix.rows.iter().any(|row| row.row_name == row_name),
            "missing canonical row {row_name}"
        );
    }
    for row_name in rejection_row_names {
        assert!(
            matrix
                .rejection_rows
                .iter()
                .any(|row| row.row_name == row_name),
            "missing rejection row {row_name}"
        );
    }
}
