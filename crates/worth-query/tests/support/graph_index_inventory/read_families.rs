use worth_query::facade::runtime::{
    QuerySchemaView, SchemaFieldKind, SchemaFieldView, SchemaRelationView, WorthQueryReadFamily,
    WorthQueryWorkspace,
};
use worth_query::facade::{
    AspectFieldSelector, AuthoredResultShapeField, EqualityPredicate, ScalarPredicateValue,
    TraversalSelector,
};

pub fn traversal_collection_family(
    workspace: &mut WorthQueryWorkspace,
    family_name: &str,
) -> WorthQueryReadFamily {
    workspace
        .define_read_family(family_name, |read| {
            read.anchored_collection(
                "user",
                relation_schema(),
                |query| {
                    query
                        .traverse(traversal("manager", 2))
                        .project(field("identity", "id"))
                },
                |shape| shape.field(result_field("identity", "id", "id")),
            )
        })
        .expect("traversal collection family should admit")
}

pub fn predicate_collection_family(
    workspace: &mut WorthQueryWorkspace,
    family_name: &str,
) -> WorthQueryReadFamily {
    workspace
        .define_read_family(family_name, |read| {
            read.explicit_broad_search_collection(
                "user",
                relation_schema(),
                |query| {
                    query
                        .where_equal(equality("status", "value", "active"))
                        .project(field("identity", "id"))
                },
                |shape| shape.field(result_field("identity", "id", "id")),
            )
        })
        .expect("predicate collection family should admit")
}

fn relation_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "graph-index-inventory-test-schema",
        [
            SchemaFieldView::new(
                worth_query::facade::AspectName::new("identity")
                    .expect("schema aspect literal must be valid"),
                worth_query::facade::FieldName::new("id")
                    .expect("schema field literal must be valid"),
                SchemaFieldKind::String,
            ),
            SchemaFieldView::new(
                worth_query::facade::AspectName::new("status")
                    .expect("schema aspect literal must be valid"),
                worth_query::facade::FieldName::new("value")
                    .expect("schema field literal must be valid"),
                SchemaFieldKind::String,
            ),
        ],
        [SchemaRelationView::new(
            worth_query::facade::RelationName::new("manager")
                .expect("schema relation literal must be valid"),
            2,
        )],
    )
}

fn field(aspect: &str, field: &str) -> AspectFieldSelector {
    AspectFieldSelector::new(aspect, field).expect("field selector should build")
}

fn result_field(aspect: &str, field: &str, delivered: &str) -> AuthoredResultShapeField {
    AuthoredResultShapeField::new(aspect, field, delivered)
        .expect("result shape field should build")
}

fn traversal(name: &str, depth: u8) -> TraversalSelector {
    TraversalSelector::bounded(name, depth).expect("traversal selector should build")
}

fn equality(aspect: &str, field: &str, value: &str) -> EqualityPredicate {
    EqualityPredicate::new(
        aspect,
        field,
        ScalarPredicateValue::String(value.to_string()),
    )
    .expect("equality predicate should build")
}
