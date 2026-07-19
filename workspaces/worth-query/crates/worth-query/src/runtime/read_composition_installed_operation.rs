use super::read_composition::WorthQueryReadBuilder;
use super::read_composition_lowering::{build_collection_read_intent, build_detail_read_intent};
use super::{QuerySchemaView, WorthQueryReadDenial, WorthQueryReadScopeClass};
use crate::authoring::{
    CollectionQueryBuilder, CollectionResultShapeBuilder, DetailQueryBuilder,
    DetailResultShapeBuilder,
};
use crate::domain_installation::WorthQueryInstalledGraphReadOperation;

impl<Output> WorthQueryReadBuilder<Output> {
    /// Declares a local collection read and seals its exact installed-operation
    /// authority into the resulting read intent before the surrounding context
    /// can admit or plan it.
    pub fn local_collection_with_installed_operation(
        self,
        operation: WorthQueryInstalledGraphReadOperation,
        root: impl Into<String>,
        schema_view: QuerySchemaView,
        declare_query: impl FnOnce(CollectionQueryBuilder) -> CollectionQueryBuilder,
        declare_result_shape: impl FnOnce(CollectionResultShapeBuilder) -> CollectionResultShapeBuilder,
    ) -> Result<Output, WorthQueryReadDenial> {
        let authored_operation = operation.clone();
        let intent = build_collection_read_intent(
            root,
            schema_view,
            |query| authored_operation.author(declare_query(query)),
            declare_result_shape,
            WorthQueryReadScopeClass::LocalNeighborhood,
        )
        .and_then(|intent| {
            intent
                .bind_installed_operation(operation)
                .map_err(WorthQueryReadDenial::new_installed_operation_binding_denied)
        });
        self.finish(intent)
    }

    /// Declares a local detail read and seals its exact installed-operation
    /// authority into the resulting read intent before the surrounding context
    /// can admit or plan it.
    pub fn local_detail_with_installed_operation(
        self,
        operation: WorthQueryInstalledGraphReadOperation,
        root: impl Into<String>,
        schema_view: QuerySchemaView,
        declare_query: impl FnOnce(DetailQueryBuilder) -> DetailQueryBuilder,
        declare_result_shape: impl FnOnce(DetailResultShapeBuilder) -> DetailResultShapeBuilder,
    ) -> Result<Output, WorthQueryReadDenial> {
        let authored_operation = operation.clone();
        let intent = build_detail_read_intent(
            root,
            schema_view,
            |query| authored_operation.author(declare_query(query)),
            declare_result_shape,
            WorthQueryReadScopeClass::LocalNeighborhood,
        )
        .and_then(|intent| {
            intent
                .bind_installed_operation(operation)
                .map_err(WorthQueryReadDenial::new_installed_operation_binding_denied)
        });
        self.finish(intent)
    }
}
