use super::read_composition_walks::{
    build_bounded_descendant_collection_read_intent, build_bounded_descendant_detail_read_intent,
};
use super::{QuerySchemaView, WorthQueryReadBuilder, WorthQueryReadDenial};
use crate::authoring::{CollectionResultShapeBuilder, DetailResultShapeBuilder, RelationName};
use crate::runtime::read_composition_operator_builders::{
    CollectionReadOperatorQueryBuilder, DetailReadOperatorQueryBuilder,
};

impl<Output> WorthQueryReadBuilder<Output> {
    pub fn anchored_bounded_descendant_collection(
        self,
        root: impl Into<String>,
        schema_view: QuerySchemaView,
        relation: RelationName,
        max_depth: u8,
        declare_query: impl FnOnce(
            CollectionReadOperatorQueryBuilder,
        ) -> CollectionReadOperatorQueryBuilder,
        declare_result_shape: impl FnOnce(CollectionResultShapeBuilder) -> CollectionResultShapeBuilder,
    ) -> Result<Output, WorthQueryReadDenial> {
        self.finish(build_bounded_descendant_collection_read_intent(
            root,
            schema_view,
            relation,
            max_depth,
            declare_query,
            declare_result_shape,
        ))
    }

    pub fn anchored_bounded_descendant_detail(
        self,
        root: impl Into<String>,
        schema_view: QuerySchemaView,
        relation: RelationName,
        max_depth: u8,
        declare_query: impl FnOnce(DetailReadOperatorQueryBuilder) -> DetailReadOperatorQueryBuilder,
        declare_result_shape: impl FnOnce(DetailResultShapeBuilder) -> DetailResultShapeBuilder,
    ) -> Result<Output, WorthQueryReadDenial> {
        self.finish(build_bounded_descendant_detail_read_intent(
            root,
            schema_view,
            relation,
            max_depth,
            declare_query,
            declare_result_shape,
        ))
    }
}
