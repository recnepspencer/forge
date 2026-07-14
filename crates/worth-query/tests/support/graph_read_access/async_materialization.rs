use worth_query::facade::foundation::{
    AspectFieldSelector, AuthoredResultShapeField, EqualityPredicate, OrderingSelector,
    ScalarPredicateValue, TraversalSelector,
};
use worth_query::facade::runtime::{
    QuerySchemaView, SchemaFieldKind, SchemaFieldView, SchemaRelationView, WorthQueryReadFamily,
    WorthQueryWorkspace,
};

use crate::support::public_bridge_runtime::PublicBridgeRuntimeHarness;

pub fn async_materialization_workspace(name: &str) -> WorthQueryWorkspace {
    PublicBridgeRuntimeHarness::new()
        .bridge_backed_runtime()
        .workspace(name)
        .expect("runtime should open workspace")
}

pub fn async_required_graph_read_family(
    workspace: &mut WorthQueryWorkspace,
    name: &str,
) -> WorthQueryReadFamily {
    workspace
        .define_read_family(name, |read| {
            read.explicit_broad_search_collection(
                "user",
                graph_read_materialization_schema(),
                |query| {
                    query
                        .traverse(traversal("manager", 8))
                        .where_equal(equality("status", "value", "active"))
                        .project(field("identity", "id"))
                        .order_by(
                            OrderingSelector::ascending("profile", "display_name")
                                .expect("ordering should build"),
                        )
                },
                |shape| shape.field(result_field("identity", "id", "id")),
            )
        })
        .expect("async-required graph read family should admit")
}

pub fn inline_graph_read_family(
    workspace: &mut WorthQueryWorkspace,
    name: &str,
) -> WorthQueryReadFamily {
    workspace
        .define_read_family(name, |read| {
            read.local_collection(
                "user",
                graph_read_materialization_schema(),
                |query| query.project(field("identity", "id")),
                |shape| shape.field(result_field("identity", "id", "id")),
            )
        })
        .expect("inline graph read family should admit")
}

fn graph_read_materialization_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "graph-read-access-async-materialization-schema",
        [
            SchemaFieldView::new(
                worth_query::facade::foundation::AspectName::new("identity")
                    .expect("schema aspect literal must be valid"),
                worth_query::facade::foundation::FieldName::new("id")
                    .expect("schema field literal must be valid"),
                SchemaFieldKind::String,
            ),
            SchemaFieldView::new(
                worth_query::facade::foundation::AspectName::new("profile")
                    .expect("schema aspect literal must be valid"),
                worth_query::facade::foundation::FieldName::new("display_name")
                    .expect("schema field literal must be valid"),
                SchemaFieldKind::String,
            ),
            SchemaFieldView::new(
                worth_query::facade::foundation::AspectName::new("status")
                    .expect("schema aspect literal must be valid"),
                worth_query::facade::foundation::FieldName::new("value")
                    .expect("schema field literal must be valid"),
                SchemaFieldKind::String,
            ),
        ],
        [SchemaRelationView::new(
            worth_query::facade::foundation::RelationName::new("manager")
                .expect("schema relation literal must be valid"),
            8,
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
