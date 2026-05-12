use super::read_composition_shared::{
    build_shared_attachment_collection_read_graph, build_shared_attachment_detail_read_graph,
    build_shared_endpoint_collection_read_graph, build_shared_endpoint_detail_read_graph,
};
use super::{ForgeQueryReadBuilder, ForgeQueryReadDenial, ForgeQueryReadGraph, QuerySchemaView};
use crate::authoring::{CollectionResultShapeBuilder, DetailResultShapeBuilder, RelationName};
use crate::runtime::read_composition_operator_builders::{
    CollectionReadOperatorQueryBuilder, DetailReadOperatorQueryBuilder,
};

impl ForgeQueryReadBuilder {
    pub fn local_shared_endpoint_collection(
        self,
        root: impl Into<String>,
        schema_view: QuerySchemaView,
        shared_relations: impl IntoIterator<Item = RelationName>,
        declare_query: impl FnOnce(
            CollectionReadOperatorQueryBuilder,
        ) -> CollectionReadOperatorQueryBuilder,
        declare_result_shape: impl FnOnce(CollectionResultShapeBuilder) -> CollectionResultShapeBuilder,
    ) -> Result<ForgeQueryReadGraph, ForgeQueryReadDenial> {
        build_shared_endpoint_collection_read_graph(
            root,
            schema_view,
            shared_relations,
            declare_query,
            declare_result_shape,
        )
    }

    pub fn local_shared_endpoint_detail(
        self,
        root: impl Into<String>,
        schema_view: QuerySchemaView,
        shared_relations: impl IntoIterator<Item = RelationName>,
        declare_query: impl FnOnce(DetailReadOperatorQueryBuilder) -> DetailReadOperatorQueryBuilder,
        declare_result_shape: impl FnOnce(DetailResultShapeBuilder) -> DetailResultShapeBuilder,
    ) -> Result<ForgeQueryReadGraph, ForgeQueryReadDenial> {
        build_shared_endpoint_detail_read_graph(
            root,
            schema_view,
            shared_relations,
            declare_query,
            declare_result_shape,
        )
    }

    pub fn local_shared_attachment_collection(
        self,
        root: impl Into<String>,
        schema_view: QuerySchemaView,
        shared_relations: impl IntoIterator<Item = RelationName>,
        declare_query: impl FnOnce(
            CollectionReadOperatorQueryBuilder,
        ) -> CollectionReadOperatorQueryBuilder,
        declare_result_shape: impl FnOnce(CollectionResultShapeBuilder) -> CollectionResultShapeBuilder,
    ) -> Result<ForgeQueryReadGraph, ForgeQueryReadDenial> {
        build_shared_attachment_collection_read_graph(
            root,
            schema_view,
            shared_relations,
            declare_query,
            declare_result_shape,
        )
    }

    pub fn local_shared_attachment_detail(
        self,
        root: impl Into<String>,
        schema_view: QuerySchemaView,
        shared_relations: impl IntoIterator<Item = RelationName>,
        declare_query: impl FnOnce(DetailReadOperatorQueryBuilder) -> DetailReadOperatorQueryBuilder,
        declare_result_shape: impl FnOnce(DetailResultShapeBuilder) -> DetailResultShapeBuilder,
    ) -> Result<ForgeQueryReadGraph, ForgeQueryReadDenial> {
        build_shared_attachment_detail_read_graph(
            root,
            schema_view,
            shared_relations,
            declare_query,
            declare_result_shape,
        )
    }
}
