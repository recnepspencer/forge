use forge_query::facade::runtime::{
    estimate_graph_read_access_cost, explain_graph_read_access_requirements_for_family,
    ForgeQueryGraphReadBudget, ForgeQueryGraphReadBudgetClassKind,
    ForgeQueryGraphReadComplexityContractKind, ForgeQueryGraphReadCostEstimateStatusKind,
    ForgeQueryGraphReadCostEvidence, QuerySchemaView, SchemaFieldKind, SchemaFieldView,
    SchemaRelationView,
};
use forge_query::facade::{
    AspectFieldSelector, AuthoredResultShapeField, EqualityPredicate, OrderingSelector,
    PresencePredicate, RelationName, ScalarPredicateValue, TraversalSelector,
};

mod support;

use support::public_bridge_runtime::PublicBridgeRuntimeHarness;

#[test]
fn dense_boolean_traversal_exceeds_inline_ephemeral_budget_conservatively() {
    let mut workspace = workspace("graph-read-access.phase-five.dense-budget");
    let family = dense_traversal_family(&mut workspace, "phase-five-dense-budget");
    let requirements = explain_graph_read_access_requirements_for_family(&family)
        .expect("requirements should derive");

    let estimate = estimate_graph_read_access_cost(
        &requirements,
        ForgeQueryGraphReadCostEvidence::unknown_conservative(),
    );
    let budget = ForgeQueryGraphReadBudget::inline_ephemeral_default();
    let budget_check = budget.check_supported_cost(&estimate);

    assert_eq!(
        estimate.status().kind(),
        &ForgeQueryGraphReadCostEstimateStatusKind::UnknownConservative
    );
    assert_eq!(
        estimate.complexity_contract().kind(),
        &ForgeQueryGraphReadComplexityContractKind::BroadTraversalCandidate
    );
    assert_eq!(
        budget_check.class().kind(),
        &ForgeQueryGraphReadBudgetClassKind::ExceedsInlineEphemeralBudget
    );
    assert_eq!(
        budget_check.cost_estimate_digest(),
        estimate.digest().as_str()
    );
}

#[test]
fn equivalent_access_requirements_produce_identical_cost_estimate_digests() {
    let mut workspace = workspace("graph-read-access.phase-five.equivalence");
    let first = simple_traversal_family(&mut workspace, "phase-five-equivalence-a");
    let second = simple_traversal_family(&mut workspace, "phase-five-equivalence-b");
    let first_requirements = explain_graph_read_access_requirements_for_family(&first)
        .expect("first requirements should derive");
    let second_requirements = explain_graph_read_access_requirements_for_family(&second)
        .expect("second requirements should derive");

    let first_estimate = estimate_graph_read_access_cost(
        &first_requirements,
        ForgeQueryGraphReadCostEvidence::unknown_conservative(),
    );
    let second_estimate = estimate_graph_read_access_cost(
        &second_requirements,
        ForgeQueryGraphReadCostEvidence::unknown_conservative(),
    );

    assert_eq!(first_requirements.digest(), second_requirements.digest());
    assert_eq!(
        first_estimate.digest().as_str(),
        second_estimate.digest().as_str()
    );
}

#[test]
fn memory_estimate_names_each_relevant_access_structure_bucket() {
    let mut workspace = workspace("graph-read-access.phase-five.memory-buckets");
    let family = frontier_search_family(&mut workspace, "phase-five-memory-buckets");
    let requirements = explain_graph_read_access_requirements_for_family(&family)
        .expect("requirements should derive");
    let estimate = estimate_graph_read_access_cost(
        &requirements,
        ForgeQueryGraphReadCostEvidence::unknown_conservative(),
    );
    let memory = estimate.supported().memory();

    assert!(memory.adjacency_bytes() > 0);
    assert!(memory.reverse_adjacency_bytes() > 0);
    assert!(memory.frontier_bytes() > 0);
    assert!(memory.visited_bytes() > 0);
    assert!(memory.dedup_bytes() > 0);
    assert!(memory.predicate_bytes() > 0);
    assert!(memory.ordering_bytes() > 0);
    assert!(memory.proof_bytes() > 0);
    assert!(memory.result_bytes() > 0);
}

#[test]
fn intermediate_set_pressure_marks_broad_even_when_index_bytes_fit() {
    let mut workspace = workspace("graph-read-access.phase-five.intermediate-broadness");
    let family = intermediate_pressure_family(&mut workspace, "phase-five-intermediate-broadness");
    let requirements = explain_graph_read_access_requirements_for_family(&family)
        .expect("requirements should derive");
    let estimate = estimate_graph_read_access_cost(
        &requirements,
        ForgeQueryGraphReadCostEvidence::unknown_conservative(),
    );

    assert!(
        estimate.supported().index_bytes()
            <= ForgeQueryGraphReadBudget::inline_ephemeral_default().max_inline_index_bytes()
    );
    assert!(
        estimate.intrinsic().intermediate_set_size()
            > ForgeQueryGraphReadBudget::inline_ephemeral_default()
                .max_inline_intermediate_set_size()
    );
    assert_eq!(
        estimate.complexity_contract().kind(),
        &ForgeQueryGraphReadComplexityContractKind::BroadTraversalCandidate
    );
}

#[test]
fn cost_estimation_is_planning_pure_not_execution_observation() {
    let mut workspace = workspace("graph-read-access.phase-five.planning-purity");
    let family = frontier_search_family(&mut workspace, "phase-five-planning-purity");
    let requirements = explain_graph_read_access_requirements_for_family(&family)
        .expect("requirements should derive");
    let estimate = estimate_graph_read_access_cost(
        &requirements,
        ForgeQueryGraphReadCostEvidence::unknown_conservative(),
    );

    assert_eq!(estimate.counters().edge_scan_count(), 0);
    assert_eq!(estimate.counters().access_buffer_allocation_count(), 0);
    assert_eq!(
        estimate.counters().requirement_row_count(),
        requirements.rows().len()
    );
    assert!(estimate.intrinsic().edge_touches() > 0);
    assert!(estimate.supported().index_bytes() > 0);
}

fn workspace(name: &str) -> forge_query::facade::runtime::ForgeQueryWorkspace {
    PublicBridgeRuntimeHarness::new()
        .bridge_backed_runtime()
        .workspace(name)
        .expect("runtime should open workspace")
}

fn simple_traversal_family(
    workspace: &mut forge_query::facade::runtime::ForgeQueryWorkspace,
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

fn dense_traversal_family(
    workspace: &mut forge_query::facade::runtime::ForgeQueryWorkspace,
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

fn intermediate_pressure_family(
    workspace: &mut forge_query::facade::runtime::ForgeQueryWorkspace,
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

fn frontier_search_family(
    workspace: &mut forge_query::facade::runtime::ForgeQueryWorkspace,
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
