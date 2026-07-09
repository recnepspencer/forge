use crate::authoring::{CollectionResultShapeBuilder, DetailResultShapeBuilder, RelationName};
use crate::runtime::{
    QuerySchemaView, WorthQueryReadBuiltInOperator, WorthQueryReadBuiltInOperatorDenialReason,
    WorthQueryReadDenial, WorthQueryReadGraph, WorthQueryReadGraphFamily, WorthQueryReadScopeClass,
};

use super::read_composition_frontier::with_frontier_traversals;
use super::read_composition_lowering::{
    build_collection_operator_authored_inputs, build_detail_operator_authored_inputs,
    build_scoped_read_graph_from_authored,
};
use super::read_composition_operator_builders::{
    CollectionReadOperatorQueryBuilder, DetailReadOperatorQueryBuilder,
};

pub(in crate::runtime) fn build_frontier_search_collection_read_graph(
    root: impl Into<String>,
    schema_view: QuerySchemaView,
    frontier_relations: impl IntoIterator<Item = RelationName>,
    max_depth: u8,
    declare_query: impl FnOnce(CollectionReadOperatorQueryBuilder) -> CollectionReadOperatorQueryBuilder,
    declare_result_shape: impl FnOnce(CollectionResultShapeBuilder) -> CollectionResultShapeBuilder,
) -> Result<WorthQueryReadGraph, WorthQueryReadDenial> {
    let (query, result_shape) =
        build_collection_operator_authored_inputs(root, declare_query, declare_result_shape)?;
    let query = with_frontier_traversals(
        query,
        frontier_relations,
        max_depth,
        WorthQueryReadBuiltInOperator::FrontierSearch,
    )?;
    require_broad_search_predicate(query.predicates().len())?;
    build_scoped_read_graph_from_authored(
        query,
        result_shape,
        schema_view,
        WorthQueryReadGraphFamily::Collection,
        WorthQueryReadScopeClass::ExplicitBroadSearch,
        vec![WorthQueryReadBuiltInOperator::FrontierSearch],
    )
}

pub(in crate::runtime) fn build_frontier_search_detail_read_graph(
    root: impl Into<String>,
    schema_view: QuerySchemaView,
    frontier_relations: impl IntoIterator<Item = RelationName>,
    max_depth: u8,
    declare_query: impl FnOnce(DetailReadOperatorQueryBuilder) -> DetailReadOperatorQueryBuilder,
    declare_result_shape: impl FnOnce(DetailResultShapeBuilder) -> DetailResultShapeBuilder,
) -> Result<WorthQueryReadGraph, WorthQueryReadDenial> {
    let (query, result_shape) =
        build_detail_operator_authored_inputs(root, declare_query, declare_result_shape)?;
    let query = with_frontier_traversals(
        query,
        frontier_relations,
        max_depth,
        WorthQueryReadBuiltInOperator::FrontierSearch,
    )?;
    require_broad_search_predicate(query.predicates().len())?;
    build_scoped_read_graph_from_authored(
        query,
        result_shape,
        schema_view,
        WorthQueryReadGraphFamily::Detail,
        WorthQueryReadScopeClass::ExplicitBroadSearch,
        vec![WorthQueryReadBuiltInOperator::FrontierSearch],
    )
}

fn require_broad_search_predicate(predicate_count: usize) -> Result<(), WorthQueryReadDenial> {
    if predicate_count == 0 {
        return Err(WorthQueryReadDenial::new_built_in_operator_denied(
            WorthQueryReadBuiltInOperator::FrontierSearch,
            WorthQueryReadBuiltInOperatorDenialReason::MissingBroadSearchPredicate,
            "frontier search requires at least one predicate to stay receipt-honest as broad search",
        ));
    }
    Ok(())
}
