mod read_journeys {
    use worth_query::facade::read::{
        current, declare, AspectFieldSelector, AspectName, AuthoredResultShapeField,
        DetailQueryBuilder, DetailResultShapeBuilder, FieldName, QuerySchemaView,
        QueryScopeDescriptor, QueryTemplateDescriptor, RootEntityKey, ScalarAspectType,
        SchemaFieldView, TemplateBindingSet, TemplateParameterSlot, WorthQueryReadContextKind,
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
                        ScalarAspectType::String,
                    )],
                    [],
                ),
                |query| {
                    query.project(
                        AspectFieldSelector::new("identity", "id")
                            .expect("projection should build"),
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
    fn ordinary_facade_converges_direct_scope_and_template_authoring() {
        let direct = declare(|read| {
            read.local_detail(
                "user",
                composition_schema(),
                |query| query.project(identity_selector()).project(name_selector()),
                |shape| shape.field(identity_field()).field(name_field()),
            )
        })
        .expect("direct declaration should canonicalize");

        let scoped = declare(|read| {
            read.local_detail_scoped(
                identity_query(),
                composition_shape(),
                composition_schema(),
                [QueryScopeDescriptor::projection(
                    "display-name",
                    [name_selector()],
                )],
            )
        })
        .expect("scoped declaration should canonicalize");

        let slot = TemplateParameterSlot::projection("display-name");
        let template = QueryTemplateDescriptor::detail(identity_query(), composition_shape())
            .with_slot(slot.clone());
        let bindings = TemplateBindingSet::new().bind_projection(&slot, name_selector());
        let templated =
            declare(|read| read.local_detail_template(template, bindings, composition_schema()))
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

    fn identity_query() -> worth_query::facade::read::DetailAuthoredQuery {
        DetailQueryBuilder::new(RootEntityKey::new("user").expect("root should build"))
            .project(identity_selector())
            .build()
            .expect("query should build")
    }

    fn composition_shape() -> worth_query::facade::read::DetailAuthoredResultShape {
        DetailResultShapeBuilder::new()
            .field(identity_field())
            .field(name_field())
            .build()
            .expect("shape should build")
    }

    fn composition_schema() -> QuerySchemaView {
        QuerySchemaView::new(
            "public-composition",
            [
                SchemaFieldView::new(
                    AspectName::new("identity").expect("aspect should build"),
                    FieldName::new("id").expect("field should build"),
                    ScalarAspectType::String,
                ),
                SchemaFieldView::new(
                    AspectName::new("profile").expect("aspect should build"),
                    FieldName::new("display_name").expect("field should build"),
                    ScalarAspectType::String,
                ),
            ],
            [],
        )
    }

    fn identity_selector() -> AspectFieldSelector {
        AspectFieldSelector::new("identity", "id").expect("selector should build")
    }

    fn name_selector() -> AspectFieldSelector {
        AspectFieldSelector::new("profile", "display_name").expect("selector should build")
    }

    fn identity_field() -> AuthoredResultShapeField {
        AuthoredResultShapeField::new("identity", "id", "id").expect("field should build")
    }

    fn name_field() -> AuthoredResultShapeField {
        AuthoredResultShapeField::new("profile", "display_name", "display_name")
            .expect("field should build")
    }
}

mod aggregate_journey {
    use worth_query::facade::aggregate::{
        current, declare, AspectFieldSelector, AspectName, AuthoredResultShapeField, FieldName,
        QuerySchemaView, ScalarAspectType, SchemaFieldView, WorthQueryReadContextKind,
    };

    #[test]
    fn count_declaration_uses_only_the_aggregate_capability_namespace() {
        let declaration = declare(|read| {
            read.local_collection(
                "user",
                QuerySchemaView::new(
                    "public-dx-count",
                    [SchemaFieldView::new(
                        AspectName::new("identity").expect("aspect should build"),
                        FieldName::new("id").expect("field should build"),
                        ScalarAspectType::String,
                    )],
                    [],
                ),
                |query| {
                    query.project(
                        AspectFieldSelector::new("identity", "id")
                            .expect("projection should build"),
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
}

mod live_declaration_prefix {
    use worth_query::facade::live::{
        current, declare, AspectFieldSelector, AspectName, AuthoredResultShapeField, FieldName,
        QuerySchemaView, ScalarAspectType, SchemaFieldView, WorthQueryReadContextKind,
    };

    #[test]
    fn managed_live_prefix_uses_only_the_live_capability_namespace() {
        let declaration = declare("users.current", |read| {
            read.local_collection(
                "user",
                QuerySchemaView::new(
                    "public-dx-managed-live",
                    [SchemaFieldView::new(
                        AspectName::new("identity").expect("aspect should build"),
                        FieldName::new("id").expect("field should build"),
                        ScalarAspectType::String,
                    )],
                    [],
                ),
                |query| {
                    query.project(
                        AspectFieldSelector::new("identity", "id")
                            .expect("projection should build"),
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
}
