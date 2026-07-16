use crate::harness::fixtures::schema_view::{
    alternate_detail_schema_view, detail_schema_view, legal_ordering_only_bundle,
    structured_content_queryable_schema_view, workflow_queryable_schema_view,
};
use crate::validation::validate_canonical_bundle;

use super::super::profiles::CertificationProfile;
use super::super::validation_matrix::{
    ValidationCertificationBundle, ValidationCertificationRow, ValidationHostileExpectation,
    ValidationParityAnchor, ValidationPerturbationClass,
};
use super::bundles::to_bundle;
use super::fixtures::*;

pub(super) fn canonical_rows() -> Vec<ValidationCertificationRow> {
    let legal_control = crate::harness::fixtures::validated_bundles::legal_detail_bundle();
    let legal_hostile =
        validate_canonical_bundle(reordered_legal_detail_bundle(), detail_schema_view()).unwrap();
    let legal_parity = crate::harness::fixtures::validated_bundles::legal_detail_bundle();
    let schema_variation_hostile = validate_canonical_bundle(
        crate::harness::fixtures::canonical_bundles::legal_detail_bundle(),
        alternate_detail_schema_view(),
    )
    .unwrap();
    let ordering_only_hostile =
        validate_canonical_bundle(legal_ordering_only_bundle(), detail_schema_view()).unwrap();
    let greater_than_control =
        validate_canonical_bundle(legal_greater_than_bundle(), detail_schema_view()).unwrap();
    let greater_than_hostile =
        validate_canonical_bundle(reordered_greater_than_bundle(), detail_schema_view()).unwrap();
    let less_than_control =
        validate_canonical_bundle(legal_less_than_bundle(), detail_schema_view()).unwrap();
    let less_than_hostile =
        validate_canonical_bundle(reordered_less_than_bundle(), detail_schema_view()).unwrap();
    let normalized_greater_than_hostile =
        validate_canonical_bundle(redundant_greater_than_bundle(), detail_schema_view()).unwrap();
    let bounded_range_control =
        validate_canonical_bundle(bounded_range_bundle(), detail_schema_view()).unwrap();
    let bounded_range_hostile =
        validate_canonical_bundle(reordered_bounded_range_bundle(), detail_schema_view()).unwrap();
    let contains_control =
        validate_canonical_bundle(legal_contains_bundle(), detail_schema_view()).unwrap();
    let contains_hostile =
        validate_canonical_bundle(reordered_contains_bundle(), detail_schema_view()).unwrap();
    let membership_control =
        validate_canonical_bundle(legal_membership_bundle(), detail_schema_view()).unwrap();
    let membership_hostile =
        validate_canonical_bundle(reordered_membership_bundle(), detail_schema_view()).unwrap();
    let membership_normalized_hostile =
        validate_canonical_bundle(intersected_membership_bundle(), detail_schema_view()).unwrap();
    let presence_control =
        validate_canonical_bundle(legal_presence_bundle(), detail_schema_view()).unwrap();
    let presence_hostile =
        validate_canonical_bundle(reordered_presence_bundle(), detail_schema_view()).unwrap();
    let structured_content_control = validate_canonical_bundle(
        legal_structured_content_bundle(),
        structured_content_queryable_schema_view(),
    )
    .unwrap();
    let structured_content_hostile = validate_canonical_bundle(
        reordered_legal_structured_content_bundle(),
        structured_content_queryable_schema_view(),
    )
    .unwrap();
    let workflow_control = validate_canonical_bundle(
        legal_workflow_predicate_bundle(),
        workflow_queryable_schema_view(),
    )
    .unwrap();
    let workflow_hostile = validate_canonical_bundle(
        reordered_legal_workflow_predicate_bundle(),
        workflow_queryable_schema_view(),
    )
    .unwrap();

    vec![
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
            to_bundle(
                CertificationProfile::BuilderReordering,
                &schema_variation_hostile,
            ),
            to_bundle(
                CertificationProfile::ReplayParity,
                &validate_canonical_bundle(
                    crate::harness::fixtures::canonical_bundles::legal_detail_bundle(),
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
            to_bundle(
                CertificationProfile::BuilderReordering,
                &ordering_only_hostile,
            ),
            to_bundle(
                CertificationProfile::ReplayParity,
                &validate_canonical_bundle(legal_ordering_only_bundle(), detail_schema_view())
                    .unwrap(),
            ),
        ),
        row(
            "native-greater-than-predicate-parity",
            ValidationPerturbationClass::PredicateLegality,
            ValidationHostileExpectation::EquivalentToControl,
            ValidationParityAnchor::Control,
            to_bundle(
                CertificationProfile::DirectConstruction,
                &greater_than_control,
            ),
            to_bundle(
                CertificationProfile::BuilderReordering,
                &greater_than_hostile,
            ),
            to_bundle(
                CertificationProfile::ReplayParity,
                &validate_canonical_bundle(legal_greater_than_bundle(), detail_schema_view())
                    .unwrap(),
            ),
        ),
        row(
            "native-less-than-predicate-parity",
            ValidationPerturbationClass::PredicateLegality,
            ValidationHostileExpectation::EquivalentToControl,
            ValidationParityAnchor::Control,
            to_bundle(CertificationProfile::DirectConstruction, &less_than_control),
            to_bundle(CertificationProfile::BuilderReordering, &less_than_hostile),
            to_bundle(
                CertificationProfile::ReplayParity,
                &validate_canonical_bundle(legal_less_than_bundle(), detail_schema_view()).unwrap(),
            ),
        ),
        row(
            "redundant-greater-than-normalization",
            ValidationPerturbationClass::PredicateLegality,
            ValidationHostileExpectation::EquivalentToControl,
            ValidationParityAnchor::Control,
            to_bundle(
                CertificationProfile::DirectConstruction,
                &validate_canonical_bundle(strongest_greater_than_bundle(), detail_schema_view())
                    .unwrap(),
            ),
            to_bundle(
                CertificationProfile::BuilderReordering,
                &normalized_greater_than_hostile,
            ),
            to_bundle(
                CertificationProfile::ReplayParity,
                &validate_canonical_bundle(strongest_greater_than_bundle(), detail_schema_view())
                    .unwrap(),
            ),
        ),
        row(
            "bounded-range-normalization",
            ValidationPerturbationClass::PredicateLegality,
            ValidationHostileExpectation::EquivalentToControl,
            ValidationParityAnchor::Control,
            to_bundle(
                CertificationProfile::DirectConstruction,
                &bounded_range_control,
            ),
            to_bundle(
                CertificationProfile::BuilderReordering,
                &bounded_range_hostile,
            ),
            to_bundle(
                CertificationProfile::ReplayParity,
                &validate_canonical_bundle(bounded_range_bundle(), detail_schema_view()).unwrap(),
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
                &validate_canonical_bundle(legal_contains_bundle(), detail_schema_view()).unwrap(),
            ),
        ),
        row(
            "scalar-membership-predicate-parity",
            ValidationPerturbationClass::PredicateLegality,
            ValidationHostileExpectation::EquivalentToControl,
            ValidationParityAnchor::Control,
            to_bundle(
                CertificationProfile::DirectConstruction,
                &membership_control,
            ),
            to_bundle(CertificationProfile::BuilderReordering, &membership_hostile),
            to_bundle(
                CertificationProfile::ReplayParity,
                &validate_canonical_bundle(legal_membership_bundle(), detail_schema_view())
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
                &validate_canonical_bundle(intersected_membership_bundle(), detail_schema_view())
                    .unwrap(),
            ),
            to_bundle(
                CertificationProfile::BuilderReordering,
                &validate_canonical_bundle(overlapping_membership_bundle(), detail_schema_view())
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
                &validate_canonical_bundle(legal_presence_bundle(), detail_schema_view()).unwrap(),
            ),
        ),
        row(
            "legal-structured-content-query-parity",
            ValidationPerturbationClass::StructuredContentLegality,
            ValidationHostileExpectation::EquivalentToControl,
            ValidationParityAnchor::Control,
            to_bundle(
                CertificationProfile::DirectConstruction,
                &structured_content_control,
            ),
            to_bundle(
                CertificationProfile::BuilderReordering,
                &structured_content_hostile,
            ),
            to_bundle(
                CertificationProfile::ReplayParity,
                &crate::harness::fixtures::validated_bundles::structured_content_bundle(),
            ),
        ),
        row(
            "legal-workflow-predicate-parity",
            ValidationPerturbationClass::WorkflowContextLegality,
            ValidationHostileExpectation::EquivalentToControl,
            ValidationParityAnchor::Control,
            to_bundle(CertificationProfile::DirectConstruction, &workflow_control),
            to_bundle(CertificationProfile::BuilderReordering, &workflow_hostile),
            to_bundle(
                CertificationProfile::ReplayParity,
                &crate::harness::fixtures::validated_bundles::workflow_bundle(),
            ),
        ),
    ]
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
    ValidationCertificationRow {
        row_name,
        perturbation_class,
        hostile_expectation,
        parity_anchor,
        control_lane,
        hostile_lane,
        parity_lane,
    }
}
