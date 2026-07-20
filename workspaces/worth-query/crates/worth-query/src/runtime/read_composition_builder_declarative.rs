use crate::authoring::{
    CollectionAuthoredQuery, CollectionAuthoredResultShape, CollectionFamily,
    CollectionResultShapeFamily, DetailAuthoredQuery, DetailAuthoredResultShape, DetailFamily,
    DetailResultShapeFamily,
};
use crate::composition::{
    GuidedCompositionPath, QueryScopeDescriptor, QueryTemplateDescriptor, TemplateBindingSet,
};
use crate::ordinary::read::WorthQueryDeclaredReadIntent;

use super::{
    QuerySchemaView, WorthQueryReadBuilder, WorthQueryReadDenial, WorthQueryReadScopeClass,
};
use crate::runtime::read_composition_lowering::build_scoped_read_intent_from_composed;
use crate::runtime::{WorthQueryReadDenialKind, WorthQueryReadGraphFamily};

impl<Output> WorthQueryReadBuilder<Output> {
    pub fn local_detail_scoped(
        self,
        query: DetailAuthoredQuery,
        result_shape: DetailAuthoredResultShape,
        schema_view: QuerySchemaView,
        scopes: impl IntoIterator<Item = QueryScopeDescriptor>,
    ) -> Result<Output, WorthQueryReadDenial> {
        let (_, expanded) =
            GuidedCompositionPath::expand_detail_scopes(query, result_shape, scopes)
                .map_err(composition_denial)?;
        self.finish(composed_detail_intent(expanded, schema_view))
    }

    pub fn local_collection_scoped(
        self,
        query: CollectionAuthoredQuery,
        result_shape: CollectionAuthoredResultShape,
        schema_view: QuerySchemaView,
        scopes: impl IntoIterator<Item = QueryScopeDescriptor>,
    ) -> Result<Output, WorthQueryReadDenial> {
        let (_, expanded) =
            GuidedCompositionPath::expand_collection_scopes(query, result_shape, scopes)
                .map_err(composition_denial)?;
        self.finish(composed_collection_intent(expanded, schema_view))
    }

    pub fn local_detail_template(
        self,
        template: QueryTemplateDescriptor<DetailFamily, DetailResultShapeFamily>,
        bindings: TemplateBindingSet,
        schema_view: QuerySchemaView,
    ) -> Result<Output, WorthQueryReadDenial> {
        let (_, expanded) = GuidedCompositionPath::instantiate_detail_template(template, bindings)
            .map_err(composition_denial)?;
        self.finish(composed_detail_intent(expanded, schema_view))
    }

    pub fn local_collection_template(
        self,
        template: QueryTemplateDescriptor<CollectionFamily, CollectionResultShapeFamily>,
        bindings: TemplateBindingSet,
        schema_view: QuerySchemaView,
    ) -> Result<Output, WorthQueryReadDenial> {
        let (_, expanded) =
            GuidedCompositionPath::instantiate_collection_template(template, bindings)
                .map_err(composition_denial)?;
        self.finish(composed_collection_intent(expanded, schema_view))
    }
}

fn composed_detail_intent(
    expanded: crate::composition::ExpandedComposedIntent,
    schema_view: QuerySchemaView,
) -> Result<WorthQueryDeclaredReadIntent, WorthQueryReadDenial> {
    build_scoped_read_intent_from_composed(
        expanded,
        schema_view,
        WorthQueryReadGraphFamily::Detail,
        WorthQueryReadScopeClass::LocalNeighborhood,
    )
}

fn composed_collection_intent(
    expanded: crate::composition::ExpandedComposedIntent,
    schema_view: QuerySchemaView,
) -> Result<WorthQueryDeclaredReadIntent, WorthQueryReadDenial> {
    build_scoped_read_intent_from_composed(
        expanded,
        schema_view,
        WorthQueryReadGraphFamily::Collection,
        WorthQueryReadScopeClass::LocalNeighborhood,
    )
}

fn composition_denial(error: impl std::fmt::Debug) -> WorthQueryReadDenial {
    WorthQueryReadDenial::new(
        WorthQueryReadDenialKind::AuthoringDenied,
        format!("{error:?}"),
    )
}
