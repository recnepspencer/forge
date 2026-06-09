use crate::harness::certification::{
    contains_row, milestone_nine_five_requirements, unmet_required_assertion_classes,
    unmet_required_rows, HostileExpectation, ParityAnchor, RequiredAssertionClass,
};
use crate::saved_query::SavedQueryTemporalAsyncSurfacePosture;

use super::bundle::MilestoneNineFiveHostileMatrixAdapter;

#[test]
fn milestone_nine_five_adapter_emits_named_matrix() {
    let artifact =
        MilestoneNineFiveHostileMatrixAdapter::debt_close_hostile_certification_matrix_artifact();

    assert_eq!(
        artifact.suite_name,
        "Milestone 9.5 Debt-Close Hostile Certification Matrix"
    );
    assert!(!artifact.certification_bundle_digest.is_empty());
    assert!(!artifact.coverage_matrix_digest.is_empty());
}

#[test]
fn milestone_nine_five_matrix_meets_required_rows() {
    let matrix =
        MilestoneNineFiveHostileMatrixAdapter::debt_close_hostile_certification_matrix_test();
    let missing = unmet_required_rows(
        &matrix,
        milestone_nine_five_requirements().required_canonical_rows,
        milestone_nine_five_requirements().required_rejection_rows,
    );
    assert!(
        missing.is_empty(),
        "missing milestone 9.5 rows: {missing:?}"
    );
}

#[test]
fn milestone_nine_five_rows_have_required_outputs() {
    let matrix =
        MilestoneNineFiveHostileMatrixAdapter::debt_close_hostile_certification_matrix_test();

    for row in &matrix.rows {
        assert!(
            row.control_lane.has_required_outputs(),
            "row '{}'",
            row.row_name
        );
        assert!(
            row.hostile_lane.has_required_outputs(),
            "row '{}'",
            row.row_name
        );
        assert!(
            row.parity_lane.has_required_outputs(),
            "row '{}'",
            row.row_name
        );
    }
    for row in &matrix.rejection_rows {
        assert!(
            row.control_lane.has_required_outputs(),
            "row '{}'",
            row.row_name
        );
        assert!(
            row.parity_lane.has_required_outputs(),
            "row '{}'",
            row.row_name
        );
        assert!(
            !row.hostile_lane.failure_kind.is_empty(),
            "row '{}'",
            row.row_name
        );
        assert!(
            !row.hostile_lane.failure_digest.is_empty(),
            "row '{}'",
            row.row_name
        );
        assert!(
            !row.hostile_lane.reuse_matrix_digest.is_empty(),
            "row '{}'",
            row.row_name
        );
    }
}

#[test]
fn milestone_nine_five_rows_enforce_required_assertion_classes() {
    let matrix =
        MilestoneNineFiveHostileMatrixAdapter::debt_close_hostile_certification_matrix_test();
    let mut covered = Vec::new();

    for row in &matrix.rows {
        let control = row.control_lane.semantic_signature();
        let hostile = row.hostile_lane.semantic_signature();
        let parity = row.parity_lane.semantic_signature();
        match row.hostile_expectation {
            HostileExpectation::EquivalentToControl => {
                assert_eq!(control, hostile, "row '{}'", row.row_name);
                covered.push(RequiredAssertionClass::Equality);
            }
            HostileExpectation::DistinctFromControl => {
                assert_ne!(control, hostile, "row '{}'", row.row_name);
                covered.push(RequiredAssertionClass::Inequality);
            }
        }
        match row.parity_anchor {
            ParityAnchor::Control => assert_eq!(parity, control, "row '{}'", row.row_name),
            ParityAnchor::Hostile => assert_eq!(parity, hostile, "row '{}'", row.row_name),
        }
    }

    for row in &matrix.rejection_rows {
        covered.push(RequiredAssertionClass::TypedFailure);
        assert!(
            row.hostile_lane.counter_snapshot.contains("residue=0"),
            "row '{}' must prove zero-residue denial",
            row.row_name
        );
        covered.push(RequiredAssertionClass::ZeroResidue);
    }

    covered.sort();
    covered.dedup();
    let missing = unmet_required_assertion_classes(
        &covered,
        milestone_nine_five_requirements().required_assertion_classes,
    );
    assert!(missing.is_empty(), "missing assertion classes: {missing:?}");
}

#[test]
fn milestone_nine_five_parity_rows_keep_canonical_identity_but_not_composition_authority() {
    let matrix =
        MilestoneNineFiveHostileMatrixAdapter::debt_close_hostile_certification_matrix_test();

    for row_name in [
        "named-scope-table-retained-derived-parity",
        "template-detail-live-artifact-parity",
        "public-bridge-bootstrap-fixed-under-template-composition",
    ] {
        let row = matrix
            .rows
            .iter()
            .find(|row| row.row_name == row_name)
            .unwrap();
        assert_eq!(
            row.control_lane.canonical_query_digest,
            row.hostile_lane.canonical_query_digest
        );
        assert_eq!(
            row.control_lane.canonical_result_shape_digest,
            row.hostile_lane.canonical_result_shape_digest
        );
        assert_eq!(
            row.control_lane.semantic_signature(),
            row.hostile_lane.semantic_signature()
        );
        assert_ne!(
            row.control_lane.composition_authority_digest,
            row.hostile_lane.composition_authority_digest
        );
        assert_ne!(
            row.control_lane.artifact_signature(),
            row.hostile_lane.artifact_signature()
        );
    }
}

#[test]
fn milestone_nine_five_projection_and_reuse_rows_shift_only_the_intended_surface() {
    let matrix =
        MilestoneNineFiveHostileMatrixAdapter::debt_close_hostile_certification_matrix_test();
    let projection = matrix
        .rows
        .iter()
        .find(|row| row.row_name == "retained-vs-live-projection-contract-distinctness")
        .unwrap();
    assert_eq!(
        projection.control_lane.canonical_query_digest,
        projection.hostile_lane.canonical_query_digest
    );
    assert_eq!(
        projection.control_lane.view_shape_digest,
        projection.hostile_lane.view_shape_digest
    );
    assert_eq!(
        projection.control_lane.reuse_matrix_digest,
        projection.hostile_lane.reuse_matrix_digest
    );
    assert_ne!(
        projection.control_lane.projection_contract_digest,
        projection.hostile_lane.projection_contract_digest
    );

    let grouped = matrix
        .rows
        .iter()
        .find(|row| row.row_name == "grouped-ordinary-vs-preserved-reuse-distinctness")
        .unwrap();
    assert_eq!(
        grouped.control_lane.canonical_query_digest,
        grouped.hostile_lane.canonical_query_digest
    );
    assert_eq!(
        grouped.control_lane.view_shape_digest,
        grouped.hostile_lane.view_shape_digest
    );
    assert_ne!(
        grouped.control_lane.reuse_matrix_digest,
        grouped.hostile_lane.reuse_matrix_digest
    );
    assert_ne!(
        grouped.control_lane.temporal_async_surface_posture,
        grouped.hostile_lane.temporal_async_surface_posture
    );
    assert_ne!(
        grouped.control_lane.saved_query_digest,
        grouped.hostile_lane.saved_query_digest
    );
}

#[test]
fn milestone_nine_five_rejection_rows_preserve_typed_temporal_async_targets() {
    let matrix =
        MilestoneNineFiveHostileMatrixAdapter::debt_close_hostile_certification_matrix_test();
    let grouped = matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == "grouped-preserved-reuse-basis-erasure-denied")
        .unwrap();
    assert_eq!(
        grouped.hostile_lane.temporal_async_surface_posture,
        SavedQueryTemporalAsyncSurfacePosture::OrdinaryOnly.as_str()
    );

    let inspector = matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == "inspector-target-preserved-reuse-downcast-denied")
        .unwrap();
    assert_eq!(
        inspector.hostile_lane.temporal_async_surface_posture,
        SavedQueryTemporalAsyncSurfacePosture::VisibleButDeferred.as_str()
    );
}

#[test]
fn milestone_nine_five_named_rows_are_exported_through_requirements() {
    let matrix =
        MilestoneNineFiveHostileMatrixAdapter::debt_close_hostile_certification_matrix_test();

    for row_name in milestone_nine_five_requirements().required_canonical_rows {
        assert!(contains_row(&matrix, row_name), "missing {row_name}");
    }
    for row_name in milestone_nine_five_requirements().required_rejection_rows {
        assert!(contains_row(&matrix, row_name), "missing {row_name}");
    }
}

#[test]
fn milestone_nine_five_artifact_signature_captures_required_narrow_artifacts() {
    let matrix =
        MilestoneNineFiveHostileMatrixAdapter::debt_close_hostile_certification_matrix_test();
    let lane = matrix
        .rows
        .iter()
        .find(|row| row.row_name == "named-scope-table-retained-derived-parity")
        .unwrap()
        .hostile_lane
        .clone();

    let mut composition_drift = lane.clone();
    composition_drift
        .composition_support_digest
        .push_str("-drift");
    assert_ne!(
        lane.artifact_signature(),
        composition_drift.artifact_signature()
    );

    let mut view_drift = lane.clone();
    view_drift.view_support_digest.push_str("-drift");
    assert_ne!(lane.artifact_signature(), view_drift.artifact_signature());

    let mut saved_query_drift = lane.clone();
    saved_query_drift.saved_query_digest.push_str("-drift");
    assert_ne!(
        lane.artifact_signature(),
        saved_query_drift.artifact_signature()
    );
}
