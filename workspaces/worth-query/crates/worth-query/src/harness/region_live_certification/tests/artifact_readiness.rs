use super::super::row_catalog::{CANONICAL_ROW_SPECS, REJECTION_ROW_SPECS};
use super::super::MilestoneFivePointOneLiveCertificationAdapter;
use crate::harness::certification::{milestone_five_point_one_requirements, unmet_required_rows};
use crate::harness::live_certification::{LiveCertificationMatrix, LivePerturbationClass};

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
            .locality_irrelevant_broad_control_count()
            > 0
    );
    assert!(artifact.counter_snapshot.locality_replay_change_count() > 0);
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
            .locality_widening_admission_count()
            > 0
    );
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
            .stream_window_width_budget_cross_count()
            > 0
    );
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
        artifact.counter_snapshot.locality_replay_divergence_count(),
        0
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
    assert!(artifact.matrix.rows.iter().any(
        |row| row.perturbation_class == LivePerturbationClass::LocalityWideningAdmissionParity
    ));
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
            == LivePerturbationClass::ForbiddenStreamWindowOverflowSuccessRejection));
    assert!(artifact
        .matrix
        .rejection_rows
        .iter()
        .any(|row| row.perturbation_class
            == LivePerturbationClass::RawStreamMemberForbiddenRejection));
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
