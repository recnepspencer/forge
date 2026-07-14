use crate::authoring::{
    AspectFieldSelector, AspectName, AuthoredResultShapeField, DetailQueryBuilder,
    DetailResultShapeBuilder, FieldName, RootEntityKey,
};
use crate::composition::{
    QueryScopeDescriptor, QueryTemplateDescriptor, TemplateBindingSet, TemplateParameterSlot,
};
use crate::schema_view::{QuerySchemaView, SchemaFieldKind, SchemaFieldView};

pub(super) fn local_named_direct_read<Output>(
    read: crate::runtime::WorthQueryReadBuilder<Output>,
) -> Result<Output, crate::runtime::WorthQueryReadDenial> {
    read.local_detail(
        "user",
        named_schema(),
        |query| query.project(identity_selector()).project(name_selector()),
        |shape| shape.field(identity_field()).field(name_field()),
    )
}

pub(super) fn local_named_scoped_read<Output>(
    read: crate::runtime::WorthQueryReadBuilder<Output>,
) -> Result<Output, crate::runtime::WorthQueryReadDenial> {
    read.local_detail_scoped(
        identity_query(),
        named_shape(),
        named_schema(),
        [QueryScopeDescriptor::projection(
            "display-name",
            [name_selector()],
        )],
    )
}

pub(super) fn local_named_template_read<Output>(
    read: crate::runtime::WorthQueryReadBuilder<Output>,
) -> Result<Output, crate::runtime::WorthQueryReadDenial> {
    let slot = TemplateParameterSlot::projection("display-name");
    let template =
        QueryTemplateDescriptor::detail(identity_query(), named_shape()).with_slot(slot.clone());
    let bindings = TemplateBindingSet::new().bind_projection(&slot, name_selector());
    read.local_detail_template(template, bindings, named_schema())
}

fn identity_query() -> crate::authoring::DetailAuthoredQuery {
    DetailQueryBuilder::new(RootEntityKey::new("user").expect("root should author"))
        .project(identity_selector())
        .build()
        .expect("query should author")
}

fn named_shape() -> crate::authoring::DetailAuthoredResultShape {
    DetailResultShapeBuilder::new()
        .field(identity_field())
        .field(name_field())
        .build()
        .expect("shape should author")
}

fn named_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "ordinary-composed-named",
        [
            schema_field("identity", "id"),
            schema_field("profile", "display_name"),
        ],
        [],
    )
}

fn schema_field(aspect: &str, field: &str) -> SchemaFieldView {
    SchemaFieldView::new(
        AspectName::new(aspect).expect("aspect should author"),
        FieldName::new(field).expect("field should author"),
        SchemaFieldKind::String,
    )
}

fn identity_selector() -> AspectFieldSelector {
    AspectFieldSelector::new("identity", "id").expect("selector should author")
}

fn name_selector() -> AspectFieldSelector {
    AspectFieldSelector::new("profile", "display_name").expect("selector should author")
}

fn identity_field() -> AuthoredResultShapeField {
    AuthoredResultShapeField::new("identity", "id", "id").expect("field should author")
}

fn name_field() -> AuthoredResultShapeField {
    AuthoredResultShapeField::new("profile", "display_name", "display_name")
        .expect("field should author")
}
