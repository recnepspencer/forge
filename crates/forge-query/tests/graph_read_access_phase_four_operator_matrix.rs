use forge_query::facade::runtime::{
    explain_graph_read_access_requirements_for_family, ForgeQueryGraphReadAccessRequirementKind,
    QuerySchemaView, SchemaFieldKind, SchemaFieldView, SchemaRelationView,
};
use forge_query::facade::{
    AspectFieldSelector, AuthoredResultShapeField, EqualityPredicate, RelationName,
    ScalarPredicateValue,
};

mod support;

use support::public_bridge_runtime::PublicBridgeRuntimeHarness;

#[test]
fn operator_matrix_derives_the_expected_structural_requirement_kinds() {
    for (name, required_kinds) in operator_cases() {
        let mut workspace = workspace(&format!("graph-read-access.phase-four.{name}"));
        let family = operator_family(&mut workspace, name);
        let requirements = explain_graph_read_access_requirements_for_family(&family)
            .expect("operator requirements should derive");

        for required_kind in required_kinds {
            assert!(
                requirements.requires_kind(required_kind.clone()),
                "{name} should require {}",
                required_kind.as_str()
            );
        }
    }
}

fn operator_cases() -> Vec<(&'static str, Vec<ForgeQueryGraphReadAccessRequirementKind>)> {
    use ForgeQueryGraphReadAccessRequirementKind as Kind;
    vec![
        (
            "direct-edge",
            vec![Kind::DirectionalAdjacency, Kind::ResultBuffer],
        ),
        (
            "successor-walk",
            vec![
                Kind::DirectionalAdjacency,
                Kind::TraversalWorkset,
                Kind::VisitedSet,
            ],
        ),
        (
            "bounded-ancestor",
            vec![
                Kind::ReverseAdjacency,
                Kind::TraversalWorkset,
                Kind::VisitedSet,
            ],
        ),
        (
            "bounded-descendant",
            vec![
                Kind::DirectionalAdjacency,
                Kind::TraversalWorkset,
                Kind::VisitedSet,
            ],
        ),
        (
            "anchored-frontier",
            vec![
                Kind::DirectionalAdjacency,
                Kind::TraversalWorkset,
                Kind::DedupSet,
            ],
        ),
        (
            "shared-endpoint",
            vec![
                Kind::ReverseAdjacency,
                Kind::TraversalWorkset,
                Kind::DedupSet,
            ],
        ),
        (
            "shared-attachment",
            vec![
                Kind::ReverseAdjacency,
                Kind::TraversalWorkset,
                Kind::DedupSet,
            ],
        ),
        (
            "frontier-search",
            vec![
                Kind::DirectionalAdjacency,
                Kind::ReverseAdjacency,
                Kind::DedupSet,
            ],
        ),
    ]
}

fn operator_family(
    workspace: &mut forge_query::facade::runtime::ForgeQueryWorkspace,
    name: &str,
) -> forge_query::facade::runtime::ForgeQueryReadFamily {
    workspace
        .define_read_family(name, |read| match name {
            "direct-edge" => read.local_direct_edge_collection(
                "user",
                two_relation_schema(),
                relation("manager"),
                |query| query.project(field("identity", "id")),
                |shape| shape.field(result_field("identity", "id", "id")),
            ),
            "successor-walk" => read.local_successor_walk_collection(
                "user",
                two_relation_schema(),
                relation("manager"),
                2,
                |query| query.project(field("identity", "id")),
                |shape| shape.field(result_field("identity", "id", "id")),
            ),
            "bounded-ancestor" => read.anchored_bounded_ancestor_collection(
                "user",
                two_relation_schema(),
                relation("manager"),
                2,
                |query| query.project(field("identity", "id")),
                |shape| shape.field(result_field("identity", "id", "id")),
            ),
            "bounded-descendant" => read.anchored_bounded_descendant_collection(
                "user",
                two_relation_schema(),
                relation("manager"),
                2,
                |query| query.project(field("identity", "id")),
                |shape| shape.field(result_field("identity", "id", "id")),
            ),
            "anchored-frontier" => read.anchored_frontier_collection(
                "user",
                two_relation_schema(),
                [relation("manager"), relation("mentor")],
                2,
                |query| query.project(field("identity", "id")),
                |shape| shape.field(result_field("identity", "id", "id")),
            ),
            "shared-endpoint" => read.local_shared_endpoint_collection(
                "user",
                two_relation_schema(),
                [relation("manager"), relation("mentor")],
                |query| query.project(field("identity", "id")),
                |shape| shape.field(result_field("identity", "id", "id")),
            ),
            "shared-attachment" => read.local_shared_attachment_collection(
                "user",
                two_relation_schema(),
                [relation("manager"), relation("mentor")],
                |query| query.project(field("identity", "id")),
                |shape| shape.field(result_field("identity", "id", "id")),
            ),
            "frontier-search" => read.explicit_broad_search_frontier_collection(
                "user",
                two_relation_schema(),
                [relation("manager"), relation("mentor")],
                2,
                |query| {
                    query
                        .project(field("identity", "id"))
                        .where_equal(equality("status", "value", "active"))
                },
                |shape| shape.field(result_field("identity", "id", "id")),
            ),
            _ => unreachable!("operator case should be declared above"),
        })
        .expect("operator family should admit")
}

fn workspace(name: &str) -> forge_query::facade::runtime::ForgeQueryWorkspace {
    PublicBridgeRuntimeHarness::new()
        .bridge_backed_runtime()
        .workspace(name)
        .expect("runtime should open workspace")
}

fn field(aspect: &str, field: &str) -> AspectFieldSelector {
    AspectFieldSelector::new(aspect, field).expect("field selector should build")
}

fn relation(name: &str) -> RelationName {
    RelationName::new(name).expect("relation name should build")
}

fn result_field(aspect: &str, field: &str, delivered: &str) -> AuthoredResultShapeField {
    AuthoredResultShapeField::new(aspect, field, delivered)
        .expect("result-shape field should build")
}

fn equality(aspect: &str, field: &str, value: &str) -> EqualityPredicate {
    EqualityPredicate::new(
        aspect,
        field,
        ScalarPredicateValue::String(value.to_string()),
    )
    .expect("equality predicate should build")
}

fn two_relation_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "graph-read-access-phase-four-two-relation",
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
