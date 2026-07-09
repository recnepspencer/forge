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

fn frontier_operator_label(operator: &WorthQueryReadBuiltInOperator) -> &'static str {
    match operator {
        WorthQueryReadBuiltInOperator::AnchoredFrontier => "anchored frontier",
        WorthQueryReadBuiltInOperator::FrontierSearch => "frontier search",
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
) -> Result<WorthQueryReadGraph, WorthQueryReadDenial> {
    let (query, result_shape) =
        build_collection_operator_authored_inputs(root, declare_query, declare_result_shape)?;
    build_scoped_read_graph_from_authored(
        with_frontier_traversals(
            query,
            frontier_relations,
            max_depth,
            WorthQueryReadBuiltInOperator::AnchoredFrontier,
        )?,
        result_shape,
        schema_view,
        WorthQueryReadGraphFamily::Collection,
        WorthQueryReadScopeClass::AnchoredExpansion,
        vec![WorthQueryReadBuiltInOperator::AnchoredFrontier],
    )
}

pub(in crate::runtime) fn build_frontier_detail_read_graph(
    root: impl Into<String>,
    schema_view: QuerySchemaView,
    frontier_relations: impl IntoIterator<Item = RelationName>,
    max_depth: u8,
    declare_query: impl FnOnce(DetailReadOperatorQueryBuilder) -> DetailReadOperatorQueryBuilder,
    declare_result_shape: impl FnOnce(DetailResultShapeBuilder) -> DetailResultShapeBuilder,
) -> Result<WorthQueryReadGraph, WorthQueryReadDenial> {
    let (query, result_shape) =
        build_detail_operator_authored_inputs(root, declare_query, declare_result_shape)?;
    build_scoped_read_graph_from_authored(
        with_frontier_traversals(
            query,
            frontier_relations,
            max_depth,
            WorthQueryReadBuiltInOperator::AnchoredFrontier,
        )?,
        result_shape,
        schema_view,
        WorthQueryReadGraphFamily::Detail,
        WorthQueryReadScopeClass::AnchoredExpansion,
        vec![WorthQueryReadBuiltInOperator::AnchoredFrontier],
    )
}

pub(super) fn with_frontier_traversals(
    mut query: RawAuthoredQuery,
    frontier_relations: impl IntoIterator<Item = RelationName>,
    max_depth: u8,
    operator: WorthQueryReadBuiltInOperator,
) -> Result<RawAuthoredQuery, WorthQueryReadDenial> {
    let operator_label = frontier_operator_label(&operator);
    let relations: Vec<_> = frontier_relations.into_iter().collect();
    if relations.is_empty() {
        return Err(WorthQueryReadDenial::new_built_in_operator_denied(
            operator.clone(),
            WorthQueryReadBuiltInOperatorDenialReason::EmptyFrontier,
            format!("{operator_label} requires at least one frontier relation"),
        ));
    }
    if max_depth == 0 {
        return Err(WorthQueryReadDenial::new_built_in_operator_denied(
            operator.clone(),
            WorthQueryReadBuiltInOperatorDenialReason::ZeroDepth,
            format!("{operator_label} requires max_depth >= 1"),
        ));
    }
    let mut seen = std::collections::BTreeSet::new();
    for relation in &relations {
        if !seen.insert(relation.as_str().to_owned()) {
            return Err(WorthQueryReadDenial::new_built_in_operator_denied(
                operator.clone(),
                WorthQueryReadBuiltInOperatorDenialReason::DuplicateFrontierRelation,
                format!("{operator_label} forbids duplicate frontier relations"),
            ));
        }
    }
    if max_depth == 1 {
        return Err(WorthQueryReadDenial::new_built_in_operator_denied(
            operator,
            WorthQueryReadBuiltInOperatorDenialReason::DegenerateFrontierShape,
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
