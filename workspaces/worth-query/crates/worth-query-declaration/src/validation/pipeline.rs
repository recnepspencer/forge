use crate::canonicalization::CanonicalQueryBundle;
use crate::schema_view::QuerySchemaView;

use super::artifacts::{
    build_validated_query_artifact, build_validated_result_shape_artifact, ValidatedOrderingSet,
    ValidatedPredicateSet, ValidatedQueryBundle,
};
use super::failure::ValidationFailureArtifact;
use super::ordering::validate_ordering_entries;
use super::predicates::validate_predicate_entries;
use super::projection::validate_projection_entries;
use super::result_shape::validate_result_shape_bindings;
use super::traversal::validate_traversal_entries;
use super::{
    QueryValidationCounters, QueryValidationError, QueryValidationReport, ValidationEvent,
    ValidationRejectionMatrix,
};

pub fn validate_canonical_bundle(
    bundle: CanonicalQueryBundle,
    schema_view: QuerySchemaView,
) -> Result<ValidatedQueryBundle, QueryValidationError> {
    validate_canonical_bundle_with_failure_artifact(bundle, schema_view)
        .map_err(|failure| failure.error)
}

pub fn validate_canonical_bundle_with_failure_artifact(
    bundle: CanonicalQueryBundle,
    schema_view: QuerySchemaView,
) -> Result<ValidatedQueryBundle, ValidationFailureArtifact> {
    let mut counters = QueryValidationCounters::default();
    let mut rejection_matrix = ValidationRejectionMatrix::default();
    let mut events = Vec::new();
    let warnings = Vec::new();
    let schema_basis = schema_view.basis().clone();

    let (validated_projection, projection_events) = validate_projection_entries(
        bundle.query().projection(),
        &schema_view,
        &mut counters,
        &mut rejection_matrix,
    )?;
    events.extend(projection_events);

    let (validated_predicates, predicate_events) = validate_predicate_entries(
        bundle.query().predicates(),
        &schema_view,
        &mut counters,
        &mut rejection_matrix,
    )?;
    events.extend(predicate_events);

    let (validated_ordering, ordering_events) = validate_ordering_entries(
        bundle.query().ordering(),
        bundle.query().projection(),
        &schema_view,
        &mut counters,
        &mut rejection_matrix,
    )?;
    events.extend(ordering_events);

    let (validated_traversal, traversal_events) = validate_traversal_entries(
        bundle.query().traversal(),
        &schema_view,
        &mut counters,
        &mut rejection_matrix,
    )?;
    events.extend(traversal_events);

    let (validated_bindings, result_shape_events) = validate_result_shape_bindings(
        bundle.result_shape().fields(),
        bundle.query().projection(),
        &schema_view,
        &mut counters,
        &mut rejection_matrix,
    )?;
    events.extend(result_shape_events);

    let query = build_validated_query_artifact(
        bundle.query(),
        &schema_view,
        validated_projection,
        validated_traversal,
        ValidatedPredicateSet::from_entries(validated_predicates),
        ValidatedOrderingSet::from_entries(validated_ordering),
    );
    let result_shape = build_validated_result_shape_artifact(
        bundle.result_shape(),
        &schema_basis,
        validated_bindings,
    );

    events.push(ValidationEvent::CompatibilityEstablished);
    events.push(ValidationEvent::IdentityFrozen {
        query_digest: query.digest().as_str().to_string(),
        result_shape_digest: result_shape.digest().as_str().to_string(),
    });

    let report = QueryValidationReport::new(
        schema_basis.clone(),
        counters.validated_projection_entry_count(),
        counters.validated_traversal_clause_count(),
        counters.validated_result_shape_binding_count(),
        counters.validated_predicate_count(),
        counters.validated_ordering_field_count(),
        events,
        warnings,
        rejection_matrix,
    );

    let validated = ValidatedQueryBundle::new(query, result_shape, report, counters);
    validated.check_invariants().map_err(|error| {
        ValidationFailureArtifact::new(
            error,
            validated.counters().clone(),
            validated.report().rejection_matrix().clone(),
        )
    })?;
    Ok(validated)
}
