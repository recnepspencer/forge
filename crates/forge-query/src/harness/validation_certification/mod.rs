mod fixtures;
mod tests;

use crate::harness::fixtures::schema_view::{
    alternate_detail_schema_view, detail_schema_view, legal_detail_bundle,
    legal_ordering_only_bundle,
};
use crate::validation::{
    validate_canonical_bundle, validate_canonical_bundle_with_failure_artifact,
};

use super::profiles::CertificationProfile;
use super::validation_matrix::{
    MilestoneTwoValidationCertificationArtifact, ValidationCertificationBundle,
    ValidationCertificationMatrix, ValidationCertificationRow, ValidationHostileExpectation,
    ValidationParityAnchor, ValidationPerturbationClass, ValidationRejectionCertificationBundle,
    ValidationRejectionCertificationRow,
};
use fixtures::*;

pub struct MilestoneTwoValidationCertificationAdapter;

impl MilestoneTwoValidationCertificationAdapter {
    pub fn schema_aware_rejection_and_projection_legality_certification_artifact(
    ) -> MilestoneTwoValidationCertificationArtifact {
        Self::schema_aware_rejection_and_projection_legality_test().into_milestone_two_artifact()
    }

    pub fn schema_aware_rejection_and_projection_legality_test() -> ValidationCertificationMatrix {
        let legal_control =
            validate_canonical_bundle(legal_detail_bundle(), detail_schema_view()).unwrap();
        let legal_hostile =
            validate_canonical_bundle(reordered_legal_detail_bundle(), detail_schema_view())
                .unwrap();
        let legal_parity =
            validate_canonical_bundle(legal_detail_bundle(), detail_schema_view()).unwrap();
        let schema_variation_hostile =
            validate_canonical_bundle(legal_detail_bundle(), alternate_detail_schema_view())
                .unwrap();
        let ordering_only_hostile =
            validate_canonical_bundle(legal_ordering_only_bundle(), detail_schema_view()).unwrap();
        let greater_than_control =
            validate_canonical_bundle(legal_greater_than_bundle(), detail_schema_view()).unwrap();
        let greater_than_hostile =
            validate_canonical_bundle(reordered_greater_than_bundle(), detail_schema_view())
                .unwrap();
        let less_than_control =
            validate_canonical_bundle(legal_less_than_bundle(), detail_schema_view()).unwrap();
        let less_than_hostile =
            validate_canonical_bundle(reordered_less_than_bundle(), detail_schema_view()).unwrap();
        let normalized_greater_than_hostile =
            validate_canonical_bundle(redundant_greater_than_bundle(), detail_schema_view())
                .unwrap();
        let bounded_range_control =
            validate_canonical_bundle(bounded_range_bundle(), detail_schema_view()).unwrap();
        let bounded_range_hostile =
            validate_canonical_bundle(reordered_bounded_range_bundle(), detail_schema_view())
                .unwrap();
        let contains_control =
            validate_canonical_bundle(legal_contains_bundle(), detail_schema_view()).unwrap();
        let contains_hostile =
            validate_canonical_bundle(reordered_contains_bundle(), detail_schema_view()).unwrap();
        let membership_control =
            validate_canonical_bundle(legal_membership_bundle(), detail_schema_view()).unwrap();
        let membership_hostile =
            validate_canonical_bundle(reordered_membership_bundle(), detail_schema_view()).unwrap();
        let membership_normalized_hostile =
            validate_canonical_bundle(intersected_membership_bundle(), detail_schema_view())
                .unwrap();
        let presence_control =
            validate_canonical_bundle(legal_presence_bundle(), detail_schema_view()).unwrap();
        let presence_hostile =
            validate_canonical_bundle(reordered_presence_bundle(), detail_schema_view()).unwrap();

        ValidationCertificationMatrix {
            suite_name: "Schema-Aware Rejection And Projection Legality Test",
            rows: vec![
                row(
                    "legal-detail-query-parity",
                    ValidationPerturbationClass::ConstructionPath,
                    ValidationHostileExpectation::EquivalentToControl,
                    ValidationParityAnchor::Control,
                    to_bundle(CertificationProfile::DirectConstruction, &legal_control),
                    to_bundle(CertificationProfile::BuilderReordering, &legal_hostile),
                    to_bundle(CertificationProfile::ReplayParity, &legal_parity),
                ),
                row(
                    "equivalent-builder-composed-legal-query",
                    ValidationPerturbationClass::ConstructionPath,
                    ValidationHostileExpectation::EquivalentToControl,
                    ValidationParityAnchor::Control,
                    to_bundle(CertificationProfile::DirectConstruction, &legal_control),
                    to_bundle(CertificationProfile::BuilderReordering, &legal_hostile),
                    to_bundle(CertificationProfile::ReplayParity, &legal_parity),
                ),
                row(
                    "schema-basis-variation-boundary",
                    ValidationPerturbationClass::SchemaBasisVariation,
                    ValidationHostileExpectation::DistinctFromControl,
                    ValidationParityAnchor::Hostile,
                    to_bundle(CertificationProfile::DirectConstruction, &legal_control),
                    to_bundle(CertificationProfile::BuilderReordering, &schema_variation_hostile),
                    to_bundle(
                        CertificationProfile::ReplayParity,
                        &validate_canonical_bundle(
                            legal_detail_bundle(),
                            alternate_detail_schema_view(),
                        )
                        .unwrap(),
                    ),
                ),
                row(
                    "ordering-only-authority-boundary",
                    ValidationPerturbationClass::OrderingLegality,
                    ValidationHostileExpectation::DistinctFromControl,
                    ValidationParityAnchor::Hostile,
                    to_bundle(CertificationProfile::DirectConstruction, &legal_control),
                    to_bundle(CertificationProfile::BuilderReordering, &ordering_only_hostile),
                    to_bundle(
                        CertificationProfile::ReplayParity,
                        &validate_canonical_bundle(
                            legal_ordering_only_bundle(),
                            detail_schema_view(),
                        )
                        .unwrap(),
                    ),
                ),
                row(
                    "integer-greater-than-predicate-parity",
                    ValidationPerturbationClass::PredicateLegality,
                    ValidationHostileExpectation::EquivalentToControl,
                    ValidationParityAnchor::Control,
                    to_bundle(CertificationProfile::DirectConstruction, &greater_than_control),
                    to_bundle(CertificationProfile::BuilderReordering, &greater_than_hostile),
                    to_bundle(
                        CertificationProfile::ReplayParity,
                        &validate_canonical_bundle(
                            legal_greater_than_bundle(),
                            detail_schema_view(),
                        )
                        .unwrap(),
                    ),
                ),
                row(
                    "integer-less-than-predicate-parity",
                    ValidationPerturbationClass::PredicateLegality,
                    ValidationHostileExpectation::EquivalentToControl,
                    ValidationParityAnchor::Control,
                    to_bundle(CertificationProfile::DirectConstruction, &less_than_control),
                    to_bundle(CertificationProfile::BuilderReordering, &less_than_hostile),
                    to_bundle(
                        CertificationProfile::ReplayParity,
                        &validate_canonical_bundle(legal_less_than_bundle(), detail_schema_view())
                            .unwrap(),
                    ),
                ),
                row(
                    "redundant-greater-than-normalization",
                    ValidationPerturbationClass::PredicateLegality,
                    ValidationHostileExpectation::EquivalentToControl,
                    ValidationParityAnchor::Control,
                    to_bundle(
                        CertificationProfile::DirectConstruction,
                        &validate_canonical_bundle(
                            strongest_greater_than_bundle(),
                            detail_schema_view(),
                        )
                        .unwrap(),
                    ),
                    to_bundle(
                        CertificationProfile::BuilderReordering,
                        &normalized_greater_than_hostile,
                    ),
                    to_bundle(
                        CertificationProfile::ReplayParity,
                        &validate_canonical_bundle(
                            strongest_greater_than_bundle(),
                            detail_schema_view(),
                        )
                        .unwrap(),
                    ),
                ),
                row(
                    "bounded-range-normalization",
                    ValidationPerturbationClass::PredicateLegality,
                    ValidationHostileExpectation::EquivalentToControl,
                    ValidationParityAnchor::Control,
                    to_bundle(CertificationProfile::DirectConstruction, &bounded_range_control),
                    to_bundle(CertificationProfile::BuilderReordering, &bounded_range_hostile),
                    to_bundle(
                        CertificationProfile::ReplayParity,
                        &validate_canonical_bundle(bounded_range_bundle(), detail_schema_view())
                            .unwrap(),
                    ),
                ),
                row(
                    "text-contains-predicate-parity",
                    ValidationPerturbationClass::PredicateLegality,
                    ValidationHostileExpectation::EquivalentToControl,
                    ValidationParityAnchor::Control,
                    to_bundle(CertificationProfile::DirectConstruction, &contains_control),
                    to_bundle(CertificationProfile::BuilderReordering, &contains_hostile),
                    to_bundle(
                        CertificationProfile::ReplayParity,
                        &validate_canonical_bundle(legal_contains_bundle(), detail_schema_view())
                            .unwrap(),
                    ),
                ),
                row(
                    "scalar-membership-predicate-parity",
                    ValidationPerturbationClass::PredicateLegality,
                    ValidationHostileExpectation::EquivalentToControl,
                    ValidationParityAnchor::Control,
                    to_bundle(CertificationProfile::DirectConstruction, &membership_control),
                    to_bundle(CertificationProfile::BuilderReordering, &membership_hostile),
                    to_bundle(
                        CertificationProfile::ReplayParity,
                        &validate_canonical_bundle(
                            legal_membership_bundle(),
                            detail_schema_view(),
                        )
                        .unwrap(),
                    ),
                ),
                row(
                    "membership-intersection-normalization",
                    ValidationPerturbationClass::PredicateLegality,
                    ValidationHostileExpectation::EquivalentToControl,
                    ValidationParityAnchor::Control,
                    to_bundle(
                        CertificationProfile::DirectConstruction,
                        &validate_canonical_bundle(
                            intersected_membership_bundle(),
                            detail_schema_view(),
                        )
                        .unwrap(),
                    ),
                    to_bundle(
                        CertificationProfile::BuilderReordering,
                        &validate_canonical_bundle(
                            overlapping_membership_bundle(),
                            detail_schema_view(),
                        )
                        .unwrap(),
                    ),
                    to_bundle(
                        CertificationProfile::ReplayParity,
                        &membership_normalized_hostile,
                    ),
                ),
                row(
                    "presence-predicate-parity",
                    ValidationPerturbationClass::PredicateLegality,
                    ValidationHostileExpectation::EquivalentToControl,
                    ValidationParityAnchor::Control,
                    to_bundle(CertificationProfile::DirectConstruction, &presence_control),
                    to_bundle(CertificationProfile::BuilderReordering, &presence_hostile),
                    to_bundle(
                        CertificationProfile::ReplayParity,
                        &validate_canonical_bundle(legal_presence_bundle(), detail_schema_view())
                            .unwrap(),
                    ),
                ),
            ],
            rejection_rows: vec![
                rejection("unknown-aspect-projection", ValidationPerturbationClass::ProjectionLegality, &legal_control, unknown_aspect_bundle()),
                rejection("illegal-traversal-edge-or-depth", ValidationPerturbationClass::TraversalLegality, &legal_control, illegal_traversal_bundle()),
                rejection("non-orderable-ordering-field", ValidationPerturbationClass::OrderingLegality, &legal_control, non_orderable_ordering_bundle()),
                rejection("predicate-contradiction-rejection", ValidationPerturbationClass::PredicateLegality, &greater_than_control, contradictory_predicate_bundle()),
                rejection("empty-range-rejection", ValidationPerturbationClass::PredicateLegality, &bounded_range_control, empty_range_bundle()),
                rejection("text-predicate-capability-rejection", ValidationPerturbationClass::PredicateLegality, &contains_control, text_capability_illegal_bundle()),
                rejection("membership-capability-rejection", ValidationPerturbationClass::PredicateLegality, &membership_control, membership_capability_illegal_bundle()),
                rejection("presence-capability-rejection", ValidationPerturbationClass::PredicateLegality, &presence_control, presence_capability_illegal_bundle()),
                rejection("incompatible-predicate-family", ValidationPerturbationClass::PredicateLegality, &legal_control, incompatible_predicate_bundle()),
                rejection("invalid-result-shape-binding", ValidationPerturbationClass::ResultShapeBindingLegality, &legal_control, invalid_result_shape_binding_bundle()),
                rejection("structured-content-illegality", ValidationPerturbationClass::StructuredContentLegality, &legal_control, structured_content_illegal_bundle()),
                rejection("workflow-context-illegality", ValidationPerturbationClass::WorkflowContextLegality, &legal_control, workflow_context_illegal_bundle()),
                rejection("forbidden-widening-case", ValidationPerturbationClass::ForbiddenWidening, &legal_control, forbidden_widening_bundle()),
            ],
        }
    }
}

fn row(
    row_name: &'static str,
    perturbation_class: ValidationPerturbationClass,
    hostile_expectation: ValidationHostileExpectation,
    parity_anchor: ValidationParityAnchor,
    control_lane: ValidationCertificationBundle,
    hostile_lane: ValidationCertificationBundle,
    parity_lane: ValidationCertificationBundle,
) -> ValidationCertificationRow {
    ValidationCertificationRow { row_name, perturbation_class, hostile_expectation, parity_anchor, control_lane, hostile_lane, parity_lane }
}

fn rejection(
    row_name: &'static str,
    perturbation_class: ValidationPerturbationClass,
    control: &crate::facade::ValidatedQueryBundle,
    hostile_bundle: crate::facade::CanonicalQueryBundle,
) -> ValidationRejectionCertificationRow {
    let hostile = validate_canonical_bundle_with_failure_artifact(hostile_bundle, detail_schema_view()).unwrap_err();
    ValidationRejectionCertificationRow {
        row_name,
        perturbation_class,
        control_lane: to_bundle(CertificationProfile::DirectConstruction, control),
        hostile_lane: to_rejection_bundle(CertificationProfile::BuilderReordering, &hostile),
        parity_lane: to_bundle(CertificationProfile::ReplayParity, control),
    }
}

fn to_bundle(
    profile: CertificationProfile,
    bundle: &crate::facade::ValidatedQueryBundle,
) -> ValidationCertificationBundle {
    ValidationCertificationBundle {
        profile,
        query_digest: bundle.query().canonical_query_digest().as_str().to_string(),
        validated_query_digest: bundle.query().digest().as_str().to_string(),
        validated_result_shape_digest: bundle.result_shape().digest().as_str().to_string(),
        basis_digest: bundle.query().schema_basis().as_str().to_string(),
        validation_report: bundle.report().clone(),
        counter_snapshot: bundle.counters().clone(),
    }
}

fn to_rejection_bundle(
    profile: CertificationProfile,
    failure: &crate::validation::ValidationFailureArtifact,
) -> ValidationRejectionCertificationBundle {
    ValidationRejectionCertificationBundle {
        profile,
        failure_class: format!("{:?}", failure.error.failure_class()),
        failure_digest: failure.error.failure_digest(),
        validation_rejection_matrix: failure.rejection_matrix.clone(),
        counter_snapshot: failure.counters.clone(),
    }
}
