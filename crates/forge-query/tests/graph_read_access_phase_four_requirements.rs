use forge_query::facade::runtime::{
    derive_graph_read_access_requirements, explain_boolean_selectivity_shape_for_family,
    explain_graph_read_access_requirements_for_family,
    explain_graph_read_access_requirements_for_family_with_operation_registry,
    explain_graph_read_access_shape_for_family, ForgeQueryGraphReadAccessComplexityContract,
    ForgeQueryGraphReadAccessInvalidationBasis, ForgeQueryGraphReadAccessMemoryEstimateBasis,
    ForgeQueryGraphReadAccessRebuildBasis, ForgeQueryGraphReadAccessRequirementKind,
    ForgeQueryGraphReadFanoutPosture, ForgeQueryGraphReadOperationRegistration,
    ForgeQueryGraphReadOperationRegistry, ForgeQueryGraphReadTraversalOperator, QuerySchemaView,
    SchemaFieldKind, SchemaFieldView, SchemaRelationView,
};
use forge_query::facade::{
    AspectFieldSelector, AuthoredResultShapeField, EqualityPredicate,
    ForgeQueryGraphReadDomainOperationDeclaration, OrderingSelector, ScalarPredicateValue,
    TraversalSelector,
};
use std::collections::BTreeMap;

mod support;

use support::public_bridge_runtime::PublicBridgeRuntimeHarness;

#[test]
fn equivalent_access_shapes_derive_stable_requirement_sets() {
    let harness = PublicBridgeRuntimeHarness::new();
    let runtime = harness.bridge_backed_runtime();
    let mut workspace = runtime
        .workspace("graph-read-access.phase-four.parity")
        .expect("runtime should open workspace");
    let first = manager_traversal_family(&mut workspace, "phase-four-parity-first", "manager");
    let second = manager_traversal_family(&mut workspace, "phase-four-parity-second", "manager");

    let first_requirements = explain_graph_read_access_requirements_for_family(&first)
        .expect("first requirements should derive");
    let second_requirements = explain_graph_read_access_requirements_for_family(&second)
        .expect("second requirements should derive");

    assert_eq!(first_requirements.rows(), second_requirements.rows());
    assert_eq!(
        first_requirements.digest().as_str(),
        second_requirements.digest().as_str()
    );
    assert_eq!(
        first_requirements.canonical_parts(),
        second_requirements.canonical_parts()
    );
    assert!(first_requirements
        .requires_kind(ForgeQueryGraphReadAccessRequirementKind::DirectionalAdjacency));
    assert!(first_requirements
        .requires_kind(ForgeQueryGraphReadAccessRequirementKind::TraversalWorkset));
    let adjacency = first_requirements
        .rows()
        .iter()
        .find(|row| row.kind() == &ForgeQueryGraphReadAccessRequirementKind::DirectionalAdjacency)
        .expect("directional adjacency row should exist");
    assert_eq!(adjacency.relation_name(), Some("manager"));
    assert_eq!(adjacency.relation_depth(), Some(2));
    assert_eq!(
        adjacency.fanout_posture(),
        Some(&ForgeQueryGraphReadFanoutPosture::SingleRelation)
    );
    let authority = adjacency
        .relation_authority()
        .expect("relation authority should be derived from schema proof");
    assert_eq!(authority.relation_name(), "manager");
    assert!(!authority.schema_basis_digest().is_empty());
    assert_eq!(
        adjacency.invalidation_basis(),
        &ForgeQueryGraphReadAccessInvalidationBasis::AuthoritativeRelationDelta
    );
    assert_eq!(
        adjacency.complexity_contract(),
        &ForgeQueryGraphReadAccessComplexityContract::DirectionalRelationLookup
    );
    assert_eq!(
        adjacency.memory_estimate_basis(),
        &ForgeQueryGraphReadAccessMemoryEstimateBasis::RelationDegreeBound
    );
}

#[test]
fn changing_one_traversal_relation_localizes_requirement_row_change() {
    let harness = PublicBridgeRuntimeHarness::new();
    let runtime = harness.bridge_backed_runtime();
    let mut workspace = runtime
        .workspace("graph-read-access.phase-four.localization")
        .expect("runtime should open workspace");
    let manager = manager_traversal_family(&mut workspace, "phase-four-manager", "manager");
    let mentor = manager_traversal_family(&mut workspace, "phase-four-mentor", "mentor");

    let manager_requirements = explain_graph_read_access_requirements_for_family(&manager)
        .expect("manager requirements should derive");
    let mentor_requirements = explain_graph_read_access_requirements_for_family(&mentor)
        .expect("mentor requirements should derive");
    let manager_rows = semantic_requirement_rows(manager_requirements.rows());
    let mentor_rows = semantic_requirement_rows(mentor_requirements.rows());

    assert_eq!(
        manager_rows.keys().collect::<Vec<_>>(),
        mentor_rows.keys().collect::<Vec<_>>()
    );

    let changed_slots = manager_rows
        .iter()
        .filter_map(|(slot, manager_digest)| {
            let mentor_digest = mentor_rows
                .get(slot)
                .expect("semantic slot should exist in mentor rows");
            (manager_digest != mentor_digest).then_some(slot.as_str())
        })
        .collect::<Vec<_>>();
    assert_eq!(
        changed_slots,
        [
            "slot:directional_adjacency:forward:2:single_relation:none:none:declaration_traversal:none:none:none",
            "slot:traversal_workset:forward:2:single_relation:none:none:declaration_traversal:none:none:none",
            "slot:visited_set:forward:2:single_relation:none:none:declaration_traversal:none:none:none"
        ]
    );
}

fn semantic_requirement_rows(
    rows: &[forge_query::facade::runtime::ForgeQueryGraphReadAccessRequirementRow],
) -> BTreeMap<String, String> {
    rows.iter()
        .map(|row| (row.semantic_slot_key(), row.digest_part()))
        .collect()
}

#[test]
fn predicate_and_ordering_shapes_add_typed_support_rows() {
    let harness = PublicBridgeRuntimeHarness::new();
    let runtime = harness.bridge_backed_runtime();
    let mut workspace = runtime
        .workspace("graph-read-access.phase-four.predicate-ordering")
        .expect("runtime should open workspace");
    let family = workspace
        .define_read_family("phase-four-predicate-ordering", |read| {
            read.explicit_broad_search_collection(
                "user",
                two_relation_schema(),
                |query| {
                    query
                        .traverse(traversal("manager", 2))
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
        .expect("predicate ordering family should admit");

    let requirements = explain_graph_read_access_requirements_for_family(&family)
        .expect("requirements should derive");

    assert!(requirements.requires_kind(ForgeQueryGraphReadAccessRequirementKind::PredicateSupport));
    assert!(requirements.requires_kind(ForgeQueryGraphReadAccessRequirementKind::OrderingSupport));
    let predicate_row = requirements
        .rows()
        .iter()
        .find(|row| row.kind() == &ForgeQueryGraphReadAccessRequirementKind::PredicateSupport)
        .expect("predicate support row should exist");
    let ordering_row = requirements
        .rows()
        .iter()
        .find(|row| row.kind() == &ForgeQueryGraphReadAccessRequirementKind::OrderingSupport)
        .expect("ordering support row should exist");
    assert_eq!(
        predicate_row.rebuild_basis(),
        &ForgeQueryGraphReadAccessRebuildBasis::SelectivityProof
    );
    assert_eq!(predicate_row.predicate_field_authorities().len(), 1);
    assert_eq!(
        predicate_row.predicate_field_authorities()[0].aspect(),
        "status"
    );
    assert_eq!(
        predicate_row.predicate_field_authorities()[0].field(),
        "value"
    );
    assert_eq!(ordering_row.ordering_field_authorities().len(), 1);
    assert_eq!(
        ordering_row.ordering_field_authorities()[0].aspect(),
        "profile"
    );
    assert_eq!(
        ordering_row.ordering_field_authorities()[0].field(),
        "display_name"
    );
}

#[test]
fn requirement_derivation_consumes_access_and_selectivity_proofs() {
    let harness = PublicBridgeRuntimeHarness::new();
    let runtime = harness.bridge_backed_runtime();
    let mut workspace = runtime
        .workspace("graph-read-access.phase-four.proof-chain")
        .expect("runtime should open workspace");
    let family = manager_traversal_family(&mut workspace, "phase-four-proof-chain", "manager");

    let explanation =
        explain_graph_read_access_shape_for_family(&family).expect("access shape should derive");
    let selectivity =
        explain_boolean_selectivity_shape_for_family(&family).expect("selectivity should derive");
    let requirements =
        derive_graph_read_access_requirements(explanation.access_shape(), &selectivity);

    assert_eq!(
        requirements.read_graph_digest(),
        explanation
            .access_shape()
            .operation_resolution()
            .read_graph_digest()
    );
    assert_eq!(
        requirements.access_shape_digest(),
        explanation.access_shape().digest().as_str()
    );
    assert_eq!(
        requirements.selectivity_shape_digest(),
        selectivity.digest().as_str()
    );
}

#[test]
fn registry_backed_domain_operations_derive_requirements_through_explicit_registry() {
    let harness = PublicBridgeRuntimeHarness::new();
    let runtime = harness.bridge_backed_runtime();
    let mut workspace = runtime
        .workspace("graph-read-access.phase-four.registry-requirements")
        .expect("runtime should open workspace");
    let operation = visible_face_neighborhood_operation();
    let family = domain_operation_family(
        &mut workspace,
        "phase-four-registry-requirements",
        operation.clone(),
    );
    let registry = ForgeQueryGraphReadOperationRegistry::admit([
        ForgeQueryGraphReadOperationRegistration::for_declared_operation(&operation)
            .lowers_to_traversal_operator(ForgeQueryGraphReadTraversalOperator::SuccessorWalk),
    ])
    .expect("domain operation registration should admit");

    let requirements = explain_graph_read_access_requirements_for_family_with_operation_registry(
        &family, &registry,
    )
    .expect("registered requirements should derive");

    assert!(
        requirements.requires_kind(ForgeQueryGraphReadAccessRequirementKind::DirectionalAdjacency)
    );
    assert!(requirements.requires_kind(ForgeQueryGraphReadAccessRequirementKind::TraversalWorkset));
    assert!(requirements
        .rows()
        .iter()
        .any(|row| row.relation_name() == Some("manager")));
}

fn manager_traversal_family(
    workspace: &mut forge_query::facade::runtime::ForgeQueryWorkspace,
    family_name: &str,
    relation: &str,
) -> forge_query::facade::runtime::ForgeQueryReadFamily {
    workspace
        .define_read_family(family_name, |read| {
            read.anchored_collection(
                "user",
                two_relation_schema(),
                |query| {
                    query
                        .traverse(traversal(relation, 2))
                        .project(field("identity", "id"))
                },
                |shape| shape.field(result_field("identity", "id", "id")),
            )
        })
        .expect("manager traversal family should admit")
}

fn domain_operation_family(
    workspace: &mut forge_query::facade::runtime::ForgeQueryWorkspace,
    family_name: &str,
    operation: ForgeQueryGraphReadDomainOperationDeclaration,
) -> forge_query::facade::runtime::ForgeQueryReadFamily {
    workspace
        .define_read_family(family_name, |read| {
            read.anchored_collection(
                "user",
                two_relation_schema(),
                |query| {
                    query
                        .domain_graph_operation(operation)
                        .traverse(traversal("manager", 2))
                        .project(field("identity", "id"))
                },
                |shape| shape.field(result_field("identity", "id", "id")),
            )
        })
        .expect("domain operation family should admit")
}

fn visible_face_neighborhood_operation() -> ForgeQueryGraphReadDomainOperationDeclaration {
    ForgeQueryGraphReadDomainOperationDeclaration::new(
        "worth.geometry.visible_face_neighborhood",
        1,
        "worth.geometry",
    )
    .expect("operation key should admit")
    .admit_relation_reference("manager")
    .expect("operation reference should admit")
    .requires_support_family("worth.geometry.visible_face_neighborhood.access")
    .expect("support family should admit")
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
