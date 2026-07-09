use super::read_composition_frontier::{
    build_frontier_collection_read_graph, build_frontier_detail_read_graph,
};
use super::read_composition_frontier_search::{
    build_frontier_search_collection_read_graph, build_frontier_search_detail_read_graph,
};
use super::read_composition_lowering::{
    build_collection_read_graph, build_detail_read_graph, build_direct_edge_collection_read_graph,
    build_direct_edge_detail_read_graph,
};
use super::read_composition_operator_builders::{
    CollectionReadOperatorQueryBuilder, DetailReadOperatorQueryBuilder,
};
use super::read_composition_successor::{
    build_successor_walk_collection_read_graph, build_successor_walk_detail_read_graph,
};
use super::read_composition_walks::{
    build_bounded_ancestor_collection_read_graph, build_bounded_ancestor_detail_read_graph,
};
use super::{QuerySchemaView, WorthQueryReadDenial, WorthQueryReadGraph, WorthQueryReadScopeClass};
use crate::authoring::{
    CollectionQueryBuilder, CollectionResultShapeBuilder, DetailQueryBuilder,
    DetailResultShapeBuilder, RelationName,
};

pub struct WorthQueryReadBuilder;

impl WorthQueryReadBuilder {
    pub(in crate::runtime) fn new() -> Self {
        Self
    }

    pub fn standalone() -> Self {
        Self
    }

    pub fn local_collection(
        self,
        root: impl Into<String>,
        schema_view: QuerySchemaView,
        declare_query: impl FnOnce(CollectionQueryBuilder) -> CollectionQueryBuilder,
        declare_result_shape: impl FnOnce(CollectionResultShapeBuilder) -> CollectionResultShapeBuilder,
    ) -> Result<WorthQueryReadGraph, WorthQueryReadDenial> {
        build_collection_read_graph(
            root,
            schema_view,
            declare_query,
            declare_result_shape,
            WorthQueryReadScopeClass::LocalNeighborhood,
        )
    }

    pub fn anchored_collection(
        self,
        root: impl Into<String>,
        schema_view: QuerySchemaView,
        declare_query: impl FnOnce(CollectionQueryBuilder) -> CollectionQueryBuilder,
        declare_result_shape: impl FnOnce(CollectionResultShapeBuilder) -> CollectionResultShapeBuilder,
    ) -> Result<WorthQueryReadGraph, WorthQueryReadDenial> {
        build_collection_read_graph(
            root,
            schema_view,
            declare_query,
            declare_result_shape,
            WorthQueryReadScopeClass::AnchoredExpansion,
        )
    }

    pub fn explicit_broad_search_collection(
        self,
        root: impl Into<String>,
        schema_view: QuerySchemaView,
        declare_query: impl FnOnce(CollectionQueryBuilder) -> CollectionQueryBuilder,
        declare_result_shape: impl FnOnce(CollectionResultShapeBuilder) -> CollectionResultShapeBuilder,
    ) -> Result<WorthQueryReadGraph, WorthQueryReadDenial> {
        build_collection_read_graph(
            root,
            schema_view,
            declare_query,
            declare_result_shape,
            WorthQueryReadScopeClass::ExplicitBroadSearch,
        )
    }

    pub fn local_detail(
        self,
        root: impl Into<String>,
        schema_view: QuerySchemaView,
        declare_query: impl FnOnce(DetailQueryBuilder) -> DetailQueryBuilder,
        declare_result_shape: impl FnOnce(DetailResultShapeBuilder) -> DetailResultShapeBuilder,
    ) -> Result<WorthQueryReadGraph, WorthQueryReadDenial> {
        build_detail_read_graph(
            root,
            schema_view,
            declare_query,
            declare_result_shape,
            WorthQueryReadScopeClass::LocalNeighborhood,
        )
    }

    pub fn local_direct_edge_collection(
        self,
        root: impl Into<String>,
        schema_view: QuerySchemaView,
        relation: RelationName,
        declare_query: impl FnOnce(
            CollectionReadOperatorQueryBuilder,
        ) -> CollectionReadOperatorQueryBuilder,
        declare_result_shape: impl FnOnce(CollectionResultShapeBuilder) -> CollectionResultShapeBuilder,
    ) -> Result<WorthQueryReadGraph, WorthQueryReadDenial> {
        build_direct_edge_collection_read_graph(
            root,
            schema_view,
            relation,
            declare_query,
            declare_result_shape,
        )
    }

    pub fn local_direct_edge_detail(
        self,
        root: impl Into<String>,
        schema_view: QuerySchemaView,
        relation: RelationName,
        declare_query: impl FnOnce(DetailReadOperatorQueryBuilder) -> DetailReadOperatorQueryBuilder,
        declare_result_shape: impl FnOnce(DetailResultShapeBuilder) -> DetailResultShapeBuilder,
    ) -> Result<WorthQueryReadGraph, WorthQueryReadDenial> {
        build_direct_edge_detail_read_graph(
            root,
            schema_view,
            relation,
            declare_query,
            declare_result_shape,
        )
    }

    pub fn local_successor_walk_collection(
        self,
        root: impl Into<String>,
        schema_view: QuerySchemaView,
        relation: RelationName,
        max_depth: u8,
        declare_query: impl FnOnce(
            CollectionReadOperatorQueryBuilder,
        ) -> CollectionReadOperatorQueryBuilder,
        declare_result_shape: impl FnOnce(CollectionResultShapeBuilder) -> CollectionResultShapeBuilder,
    ) -> Result<WorthQueryReadGraph, WorthQueryReadDenial> {
        build_successor_walk_collection_read_graph(
            root,
            schema_view,
            relation,
            max_depth,
            declare_query,
            declare_result_shape,
        )
    }

    pub fn local_successor_walk_detail(
        self,
        root: impl Into<String>,
        schema_view: QuerySchemaView,
        relation: RelationName,
        max_depth: u8,
        declare_query: impl FnOnce(DetailReadOperatorQueryBuilder) -> DetailReadOperatorQueryBuilder,
        declare_result_shape: impl FnOnce(DetailResultShapeBuilder) -> DetailResultShapeBuilder,
    ) -> Result<WorthQueryReadGraph, WorthQueryReadDenial> {
        build_successor_walk_detail_read_graph(
            root,
            schema_view,
            relation,
            max_depth,
            declare_query,
            declare_result_shape,
        )
    }

    pub fn anchored_detail(
        self,
        root: impl Into<String>,
        schema_view: QuerySchemaView,
        declare_query: impl FnOnce(DetailQueryBuilder) -> DetailQueryBuilder,
        declare_result_shape: impl FnOnce(DetailResultShapeBuilder) -> DetailResultShapeBuilder,
    ) -> Result<WorthQueryReadGraph, WorthQueryReadDenial> {
        build_detail_read_graph(
            root,
            schema_view,
            declare_query,
            declare_result_shape,
            WorthQueryReadScopeClass::AnchoredExpansion,
        )
    }

    pub fn anchored_bounded_ancestor_collection(
        self,
        root: impl Into<String>,
        schema_view: QuerySchemaView,
        relation: RelationName,
        max_depth: u8,
        declare_query: impl FnOnce(
            CollectionReadOperatorQueryBuilder,
        ) -> CollectionReadOperatorQueryBuilder,
        declare_result_shape: impl FnOnce(CollectionResultShapeBuilder) -> CollectionResultShapeBuilder,
    ) -> Result<WorthQueryReadGraph, WorthQueryReadDenial> {
        build_bounded_ancestor_collection_read_graph(
            root,
            schema_view,
            relation,
            max_depth,
            declare_query,
            declare_result_shape,
        )
    }

    pub fn anchored_bounded_ancestor_detail(
        self,
        root: impl Into<String>,
        schema_view: QuerySchemaView,
        relation: RelationName,
        max_depth: u8,
        declare_query: impl FnOnce(DetailReadOperatorQueryBuilder) -> DetailReadOperatorQueryBuilder,
        declare_result_shape: impl FnOnce(DetailResultShapeBuilder) -> DetailResultShapeBuilder,
    ) -> Result<WorthQueryReadGraph, WorthQueryReadDenial> {
        build_bounded_ancestor_detail_read_graph(
            root,
            schema_view,
            relation,
            max_depth,
            declare_query,
            declare_result_shape,
        )
    }

    pub fn anchored_frontier_collection(
        self,
        root: impl Into<String>,
        schema_view: QuerySchemaView,
        frontier_relations: impl IntoIterator<Item = RelationName>,
        max_depth: u8,
        declare_query: impl FnOnce(
            CollectionReadOperatorQueryBuilder,
        ) -> CollectionReadOperatorQueryBuilder,
        declare_result_shape: impl FnOnce(CollectionResultShapeBuilder) -> CollectionResultShapeBuilder,
    ) -> Result<WorthQueryReadGraph, WorthQueryReadDenial> {
        build_frontier_collection_read_graph(
            root,
            schema_view,
            frontier_relations,
            max_depth,
            declare_query,
            declare_result_shape,
        )
    }

    pub fn anchored_frontier_detail(
        self,
        root: impl Into<String>,
        schema_view: QuerySchemaView,
        frontier_relations: impl IntoIterator<Item = RelationName>,
        max_depth: u8,
        declare_query: impl FnOnce(DetailReadOperatorQueryBuilder) -> DetailReadOperatorQueryBuilder,
        declare_result_shape: impl FnOnce(DetailResultShapeBuilder) -> DetailResultShapeBuilder,
    ) -> Result<WorthQueryReadGraph, WorthQueryReadDenial> {
        build_frontier_detail_read_graph(
            root,
            schema_view,
            frontier_relations,
            max_depth,
            declare_query,
            declare_result_shape,
        )
    }

    pub fn explicit_broad_search_frontier_collection(
        self,
        root: impl Into<String>,
        schema_view: QuerySchemaView,
        frontier_relations: impl IntoIterator<Item = RelationName>,
        max_depth: u8,
        declare_query: impl FnOnce(
            CollectionReadOperatorQueryBuilder,
        ) -> CollectionReadOperatorQueryBuilder,
        declare_result_shape: impl FnOnce(CollectionResultShapeBuilder) -> CollectionResultShapeBuilder,
    ) -> Result<WorthQueryReadGraph, WorthQueryReadDenial> {
        build_frontier_search_collection_read_graph(
            root,
            schema_view,
            frontier_relations,
            max_depth,
            declare_query,
            declare_result_shape,
        )
    }

    pub fn explicit_broad_search_frontier_detail(
        self,
        root: impl Into<String>,
        schema_view: QuerySchemaView,
        frontier_relations: impl IntoIterator<Item = RelationName>,
        max_depth: u8,
        declare_query: impl FnOnce(DetailReadOperatorQueryBuilder) -> DetailReadOperatorQueryBuilder,
        declare_result_shape: impl FnOnce(DetailResultShapeBuilder) -> DetailResultShapeBuilder,
    ) -> Result<WorthQueryReadGraph, WorthQueryReadDenial> {
        build_frontier_search_detail_read_graph(
            root,
            schema_view,
            frontier_relations,
            max_depth,
            declare_query,
            declare_result_shape,
        )
    }

    pub fn explicit_broad_search_detail(
        self,
        root: impl Into<String>,
        schema_view: QuerySchemaView,
        declare_query: impl FnOnce(DetailQueryBuilder) -> DetailQueryBuilder,
        declare_result_shape: impl FnOnce(DetailResultShapeBuilder) -> DetailResultShapeBuilder,
    ) -> Result<WorthQueryReadGraph, WorthQueryReadDenial> {
        build_detail_read_graph(
            root,
            schema_view,
            declare_query,
            declare_result_shape,
            WorthQueryReadScopeClass::ExplicitBroadSearch,
        )
    }
}
