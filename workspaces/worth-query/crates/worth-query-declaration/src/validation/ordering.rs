use crate::canonicalization::{CanonicalOrderingEntry, CanonicalProjectionEntry};
use crate::schema_view::QuerySchemaView;

use super::{
    failure::ValidationFailureArtifact, QueryValidationCounters, QueryValidationError,
    ValidatedOrderingEntry, ValidationEvent, ValidationRejectionMatrix,
};

pub fn validate_ordering_entries(
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
        let key = entry.field_key();
        let Some(field) = schema_view.field(key.aspect(), key.field()) else {
            counters.record_rejection();
            rejection_matrix.record_ordering_rejection();
            return Err(ValidationFailureArtifact::new(
                QueryValidationError::UnknownOrderingField {
                    aspect: key.aspect().to_string(),
                    field: key.field().to_string(),
                },
                counters.clone(),
                rejection_matrix.clone(),
            ));
        };

        if !field.is_orderable() {
            counters.record_rejection();
            rejection_matrix.record_ordering_rejection();
            return Err(ValidationFailureArtifact::new(
                QueryValidationError::NonOrderableField {
                    aspect: key.aspect().to_string(),
                    field: key.field().to_string(),
                },
                counters.clone(),
                rejection_matrix.clone(),
            ));
        }

        let projected = projections
            .iter()
            .any(|projection| projection.field_key() == entry.field_key());

        counters.record_ordering_validated();
        validated_ordering.push(ValidatedOrderingEntry::from_canonical(
            entry,
            field.kind().clone(),
            projected,
        ));
        events.push(ValidationEvent::OrderingValidated {
            aspect: key.aspect().to_string(),
            field: key.field().to_string(),
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
