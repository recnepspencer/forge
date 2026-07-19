use crate::canonicalization::CanonicalTraversalEntry;
use crate::schema_view::QuerySchemaView;

use super::{
    failure::ValidationFailureArtifact, QueryValidationCounters, QueryValidationError,
    ValidatedTraversalEntry, ValidationEvent, ValidationRejectionMatrix,
};

pub fn validate_traversal_entries(
    traversals: &[CanonicalTraversalEntry],
    schema_view: &QuerySchemaView,
    counters: &mut QueryValidationCounters,
    rejection_matrix: &mut ValidationRejectionMatrix,
) -> Result<(Vec<ValidatedTraversalEntry>, Vec<ValidationEvent>), ValidationFailureArtifact> {
    let mut validated_traversal = Vec::new();
    let mut events = Vec::new();

    for traversal in traversals {
        counters.record_schema_lookup();
        let Some(relation) = schema_view.relation(&traversal.relation) else {
            counters.record_rejection();
            rejection_matrix.record_traversal_rejection();
            return Err(ValidationFailureArtifact::new(
                QueryValidationError::IllegalTraversalRelation {
                    relation: traversal.relation.to_string(),
                },
                counters.clone(),
                rejection_matrix.clone(),
            ));
        };

        if traversal.depth > relation.max_depth() {
            counters.record_rejection();
            rejection_matrix.record_traversal_rejection();
            return Err(ValidationFailureArtifact::new(
                QueryValidationError::IllegalTraversalDepth {
                    relation: traversal.relation.to_string(),
                    requested_depth: traversal.depth,
                    max_depth: relation.max_depth(),
                },
                counters.clone(),
                rejection_matrix.clone(),
            ));
        }

        counters.record_traversal_validated();
        validated_traversal.push(ValidatedTraversalEntry::from_canonical(
            traversal,
            relation.max_depth(),
        ));
        events.push(ValidationEvent::TraversalValidated {
            relation: traversal.relation.to_string(),
            depth: traversal.depth,
            max_depth: relation.max_depth(),
        });
    }

    Ok((validated_traversal, events))
}
