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

        if !schema_view.has_aspect(&projection.aspect) {
            counters.record_rejection();
            counters.record_projection_widening_denial();
            rejection_matrix.record_projection_rejection();
            return Err(ValidationFailureArtifact::new(
                QueryValidationError::UnknownAspect {
                    aspect: projection.aspect.clone(),
                },
                counters.clone(),
                rejection_matrix.clone(),
            ));
        }

        let Some(field) = schema_view.field(&projection.aspect, &projection.field) else {
            counters.record_rejection();
            counters.record_projection_widening_denial();
            rejection_matrix.record_projection_rejection();
            return Err(ValidationFailureArtifact::new(
                QueryValidationError::ProjectionWideningDenied {
                    aspect: projection.aspect.clone(),
                    field: projection.field.clone(),
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
                    aspect: projection.aspect.clone(),
                    field: projection.field.clone(),
                }
            } else {
                QueryValidationError::NonQueryableField {
                    aspect: projection.aspect.clone(),
                    field: projection.field.clone(),
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
            aspect: projection.aspect.clone(),
            field: projection.field.clone(),
            field_kind: format!("{:?}", field.kind()),
        });
    }

    Ok((validated_projection, events))
}
