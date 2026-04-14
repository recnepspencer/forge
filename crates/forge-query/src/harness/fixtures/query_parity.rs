use crate::authoring::RawAuthoredQuery;
use crate::facade::{AspectFieldSelector, RootEntityKey, TraversalSelector};

pub fn direct_detail_query() -> crate::authoring::DetailAuthoredQuery {
    RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("status", "kind").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .traverse(TraversalSelector::bounded("owner", 1).unwrap())
        .build()
        .unwrap()
}

pub fn reordered_detail_query() -> crate::authoring::DetailAuthoredQuery {
    RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .traverse(TraversalSelector::bounded("owner", 1).unwrap())
        .project(AspectFieldSelector::new("status", "kind").unwrap())
        .build()
        .unwrap()
}

pub fn collection_query() -> crate::authoring::CollectionAuthoredQuery {
    RawAuthoredQuery::collection_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .build()
        .unwrap()
}

pub fn reordered_collection_query() -> crate::authoring::CollectionAuthoredQuery {
    RawAuthoredQuery::collection_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("status", "kind").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .build()
        .unwrap()
}

pub fn collection_query_with_two_projections() -> crate::authoring::CollectionAuthoredQuery {
    RawAuthoredQuery::collection_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .project(AspectFieldSelector::new("status", "kind").unwrap())
        .build()
        .unwrap()
}

pub fn duplicate_projection_detail_query() -> crate::authoring::DetailAuthoredQuery {
    RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .build()
        .unwrap()
}

pub fn single_projection_detail_query() -> crate::authoring::DetailAuthoredQuery {
    RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .build()
        .unwrap()
}

pub fn unsupported_detail_query_for_test() -> crate::authoring::RawAuthoredQuery {
    RawAuthoredQuery::unsupported_for_test(RootEntityKey::new("task").unwrap(), "grouped")
        .with_projection(AspectFieldSelector::new("title", "text").unwrap())
}
