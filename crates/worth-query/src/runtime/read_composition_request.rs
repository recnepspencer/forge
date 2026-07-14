use crate::authoring::{
    IntegerComparisonOperator, PredicateSelector, RawAuthoredQuery, RawAuthoredResultShape,
};
use crate::declarative_live::{
    DeclarativeEqualityFilter, DeclarativeIntegerComparisonFilter, DeclarativeLiveQueryError,
    DeclarativeLiveQueryRequest, DeclarativeLiveViewShape, DeclarativeOrderingField,
    DeclarativePresenceFilter, DeclarativeProjectionField, DeclarativeSetMembershipFilter,
    DeclarativeStringContainsFilter,
};

pub(in crate::runtime) fn declarative_request_from_authored_shape(
    query: RawAuthoredQuery,
    result_shape: RawAuthoredResultShape,
) -> Result<DeclarativeLiveQueryRequest, DeclarativeLiveQueryError> {
    let view_shape = match query.family() {
        crate::authoring::QueryFamily::Detail => DeclarativeLiveViewShape::detail(),
        crate::authoring::QueryFamily::Collection => DeclarativeLiveViewShape::table(),
    };
    let mut request = DeclarativeLiveQueryRequest::new(query.root().as_str(), view_shape);
    for field in query.projection() {
        let delivered_name = result_shape
            .fields()
            .iter()
            .find(|result_field| result_field.source_field_key() == field.source_field_key())
            .map(|result_field| result_field.delivered_name())
            .unwrap_or_else(|| field.source_field_key().field().as_str());
        request = request.project_query_only(
            DeclarativeProjectionField::new(field.source_field_key().clone())
                .delivered_as(delivered_name),
        );
    }
    for field in result_shape.fields() {
        request = request.result_field(
            DeclarativeProjectionField::new(field.source_field_key().clone())
                .delivered_as(field.delivered_name()),
        );
    }
    for predicate in query.predicates() {
        request = add_predicate(request, predicate);
    }
    for traversal in query.traversal() {
        request = request.traverse(traversal.clone());
    }
    for ordering in query.ordering() {
        let ordering = match ordering.direction() {
            crate::authoring::OrderingDirection::Ascending => {
                DeclarativeOrderingField::ascending(ordering.source_field_key().clone())
            }
            crate::authoring::OrderingDirection::Descending => {
                DeclarativeOrderingField::descending(ordering.source_field_key().clone())
            }
        };
        request = request.order_by_direction(ordering);
    }
    Ok(request)
}

fn add_predicate(
    request: DeclarativeLiveQueryRequest,
    predicate: &PredicateSelector,
) -> DeclarativeLiveQueryRequest {
    match predicate {
        PredicateSelector::Equality(predicate) => {
            request.where_equal(DeclarativeEqualityFilter::new(
                predicate.target_field_key().clone(),
                predicate.value().clone(),
            ))
        }
        PredicateSelector::IntegerComparison(predicate) => match predicate.operator() {
            IntegerComparisonOperator::GreaterThan => {
                request.where_greater_than(DeclarativeIntegerComparisonFilter::greater_than(
                    predicate.target_field_key().clone(),
                    predicate.value(),
                ))
            }
            IntegerComparisonOperator::LessThan => {
                request.where_less_than(DeclarativeIntegerComparisonFilter::less_than(
                    predicate.target_field_key().clone(),
                    predicate.value(),
                ))
            }
        },
        PredicateSelector::StringContains(predicate) => {
            request.where_contains(DeclarativeStringContainsFilter::new(
                predicate.target_field_key().clone(),
                predicate.value(),
            ))
        }
        PredicateSelector::SetMembership(predicate) => {
            request.where_in(DeclarativeSetMembershipFilter::new(
                predicate.target_field_key().clone(),
                predicate.values().iter().cloned(),
            ))
        }
        PredicateSelector::Presence(predicate) => request.where_present(
            DeclarativePresenceFilter::is_present(predicate.target_field_key().clone()),
        ),
    }
}
