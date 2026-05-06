use crate::authoring::{
    CollectionResultShapeBuilder, DetailResultShapeBuilder, RawAuthoredQuery, RelationName,
};
use crate::runtime::{
    ForgeQueryReadBuiltInOperator, ForgeQueryReadBuiltInOperatorDenialReason, ForgeQueryReadDenial,
    ForgeQueryReadGraph, ForgeQueryReadGraphFamily, ForgeQueryReadScopeClass, QuerySchemaView,
};

use super::read_composition_lowering::{
    build_collection_operator_authored_inputs, build_detail_operator_authored_inputs,
    build_scoped_read_graph_from_authored, traversal_selector,
};
use super::read_composition_operator_builders::{
    CollectionReadOperatorQueryBuilder, DetailReadOperatorQueryBuilder,
};

pub(in crate::runtime) fn build_bounded_ancestor_collection_read_graph(
    root: impl Into<String>,
    schema_view: QuerySchemaView,
    relation: RelationName,
    max_depth: u8,
    declare_query: impl FnOnce(CollectionReadOperatorQueryBuilder) -> CollectionReadOperatorQueryBuilder,
    declare_result_shape: impl FnOnce(CollectionResultShapeBuilder) -> CollectionResultShapeBuilder,
) -> Result<ForgeQueryReadGraph, ForgeQueryReadDenial> {
    let (query, result_shape) =
        build_collection_operator_authored_inputs(root, declare_query, declare_result_shape)?;
    build_scoped_read_graph_from_authored(
        with_bounded_walk_traversal(
            query,
            relation,
            max_depth,
            ForgeQueryReadBuiltInOperator::BoundedAncestor,
            "bounded ancestor walk requires max_depth > 1; use direct edge for one-hop reads",
        )?,
        result_shape,
        schema_view,
        ForgeQueryReadGraphFamily::Collection,
        ForgeQueryReadScopeClass::AnchoredExpansion,
        vec![ForgeQueryReadBuiltInOperator::BoundedAncestor],
    )
}

pub(in crate::runtime) fn build_bounded_ancestor_detail_read_graph(
    root: impl Into<String>,
    schema_view: QuerySchemaView,
    relation: RelationName,
    max_depth: u8,
    declare_query: impl FnOnce(DetailReadOperatorQueryBuilder) -> DetailReadOperatorQueryBuilder,
    declare_result_shape: impl FnOnce(DetailResultShapeBuilder) -> DetailResultShapeBuilder,
) -> Result<ForgeQueryReadGraph, ForgeQueryReadDenial> {
    let (query, result_shape) =
        build_detail_operator_authored_inputs(root, declare_query, declare_result_shape)?;
    build_scoped_read_graph_from_authored(
        with_bounded_walk_traversal(
            query,
            relation,
            max_depth,
            ForgeQueryReadBuiltInOperator::BoundedAncestor,
            "bounded ancestor walk requires max_depth > 1; use direct edge for one-hop reads",
        )?,
        result_shape,
        schema_view,
        ForgeQueryReadGraphFamily::Detail,
        ForgeQueryReadScopeClass::AnchoredExpansion,
        vec![ForgeQueryReadBuiltInOperator::BoundedAncestor],
    )
}

pub(in crate::runtime) fn build_bounded_descendant_collection_read_graph(
    root: impl Into<String>,
    schema_view: QuerySchemaView,
    relation: RelationName,
    max_depth: u8,
    declare_query: impl FnOnce(CollectionReadOperatorQueryBuilder) -> CollectionReadOperatorQueryBuilder,
    declare_result_shape: impl FnOnce(CollectionResultShapeBuilder) -> CollectionResultShapeBuilder,
) -> Result<ForgeQueryReadGraph, ForgeQueryReadDenial> {
    let (query, result_shape) =
        build_collection_operator_authored_inputs(root, declare_query, declare_result_shape)?;
    build_scoped_read_graph_from_authored(
        with_bounded_walk_traversal(
            query,
            relation,
            max_depth,
            ForgeQueryReadBuiltInOperator::BoundedDescendant,
            "bounded descendant walk requires max_depth > 1; use direct edge for one-hop reads",
        )?,
        result_shape,
        schema_view,
        ForgeQueryReadGraphFamily::Collection,
        ForgeQueryReadScopeClass::AnchoredExpansion,
        vec![ForgeQueryReadBuiltInOperator::BoundedDescendant],
    )
}

pub(in crate::runtime) fn build_bounded_descendant_detail_read_graph(
    root: impl Into<String>,
    schema_view: QuerySchemaView,
    relation: RelationName,
    max_depth: u8,
    declare_query: impl FnOnce(DetailReadOperatorQueryBuilder) -> DetailReadOperatorQueryBuilder,
    declare_result_shape: impl FnOnce(DetailResultShapeBuilder) -> DetailResultShapeBuilder,
) -> Result<ForgeQueryReadGraph, ForgeQueryReadDenial> {
    let (query, result_shape) =
        build_detail_operator_authored_inputs(root, declare_query, declare_result_shape)?;
    build_scoped_read_graph_from_authored(
        with_bounded_walk_traversal(
            query,
            relation,
            max_depth,
            ForgeQueryReadBuiltInOperator::BoundedDescendant,
            "bounded descendant walk requires max_depth > 1; use direct edge for one-hop reads",
        )?,
        result_shape,
        schema_view,
        ForgeQueryReadGraphFamily::Detail,
        ForgeQueryReadScopeClass::AnchoredExpansion,
        vec![ForgeQueryReadBuiltInOperator::BoundedDescendant],
    )
}

fn with_bounded_walk_traversal(
    query: RawAuthoredQuery,
    relation: RelationName,
    max_depth: u8,
    operator: ForgeQueryReadBuiltInOperator,
    degenerate_message: &str,
) -> Result<RawAuthoredQuery, ForgeQueryReadDenial> {
    if max_depth == 0 {
        return Err(ForgeQueryReadDenial::new_built_in_operator_denied(
            operator,
            ForgeQueryReadBuiltInOperatorDenialReason::ZeroDepth,
            "bounded walk operators require max_depth >= 1",
        ));
    }
    if max_depth == 1 {
        return Err(ForgeQueryReadDenial::new_built_in_operator_denied(
            operator,
            ForgeQueryReadBuiltInOperatorDenialReason::DegenerateBoundedWalkShape,
            degenerate_message,
        ));
    }
    Ok(query.with_traversal(traversal_selector(relation, max_depth)?))
}
