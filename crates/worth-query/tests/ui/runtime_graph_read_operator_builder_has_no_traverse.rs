use worth_query::facade::foundation::TraversalSelector;
use worth_query::facade::runtime::DetailReadOperatorQueryBuilder;

fn direct_edge_detail_builder_still_cannot_traverse(query: DetailReadOperatorQueryBuilder) {
    let _ = query.traverse(TraversalSelector::bounded("manager", 1).unwrap());
}

fn main() {}
