use forge_query::facade::runtime::{
    resolve_graph_read_operations_for_family_with_registry,
    ForgeQueryGraphReadOperationRegistration, ForgeQueryGraphReadOperationRegistry,
    ForgeQueryGraphReadOperationUnsupportedShapeDeclaration,
    ForgeQueryGraphReadRegistryAdmissionError, ForgeQueryGraphReadResolvedOperationFamily,
    ForgeQueryGraphReadResolvedOperationKind, ForgeQueryGraphReadTraversalOperator,
    ForgeQueryReadBuiltInOperator, QuerySchemaView, SchemaFieldKind, SchemaFieldView,
    SchemaRelationView,
};
use forge_query::facade::{
    AspectFieldSelector, AuthoredResultShapeField, ForgeQueryGraphReadDomainOperationDeclaration,
    RelationName, TraversalSelector,
};

mod support;

use support::public_bridge_runtime::PublicBridgeRuntimeHarness;

#[test]
fn built_in_operation_resolution_is_stable_across_equivalent_read_declarations() {
    let harness = PublicBridgeRuntimeHarness::new();
    let runtime = harness.bridge_backed_runtime();
    let mut workspace = runtime
        .workspace("graph-read-access.phase-three.built-in-parity")
        .expect("runtime should open workspace");
    let first = direct_edge_family(&mut workspace, "direct-edge-first");
    let second = direct_edge_family(&mut workspace, "direct-edge-second");
    let registry = ForgeQueryGraphReadOperationRegistry::empty();

    let first_resolution =
        resolve_graph_read_operations_for_family_with_registry(&first, &registry)
            .expect("first direct edge should resolve")
            .resolved()
            .expect("first direct edge should be resolved")
            .clone();
    let second_resolution =
        resolve_graph_read_operations_for_family_with_registry(&second, &registry)
            .expect("second direct edge should resolve")
            .resolved()
            .expect("second direct edge should be resolved")
            .clone();

    assert_eq!(
        first_resolution.operations(),
        second_resolution.operations()
    );
    assert_eq!(
        first_resolution.operations()[0].family(),
        &ForgeQueryGraphReadResolvedOperationFamily::BuiltIn
    );
    assert_eq!(
        first_resolution.operations()[0].kind(),
        &ForgeQueryGraphReadResolvedOperationKind::BuiltIn(
            ForgeQueryReadBuiltInOperator::DirectEdge
        )
    );
}

#[test]
fn relation_shape_rules_do_not_override_built_in_operations() {
    let harness = PublicBridgeRuntimeHarness::new();
    let runtime = harness.bridge_backed_runtime();
    let mut workspace = runtime
        .workspace("graph-read-access.phase-three.built-in-not-overridden")
        .expect("runtime should open workspace");
    let family = direct_edge_family(&mut workspace, "direct-edge-not-overridden");
    let registry = ForgeQueryGraphReadOperationRegistry::empty()
        .with_unsupported_shape_for_relations(
            ["manager"],
            ForgeQueryGraphReadOperationUnsupportedShapeDeclaration::unsupported_shape(
                "legacy-relation-rule",
                "relation rules are not operation intent",
            ),
        );

    let outcome = resolve_graph_read_operations_for_family_with_registry(&family, &registry)
        .expect("built-in direct edge should resolve");
    let resolution = outcome
        .resolved()
        .expect("direct edge should not be denied by relation rule");

    assert_eq!(
        resolution.operations()[0].kind(),
        &ForgeQueryGraphReadResolvedOperationKind::BuiltIn(
            ForgeQueryReadBuiltInOperator::DirectEdge
        )
    );
}

#[test]
fn declared_domain_operation_matches_registered_reference_order_once() {
    let harness = PublicBridgeRuntimeHarness::new();
    let runtime = harness.bridge_backed_runtime();
    let mut workspace = runtime
        .workspace("graph-read-access.phase-three.order-equivalence")
        .expect("runtime should open workspace");
    let operation = dual_chain_operation();
    let family =
        declared_two_relation_domain_family(&mut workspace, "order-equivalence", operation.clone());
    let registry = ForgeQueryGraphReadOperationRegistry::admit([
        ForgeQueryGraphReadOperationRegistration::for_declared_operation(&operation)
            .lowers_to_traversal_operator(ForgeQueryGraphReadTraversalOperator::SuccessorWalk),
    ])
    .expect("registry should admit declared domain operation");

    let outcome = resolve_graph_read_operations_for_family_with_registry(&family, &registry)
        .expect("registered operation should resolve");
    let resolution = outcome.resolved().expect("outcome should be resolved");

    assert_eq!(resolution.operations().len(), 1);
    assert_eq!(
        resolution.operations()[0].family(),
        &ForgeQueryGraphReadResolvedOperationFamily::DomainRegistered
    );
}

#[test]
fn duplicate_registry_operation_keys_are_denied_at_admission() {
    let operation = dual_chain_operation();
    let registration = ForgeQueryGraphReadOperationRegistration::for_declared_operation(&operation)
        .lowers_to_traversal_operator(ForgeQueryGraphReadTraversalOperator::SuccessorWalk);

    let error = ForgeQueryGraphReadOperationRegistry::admit([registration.clone(), registration])
        .expect_err("duplicate operation key should deny registry admission");

    assert_eq!(
        error,
        ForgeQueryGraphReadRegistryAdmissionError::DuplicateOperationKey
    );
}

#[test]
fn ambiguous_registry_reference_admission_is_denied() {
    let first = dual_chain_operation();
    let second = ForgeQueryGraphReadDomainOperationDeclaration::new(
        "worth.geometry.different_dual_chain",
        1,
        "worth.geometry",
    )
    .expect("operation key should admit")
    .admit_relation_reference("manager")
    .expect("reference should admit")
    .admit_relation_reference("mentor")
    .expect("reference should admit")
    .requires_support_family("worth.geometry.different_dual_chain.access")
    .expect("support should admit");

    let error = ForgeQueryGraphReadOperationRegistry::admit([
        ForgeQueryGraphReadOperationRegistration::for_declared_operation(&first)
            .lowers_to_traversal_operator(ForgeQueryGraphReadTraversalOperator::SuccessorWalk),
        ForgeQueryGraphReadOperationRegistration::for_declared_operation(&second)
            .lowers_to_traversal_operator(ForgeQueryGraphReadTraversalOperator::AnchoredFrontier),
    ])
    .expect_err("same admitted refs with different operation keys should deny");

    assert_eq!(
        error,
        ForgeQueryGraphReadRegistryAdmissionError::AmbiguousDomainReferenceAdmission
    );
}

fn direct_edge_family(
    workspace: &mut forge_query::facade::runtime::ForgeQueryWorkspace,
    family_name: &str,
) -> forge_query::facade::runtime::ForgeQueryReadFamily {
    workspace
        .define_read_family(family_name, |read| {
            read.local_direct_edge_collection(
                "user",
                two_relation_schema(),
                relation("manager"),
                |query| query.project(field("identity", "id")),
                |shape| shape.field(result_field("identity", "id", "id")),
            )
        })
        .expect("direct-edge family should be admitted")
}

fn declared_two_relation_domain_family(
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
                        .traverse(traversal("mentor", 2))
                        .project(field("identity", "id"))
                },
                |shape| shape.field(result_field("identity", "id", "id")),
            )
        })
        .expect("two-relation domain family should be admitted")
}

fn dual_chain_operation() -> ForgeQueryGraphReadDomainOperationDeclaration {
    ForgeQueryGraphReadDomainOperationDeclaration::new(
        "worth.geometry.dual_chain",
        1,
        "worth.geometry",
    )
    .expect("operation key should admit")
    .admit_relation_reference("mentor")
    .expect("reference should admit")
    .admit_relation_reference("manager")
    .expect("reference should admit")
    .requires_support_family("worth.geometry.dual_chain.access")
    .expect("support should admit")
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

fn two_relation_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "graph-read-access-phase-three-two-relation",
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
        [
            SchemaRelationView::new(
                forge_query::facade::RelationName::new("manager")
                    .expect("schema relation literal must be valid"),
                2,
            ),
            SchemaRelationView::new(
                forge_query::facade::RelationName::new("mentor")
                    .expect("schema relation literal must be valid"),
                2,
            ),
        ],
    )
}
