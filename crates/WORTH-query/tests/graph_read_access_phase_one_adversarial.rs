use worth_query::facade::runtime::{
    QuerySchemaView, SchemaFieldKind, SchemaFieldView, SchemaRelationView,
    WorthQueryAdmittedGraphReadRelationDirection, WorthQueryGraphReadAccessShape,
    WorthQueryGraphReadFanoutPosture, WorthQueryGraphReadOrderingPosture,
    WorthQueryGraphReadPredicateFamily, WorthQueryGraphReadResultPressure,
    WorthQueryGraphReadRootPosture, WorthQueryGraphReadTraversalOperator, WorthQueryReadScopeClass,
};
use worth_query::facade::{
    AspectFieldSelector, AuthoredResultShapeField, CollectionQueryBuilder, EqualityPredicate,
    IntegerComparisonPredicate, OrderingSelector, PresencePredicate, RelationName,
    ScalarPredicateValue, SetMembershipPredicate, StringContainsPredicate,
};

mod support;

use support::public_bridge_runtime::PublicBridgeRuntimeHarness;

#[test]
fn graph_read_access_shape_changes_for_relation_direction_and_depth() {
    let harness = PublicBridgeRuntimeHarness::new();
    let runtime = harness.bridge_backed_runtime();
    let mut workspace = runtime
        .workspace("graph-read-access.phase-one.direction-depth")
        .expect("runtime should open workspace");
    let ancestor = workspace
        .define_read_family("ancestor", |read| {
            read.anchored_bounded_ancestor_collection(
                "user",
                relation_schema(),
                relation("manager"),
                2,
                |query| query.project(field("identity", "id")),
                |shape| shape.field(result_field("identity", "id", "id")),
            )
        })
        .expect("ancestor family should be admitted");
    let descendant = workspace
        .define_read_family("descendant", |read| {
            read.anchored_bounded_descendant_collection(
                "user",
                relation_schema(),
                relation("manager"),
                2,
                |query| query.project(field("identity", "id")),
                |shape| shape.field(result_field("identity", "id", "id")),
            )
        })
        .expect("descendant family should be admitted");
    let deeper_ancestor = workspace
        .define_read_family("deeper-ancestor", |read| {
            read.anchored_bounded_ancestor_collection(
                "user",
                relation_schema(),
                relation("manager"),
                3,
                |query| query.project(field("identity", "id")),
                |shape| shape.field(result_field("identity", "id", "id")),
            )
        })
        .expect("deeper ancestor family should be admitted");

    let ancestor_shape = access_shape(&workspace, &ancestor);
    let descendant_shape = access_shape(&workspace, &descendant);
    let deeper_shape = access_shape(&workspace, &deeper_ancestor);

    assert_ne!(ancestor_shape.digest(), descendant_shape.digest());
    assert_ne!(ancestor_shape.digest(), deeper_shape.digest());
    assert_shape_dimensions(
        &ancestor_shape,
        ExpectedGraphReadShape {
            root: WorthQueryGraphReadRootPosture::Anchored,
            scope: WorthQueryReadScopeClass::AnchoredExpansion,
            operators: &[WorthQueryGraphReadTraversalOperator::BoundedAncestor],
            directions: &[WorthQueryAdmittedGraphReadRelationDirection::Ancestor],
            max_depth: 2,
            fanout: WorthQueryGraphReadFanoutPosture::SingleRelation,
            predicate: WorthQueryGraphReadPredicateFamily::None,
            ordering: WorthQueryGraphReadOrderingPosture::Unordered,
            result: WorthQueryGraphReadResultPressure::CollectionNarrow,
        },
    );
    assert_eq!(
        descendant_shape.relation_directions(),
        [WorthQueryAdmittedGraphReadRelationDirection::Descendant]
    );
    assert_eq!(deeper_shape.max_depth(), 3);
}

#[test]
fn graph_read_access_shape_classifies_predicate_families_and_mixed_predicates() {
    let equality = predicate_shape("equality", |query| {
        query
            .where_equal(
                EqualityPredicate::new(
                    "status",
                    "value",
                    ScalarPredicateValue::String("active".to_string()),
                )
                .expect("equality predicate should build"),
            )
            .project(field("identity", "id"))
    });
    let range = predicate_shape("range", |query| {
        query
            .where_greater_than(
                IntegerComparisonPredicate::greater_than("profile", "age", 21)
                    .expect("range predicate should build"),
            )
            .project(field("identity", "id"))
    });
    let text = predicate_shape("text", |query| {
        query
            .where_contains(
                StringContainsPredicate::new("profile", "display_name", "Ada")
                    .expect("text predicate should build"),
            )
            .project(field("identity", "id"))
    });
    let membership = predicate_shape("membership", |query| {
        query
            .where_in(
                SetMembershipPredicate::new(
                    "status",
                    "value",
                    [ScalarPredicateValue::String("active".to_string())],
                )
                .expect("membership predicate should build"),
            )
            .project(field("identity", "id"))
    });
    let presence = predicate_shape("presence", |query| {
        query
            .where_present(
                PresencePredicate::is_present("profile", "display_name")
                    .expect("presence predicate should build"),
            )
            .project(field("identity", "id"))
    });
    let mixed = predicate_shape("mixed", |query| {
        query
            .where_contains(
                StringContainsPredicate::new("profile", "display_name", "Ada")
                    .expect("text predicate should build"),
            )
            .where_greater_than(
                IntegerComparisonPredicate::greater_than("profile", "age", 21)
                    .expect("range predicate should build"),
            )
            .project(field("identity", "id"))
    });

    assert_eq!(
        equality.predicate_family(),
        &WorthQueryGraphReadPredicateFamily::Equality
    );
    assert_eq!(
        range.predicate_family(),
        &WorthQueryGraphReadPredicateFamily::Range
    );
    assert_eq!(
        text.predicate_family(),
        &WorthQueryGraphReadPredicateFamily::Text
    );
    assert_eq!(
        membership.predicate_family(),
        &WorthQueryGraphReadPredicateFamily::Membership
    );
    assert_eq!(
        presence.predicate_family(),
        &WorthQueryGraphReadPredicateFamily::Presence
    );
    assert_eq!(
        mixed.predicate_family(),
        &WorthQueryGraphReadPredicateFamily::Mixed
    );
}

#[test]
fn graph_read_access_shape_classifies_ordering_without_digest_theater() {
    let harness = PublicBridgeRuntimeHarness::new();
    let runtime = harness.bridge_backed_runtime();
    let mut workspace = runtime
        .workspace("graph-read-access.phase-one.ordering")
        .expect("runtime should open workspace");
    let unordered = workspace
        .define_read_family("unordered", |read| {
            read.local_collection(
                "user",
                predicate_schema(),
                |query| query.project(field("identity", "id")),
                |shape| shape.field(result_field("identity", "id", "id")),
            )
        })
        .expect("unordered family should be admitted");
    let ordered = workspace
        .define_read_family("ordered", |read| {
            read.local_collection(
                "user",
                predicate_schema(),
                |query| {
                    query.project(field("identity", "id")).order_by(
                        OrderingSelector::ascending("profile", "display_name")
                            .expect("ordering should build"),
                    )
                },
                |shape| shape.field(result_field("identity", "id", "id")),
            )
        })
        .expect("ordered family should be admitted");

    let unordered_shape = access_shape(&workspace, &unordered);
    let ordered_shape = access_shape(&workspace, &ordered);

    assert_ne!(unordered_shape.digest(), ordered_shape.digest());
    assert_eq!(
        unordered_shape.ordering_posture(),
        &WorthQueryGraphReadOrderingPosture::Unordered
    );
    assert_eq!(
        ordered_shape.ordering_posture(),
        &WorthQueryGraphReadOrderingPosture::Ordered
    );
    assert_eq!(
        ordered_shape
            .operation_resolution()
            .references()
            .orderings()
            .len(),
        1
    );
}

struct ExpectedGraphReadShape<'a> {
    root: WorthQueryGraphReadRootPosture,
    scope: WorthQueryReadScopeClass,
    operators: &'a [WorthQueryGraphReadTraversalOperator],
    directions: &'a [WorthQueryAdmittedGraphReadRelationDirection],
    max_depth: usize,
    fanout: WorthQueryGraphReadFanoutPosture,
    predicate: WorthQueryGraphReadPredicateFamily,
    ordering: WorthQueryGraphReadOrderingPosture,
    result: WorthQueryGraphReadResultPressure,
}

fn assert_shape_dimensions(
    shape: &WorthQueryGraphReadAccessShape,
    expected: ExpectedGraphReadShape<'_>,
) {
    assert_eq!(shape.root_posture(), &expected.root);
    assert_eq!(shape.scope_class(), &expected.scope);
    assert_eq!(shape.traversal_operators(), expected.operators);
    assert_eq!(shape.relation_directions(), expected.directions);
    assert_eq!(shape.max_depth(), expected.max_depth);
    assert_eq!(shape.fanout_posture(), &expected.fanout);
    assert_eq!(shape.predicate_family(), &expected.predicate);
    assert_eq!(shape.ordering_posture(), &expected.ordering);
    assert_eq!(shape.result_pressure(), &expected.result);
}

fn access_shape<'a>(
    workspace: &'a worth_query::facade::runtime::WorthQueryWorkspace,
    family: &'a worth_query::facade::runtime::WorthQueryReadFamily,
) -> WorthQueryGraphReadAccessShape {
    let explanation = workspace
        .explain_graph_read_access_shape(family)
        .expect("access shape should explain");
    explanation.access_shape().clone()
}

fn predicate_shape(
    label: &str,
    declare_query: impl FnOnce(CollectionQueryBuilder) -> CollectionQueryBuilder,
) -> WorthQueryGraphReadAccessShape {
    let harness = PublicBridgeRuntimeHarness::new();
    let runtime = harness.bridge_backed_runtime();
    let mut workspace = runtime
        .workspace(format!("graph-read-access.phase-one.predicate.{label}"))
        .expect("runtime should open workspace");
    let family = workspace
        .define_read_family(label, |read| {
            read.explicit_broad_search_collection(
                "user",
                predicate_schema(),
                declare_query,
                |shape| shape.field(result_field("identity", "id", "id")),
            )
        })
        .expect("predicate family should be admitted");
    let explanation = workspace
        .explain_graph_read_access_shape(&family)
        .expect("predicate access shape should explain");
    explanation.access_shape().clone()
}

fn field(aspect: &str, field: &str) -> AspectFieldSelector {
    AspectFieldSelector::new(aspect, field).expect("field selector should build")
}

fn result_field(aspect: &str, field: &str, delivered: &str) -> AuthoredResultShapeField {
    AuthoredResultShapeField::new(aspect, field, delivered)
        .expect("result-shape field should build")
}

fn relation(name: &str) -> RelationName {
    RelationName::new(name).expect("relation name should build")
}

fn relation_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "graph-read-access-direction",
        [SchemaFieldView::new(
            worth_query::facade::AspectName::new("identity")
                .expect("schema aspect literal must be valid"),
            worth_query::facade::FieldName::new("id").expect("schema field literal must be valid"),
            SchemaFieldKind::String,
        )],
        [SchemaRelationView::new(
            worth_query::facade::RelationName::new("manager")
                .expect("schema relation literal must be valid"),
            3,
        )],
    )
}

fn predicate_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "graph-read-access-predicate",
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
            )
            .membership_predicate_queryable(),
            SchemaFieldView::new(
                worth_query::facade::AspectName::new("profile")
                    .expect("schema aspect literal must be valid"),
                worth_query::facade::FieldName::new("age")
                    .expect("schema field literal must be valid"),
                SchemaFieldKind::Integer,
            ),
            SchemaFieldView::new(
                worth_query::facade::AspectName::new("profile")
                    .expect("schema aspect literal must be valid"),
                worth_query::facade::FieldName::new("display_name")
                    .expect("schema field literal must be valid"),
                SchemaFieldKind::String,
            )
            .text_predicate_queryable()
            .presence_predicate_queryable(),
        ],
        [],
    )
}
