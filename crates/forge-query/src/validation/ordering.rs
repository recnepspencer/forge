use crate::canonicalization::{CanonicalOrderingEntry, CanonicalProjectionEntry};
use crate::schema_view::{QuerySchemaView, SchemaFieldKind};

use super::{
    failure::ValidationFailureArtifact, QueryValidationCounters, QueryValidationError,
    ValidatedOrderingEntry, ValidationEvent, ValidationRejectionMatrix,
};

pub(crate) fn validate_ordering_entries(
    ordering: &[CanonicalOrderingEntry],
    projections: &[CanonicalProjectionEntry],
    schema_view: &QuerySchemaView,
    counters: &mut QueryValidationCounters,
    rejection_matrix: &mut ValidationRejectionMatrix,
) -> Result<(Vec<ValidatedOrderingEntry>, Vec<ValidationEvent>), ValidationFailureArtifact> {
    let mut validated_ordering = Vec::new();
    let mut events = Vec::new();

    for entry in ordering {
        counters.record_schema_lookup();
        let Some(field) = schema_view.field(entry.aspect.as_str(), entry.field.as_str()) else {
            counters.record_rejection();
            rejection_matrix.record_ordering_rejection();
            return Err(ValidationFailureArtifact::new(
                QueryValidationError::UnknownOrderingField {
                    aspect: entry.aspect.to_string(),
                    field: entry.field.to_string(),
                },
                counters.clone(),
                rejection_matrix.clone(),
            ));
        };

        if matches!(field.kind(), SchemaFieldKind::StructuredContent) {
            counters.record_rejection();
            rejection_matrix.record_ordering_rejection();
            return Err(ValidationFailureArtifact::new(
                QueryValidationError::UnsupportedStructuredContentOrdering {
                    aspect: entry.aspect.to_string(),
                    field: entry.field.to_string(),
                    direction: direction_name(entry),
                },
                counters.clone(),
                rejection_matrix.clone(),
            ));
        }

        if !field.is_orderable() {
            counters.record_rejection();
            rejection_matrix.record_ordering_rejection();
            return Err(ValidationFailureArtifact::new(
                QueryValidationError::NonOrderableField {
                    aspect: entry.aspect.to_string(),
                    field: entry.field.to_string(),
                },
                counters.clone(),
                rejection_matrix.clone(),
            ));
        }

        let projected = projections
            .iter()
            .any(|projection| projection.aspect == entry.aspect && projection.field == entry.field);

        counters.record_ordering_validated();
        validated_ordering.push(ValidatedOrderingEntry::from_canonical(
            entry,
            field.kind().clone(),
            projected,
        ));
        events.push(ValidationEvent::OrderingValidated {
            aspect: entry.aspect.to_string(),
            field: entry.field.to_string(),
            direction: direction_name(entry),
            field_kind: format!("{:?}", field.kind()),
            projected,
        });
    }

    Ok((validated_ordering, events))
}

fn direction_name(entry: &CanonicalOrderingEntry) -> &'static str {
    match entry.direction {
        crate::authoring::OrderingDirection::Ascending => "ascending",
        crate::authoring::OrderingDirection::Descending => "descending",
    }
}
