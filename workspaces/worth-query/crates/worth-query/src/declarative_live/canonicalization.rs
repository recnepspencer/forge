use std::collections::BTreeSet;

use crate::authoring::{
    AspectFieldKey, AspectFieldSelector, AuthoredResultShapeField, EqualityPredicate,
    GuidedAuthoringPath, NativeComparisonOperator, NativeComparisonPredicate, OrderingDirection,
    OrderingSelector, PresencePredicate, RawAuthoredQuery, RawAuthoredResultShape, RootEntityKey,
    SetMembershipPredicate, StringContainsPredicate,
};
use crate::canonicalization::CanonicalQueryBundle;
use crate::schema_view::QuerySchemaView;

use super::predicates::DeclarativePredicateFilter;
use super::request::{
    DeclarativeLiveQueryRequest, DeclarativeOrderingField, DeclarativeProjectionField,
};
use super::DeclarativeLiveQueryError;

pub(crate) fn canonicalize_declarative_request(
    request: &DeclarativeLiveQueryRequest,
) -> Result<CanonicalQueryBundle, DeclarativeLiveQueryError> {
    let root = RootEntityKey::new(request.target())
        .map_err(|_| DeclarativeLiveQueryError::InvalidTarget)?;
    let query_projection = normalized_query_projection(request);
    let result_fields = normalized_result_fields(request, &query_projection);

    if request.view_shape().collection_backed() {
        let ordering = normalized_ordering(request);
        let mut query = RawAuthoredQuery::collection_builder(root);
        for field in &query_projection {
            query = query.project(AspectFieldSelector::from_source_field_key(
                field.source_field_key().clone(),
            ));
        }
        for filter in request.predicate_filters() {
            query = apply_declarative_predicate_filter(query, filter)?;
        }
        for traversal in request.traversal() {
            query = query.traverse(traversal.clone());
        }
        for ordering in &ordering {
            query = apply_declarative_ordering(query, ordering)?;
        }

        let mut shape = RawAuthoredResultShape::collection_builder();
        for field in &result_fields {
            shape = shape.field(
                AuthoredResultShapeField::from_source_field_key(
                    field.source_field_key().clone(),
                    field.delivered_name(),
                )
                .map_err(|error| DeclarativeLiveQueryError::Authoring(format!("{error:?}")))?,
            );
        }
        GuidedAuthoringPath::canonicalize_collection(
            query
                .build()
                .map_err(|error| DeclarativeLiveQueryError::Authoring(format!("{error:?}")))?,
            shape
                .build()
                .map_err(|error| DeclarativeLiveQueryError::Authoring(format!("{error:?}")))?,
        )
        .map_err(|error| DeclarativeLiveQueryError::Canonicalization(format!("{error:?}")))
    } else {
        let mut query = RawAuthoredQuery::detail_builder(root);
        for field in &query_projection {
            query = query.project(AspectFieldSelector::from_source_field_key(
                field.source_field_key().clone(),
            ));
        }
        for filter in request.predicate_filters() {
            query = apply_declarative_predicate_filter(query, filter)?;
        }
        for traversal in request.traversal() {
            query = query.traverse(traversal.clone());
        }

        let mut shape = RawAuthoredResultShape::detail_builder();
        for field in &result_fields {
            shape = shape.field(
                AuthoredResultShapeField::from_source_field_key(
                    field.source_field_key().clone(),
                    field.delivered_name(),
                )
                .map_err(|error| DeclarativeLiveQueryError::Authoring(format!("{error:?}")))?,
            );
        }
        GuidedAuthoringPath::canonicalize_detail(
            query
                .build()
                .map_err(|error| DeclarativeLiveQueryError::Authoring(format!("{error:?}")))?,
            shape
                .build()
                .map_err(|error| DeclarativeLiveQueryError::Authoring(format!("{error:?}")))?,
        )
        .map_err(|error| DeclarativeLiveQueryError::Canonicalization(format!("{error:?}")))
    }
}

pub(super) fn normalized_query_projection(
    request: &DeclarativeLiveQueryRequest,
) -> Vec<DeclarativeProjectionField> {
    let mut fields = request.query_projection().to_vec();
    if fields.is_empty() {
        fields.push(DeclarativeProjectionField::from_authoring_parts(
            "identity", "id",
        ));
        for filter in request.predicate_filters() {
            push_unique_field(&mut fields, declarative_field_from_predicate(filter));
        }
    }
    if request.view_shape().collection_backed() {
        let ordering = normalized_ordering(request);
        for field in ordering {
            push_unique_field(
                &mut fields,
                DeclarativeProjectionField::new(field.source_field_key().clone()),
            );
        }
    }
    fields
}

pub(super) fn normalized_result_fields(
    request: &DeclarativeLiveQueryRequest,
    query_projection: &[DeclarativeProjectionField],
) -> Vec<DeclarativeProjectionField> {
    if request.result_fields().is_empty() {
        query_projection.to_vec()
    } else {
        request.result_fields().to_vec()
    }
}

fn normalized_ordering(request: &DeclarativeLiveQueryRequest) -> Vec<DeclarativeOrderingField> {
    if request.ordering().is_empty() && request.view_shape().collection_backed() {
        vec![DeclarativeOrderingField::ascending(
            AspectFieldKey::from_authoring_parts("identity", "id")
                .expect("default collection ordering requires identity.id"),
        )]
    } else {
        request.ordering().to_vec()
    }
}

fn declarative_field_from_predicate(
    filter: &DeclarativePredicateFilter,
) -> DeclarativeProjectionField {
    DeclarativeProjectionField::new(filter.source_field_key().clone())
}

pub(crate) fn validate_declared_traversal_contract(
    request: &DeclarativeLiveQueryRequest,
    schema_view: &QuerySchemaView,
) -> Result<(), DeclarativeLiveQueryError> {
    let mut seen = BTreeSet::new();
    for traversal in request.traversal() {
        let relation = traversal
            .terminal_relation_projection_for_boundary()
            .to_string();
        let depth = traversal.depth();
        if !seen.insert((relation.clone(), depth)) {
            return Err(DeclarativeLiveQueryError::DuplicateTraversal { relation, depth });
        }
        let Some(schema_relation) = schema_view.relation(traversal.relation_name()) else {
            return Err(DeclarativeLiveQueryError::TraversalNotDeclaredInSchema {
                relation,
                requested_depth: depth,
            });
        };
        if depth > schema_relation.max_depth() {
            return Err(DeclarativeLiveQueryError::TraversalExceedsSchemaDepth {
                relation,
                requested_depth: depth,
                max_depth: schema_relation.max_depth(),
            });
        }
    }
    Ok(())
}

fn push_unique_field(
    fields: &mut Vec<DeclarativeProjectionField>,
    candidate: DeclarativeProjectionField,
) {
    if !fields
        .iter()
        .any(|field| field.source_field_key() == candidate.source_field_key())
    {
        fields.push(candidate);
    }
}

fn apply_declarative_predicate_filter<F: crate::authoring::QueryAuthoringFamily>(
    mut query: crate::authoring::QueryBuilder<F>,
    filter: &DeclarativePredicateFilter,
) -> Result<crate::authoring::QueryBuilder<F>, DeclarativeLiveQueryError> {
    query = match filter {
        DeclarativePredicateFilter::Equality(filter) => {
            query.where_equal(EqualityPredicate::from_target_field_key(
                filter.source_field_key().clone(),
                filter.value().clone(),
            ))
        }
        DeclarativePredicateFilter::NativeComparison(filter) => match filter.operator() {
            NativeComparisonOperator::GreaterThan => query.where_greater_than(
                NativeComparisonPredicate::greater_than_native_target_field_key(
                    filter.source_field_key().clone(),
                    filter.value().clone(),
                ),
            ),
            NativeComparisonOperator::LessThan => query.where_less_than(
                NativeComparisonPredicate::less_than_native_target_field_key(
                    filter.source_field_key().clone(),
                    filter.value().clone(),
                ),
            ),
        },
        DeclarativePredicateFilter::StringContains(filter) => {
            query.where_contains(StringContainsPredicate::from_target_field_key(
                filter.source_field_key().clone(),
                filter.value(),
            ))
        }
        DeclarativePredicateFilter::SetMembership(filter) => query.where_in(
            SetMembershipPredicate::from_target_field_key(
                filter.source_field_key().clone(),
                filter.values().iter().cloned(),
            )
            .map_err(|error| DeclarativeLiveQueryError::Authoring(format!("{error:?}")))?,
        ),
        DeclarativePredicateFilter::Presence(filter) => query.where_present(
            PresencePredicate::is_present_target_field_key(filter.source_field_key().clone()),
        ),
    };
    Ok(query)
}

fn apply_declarative_ordering<F: crate::authoring::QueryAuthoringFamily>(
    query: crate::authoring::QueryBuilder<F>,
    ordering: &DeclarativeOrderingField,
) -> Result<crate::authoring::QueryBuilder<F>, DeclarativeLiveQueryError> {
    let selector = match ordering.direction() {
        OrderingDirection::Ascending => {
            OrderingSelector::ascending_source_field_key(ordering.source_field_key().clone())
        }
        OrderingDirection::Descending => {
            OrderingSelector::descending_source_field_key(ordering.source_field_key().clone())
        }
    };
    Ok(query.order_by(selector))
}
