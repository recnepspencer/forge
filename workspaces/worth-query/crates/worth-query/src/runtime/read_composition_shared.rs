use crate::authoring::{
    CollectionResultShapeBuilder, DetailResultShapeBuilder, RawAuthoredQuery, RelationName,
};
use crate::ordinary::read::WorthQueryDeclaredReadIntent;
use crate::runtime::{
    QuerySchemaView, WorthQueryReadBuiltInOperator, WorthQueryReadBuiltInOperatorDenialReason,
    WorthQueryReadDenial, WorthQueryReadGraphFamily, WorthQueryReadScopeClass,
};

use super::read_composition_lowering::{
    build_collection_operator_authored_inputs, build_detail_operator_authored_inputs,
    build_scoped_read_intent_from_authored, traversal_selector,
};
use super::read_composition_operator_builders::{
    CollectionReadOperatorQueryBuilder, DetailReadOperatorQueryBuilder,
};

pub(in crate::runtime) fn build_shared_attachment_collection_read_intent(
    root: impl Into<String>,
    schema_view: QuerySchemaView,
    shared_relations: impl IntoIterator<Item = RelationName>,
    declare_query: impl FnOnce(CollectionReadOperatorQueryBuilder) -> CollectionReadOperatorQueryBuilder,
    declare_result_shape: impl FnOnce(CollectionResultShapeBuilder) -> CollectionResultShapeBuilder,
) -> Result<WorthQueryDeclaredReadIntent, WorthQueryReadDenial> {
    build_shared_local_collection_read_intent(
        root,
        schema_view,
        shared_relations,
        declare_query,
        declare_result_shape,
        WorthQueryReadBuiltInOperator::SharedAttachment,
        "shared attachment",
    )
}

pub(in crate::runtime) fn build_shared_endpoint_collection_read_intent(
    root: impl Into<String>,
    schema_view: QuerySchemaView,
    shared_relations: impl IntoIterator<Item = RelationName>,
    declare_query: impl FnOnce(CollectionReadOperatorQueryBuilder) -> CollectionReadOperatorQueryBuilder,
    declare_result_shape: impl FnOnce(CollectionResultShapeBuilder) -> CollectionResultShapeBuilder,
) -> Result<WorthQueryDeclaredReadIntent, WorthQueryReadDenial> {
    build_shared_local_collection_read_intent(
        root,
        schema_view,
        shared_relations,
        declare_query,
        declare_result_shape,
        WorthQueryReadBuiltInOperator::SharedEndpoint,
        "shared endpoint",
    )
}

pub(in crate::runtime) fn build_shared_attachment_detail_read_intent(
    root: impl Into<String>,
    schema_view: QuerySchemaView,
    shared_relations: impl IntoIterator<Item = RelationName>,
    declare_query: impl FnOnce(DetailReadOperatorQueryBuilder) -> DetailReadOperatorQueryBuilder,
    declare_result_shape: impl FnOnce(DetailResultShapeBuilder) -> DetailResultShapeBuilder,
) -> Result<WorthQueryDeclaredReadIntent, WorthQueryReadDenial> {
    build_shared_local_detail_read_intent(
        root,
        schema_view,
        shared_relations,
        declare_query,
        declare_result_shape,
        WorthQueryReadBuiltInOperator::SharedAttachment,
        "shared attachment",
    )
}

pub(in crate::runtime) fn build_shared_endpoint_detail_read_intent(
    root: impl Into<String>,
    schema_view: QuerySchemaView,
    shared_relations: impl IntoIterator<Item = RelationName>,
    declare_query: impl FnOnce(DetailReadOperatorQueryBuilder) -> DetailReadOperatorQueryBuilder,
    declare_result_shape: impl FnOnce(DetailResultShapeBuilder) -> DetailResultShapeBuilder,
) -> Result<WorthQueryDeclaredReadIntent, WorthQueryReadDenial> {
    build_shared_local_detail_read_intent(
        root,
        schema_view,
        shared_relations,
        declare_query,
        declare_result_shape,
        WorthQueryReadBuiltInOperator::SharedEndpoint,
        "shared endpoint",
    )
}

fn build_shared_local_collection_read_intent(
    root: impl Into<String>,
    schema_view: QuerySchemaView,
    shared_relations: impl IntoIterator<Item = RelationName>,
    declare_query: impl FnOnce(CollectionReadOperatorQueryBuilder) -> CollectionReadOperatorQueryBuilder,
    declare_result_shape: impl FnOnce(CollectionResultShapeBuilder) -> CollectionResultShapeBuilder,
    operator: WorthQueryReadBuiltInOperator,
    label: &'static str,
) -> Result<WorthQueryDeclaredReadIntent, WorthQueryReadDenial> {
    let (query, result_shape) =
        build_collection_operator_authored_inputs(root, declare_query, declare_result_shape)?;
    build_scoped_read_intent_from_authored(
        with_shared_local_traversals(query, shared_relations, operator.clone(), label)?,
        result_shape,
        schema_view,
        WorthQueryReadGraphFamily::Collection,
        WorthQueryReadScopeClass::LocalNeighborhood,
        vec![operator],
    )
}

fn build_shared_local_detail_read_intent(
    root: impl Into<String>,
    schema_view: QuerySchemaView,
    shared_relations: impl IntoIterator<Item = RelationName>,
    declare_query: impl FnOnce(DetailReadOperatorQueryBuilder) -> DetailReadOperatorQueryBuilder,
    declare_result_shape: impl FnOnce(DetailResultShapeBuilder) -> DetailResultShapeBuilder,
    operator: WorthQueryReadBuiltInOperator,
    label: &'static str,
) -> Result<WorthQueryDeclaredReadIntent, WorthQueryReadDenial> {
    let (query, result_shape) =
        build_detail_operator_authored_inputs(root, declare_query, declare_result_shape)?;
    build_scoped_read_intent_from_authored(
        with_shared_local_traversals(query, shared_relations, operator.clone(), label)?,
        result_shape,
        schema_view,
        WorthQueryReadGraphFamily::Detail,
        WorthQueryReadScopeClass::LocalNeighborhood,
        vec![operator],
    )
}

fn with_shared_local_traversals(
    mut query: RawAuthoredQuery,
    shared_relations: impl IntoIterator<Item = RelationName>,
    operator: WorthQueryReadBuiltInOperator,
    label: &'static str,
) -> Result<RawAuthoredQuery, WorthQueryReadDenial> {
    let relations: Vec<_> = shared_relations.into_iter().collect();
    if relations.len() < 2 {
        return Err(WorthQueryReadDenial::new_built_in_operator_denied(
            operator.clone(),
            WorthQueryReadBuiltInOperatorDenialReason::TooFewSharedRelations,
            format!("{label} requires at least two relations"),
        ));
    }
    let mut seen = std::collections::BTreeSet::new();
    for relation in &relations {
        if !seen.insert(relation.as_str().to_owned()) {
            return Err(WorthQueryReadDenial::new_built_in_operator_denied(
                operator.clone(),
                WorthQueryReadBuiltInOperatorDenialReason::DuplicateSharedRelation,
                format!("{label} forbids duplicate relations"),
            ));
        }
    }
    for relation in relations {
        query = query.with_traversal(traversal_selector(relation, 1)?);
    }
    Ok(query)
}
