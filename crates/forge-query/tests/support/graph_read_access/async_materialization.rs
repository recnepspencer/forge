use forge_query::facade::runtime::{
    ForgeQueryReadFamily, ForgeQueryWorkspace, QuerySchemaView, SchemaFieldKind, SchemaFieldView,
    SchemaRelationView,
};
use forge_query::facade::{
    AspectFieldSelector, AuthoredResultShapeField, EqualityPredicate, OrderingSelector,
    ScalarPredicateValue, TraversalSelector,
};

use crate::support::public_bridge_runtime::PublicBridgeRuntimeHarness;

pub fn async_materialization_workspace(name: &str) -> ForgeQueryWorkspace {
    PublicBridgeRuntimeHarness::new()
        .bridge_backed_runtime()
        .workspace(name)
        .expect("runtime should open workspace")
}

pub fn async_required_graph_read_family(
    workspace: &mut ForgeQueryWorkspace,
    name: &str,
) -> ForgeQueryReadFamily {
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
    workspace: &mut ForgeQueryWorkspace,
    name: &str,
) -> ForgeQueryReadFamily {
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
            SchemaFieldView::new("identity", "id", SchemaFieldKind::String),
            SchemaFieldView::new("profile", "display_name", SchemaFieldKind::String),
            SchemaFieldView::new("status", "value", SchemaFieldKind::String),
        ],
        [SchemaRelationView::new("manager", 8)],
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
