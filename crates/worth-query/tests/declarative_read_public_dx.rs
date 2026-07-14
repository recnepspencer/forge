use worth_query::facade::aggregate::declare as declare_aggregate;
use worth_query::facade::live::declare as declare_live;
use worth_query::facade::read::{
    current, declare, AspectFieldSelector, AspectName, AuthoredResultShapeField,
    DetailQueryBuilder, DetailResultShapeBuilder, FieldName, QuerySchemaView, QueryScopeDescriptor,
    QueryTemplateDescriptor, RootEntityKey, SchemaFieldKind, SchemaFieldView, TemplateBindingSet,
    TemplateParameterSlot, WorthQueryReadContextKind,
};

#[test]
fn read_declaration_uses_only_the_read_capability_namespace() {
    let declaration = declare(|read| {
        read.local_detail(
            "user",
            QuerySchemaView::new(
                "public-dx-read",
                [SchemaFieldView::new(
                    AspectName::new("identity").expect("aspect should build"),
                    FieldName::new("id").expect("field should build"),
                    SchemaFieldKind::String,
                )],
                [],
            ),
            |query| {
                query.project(
                    AspectFieldSelector::new("identity", "id").expect("projection should build"),
                )
            },
            |shape| {
                shape.field(
                    AuthoredResultShapeField::new("identity", "id", "id")
                        .expect("result field should build"),
                )
            },
        )
    })
    .expect("declaration should canonicalize");

    assert!(!declaration.identity().as_str().is_empty());
    let request = declaration.using(current());
    assert_eq!(request.context_kind(), WorthQueryReadContextKind::Current);
}

#[test]
fn count_declaration_is_a_typed_collection_capability() {
    let declaration = declare_aggregate(|read| {
        read.local_collection(
            "user",
            QuerySchemaView::new(
                "public-dx-count",
                [SchemaFieldView::new(
                    AspectName::new("identity").expect("aspect should build"),
                    FieldName::new("id").expect("field should build"),
                    SchemaFieldKind::String,
                )],
                [],
            ),
            |query| {
                query.project(
                    AspectFieldSelector::new("identity", "id").expect("projection should build"),
                )
            },
            |shape| {
                shape.field(
                    AuthoredResultShapeField::new("identity", "id", "id")
                        .expect("result field should build"),
                )
            },
        )
    })
    .expect("count declaration should canonicalize");

    assert!(!declaration.identity().as_str().is_empty());
    assert_eq!(
        declaration.using(current()).context_kind(),
        WorthQueryReadContextKind::Current
    );
}

#[test]
fn managed_live_declaration_uses_the_same_read_grammar_before_open() {
    let declaration = declare_live("users.current", |read| {
        read.local_collection(
            "user",
            QuerySchemaView::new(
                "public-dx-managed-live",
                [SchemaFieldView::new(
                    AspectName::new("identity").expect("aspect should build"),
                    FieldName::new("id").expect("field should build"),
                    SchemaFieldKind::String,
                )],
                [],
            ),
            |query| {
                query.project(
                    AspectFieldSelector::new("identity", "id").expect("projection should build"),
                )
            },
            |shape| {
                shape.field(
                    AuthoredResultShapeField::new("identity", "id", "id")
                        .expect("result field should build"),
                )
            },
        )
    })
    .expect("managed live declaration should canonicalize");

    assert_eq!(declaration.name(), "users.current");
    assert!(!declaration.identity().as_str().is_empty());
    assert_eq!(
        declaration.using(current()).context_kind(),
        WorthQueryReadContextKind::Current
    );
}

#[test]
fn ordinary_facade_converges_direct_scope_and_template_authoring() {
    let direct = declare(|read| {
        read.local_detail(
            "user",
            public_composition_schema(),
            |query| {
                query
                    .project(public_identity_selector())
                    .project(public_name_selector())
            },
            |shape| {
                shape
                    .field(public_identity_field())
                    .field(public_name_field())
            },
        )
    })
    .expect("direct declaration should canonicalize");

    let scoped = declare(|read| {
        read.local_detail_scoped(
            public_identity_query(),
            public_composition_shape(),
            public_composition_schema(),
            [QueryScopeDescriptor::projection(
                "display-name",
                [public_name_selector()],
            )],
        )
    })
    .expect("scoped declaration should canonicalize");

    let slot = TemplateParameterSlot::projection("display-name");
    let template =
        QueryTemplateDescriptor::detail(public_identity_query(), public_composition_shape())
            .with_slot(slot.clone());
    let bindings = TemplateBindingSet::new().bind_projection(&slot, public_name_selector());
    let templated =
        declare(|read| read.local_detail_template(template, bindings, public_composition_schema()))
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

fn public_identity_query() -> worth_query::facade::read::DetailAuthoredQuery {
    DetailQueryBuilder::new(RootEntityKey::new("user").expect("root should build"))
        .project(public_identity_selector())
        .build()
        .expect("query should build")
}

fn public_composition_shape() -> worth_query::facade::read::DetailAuthoredResultShape {
    DetailResultShapeBuilder::new()
        .field(public_identity_field())
        .field(public_name_field())
        .build()
        .expect("shape should build")
}

fn public_composition_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "public-composition",
        [
            SchemaFieldView::new(
                AspectName::new("identity").expect("aspect should build"),
                FieldName::new("id").expect("field should build"),
                SchemaFieldKind::String,
            ),
            SchemaFieldView::new(
                AspectName::new("profile").expect("aspect should build"),
                FieldName::new("display_name").expect("field should build"),
                SchemaFieldKind::String,
            ),
        ],
        [],
    )
}

fn public_identity_selector() -> AspectFieldSelector {
    AspectFieldSelector::new("identity", "id").expect("selector should build")
}

fn public_name_selector() -> AspectFieldSelector {
    AspectFieldSelector::new("profile", "display_name").expect("selector should build")
}

fn public_identity_field() -> AuthoredResultShapeField {
    AuthoredResultShapeField::new("identity", "id", "id").expect("field should build")
}

fn public_name_field() -> AuthoredResultShapeField {
    AuthoredResultShapeField::new("profile", "display_name", "display_name")
        .expect("field should build")
}
