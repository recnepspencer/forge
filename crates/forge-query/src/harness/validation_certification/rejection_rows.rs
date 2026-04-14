use crate::harness::fixtures::schema_view::{detail_schema_view, legal_detail_bundle};
use crate::validation::validate_canonical_bundle;

use super::super::validation_matrix::{
    ValidationPerturbationClass, ValidationRejectionCertificationRow,
};
use super::bundles::rejection_row;
use super::fixtures::*;

pub(super) fn rejection_rows() -> Vec<ValidationRejectionCertificationRow> {
    let legal_control =
        validate_canonical_bundle(legal_detail_bundle(), detail_schema_view()).unwrap();
    let greater_than_control =
        validate_canonical_bundle(legal_greater_than_bundle(), detail_schema_view()).unwrap();
    let bounded_range_control =
        validate_canonical_bundle(bounded_range_bundle(), detail_schema_view()).unwrap();
    let contains_control =
        validate_canonical_bundle(legal_contains_bundle(), detail_schema_view()).unwrap();
    let membership_control =
        validate_canonical_bundle(legal_membership_bundle(), detail_schema_view()).unwrap();
    let presence_control =
        validate_canonical_bundle(legal_presence_bundle(), detail_schema_view()).unwrap();

    vec![
        rejection_row(
            "unknown-aspect-projection",
            ValidationPerturbationClass::ProjectionLegality,
            &legal_control,
            unknown_aspect_bundle(),
        ),
        rejection_row(
            "illegal-traversal-edge-or-depth",
            ValidationPerturbationClass::TraversalLegality,
            &legal_control,
            illegal_traversal_bundle(),
        ),
        rejection_row(
            "non-orderable-ordering-field",
            ValidationPerturbationClass::OrderingLegality,
            &legal_control,
            non_orderable_ordering_bundle(),
        ),
        rejection_row(
            "predicate-contradiction-rejection",
            ValidationPerturbationClass::PredicateLegality,
            &greater_than_control,
            contradictory_predicate_bundle(),
        ),
        rejection_row(
            "empty-range-rejection",
            ValidationPerturbationClass::PredicateLegality,
            &bounded_range_control,
            empty_range_bundle(),
        ),
        rejection_row(
            "text-predicate-capability-rejection",
            ValidationPerturbationClass::PredicateLegality,
            &contains_control,
            text_capability_illegal_bundle(),
        ),
        rejection_row(
            "membership-capability-rejection",
            ValidationPerturbationClass::PredicateLegality,
            &membership_control,
            membership_capability_illegal_bundle(),
        ),
        rejection_row(
            "presence-capability-rejection",
            ValidationPerturbationClass::PredicateLegality,
            &presence_control,
            presence_capability_illegal_bundle(),
        ),
        rejection_row(
            "incompatible-predicate-family",
            ValidationPerturbationClass::PredicateLegality,
            &legal_control,
            incompatible_predicate_bundle(),
        ),
        rejection_row(
            "invalid-result-shape-binding",
            ValidationPerturbationClass::ResultShapeBindingLegality,
            &legal_control,
            invalid_result_shape_binding_bundle(),
        ),
        rejection_row(
            "structured-content-illegality",
            ValidationPerturbationClass::StructuredContentLegality,
            &legal_control,
            structured_content_illegal_bundle(),
        ),
        rejection_row(
            "workflow-context-illegality",
            ValidationPerturbationClass::WorkflowContextLegality,
            &legal_control,
            workflow_context_illegal_bundle(),
        ),
        rejection_row(
            "forbidden-widening-case",
            ValidationPerturbationClass::ForbiddenWidening,
            &legal_control,
            forbidden_widening_bundle(),
        ),
    ]
}
