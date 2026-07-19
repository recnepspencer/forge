use crate::canonicalization::{CanonicalProjectionEntry, CanonicalResultField};
use crate::schema_view::QuerySchemaView;

use super::{
    failure::ValidationFailureArtifact, QueryValidationCounters, QueryValidationError,
    ValidatedResultShapeBinding, ValidationEvent, ValidationRejectionMatrix,
};

pub fn validate_result_shape_bindings(
    fields: &[CanonicalResultField],
    projections: &[CanonicalProjectionEntry],
    schema_view: &QuerySchemaView,
    counters: &mut QueryValidationCounters,
    rejection_matrix: &mut ValidationRejectionMatrix,
) -> Result<(Vec<ValidatedResultShapeBinding>, Vec<ValidationEvent>), ValidationFailureArtifact> {
    let mut validated_bindings = Vec::new();
    let mut events = Vec::new();

    for field in fields {
        counters.record_schema_lookup();
        let source = field.source_field_key();
        let Some(schema_field) = schema_view.field(source.aspect(), source.field()) else {
            counters.record_rejection();
            rejection_matrix.record_result_shape_rejection();
            return Err(ValidationFailureArtifact::new(
                QueryValidationError::IllegalResultShapeBinding {
                    aspect: source.aspect().to_string(),
                    field: source.field().to_string(),
                    delivered_name: field.delivered_name.to_string(),
                },
                counters.clone(),
                rejection_matrix.clone(),
            ));
        };

        if !schema_field.is_queryable() {
            counters.record_rejection();
            rejection_matrix.record_result_shape_rejection();
            return Err(ValidationFailureArtifact::new(
                QueryValidationError::IllegalResultShapeBinding {
                    aspect: source.aspect().to_string(),
                    field: source.field().to_string(),
                    delivered_name: field.delivered_name.to_string(),
                },
                counters.clone(),
                rejection_matrix.clone(),
            ));
        }

        if !projections
            .iter()
            .any(|projection| projection.field_key() == source)
        {
            counters.record_rejection();
            rejection_matrix.record_compatibility_rejection();
            return Err(ValidationFailureArtifact::new(
                QueryValidationError::ValidatedBundleCompatibilityFailure {
                    message: "result-shape binding escaped canonical projection surface",
                },
                counters.clone(),
                rejection_matrix.clone(),
            ));
        }

        counters.record_result_shape_binding_validated();
        validated_bindings.push(ValidatedResultShapeBinding::from_canonical(
            field,
            schema_field.kind().clone(),
        ));
        events.push(ValidationEvent::ResultShapeBindingValidated {
            aspect: source.aspect().to_string(),
            field: source.field().to_string(),
            delivered_name: field.delivered_name.to_string(),
            field_kind: format!("{:?}", schema_field.kind()),
        });
    }

    Ok((validated_bindings, events))
}
