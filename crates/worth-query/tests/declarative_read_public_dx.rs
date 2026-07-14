use worth_query::facade::aggregate::declare as declare_aggregate;
use worth_query::facade::live::declare as declare_live;
use worth_query::facade::read::{
    current, declare, AspectFieldSelector, AspectName, AuthoredResultShapeField, FieldName,
    QuerySchemaView, SchemaFieldKind, SchemaFieldView, WorthQueryReadContextKind,
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
