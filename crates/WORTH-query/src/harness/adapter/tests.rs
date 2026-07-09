use super::MilestoneOneCertificationAdapter;
use crate::harness::matrices::CertificationPerturbationClass;

#[test]
fn canonical_query_normalization_parity_adapter_emits_named_matrix() {
    let matrix = MilestoneOneCertificationAdapter::canonical_query_normalization_parity_test();

    assert_eq!(
        matrix.suite_name,
        "Canonical Query Normalization Parity Test"
    );
    assert_eq!(matrix.rows.len(), 6);
    assert_eq!(matrix.rejection_rows.len(), 3);

    let detail_row = &matrix.rows[0];
    assert_eq!(detail_row.row_name, "detail-query-parity");
    assert_eq!(
        detail_row.control_lane.query_digest,
        detail_row.hostile_lane.query_digest
    );
    assert_eq!(
        detail_row.control_lane.result_shape_digest,
        detail_row.hostile_lane.result_shape_digest
    );
    assert_eq!(
        detail_row
            .control_lane
            .counter_snapshot
            .canonicalization_fallback_count,
        0
    );
    assert_eq!(
        detail_row
            .hostile_lane
            .counter_snapshot
            .canonicalization_fallback_count,
        0
    );
    assert_eq!(
        detail_row
            .parity_lane
            .counter_snapshot
            .canonicalization_fallback_count,
        0
    );

    let shape_row = &matrix.rows[1];
    assert_eq!(shape_row.row_name, "result-shape-helper-composition");
    assert_eq!(
        shape_row.control_lane.query_digest,
        shape_row.hostile_lane.query_digest
    );
    assert_eq!(
        shape_row.control_lane.result_shape_digest,
        shape_row.hostile_lane.result_shape_digest
    );
    assert_eq!(
        shape_row
            .control_lane
            .counter_snapshot
            .query_deduplication_count,
        0
    );
    assert_eq!(
        shape_row
            .control_lane
            .counter_snapshot
            .result_shape_deduplication_count,
        0
    );

    let binding_row = &matrix.rows[2];
    assert_eq!(binding_row.row_name, "binding-descriptor-parity");
    assert_eq!(
        binding_row.control_lane.query_digest,
        binding_row.hostile_lane.query_digest
    );
    assert_eq!(
        binding_row.control_lane.result_shape_digest,
        binding_row.hostile_lane.result_shape_digest
    );
    assert_eq!(
        binding_row
            .control_lane
            .counter_snapshot
            .canonicalization_fallback_count,
        0
    );
    assert_eq!(
        binding_row
            .hostile_lane
            .counter_snapshot
            .canonicalization_fallback_count,
        0
    );
    assert_eq!(
        binding_row
            .parity_lane
            .counter_snapshot
            .canonicalization_fallback_count,
        0
    );
    assert_eq!(
        binding_row
            .control_lane
            .counter_snapshot
            .binding_descriptor_count,
        2
    );
    assert_eq!(
        binding_row
            .hostile_lane
            .counter_snapshot
            .binding_descriptor_count,
        2
    );

    let collection_row = &matrix.rows[3];
    assert_eq!(
        collection_row.row_name,
        "collection-reordered-projection-parity"
    );
    assert_eq!(
        collection_row.control_lane.query_digest,
        collection_row.hostile_lane.query_digest
    );
    assert_eq!(
        collection_row.control_lane.result_shape_digest,
        collection_row.hostile_lane.result_shape_digest
    );
    assert_eq!(
        collection_row
            .control_lane
            .counter_snapshot
            .projection_entry_count,
        2
    );
    assert_eq!(
        collection_row
            .hostile_lane
            .counter_snapshot
            .projection_entry_count,
        2
    );

    let dedup_row = &matrix.rows[4];
    assert_eq!(dedup_row.row_name, "duplicate-clause-deduplication");
    assert_eq!(
        dedup_row
            .control_lane
            .counter_snapshot
            .query_deduplication_count,
        1
    );
    assert_eq!(
        dedup_row
            .hostile_lane
            .counter_snapshot
            .query_deduplication_count,
        1
    );
    assert_eq!(
        dedup_row.hostile_lane.query_digest,
        dedup_row.parity_lane.query_digest
    );
    assert_eq!(
        dedup_row.hostile_lane.result_shape_digest,
        dedup_row.parity_lane.result_shape_digest
    );
    assert_eq!(
        dedup_row
            .control_lane
            .counter_snapshot
            .result_shape_deduplication_count,
        0
    );
    assert_eq!(
        dedup_row
            .parity_lane
            .counter_snapshot
            .query_deduplication_count,
        0
    );

    let distinct_row = &matrix.rows[5];
    assert_eq!(distinct_row.row_name, "semantic-distinction-boundary");
    assert_ne!(
        distinct_row.control_lane.query_digest,
        distinct_row.hostile_lane.query_digest
    );
    assert_ne!(
        distinct_row.control_lane.result_shape_digest,
        distinct_row.hostile_lane.result_shape_digest
    );
    assert_eq!(
        distinct_row.hostile_lane.query_digest,
        distinct_row.parity_lane.query_digest
    );
    assert_eq!(
        distinct_row.hostile_lane.result_shape_digest,
        distinct_row.parity_lane.result_shape_digest
    );
    assert_eq!(
        distinct_row
            .control_lane
            .counter_snapshot
            .projection_entry_count,
        2
    );
    assert_eq!(
        distinct_row
            .hostile_lane
            .counter_snapshot
            .projection_entry_count,
        1
    );

    let rejection_row = &matrix.rejection_rows[0];
    assert_eq!(rejection_row.row_name, "unsupported-authored-query-family");
    assert!(!rejection_row.hostile_lane.failure_digest.is_empty());
    assert_eq!(
        rejection_row
            .control_lane
            .counter_snapshot
            .canonicalization_fallback_count,
        0
    );

    let forbidden_fallback_row = &matrix.rejection_rows[2];
    assert_eq!(forbidden_fallback_row.row_name, "forbidden-fallback-case");
    assert!(forbidden_fallback_row
        .hostile_lane
        .failure_digest
        .contains("NonCanonicalHelperResidueDetected"));
}

#[test]
fn canonical_query_normalization_certification_artifact_is_offline_ready() {
    let artifact =
        MilestoneOneCertificationAdapter::canonical_query_normalization_certification_artifact();

    assert_eq!(
        artifact.suite_name,
        "Canonical Query Normalization Parity Test"
    );
    assert!(!artifact.certification_bundle_digest.is_empty());
    assert!(!artifact.coverage_matrix_digest.is_empty());
    assert_eq!(artifact.bundle_completeness_report.canonical_row_count, 6);
    assert_eq!(artifact.bundle_completeness_report.rejection_row_count, 3);
    assert_eq!(artifact.bundle_completeness_report.supported_lane_count, 24);
    assert_eq!(
        artifact.bundle_completeness_report.successful_lane_count,
        24
    );
    assert_eq!(
        artifact.bundle_completeness_report.zero_fallback_lane_count,
        24
    );
    assert!(artifact
        .bundle_completeness_report
        .covered_perturbation_classes
        .contains(&CertificationPerturbationClass::ConstructionPath));
    assert!(artifact
        .bundle_completeness_report
        .covered_perturbation_classes
        .contains(&CertificationPerturbationClass::MeaningChange));
    assert!(artifact
        .bundle_completeness_report
        .covered_perturbation_classes
        .contains(&CertificationPerturbationClass::ForbiddenFallback));
    assert!(
        artifact
            .bundle_completeness_report
            .all_lanes_emit_required_outputs
    );
    assert!(
        artifact
            .bundle_completeness_report
            .all_rows_have_hostile_coverage
    );
    assert!(artifact
        .bundle_completeness_report
        .unmet_required_rows
        .is_empty());
    assert!(artifact
        .bundle_completeness_report
        .unmet_required_assertion_classes
        .is_empty());
    assert!(
        artifact
            .bundle_completeness_report
            .covers_all_mutation_sensitivity_classes
    );
    assert!(
        artifact
            .bundle_completeness_report
            .covers_all_milestone_one_normative_scenarios
    );
    assert!(artifact.bundle_completeness_report.offline_analysis_ready);
    assert_eq!(artifact.counter_snapshot.canonicalization_fallback_count, 0);
    assert_eq!(artifact.matrix.rows.len(), 6);
    assert_eq!(artifact.matrix.rejection_rows.len(), 3);
}

#[test]
fn canonical_query_normalization_certification_artifact_is_deterministic() {
    let artifact_a =
        MilestoneOneCertificationAdapter::canonical_query_normalization_certification_artifact();
    let artifact_b =
        MilestoneOneCertificationAdapter::canonical_query_normalization_certification_artifact();

    assert_eq!(
        artifact_a.certification_bundle_digest,
        artifact_b.certification_bundle_digest
    );
    assert_eq!(
        artifact_a.coverage_matrix_digest,
        artifact_b.coverage_matrix_digest
    );
    assert_eq!(
        artifact_a.bundle_completeness_report,
        artifact_b.bundle_completeness_report
    );
    assert_eq!(artifact_a.counter_snapshot, artifact_b.counter_snapshot);
}
