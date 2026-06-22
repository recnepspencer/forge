use forge_query::facade::runtime::{
    ForgeQueryGraphReadAccessCostEstimate, ForgeQueryGraphReadAccessRequirementKind,
    ForgeQueryGraphReadCostAttributionRow, ForgeQueryGraphReadMemoryByteEstimate,
    ForgeQueryWorkspace, QuerySchemaView, SchemaFieldKind, SchemaFieldView, SchemaRelationView,
};
use forge_query::facade::{
    AspectFieldSelector, AuthoredResultShapeField, EqualityPredicate, OrderingSelector,
    PresencePredicate, RelationName, ScalarPredicateValue, TraversalSelector,
};

#[path = "support/mod.rs"]
pub mod support;

use crate::support::public_bridge_runtime::PublicBridgeRuntimeHarness;

pub fn workspace(name: &str) -> ForgeQueryWorkspace {
    PublicBridgeRuntimeHarness::new()
        .bridge_backed_runtime()
        .workspace(name)
        .expect("runtime should open workspace")
}

pub fn simple_traversal_family(
    workspace: &mut ForgeQueryWorkspace,
    name: &str,
) -> forge_query::facade::runtime::ForgeQueryReadFamily {
    workspace
        .define_read_family(name, |read| {
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
        .expect("simple traversal family should admit")
}

pub fn projection_only_family(
    workspace: &mut ForgeQueryWorkspace,
    name: &str,
) -> forge_query::facade::runtime::ForgeQueryReadFamily {
    workspace
        .define_read_family(name, |read| {
            read.local_collection(
                "user",
                relation_schema(),
                |query| query.project(field("identity", "id")),
                |shape| shape.field(result_field("identity", "id", "id")),
            )
        })
        .expect("projection-only family should admit")
}

pub fn reordered_simple_traversal_family(
    workspace: &mut ForgeQueryWorkspace,
    name: &str,
) -> forge_query::facade::runtime::ForgeQueryReadFamily {
    workspace
        .define_read_family(name, |read| {
            read.anchored_collection(
                "user",
                relation_schema(),
                |query| {
                    query
                        .project(field("identity", "id"))
                        .traverse(traversal("manager", 2))
                },
                |shape| shape.field(result_field("identity", "id", "id")),
            )
        })
        .expect("reordered simple traversal family should admit")
}

pub fn dense_traversal_family(
    workspace: &mut ForgeQueryWorkspace,
    name: &str,
) -> forge_query::facade::runtime::ForgeQueryReadFamily {
    workspace
        .define_read_family(name, |read| {
            read.explicit_broad_search_collection(
                "user",
                relation_schema(),
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
        .expect("dense traversal family should admit")
}

pub fn intermediate_pressure_family(
    workspace: &mut ForgeQueryWorkspace,
    name: &str,
) -> forge_query::facade::runtime::ForgeQueryReadFamily {
    workspace
        .define_read_family(name, |read| {
            read.explicit_broad_search_collection(
                "user",
                relation_schema(),
                |query| {
                    query
                        .traverse(traversal("manager", 1))
                        .where_present(presence("profile", "display_name"))
                        .project(field("identity", "id"))
                },
                |shape| shape.field(result_field("identity", "id", "id")),
            )
        })
        .expect("intermediate pressure family should admit")
}

pub fn frontier_search_family(
    workspace: &mut ForgeQueryWorkspace,
    name: &str,
) -> forge_query::facade::runtime::ForgeQueryReadFamily {
    workspace
        .define_read_family(name, |read| {
            read.explicit_broad_search_frontier_collection(
                "user",
                two_relation_schema(),
                [relation("manager"), relation("mentor")],
                2,
                |query| {
                    query
                        .project(field("identity", "id"))
                        .where_equal(equality("status", "value", "active"))
                        .order_by(
                            OrderingSelector::ascending("profile", "display_name")
                                .expect("ordering should build"),
                        )
                },
                |shape| shape.field(result_field("identity", "id", "id")),
            )
        })
        .expect("frontier search family should admit")
}

pub fn bucket_sum(
    estimate: &ForgeQueryGraphReadAccessCostEstimate,
    read_bucket: impl Fn(&ForgeQueryGraphReadMemoryByteEstimate) -> usize,
) -> usize {
    estimate
        .attribution_rows()
        .iter()
        .map(|row| read_bucket(row.supported().memory()))
        .sum()
}

pub fn assert_exact_bucket_contribution(
    estimate: &ForgeQueryGraphReadAccessCostEstimate,
    kind: ForgeQueryGraphReadAccessRequirementKind,
    read_bucket: impl Fn(&ForgeQueryGraphReadMemoryByteEstimate) -> usize,
    expected_bytes: usize,
) {
    let row = attribution_row(estimate.attribution_rows(), kind);
    assert_eq!(read_bucket(row.supported().memory()), expected_bytes);
}

fn attribution_row(
    rows: &[ForgeQueryGraphReadCostAttributionRow],
    kind: ForgeQueryGraphReadAccessRequirementKind,
) -> &ForgeQueryGraphReadCostAttributionRow {
    rows.iter()
        .find(|row| row.requirement_kind() == &kind)
        .expect("expected attribution row should exist")
}

fn field(aspect: &str, field: &str) -> AspectFieldSelector {
    AspectFieldSelector::new(aspect, field).expect("field selector should build")
}

fn result_field(aspect: &str, field: &str, delivered: &str) -> AuthoredResultShapeField {
    AuthoredResultShapeField::new(aspect, field, delivered)
        .expect("result-shape field should build")
}

fn traversal(name: &str, depth: u8) -> TraversalSelector {
    TraversalSelector::bounded(name, depth).expect("traversal selector should build")
}

fn relation(name: &str) -> RelationName {
    RelationName::new(name).expect("relation name should build")
}

fn equality(aspect: &str, field: &str, value: &str) -> EqualityPredicate {
    EqualityPredicate::new(
        aspect,
        field,
        ScalarPredicateValue::String(value.to_string()),
    )
    .expect("equality predicate should build")
}

fn presence(aspect: &str, field: &str) -> PresencePredicate {
    PresencePredicate::is_present(aspect, field).expect("presence predicate should build")
}

fn relation_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "graph-read-access-phase-five-relation",
        [
            SchemaFieldView::new("identity", "id", SchemaFieldKind::String),
            SchemaFieldView::new("profile", "display_name", SchemaFieldKind::String)
                .presence_predicate_queryable(),
            SchemaFieldView::new("status", "value", SchemaFieldKind::String),
        ],
        [SchemaRelationView::new("manager", 8)],
    )
}

fn two_relation_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "graph-read-access-phase-five-two-relation",
        [
            SchemaFieldView::new("identity", "id", SchemaFieldKind::String),
            SchemaFieldView::new("profile", "display_name", SchemaFieldKind::String),
            SchemaFieldView::new("status", "value", SchemaFieldKind::String),
        ],
        [
            SchemaRelationView::new("manager", 2),
            SchemaRelationView::new("mentor", 2),
        ],
    )
}
