use crate::canonicalization::CanonicalProjectionEntry;
use crate::schema_view::{QuerySchemaView, SchemaFieldKind};

use super::{
    failure::ValidationFailureArtifact, QueryValidationCounters, QueryValidationError,
    ValidatedProjectionEntry, ValidationEvent, ValidationRejectionMatrix,
};

pub(crate) fn validate_projection_entries(
    projections: &[CanonicalProjectionEntry],
    schema_view: &QuerySchemaView,
    counters: &mut QueryValidationCounters,
    rejection_matrix: &mut ValidationRejectionMatrix,
) -> Result<(Vec<ValidatedProjectionEntry>, Vec<ValidationEvent>), ValidationFailureArtifact> {
    let mut validated_projection = Vec::new();
    let mut events = Vec::new();

    for projection in projections {
        counters.record_schema_lookup();
        let key = projection.field_key();

        if !schema_view.has_aspect(key.aspect()) {
            counters.record_rejection();
            counters.record_projection_widening_denial();
            rejection_matrix.record_projection_rejection();
            return Err(ValidationFailureArtifact::new(
                QueryValidationError::UnknownAspect {
                    aspect: key.aspect().to_string(),
                },
                counters.clone(),
                rejection_matrix.clone(),
            ));
        }

        let Some(field) = schema_view.field(key.aspect(), key.field()) else {
            counters.record_rejection();
            counters.record_projection_widening_denial();
            rejection_matrix.record_projection_rejection();
            return Err(ValidationFailureArtifact::new(
                QueryValidationError::ProjectionWideningDenied {
                    aspect: key.aspect().to_string(),
                    field: key.field().to_string(),
                },
                counters.clone(),
                rejection_matrix.clone(),
            ));
        };

        if !field.is_queryable() {
            counters.record_rejection();
            counters.record_projection_widening_denial();
            rejection_matrix.record_projection_rejection();
            let error = if matches!(field.kind(), SchemaFieldKind::StructuredContent) {
                QueryValidationError::UnsupportedStructuredContentProjection {
                    aspect: key.aspect().to_string(),
                    field: key.field().to_string(),
                }
            } else {
                QueryValidationError::NonQueryableField {
                    aspect: key.aspect().to_string(),
                    field: key.field().to_string(),
                }
            };
            return Err(ValidationFailureArtifact::new(
                error,
                counters.clone(),
                rejection_matrix.clone(),
            ));
        }

        counters.record_projection_validated();
        validated_projection.push(ValidatedProjectionEntry::from_canonical(
            projection,
            field.kind().clone(),
        ));
        events.push(ValidationEvent::ProjectionValidated {
            aspect: key.aspect().to_string(),
            field: key.field().to_string(),
            field_kind: format!("{:?}", field.kind()),
        });
    }

    Ok((validated_projection, events))
}
