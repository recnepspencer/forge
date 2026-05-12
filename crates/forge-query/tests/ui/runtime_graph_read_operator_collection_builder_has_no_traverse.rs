use forge_query::facade::{CollectionReadOperatorQueryBuilder, TraversalSelector};

fn direct_edge_collection_builder_still_cannot_traverse(query: CollectionReadOperatorQueryBuilder) {
    let _ = query.traverse(TraversalSelector::bounded("manager", 1).unwrap());
}

fn main() {}
