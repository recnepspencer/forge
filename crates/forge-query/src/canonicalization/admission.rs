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
        if projection.aspect().trim().is_empty() || projection.field().trim().is_empty() {
            return Err(QueryCanonicalizationError::EmptyProjectionSelector);
        }
    }
    if query.projection().is_empty() {
        return Err(QueryCanonicalizationError::EmptyProjectionSet);
    }

    for predicate in query.predicates() {
        if predicate.aspect().trim().is_empty() || predicate.field().trim().is_empty() {
            return Err(QueryCanonicalizationError::EmptyProjectionSelector);
        }
    }

    for ordering in query.ordering() {
        if ordering.aspect().trim().is_empty() || ordering.field().trim().is_empty() {
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
        if field.source_aspect().trim().is_empty() || field.source_field().trim().is_empty() {
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
