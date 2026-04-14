use crate::facade::{canonicalize_request, GuidedAuthoringPath};

use super::super::fixtures::{binding_descriptor_parity, query_parity, result_shape_parity};
use super::super::matrices::{
    CertificationMatrix, CertificationPerturbationClass, CertificationRow, HostileLaneExpectation,
    MilestoneOneCertificationArtifact, ParityAnchor, RejectionCertificationRow,
};
use super::super::profiles::CertificationProfile;
use super::bundles::{to_bundle, to_rejection_bundle};

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
