use super::{declare, WorthQueryReadNextAction};
use crate::authoring::{
    AspectFieldSelector, AspectName, AuthoredResultShapeField, DetailQueryBuilder,
    DetailResultShapeBuilder, FieldName, RootEntityKey,
};
use crate::composition::{
    QueryScopeDescriptor, QueryTemplateDescriptor, TemplateBindingSet, TemplateParameterSlot,
};
use crate::schema_view::{QuerySchemaView, SchemaFieldKind, SchemaFieldView};

#[test]
fn equivalent_read_declarations_converge_on_one_canonical_identity() {
    let first = declare(local_identity_read).expect("first declaration should canonicalize");
    let second = declare(local_identity_read).expect("second declaration should canonicalize");

    assert_eq!(first.identity(), second.identity());
}

#[test]
fn direct_scope_and_template_declarations_share_canonical_identities() {
    let direct = declare(local_named_read).expect("direct declaration should canonicalize");
    let scoped = declare(|read| {
        read.local_detail_scoped(
            identity_only_query(),
            named_result_shape(),
            named_schema(),
            [QueryScopeDescriptor::projection(
                "named_fields",
                [name_selector()],
            )],
        )
    })
    .expect("scoped declaration should canonicalize");

    let slot = TemplateParameterSlot::projection("name_projection");
    let template = QueryTemplateDescriptor::detail(identity_only_query(), named_result_shape())
        .with_slot(slot.clone());
    let bindings = TemplateBindingSet::new().bind_projection(&slot, name_selector());
    let templated = declare(|read| read.local_detail_template(template, bindings, named_schema()))
        .expect("template declaration should canonicalize");

    for declaration in [&scoped, &templated] {
        assert_eq!(
            declaration.identity().canonical_query_digest(),
            direct.identity().canonical_query_digest()
        );
        assert_eq!(
            declaration.identity().canonical_result_shape_digest(),
            direct.identity().canonical_result_shape_digest()
        );
    }
}

#[test]
fn invalid_scope_shape_stops_during_declaration_without_a_workspace() {
    let stop = declare(|read| {
        read.anchored_detail(
            "user",
            identity_schema(),
            |query| query.project(identity_selector()),
            |shape| shape.field(identity_result_field()),
        )
    })
    .expect_err("anchored reads without traversal must stop during declaration");

    assert_eq!(
        stop.next_action(),
        WorthQueryReadNextAction::ReviseDeclaration
    );
    assert_eq!(
        stop.denial().kind(),
        &crate::runtime::WorthQueryReadDenialKind::ScopeShapeDenied
    );
}

fn local_identity_read<Output>(
    read: crate::runtime::WorthQueryReadBuilder<Output>,
) -> Result<Output, crate::runtime::WorthQueryReadDenial> {
    read.local_detail(
        "user",
        identity_schema(),
        |query| query.project(identity_selector()),
        |shape| shape.field(identity_result_field()),
    )
}

fn identity_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "ordinary-read-identity",
        [SchemaFieldView::new(
            AspectName::new("identity").expect("test aspect must be valid"),
            FieldName::new("id").expect("test field must be valid"),
            SchemaFieldKind::String,
        )],
        [],
    )
}

fn identity_selector() -> AspectFieldSelector {
    AspectFieldSelector::new("identity", "id").expect("test selector must be valid")
}

fn identity_result_field() -> AuthoredResultShapeField {
    AuthoredResultShapeField::new("identity", "id", "id").expect("test result field must be valid")
}

fn local_named_read<Output>(
    read: crate::runtime::WorthQueryReadBuilder<Output>,
) -> Result<Output, crate::runtime::WorthQueryReadDenial> {
    read.local_detail(
        "user",
        named_schema(),
        |query| query.project(identity_selector()).project(name_selector()),
        |shape| {
            shape
                .field(identity_result_field())
                .field(name_result_field())
        },
    )
}

fn identity_only_query() -> crate::authoring::DetailAuthoredQuery {
    DetailQueryBuilder::new(RootEntityKey::new("user").expect("test root must be valid"))
        .project(identity_selector())
        .build()
        .expect("base query must be valid")
}

fn named_result_shape() -> crate::authoring::DetailAuthoredResultShape {
    DetailResultShapeBuilder::new()
        .field(identity_result_field())
        .field(name_result_field())
        .build()
        .expect("result shape must be valid")
}

fn named_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "ordinary-read-named",
        [
            SchemaFieldView::new(
                AspectName::new("identity").expect("test aspect must be valid"),
                FieldName::new("id").expect("test field must be valid"),
                SchemaFieldKind::String,
            ),
            SchemaFieldView::new(
                AspectName::new("profile").expect("test aspect must be valid"),
                FieldName::new("display_name").expect("test field must be valid"),
                SchemaFieldKind::String,
            ),
        ],
        [],
    )
}

fn name_selector() -> AspectFieldSelector {
    AspectFieldSelector::new("profile", "display_name").expect("test selector must be valid")
}

fn name_result_field() -> AuthoredResultShapeField {
    AuthoredResultShapeField::new("profile", "display_name", "display_name")
        .expect("test result field must be valid")
}
