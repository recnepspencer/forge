use crate::authoring::{
    CollectionQueryBuilder, CollectionResultShapeBuilder, DetailQueryBuilder,
    DetailResultShapeBuilder, IntegerComparisonOperator, PredicateSelector, RawAuthoredQuery,
    RawAuthoredResultShape, RelationName, RootEntityKey, TraversalSelector,
};
use crate::declarative_live::{
    canonicalize_declarative_request, validate_declared_traversal_contract,
    DeclarativeEqualityFilter, DeclarativeIntegerComparisonFilter, DeclarativeLiveQueryError,
    DeclarativeLiveQueryRequest, DeclarativeLiveViewShape, DeclarativeOrderingField,
    DeclarativePresenceFilter, DeclarativeProjectionField, DeclarativeSetMembershipFilter,
    DeclarativeStringContainsFilter,
};
use crate::ordinary::read::{WorthQueryDeclaredReadIntent, WorthQueryReadPlanningAuthority};
use crate::runtime::{
    QuerySchemaView, WorthQueryReadBuiltInOperator, WorthQueryReadDenial, WorthQueryReadDenialKind,
    WorthQueryReadGraph, WorthQueryReadGraphFamily, WorthQueryReadScopeClass,
};
use crate::validation::validate_canonical_bundle;

use super::read_composition_operator_builders::{
    CollectionReadOperatorQueryBuilder, DetailReadOperatorQueryBuilder,
};
use super::read_composition_relationship_proof::admit_read_relationship_proof;
use super::read_composition_runtime::classify_scope_shape_with_operators;

pub(in crate::runtime) fn build_collection_read_intent(
    root: impl Into<String>,
    schema_view: QuerySchemaView,
    declare_query: impl FnOnce(CollectionQueryBuilder) -> CollectionQueryBuilder,
    declare_result_shape: impl FnOnce(CollectionResultShapeBuilder) -> CollectionResultShapeBuilder,
    expected_scope_class: WorthQueryReadScopeClass,
) -> Result<WorthQueryDeclaredReadIntent, WorthQueryReadDenial> {
    let (query, result_shape) =
        build_collection_authored_inputs(root, declare_query, declare_result_shape)?;
    build_scoped_read_intent_from_authored(
        query,
        result_shape,
        schema_view,
        WorthQueryReadGraphFamily::Collection,
        expected_scope_class,
        Vec::new(),
    )
}

pub(in crate::runtime) fn build_detail_read_intent(
    root: impl Into<String>,
    schema_view: QuerySchemaView,
    declare_query: impl FnOnce(DetailQueryBuilder) -> DetailQueryBuilder,
    declare_result_shape: impl FnOnce(DetailResultShapeBuilder) -> DetailResultShapeBuilder,
    expected_scope_class: WorthQueryReadScopeClass,
) -> Result<WorthQueryDeclaredReadIntent, WorthQueryReadDenial> {
    let (query, result_shape) =
        build_detail_authored_inputs(root, declare_query, declare_result_shape)?;
    build_scoped_read_intent_from_authored(
        query,
        result_shape,
        schema_view,
        WorthQueryReadGraphFamily::Detail,
        expected_scope_class,
        Vec::new(),
    )
}

pub(in crate::runtime) fn build_direct_edge_collection_read_intent(
    root: impl Into<String>,
    schema_view: QuerySchemaView,
    relation: RelationName,
    declare_query: impl FnOnce(CollectionReadOperatorQueryBuilder) -> CollectionReadOperatorQueryBuilder,
    declare_result_shape: impl FnOnce(CollectionResultShapeBuilder) -> CollectionResultShapeBuilder,
) -> Result<WorthQueryDeclaredReadIntent, WorthQueryReadDenial> {
    let (query, result_shape) =
        build_collection_operator_authored_inputs(root, declare_query, declare_result_shape)?;
    build_scoped_read_intent_from_authored(
        query.with_traversal(traversal_selector(relation, 1)?),
        result_shape,
        schema_view,
        WorthQueryReadGraphFamily::Collection,
        WorthQueryReadScopeClass::LocalNeighborhood,
        vec![WorthQueryReadBuiltInOperator::DirectEdge],
    )
}

pub(in crate::runtime) fn build_direct_edge_detail_read_intent(
    root: impl Into<String>,
    schema_view: QuerySchemaView,
    relation: RelationName,
    declare_query: impl FnOnce(DetailReadOperatorQueryBuilder) -> DetailReadOperatorQueryBuilder,
    declare_result_shape: impl FnOnce(DetailResultShapeBuilder) -> DetailResultShapeBuilder,
) -> Result<WorthQueryDeclaredReadIntent, WorthQueryReadDenial> {
    let (query, result_shape) =
        build_detail_operator_authored_inputs(root, declare_query, declare_result_shape)?;
    build_scoped_read_intent_from_authored(
        query.with_traversal(traversal_selector(relation, 1)?),
        result_shape,
        schema_view,
        WorthQueryReadGraphFamily::Detail,
        WorthQueryReadScopeClass::LocalNeighborhood,
        vec![WorthQueryReadBuiltInOperator::DirectEdge],
    )
}

fn build_collection_authored_inputs(
    root: impl Into<String>,
    declare_query: impl FnOnce(CollectionQueryBuilder) -> CollectionQueryBuilder,
    declare_result_shape: impl FnOnce(CollectionResultShapeBuilder) -> CollectionResultShapeBuilder,
) -> Result<(RawAuthoredQuery, RawAuthoredResultShape), WorthQueryReadDenial> {
    let root = parse_root(root)?;
    let query = declare_query(CollectionQueryBuilder::new(root))
        .build()
        .map_err(authoring_denial)?
        .into_raw();
    let result_shape = declare_result_shape(CollectionResultShapeBuilder::new())
        .build()
        .map_err(authoring_denial)?
        .into_raw();
    Ok((query, result_shape))
}

fn build_detail_authored_inputs(
    root: impl Into<String>,
    declare_query: impl FnOnce(DetailQueryBuilder) -> DetailQueryBuilder,
    declare_result_shape: impl FnOnce(DetailResultShapeBuilder) -> DetailResultShapeBuilder,
) -> Result<(RawAuthoredQuery, RawAuthoredResultShape), WorthQueryReadDenial> {
    let root = parse_root(root)?;
    let query = declare_query(DetailQueryBuilder::new(root))
        .build()
        .map_err(authoring_denial)?
        .into_raw();
    let result_shape = declare_result_shape(DetailResultShapeBuilder::new())
        .build()
        .map_err(authoring_denial)?
        .into_raw();
    Ok((query, result_shape))
}

pub(super) fn build_collection_operator_authored_inputs(
    root: impl Into<String>,
    declare_query: impl FnOnce(CollectionReadOperatorQueryBuilder) -> CollectionReadOperatorQueryBuilder,
    declare_result_shape: impl FnOnce(CollectionResultShapeBuilder) -> CollectionResultShapeBuilder,
) -> Result<(RawAuthoredQuery, RawAuthoredResultShape), WorthQueryReadDenial> {
    let root = parse_root(root)?;
    let query = declare_query(CollectionReadOperatorQueryBuilder::new(
        CollectionQueryBuilder::new(root),
    ))
    .finish()
    .build()
    .map_err(authoring_denial)?
    .into_raw();
    let result_shape = declare_result_shape(CollectionResultShapeBuilder::new())
        .build()
        .map_err(authoring_denial)?
        .into_raw();
    Ok((query, result_shape))
}

pub(super) fn build_detail_operator_authored_inputs(
    root: impl Into<String>,
    declare_query: impl FnOnce(DetailReadOperatorQueryBuilder) -> DetailReadOperatorQueryBuilder,
    declare_result_shape: impl FnOnce(DetailResultShapeBuilder) -> DetailResultShapeBuilder,
) -> Result<(RawAuthoredQuery, RawAuthoredResultShape), WorthQueryReadDenial> {
    let root = parse_root(root)?;
    let query = declare_query(DetailReadOperatorQueryBuilder::new(
        DetailQueryBuilder::new(root),
    ))
    .finish()
    .build()
    .map_err(authoring_denial)?
    .into_raw();
    let result_shape = declare_result_shape(DetailResultShapeBuilder::new())
        .build()
        .map_err(authoring_denial)?
        .into_raw();
    Ok((query, result_shape))
}

pub(super) fn build_scoped_read_intent_from_authored(
    query: RawAuthoredQuery,
    result_shape: RawAuthoredResultShape,
    schema_view: QuerySchemaView,
    family: WorthQueryReadGraphFamily,
    expected_scope_class: WorthQueryReadScopeClass,
    built_in_operators: Vec<WorthQueryReadBuiltInOperator>,
) -> Result<WorthQueryDeclaredReadIntent, WorthQueryReadDenial> {
    let domain_graph_operations = query.domain_graph_operations().to_vec();
    let request =
        declarative_request_from_authored_shape(query, result_shape).map_err(declarative_denial)?;
    validate_declared_traversal_contract(&request, &schema_view).map_err(declarative_denial)?;
    let canonical = canonicalize_declarative_request(&request).map_err(declarative_denial)?;
    let schema_view_for_runtime = schema_view.clone();
    let validated =
        validate_canonical_bundle(canonical.clone(), schema_view).map_err(validation_denial)?;
    let scope_class = classify_scope_shape_with_operators(&validated, &built_in_operators);
    if scope_class != expected_scope_class {
        return Err(WorthQueryReadDenial::new_scope_shape_denied(
            expected_scope_class,
            scope_class,
        ));
    }
    Ok(WorthQueryDeclaredReadIntent::new(
        family,
        scope_class,
        validated.query().schema_basis().clone(),
        built_in_operators,
        domain_graph_operations,
        validated.query().traversal().len(),
        validated
            .query()
            .traversal()
            .iter()
            .map(|entry| usize::from(entry.depth()))
            .max()
            .unwrap_or(0),
        request,
        schema_view_for_runtime,
        canonical,
        validated,
    ))
}

pub(in crate::runtime) fn plan_standalone_read_intent(
    intent: WorthQueryDeclaredReadIntent,
) -> Result<WorthQueryReadGraph, WorthQueryReadDenial> {
    let relationship_proof_admission = admit_read_relationship_proof(
        intent.canonical().query(),
        intent.validated().query().schema_basis(),
        intent.validated().query().traversal(),
        intent.built_in_operators(),
    )?;
    intent.plan(WorthQueryReadPlanningAuthority::canonical(
        relationship_proof_admission,
    ))
}

pub(in crate::runtime) fn declarative_request_from_authored_shape(
    query: RawAuthoredQuery,
    result_shape: RawAuthoredResultShape,
) -> Result<DeclarativeLiveQueryRequest, crate::declarative_live::DeclarativeLiveQueryError> {
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
        request = match predicate {
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
        };
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

fn parse_root(root: impl Into<String>) -> Result<RootEntityKey, WorthQueryReadDenial> {
    RootEntityKey::new(root.into()).map_err(|error| {
        WorthQueryReadDenial::new(WorthQueryReadDenialKind::InvalidRoot, format!("{error:?}"))
    })
}

pub(super) fn traversal_selector(
    relation: RelationName,
    max_depth: u8,
) -> Result<TraversalSelector, WorthQueryReadDenial> {
    TraversalSelector::bounded_relation_name(relation, max_depth).map_err(authoring_denial)
}

fn authoring_denial(error: impl std::fmt::Debug) -> WorthQueryReadDenial {
    WorthQueryReadDenial::new(
        WorthQueryReadDenialKind::AuthoringDenied,
        format!("{error:?}"),
    )
}

fn declarative_denial(error: DeclarativeLiveQueryError) -> WorthQueryReadDenial {
    let kind = match error {
        DeclarativeLiveQueryError::InvalidTarget
        | DeclarativeLiveQueryError::DuplicateTraversal { .. }
        | DeclarativeLiveQueryError::TraversalNotDeclaredInSchema { .. }
        | DeclarativeLiveQueryError::TraversalExceedsSchemaDepth { .. } => {
            WorthQueryReadDenialKind::ValidationDenied
        }
        DeclarativeLiveQueryError::Authoring(_) => WorthQueryReadDenialKind::AuthoringDenied,
        DeclarativeLiveQueryError::Canonicalization(_) => {
            WorthQueryReadDenialKind::CanonicalizationDenied
        }
        _ => WorthQueryReadDenialKind::PlanningDenied,
    };
    WorthQueryReadDenial::new(kind, format!("{error:?}"))
}

fn validation_denial(error: impl std::fmt::Debug) -> WorthQueryReadDenial {
    WorthQueryReadDenial::new(
        WorthQueryReadDenialKind::ValidationDenied,
        format!("{error:?}"),
    )
}
