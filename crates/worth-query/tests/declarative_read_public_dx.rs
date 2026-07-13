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
