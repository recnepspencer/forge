use crate::facade::{canonicalize_request, GuidedAuthoringPath};

use super::fixtures::{binding_descriptor_parity, query_parity, result_shape_parity};
use super::matrices::{
    CanonicalCertificationBundle, CertificationMatrix, CertificationPerturbationClass,
    CertificationRow, HostileLaneExpectation, MilestoneOneCertificationArtifact, ParityAnchor,
    RejectionCertificationBundle, RejectionCertificationRow,
};
use super::profiles::CertificationProfile;

pub struct MilestoneOneCertificationAdapter;

impl MilestoneOneCertificationAdapter {
    pub fn canonical_query_normalization_certification_artifact(
    ) -> MilestoneOneCertificationArtifact {
        Self::canonical_query_normalization_parity_test().into_milestone_one_artifact()
    }

    pub fn canonical_query_normalization_parity_test() -> CertificationMatrix {
        let detail_control = canonicalize_request(
            GuidedAuthoringPath::pair_detail(
                query_parity::direct_detail_query(),
                result_shape_parity::direct_detail_shape(),
            )
            .unwrap(),
        )
        .unwrap();
        let detail_hostile = canonicalize_request(
            GuidedAuthoringPath::pair_detail(
                query_parity::reordered_detail_query(),
                result_shape_parity::reordered_detail_shape(),
            )
            .unwrap(),
        )
        .unwrap();
        let detail_parity = canonicalize_request(
            GuidedAuthoringPath::pair_detail(
                query_parity::direct_detail_query(),
                result_shape_parity::reordered_detail_shape(),
            )
            .unwrap(),
        )
        .unwrap();

        let binding_control = canonicalize_request(
            GuidedAuthoringPath::pair_collection_with_bindings(
                query_parity::collection_query(),
                result_shape_parity::collection_shape(),
                binding_descriptor_parity::ordered_bindings(),
            )
            .unwrap(),
        )
        .unwrap();
        let binding_hostile = canonicalize_request(
            GuidedAuthoringPath::pair_collection_with_bindings(
                query_parity::collection_query(),
                result_shape_parity::collection_shape(),
                binding_descriptor_parity::reordered_bindings(),
            )
            .unwrap(),
        )
        .unwrap();
        let binding_parity = canonicalize_request(
            GuidedAuthoringPath::pair_collection_with_bindings(
                query_parity::collection_query(),
                result_shape_parity::collection_shape(),
                binding_descriptor_parity::ordered_bindings(),
            )
            .unwrap(),
        )
        .unwrap();

        let shape_control = canonicalize_request(
            GuidedAuthoringPath::pair_detail(
                query_parity::direct_detail_query(),
                result_shape_parity::direct_detail_shape(),
            )
            .unwrap(),
        )
        .unwrap();
        let shape_hostile = canonicalize_request(
            GuidedAuthoringPath::pair_detail(
                query_parity::direct_detail_query(),
                result_shape_parity::reordered_detail_shape(),
            )
            .unwrap(),
        )
        .unwrap();
        let shape_parity = canonicalize_request(
            GuidedAuthoringPath::pair_detail(
                query_parity::reordered_detail_query(),
                result_shape_parity::reordered_detail_shape(),
            )
            .unwrap(),
        )
        .unwrap();

        let collection_control = canonicalize_request(
            GuidedAuthoringPath::pair_collection(
                query_parity::collection_query_with_two_projections(),
                result_shape_parity::reordered_collection_shape(),
            )
            .unwrap(),
        )
        .unwrap();
        let collection_hostile = canonicalize_request(
            GuidedAuthoringPath::pair_collection(
                query_parity::reordered_collection_query(),
                result_shape_parity::reordered_collection_shape(),
            )
            .unwrap(),
        )
        .unwrap();
        let collection_parity = canonicalize_request(
            GuidedAuthoringPath::pair_collection(
                query_parity::collection_query_with_two_projections(),
                result_shape_parity::reordered_collection_shape(),
            )
            .unwrap(),
        )
        .unwrap();

        let dedup_control = canonicalize_request(
            GuidedAuthoringPath::pair_detail(
                query_parity::duplicate_projection_detail_query(),
                result_shape_parity::duplicate_projection_detail_shape(),
            )
            .unwrap(),
        )
        .unwrap();
        let dedup_hostile = canonicalize_request(
            GuidedAuthoringPath::pair_detail(
                query_parity::duplicate_projection_detail_query(),
                result_shape_parity::duplicate_projection_detail_shape(),
            )
            .unwrap(),
        )
        .unwrap();
        let dedup_parity = canonicalize_request(
            GuidedAuthoringPath::pair_detail(
                query_parity::single_projection_detail_query(),
                result_shape_parity::single_projection_detail_shape(),
            )
            .unwrap(),
        )
        .unwrap();

        let semantic_distinct_control = canonicalize_request(
            GuidedAuthoringPath::pair_detail(
                query_parity::direct_detail_query(),
                result_shape_parity::direct_detail_shape(),
            )
            .unwrap(),
        )
        .unwrap();
        let semantic_distinct_hostile = canonicalize_request(
            GuidedAuthoringPath::pair_detail(
                query_parity::single_projection_detail_query(),
                result_shape_parity::single_projection_detail_shape(),
            )
            .unwrap(),
        )
        .unwrap();
        let semantic_distinct_parity = canonicalize_request(
            GuidedAuthoringPath::pair_detail(
                query_parity::duplicate_projection_detail_query(),
                result_shape_parity::duplicate_projection_detail_shape(),
            )
            .unwrap(),
        )
        .unwrap();

        let unsupported_query_error =
            crate::canonicalization::pipeline::QueryCanonicalizer::canonicalize_bundle(
                query_parity::unsupported_detail_query_for_test(),
                result_shape_parity::direct_detail_shape().into_raw(),
                crate::facade::QueryBindingDescriptor::default(),
            )
            .unwrap_err();
        let unsupported_shape_error =
            crate::canonicalization::pipeline::QueryCanonicalizer::canonicalize_bundle(
                query_parity::direct_detail_query().into_raw(),
                result_shape_parity::unsupported_detail_shape_for_test(),
                crate::facade::QueryBindingDescriptor::default(),
            )
            .unwrap_err();
        let helper_residue_error = canonicalize_request(
            GuidedAuthoringPath::pair_detail(
                query_parity::single_projection_detail_query(),
                result_shape_parity::single_projection_detail_shape(),
            )
            .unwrap()
            .with_helper_residue_for_test("builder_history"),
        )
        .unwrap_err();

        CertificationMatrix {
            suite_name: "Canonical Query Normalization Parity Test",
            rows: vec![
                CertificationRow {
                    row_name: "detail-query-parity",
                    perturbation_class: CertificationPerturbationClass::ConstructionPath,
                    hostile_expectation: HostileLaneExpectation::EquivalentToControl,
                    parity_anchor: ParityAnchor::Control,
                    control_lane: to_bundle(
                        CertificationProfile::DirectConstruction,
                        &detail_control,
                    ),
                    hostile_lane: to_bundle(
                        CertificationProfile::BuilderReordering,
                        &detail_hostile,
                    ),
                    parity_lane: to_bundle(CertificationProfile::ReplayParity, &detail_parity),
                },
                CertificationRow {
                    row_name: "result-shape-helper-composition",
                    perturbation_class: CertificationPerturbationClass::ResultShapeComposition,
                    hostile_expectation: HostileLaneExpectation::EquivalentToControl,
                    parity_anchor: ParityAnchor::Control,
                    control_lane: to_bundle(
                        CertificationProfile::DirectConstruction,
                        &shape_control,
                    ),
                    hostile_lane: to_bundle(
                        CertificationProfile::BuilderReordering,
                        &shape_hostile,
                    ),
                    parity_lane: to_bundle(CertificationProfile::ReplayParity, &shape_parity),
                },
                CertificationRow {
                    row_name: "binding-descriptor-parity",
                    perturbation_class: CertificationPerturbationClass::BindingDescriptorVariation,
                    hostile_expectation: HostileLaneExpectation::EquivalentToControl,
                    parity_anchor: ParityAnchor::Control,
                    control_lane: to_bundle(
                        CertificationProfile::DirectConstruction,
                        &binding_control,
                    ),
                    hostile_lane: to_bundle(
                        CertificationProfile::BindingVariation,
                        &binding_hostile,
                    ),
                    parity_lane: to_bundle(CertificationProfile::ReplayParity, &binding_parity),
                },
                CertificationRow {
                    row_name: "collection-reordered-projection-parity",
                    perturbation_class: CertificationPerturbationClass::ConstructionPath,
                    hostile_expectation: HostileLaneExpectation::EquivalentToControl,
                    parity_anchor: ParityAnchor::Control,
                    control_lane: to_bundle(
                        CertificationProfile::DirectConstruction,
                        &collection_control,
                    ),
                    hostile_lane: to_bundle(
                        CertificationProfile::BuilderReordering,
                        &collection_hostile,
                    ),
                    parity_lane: to_bundle(CertificationProfile::ReplayParity, &collection_parity),
                },
                CertificationRow {
                    row_name: "duplicate-clause-deduplication",
                    perturbation_class: CertificationPerturbationClass::Deduplication,
                    hostile_expectation: HostileLaneExpectation::EquivalentToControl,
                    parity_anchor: ParityAnchor::Hostile,
                    control_lane: to_bundle(
                        CertificationProfile::DirectConstruction,
                        &dedup_control,
                    ),
                    hostile_lane: to_bundle(
                        CertificationProfile::BuilderReordering,
                        &dedup_hostile,
                    ),
                    parity_lane: to_bundle(CertificationProfile::ReplayParity, &dedup_parity),
                },
                CertificationRow {
                    row_name: "semantic-distinction-boundary",
                    perturbation_class: CertificationPerturbationClass::MeaningChange,
                    hostile_expectation: HostileLaneExpectation::DistinctFromControl,
                    parity_anchor: ParityAnchor::Hostile,
                    control_lane: to_bundle(
                        CertificationProfile::DirectConstruction,
                        &semantic_distinct_control,
                    ),
                    hostile_lane: to_bundle(
                        CertificationProfile::BuilderReordering,
                        &semantic_distinct_hostile,
                    ),
                    parity_lane: to_bundle(
                        CertificationProfile::ReplayParity,
                        &semantic_distinct_parity,
                    ),
                },
            ],
            rejection_rows: vec![
                RejectionCertificationRow {
                    row_name: "unsupported-authored-query-family",
                    perturbation_class: CertificationPerturbationClass::UnsupportedAuthoredForm,
                    control_lane: to_bundle(
                        CertificationProfile::DirectConstruction,
                        &detail_control,
                    ),
                    hostile_lane: to_rejection_bundle(
                        CertificationProfile::BuilderReordering,
                        &unsupported_query_error,
                    ),
                    parity_lane: to_bundle(CertificationProfile::ReplayParity, &detail_parity),
                },
                RejectionCertificationRow {
                    row_name: "unsupported-authored-result-shape-family",
                    perturbation_class: CertificationPerturbationClass::UnsupportedAuthoredForm,
                    control_lane: to_bundle(
                        CertificationProfile::DirectConstruction,
                        &detail_control,
                    ),
                    hostile_lane: to_rejection_bundle(
                        CertificationProfile::BuilderReordering,
                        &unsupported_shape_error,
                    ),
                    parity_lane: to_bundle(CertificationProfile::ReplayParity, &detail_parity),
                },
                RejectionCertificationRow {
                    row_name: "forbidden-fallback-case",
                    perturbation_class: CertificationPerturbationClass::ForbiddenFallback,
                    control_lane: to_bundle(
                        CertificationProfile::DirectConstruction,
                        &semantic_distinct_hostile,
                    ),
                    hostile_lane: to_rejection_bundle(
                        CertificationProfile::BuilderReordering,
                        &helper_residue_error,
                    ),
                    parity_lane: to_bundle(
                        CertificationProfile::ReplayParity,
                        &semantic_distinct_parity,
                    ),
                },
            ],
        }
    }
}

fn to_bundle(
    profile: CertificationProfile,
    bundle: &crate::facade::CanonicalQueryBundle,
) -> CanonicalCertificationBundle {
    CanonicalCertificationBundle {
        profile,
        query_digest: bundle.query().digest().as_str().to_string(),
        result_shape_digest: bundle.result_shape().digest().as_str().to_string(),
        canonicalization_report: bundle.report().clone(),
        warning_count: bundle.report().warnings().len(),
        event_count: bundle.report().events().len(),
        counter_snapshot: bundle.counters().clone(),
    }
}

fn to_rejection_bundle(
    profile: CertificationProfile,
    error: &crate::facade::QueryCanonicalizationError,
) -> RejectionCertificationBundle {
    RejectionCertificationBundle {
        profile,
        failure_class: format!("{:?}", error.failure_class()),
        failure_digest: format!("{error:?}"),
    }
}

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
