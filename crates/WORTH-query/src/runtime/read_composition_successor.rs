use crate::authoring::{
    CollectionResultShapeBuilder, DetailResultShapeBuilder, RawAuthoredQuery, RelationName,
};
use crate::runtime::{
    QuerySchemaView, WorthQueryReadBuiltInOperator, WorthQueryReadBuiltInOperatorDenialReason,
    WorthQueryReadDenial, WorthQueryReadGraph, WorthQueryReadGraphFamily, WorthQueryReadScopeClass,
};

use super::read_composition_lowering::{
    build_collection_operator_authored_inputs, build_detail_operator_authored_inputs,
    build_scoped_read_graph_from_authored, traversal_selector,
};
use super::read_composition_operator_builders::{
    CollectionReadOperatorQueryBuilder, DetailReadOperatorQueryBuilder,
};

pub(in crate::runtime) fn build_successor_walk_collection_read_graph(
    root: impl Into<String>,
    schema_view: QuerySchemaView,
    relation: RelationName,
    max_depth: u8,
    declare_query: impl FnOnce(CollectionReadOperatorQueryBuilder) -> CollectionReadOperatorQueryBuilder,
    declare_result_shape: impl FnOnce(CollectionResultShapeBuilder) -> CollectionResultShapeBuilder,
) -> Result<WorthQueryReadGraph, WorthQueryReadDenial> {
    let (query, result_shape) =
        build_collection_operator_authored_inputs(root, declare_query, declare_result_shape)?;
    build_scoped_read_graph_from_authored(
        with_successor_walk_traversal(query, relation, max_depth)?,
        result_shape,
        schema_view,
        WorthQueryReadGraphFamily::Collection,
        WorthQueryReadScopeClass::LocalNeighborhood,
        vec![WorthQueryReadBuiltInOperator::SuccessorWalk],
    )
}

pub(in crate::runtime) fn build_successor_walk_detail_read_graph(
    root: impl Into<String>,
    schema_view: QuerySchemaView,
    relation: RelationName,
    max_depth: u8,
    declare_query: impl FnOnce(DetailReadOperatorQueryBuilder) -> DetailReadOperatorQueryBuilder,
    declare_result_shape: impl FnOnce(DetailResultShapeBuilder) -> DetailResultShapeBuilder,
) -> Result<WorthQueryReadGraph, WorthQueryReadDenial> {
    let (query, result_shape) =
        build_detail_operator_authored_inputs(root, declare_query, declare_result_shape)?;
    build_scoped_read_graph_from_authored(
        with_successor_walk_traversal(query, relation, max_depth)?,
        result_shape,
        schema_view,
        WorthQueryReadGraphFamily::Detail,
        WorthQueryReadScopeClass::LocalNeighborhood,
        vec![WorthQueryReadBuiltInOperator::SuccessorWalk],
    )
}

fn with_successor_walk_traversal(
    query: RawAuthoredQuery,
    relation: RelationName,
    max_depth: u8,
) -> Result<RawAuthoredQuery, WorthQueryReadDenial> {
    if max_depth == 0 {
        return Err(WorthQueryReadDenial::new_built_in_operator_denied(
            WorthQueryReadBuiltInOperator::SuccessorWalk,
            WorthQueryReadBuiltInOperatorDenialReason::ZeroDepth,
            "successor walk requires max_depth >= 1",
        ));
    }
    if max_depth == 1 {
        return Err(WorthQueryReadDenial::new_built_in_operator_denied(
            WorthQueryReadBuiltInOperator::SuccessorWalk,
            WorthQueryReadBuiltInOperatorDenialReason::DegenerateSuccessorWalkShape,
            "successor walk requires max_depth > 1; use direct edge for one-hop reads",
        ));
    }
    Ok(query.with_traversal(traversal_selector(relation, max_depth)?))
}
