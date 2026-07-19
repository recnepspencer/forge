use crate::authoring::{RawAuthoredQuery, RawAuthoredResultShape};

use super::errors::QueryCanonicalizationError;

pub(super) fn enforce_admitted_authored_boundary(
    query: &RawAuthoredQuery,
    result_shape: &RawAuthoredResultShape,
) -> Result<(), QueryCanonicalizationError> {
    if query.root().as_str().trim().is_empty() {
        return Err(QueryCanonicalizationError::EmptyRootEntityKey);
    }

    for projection in query.projection() {
        let source = projection.source_field_key();
        if source.aspect().as_str().trim().is_empty() || source.field().as_str().trim().is_empty() {
            return Err(QueryCanonicalizationError::EmptyProjectionSelector);
        }
    }
    if query.projection().is_empty() {
        return Err(QueryCanonicalizationError::EmptyProjectionSet);
    }

    for predicate in query.predicates() {
        let target = predicate.target_field_key();
        if target.aspect().as_str().trim().is_empty() || target.field().as_str().trim().is_empty() {
            return Err(QueryCanonicalizationError::EmptyProjectionSelector);
        }
    }

    for ordering in query.ordering() {
        let source = ordering.source_field_key();
        if source.aspect().as_str().trim().is_empty() || source.field().as_str().trim().is_empty() {
            return Err(QueryCanonicalizationError::EmptyOrderingSelector);
        }
    }

    for traversal in query.traversal() {
        if traversal
            .terminal_relation_projection_for_boundary()
            .trim()
            .is_empty()
        {
            return Err(QueryCanonicalizationError::EmptyTraversalRelation);
        }
        if traversal.depth() == 0 {
            return Err(QueryCanonicalizationError::UnsupportedTraversalDepth {
                relation: traversal
                    .terminal_relation_projection_for_boundary()
                    .to_string(),
                depth: traversal.depth(),
            });
        }
    }

    for field in result_shape.fields() {
        let source = field.source_field_key();
        if source.aspect().as_str().trim().is_empty() || source.field().as_str().trim().is_empty() {
            return Err(QueryCanonicalizationError::EmptyResultFieldSource);
        }
        if field.delivered_name().trim().is_empty() {
            return Err(QueryCanonicalizationError::EmptyDeliveredFieldName);
        }
    }
    if result_shape.fields().is_empty() {
        return Err(QueryCanonicalizationError::EmptyResultShapeFieldSet);
    }

    Ok(())
}
