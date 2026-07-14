use worth_query::facade::foundation::TraversalSelector;
use worth_query::facade::runtime::CollectionReadOperatorQueryBuilder;

fn direct_edge_collection_builder_still_cannot_traverse(query: CollectionReadOperatorQueryBuilder) {
    let _ = query.traverse(TraversalSelector::bounded("manager", 1).unwrap());
}

fn main() {}
