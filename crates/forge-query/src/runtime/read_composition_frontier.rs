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

fn frontier_operator_label(operator: &ForgeQueryReadBuiltInOperator) -> &'static str {
    match operator {
        ForgeQueryReadBuiltInOperator::AnchoredFrontier => "anchored frontier",
        ForgeQueryReadBuiltInOperator::FrontierSearch => "frontier search",
        _ => "frontier operator",
    }
}

pub(in crate::runtime) fn build_frontier_collection_read_graph(
    root: impl Into<String>,
    schema_view: QuerySchemaView,
    frontier_relations: impl IntoIterator<Item = RelationName>,
    max_depth: u8,
    declare_query: impl FnOnce(CollectionReadOperatorQueryBuilder) -> CollectionReadOperatorQueryBuilder,
    declare_result_shape: impl FnOnce(CollectionResultShapeBuilder) -> CollectionResultShapeBuilder,
) -> Result<ForgeQueryReadGraph, ForgeQueryReadDenial> {
    let (query, result_shape) =
        build_collection_operator_authored_inputs(root, declare_query, declare_result_shape)?;
    build_scoped_read_graph_from_authored(
        with_frontier_traversals(
            query,
            frontier_relations,
            max_depth,
            ForgeQueryReadBuiltInOperator::AnchoredFrontier,
        )?,
        result_shape,
        schema_view,
        ForgeQueryReadGraphFamily::Collection,
        ForgeQueryReadScopeClass::AnchoredExpansion,
        vec![ForgeQueryReadBuiltInOperator::AnchoredFrontier],
    )
}

pub(in crate::runtime) fn build_frontier_detail_read_graph(
    root: impl Into<String>,
    schema_view: QuerySchemaView,
    frontier_relations: impl IntoIterator<Item = RelationName>,
    max_depth: u8,
    declare_query: impl FnOnce(DetailReadOperatorQueryBuilder) -> DetailReadOperatorQueryBuilder,
    declare_result_shape: impl FnOnce(DetailResultShapeBuilder) -> DetailResultShapeBuilder,
) -> Result<ForgeQueryReadGraph, ForgeQueryReadDenial> {
    let (query, result_shape) =
        build_detail_operator_authored_inputs(root, declare_query, declare_result_shape)?;
    build_scoped_read_graph_from_authored(
        with_frontier_traversals(
            query,
            frontier_relations,
            max_depth,
            ForgeQueryReadBuiltInOperator::AnchoredFrontier,
        )?,
        result_shape,
        schema_view,
        ForgeQueryReadGraphFamily::Detail,
        ForgeQueryReadScopeClass::AnchoredExpansion,
        vec![ForgeQueryReadBuiltInOperator::AnchoredFrontier],
    )
}

pub(super) fn with_frontier_traversals(
    mut query: RawAuthoredQuery,
    frontier_relations: impl IntoIterator<Item = RelationName>,
    max_depth: u8,
    operator: ForgeQueryReadBuiltInOperator,
) -> Result<RawAuthoredQuery, ForgeQueryReadDenial> {
    let operator_label = frontier_operator_label(&operator);
    let relations: Vec<_> = frontier_relations.into_iter().collect();
    if relations.is_empty() {
        return Err(ForgeQueryReadDenial::new_built_in_operator_denied(
            operator.clone(),
            ForgeQueryReadBuiltInOperatorDenialReason::EmptyFrontier,
            format!("{operator_label} requires at least one frontier relation"),
        ));
    }
    if max_depth == 0 {
        return Err(ForgeQueryReadDenial::new_built_in_operator_denied(
            operator.clone(),
            ForgeQueryReadBuiltInOperatorDenialReason::ZeroDepth,
            format!("{operator_label} requires max_depth >= 1"),
        ));
    }
    let mut seen = std::collections::BTreeSet::new();
    for relation in &relations {
        if !seen.insert(relation.as_str().to_owned()) {
            return Err(ForgeQueryReadDenial::new_built_in_operator_denied(
                operator.clone(),
                ForgeQueryReadBuiltInOperatorDenialReason::DuplicateFrontierRelation,
                format!("{operator_label} forbids duplicate frontier relations"),
            ));
        }
    }
    if max_depth == 1 {
        return Err(ForgeQueryReadDenial::new_built_in_operator_denied(
            operator,
            ForgeQueryReadBuiltInOperatorDenialReason::DegenerateFrontierShape,
            format!(
                "{operator_label} requires max_depth > 1; use a local operator for one-hop shared reads"
            ),
        ));
    }
    for relation in relations {
        query = query.with_traversal(traversal_selector(relation, max_depth)?);
    }
    Ok(query)
}
