use forge_query::facade::runtime::{
    explain_graph_read_access_requirements_for_family, ForgeQueryGraphReadAccessRequirementKind,
    QuerySchemaView, SchemaFieldKind, SchemaFieldView, SchemaRelationView,
};
use forge_query::facade::{AspectFieldSelector, AuthoredResultShapeField, TraversalSelector};

mod support;

use support::public_bridge_runtime::PublicBridgeRuntimeHarness;

#[test]
fn every_traversal_bearing_read_derives_access_structure_requirements() {
    let harness = PublicBridgeRuntimeHarness::new();
    let runtime = harness.bridge_backed_runtime();
    let mut workspace = runtime
        .workspace("graph-read-access.phase-four.completeness")
        .expect("runtime should open workspace");
    let family = workspace
        .define_read_family("phase-four-completeness", |read| {
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
        .expect("traversal family should admit");

    let requirements = explain_graph_read_access_requirements_for_family(&family)
        .expect("requirements should derive");
    let has_access_structure = requirements
        .requires_kind(ForgeQueryGraphReadAccessRequirementKind::DirectionalAdjacency)
        || requirements.requires_kind(ForgeQueryGraphReadAccessRequirementKind::ReverseAdjacency)
        || requirements.requires_kind(ForgeQueryGraphReadAccessRequirementKind::TraversalWorkset);

    assert!(has_access_structure);
    assert!(requirements.counters().row_count() >= 1);
}

#[test]
fn requirement_counters_explain_structural_breadth_not_elapsed_time() {
    let harness = PublicBridgeRuntimeHarness::new();
    let runtime = harness.bridge_backed_runtime();
    let mut workspace = runtime
        .workspace("graph-read-access.phase-four.counters")
        .expect("runtime should open workspace");
    let family = workspace
        .define_read_family("phase-four-counters", |read| {
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
        .expect("traversal family should admit");

    let requirements = explain_graph_read_access_requirements_for_family(&family)
        .expect("requirements should derive");

    assert_eq!(
        requirements.counters().row_count(),
        requirements.rows().len()
    );
    assert!(requirements.counters().directional_adjacency_count() > 0);
    assert!(requirements.counters().workset_count() > 0);
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

fn relation_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "graph-read-access-phase-four-relation",
        [
            SchemaFieldView::new(
                forge_query::facade::AspectName::new("identity")
                    .expect("schema aspect literal must be valid"),
                forge_query::facade::FieldName::new("id")
                    .expect("schema field literal must be valid"),
                SchemaFieldKind::String,
            ),
            SchemaFieldView::new(
                forge_query::facade::AspectName::new("profile")
                    .expect("schema aspect literal must be valid"),
                forge_query::facade::FieldName::new("display_name")
                    .expect("schema field literal must be valid"),
                SchemaFieldKind::String,
            ),
        ],
        [SchemaRelationView::new(
            forge_query::facade::RelationName::new("manager")
                .expect("schema relation literal must be valid"),
            2,
        )],
    )
}
