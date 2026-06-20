use forge_query::facade::runtime::{
    explain_graph_read_access_shape_for_family_with_operation_registry,
    resolve_graph_read_operations_for_family_with_registry, ForgeQueryGraphReadOperationOutcome,
    ForgeQueryGraphReadOperationRegistration, ForgeQueryGraphReadOperationRegistry,
    ForgeQueryGraphReadOperationUnsupportedShapeDeclaration,
    ForgeQueryGraphReadResolvedOperationFamily, ForgeQueryGraphReadResolvedOperationKind,
    ForgeQueryGraphReadTraversalOperator, QuerySchemaView, SchemaFieldKind, SchemaFieldView,
    SchemaRelationView,
};
use forge_query::facade::{
    AspectFieldSelector, AuthoredResultShapeField, ForgeQueryGraphReadDomainOperationDeclaration,
    TraversalSelector,
};

mod support;

use support::public_bridge_runtime::PublicBridgeRuntimeHarness;

#[test]
fn declared_domain_operation_resolves_through_registry_by_operation_key() {
    let harness = PublicBridgeRuntimeHarness::new();
    let runtime = harness.bridge_backed_runtime();
    let mut workspace = runtime
        .workspace("graph-read-access.phase-three.domain-key")
        .expect("runtime should open workspace");
    let operation = visible_face_neighborhood_operation();
    let family = domain_operation_family(&mut workspace, "declared-domain-key", operation.clone());
    let registry = ForgeQueryGraphReadOperationRegistry::admit([
        ForgeQueryGraphReadOperationRegistration::for_declared_operation(&operation)
            .lowers_to_traversal_operator(ForgeQueryGraphReadTraversalOperator::SuccessorWalk),
    ])
    .expect("domain operation registration should admit");

    let explanation =
        explain_graph_read_access_shape_for_family_with_operation_registry(&family, &registry)
            .expect("registered domain operation should lower");
    let operation = &explanation
        .access_shape()
        .operation_resolution()
        .operations()[0];

    assert_eq!(
        operation.family(),
        &ForgeQueryGraphReadResolvedOperationFamily::DomainRegistered
    );
    assert_eq!(
        explanation.access_shape().traversal_operators(),
        [ForgeQueryGraphReadTraversalOperator::SuccessorWalk]
    );
    match operation.kind() {
        ForgeQueryGraphReadResolvedOperationKind::DomainRegistered(domain_operation) => {
            assert_eq!(
                domain_operation.operation_name(),
                "worth.geometry.visible_face_neighborhood"
            );
            assert_eq!(domain_operation.operation_version(), 1);
            assert_eq!(domain_operation.domain_owner(), "worth.geometry");
            assert_eq!(domain_operation.accepted_relations(), ["manager"]);
        }
        other => panic!("expected domain registered operation, got {other:?}"),
    }
}

#[test]
fn declared_domain_operation_without_registry_support_returns_required_capability() {
    let harness = PublicBridgeRuntimeHarness::new();
    let runtime = harness.bridge_backed_runtime();
    let mut workspace = runtime
        .workspace("graph-read-access.phase-three.missing-domain-registry")
        .expect("runtime should open workspace");
    let family = domain_operation_family(
        &mut workspace,
        "declared-domain-without-registry",
        visible_face_neighborhood_operation(),
    );

    let outcome = resolve_graph_read_operations_for_family_with_registry(
        &family,
        &ForgeQueryGraphReadOperationRegistry::empty(),
    )
    .expect("schema references should admit before capability classification");

    match outcome {
        ForgeQueryGraphReadOperationOutcome::RequiresAccessCapabilityRegistration(requirement) => {
            assert_eq!(
                requirement.operation_name(),
                "worth.geometry.visible_face_neighborhood"
            );
            assert_eq!(requirement.domain_owner(), "worth.geometry");
            assert_eq!(
                requirement.support_family(),
                "worth.geometry.visible_face_neighborhood.access"
            );
            assert_eq!(requirement.matched_relations(), ["manager".to_string()]);
            assert_eq!(
                requirement.read_graph_digest(),
                family.read_graph().digest()
            );
        }
        other => panic!("expected required capability registration, got {other:?}"),
    }
}

#[test]
fn unsupported_declared_domain_shape_denies_without_generic_traversal_fallback() {
    let harness = PublicBridgeRuntimeHarness::new();
    let runtime = harness.bridge_backed_runtime();
    let mut workspace = runtime
        .workspace("graph-read-access.phase-three.unsupported-domain")
        .expect("runtime should open workspace");
    let operation = visible_face_neighborhood_operation();
    let family = domain_operation_family(&mut workspace, "unsupported-domain", operation.clone());
    let registry = ForgeQueryGraphReadOperationRegistry::empty()
        .with_unsupported_shape_for_operation(
            operation.key().clone(),
            ForgeQueryGraphReadOperationUnsupportedShapeDeclaration::unsupported_shape(
                "unbounded-visible-face-neighborhood",
                "shape cannot lower into Query graph access planning",
            ),
        );

    let outcome = resolve_graph_read_operations_for_family_with_registry(&family, &registry)
        .expect("schema references should admit before unsupported-shape classification");

    match outcome {
        ForgeQueryGraphReadOperationOutcome::DeniedUnsupportedShape(denial) => {
            assert_eq!(denial.shape_name(), "unbounded-visible-face-neighborhood");
            assert_eq!(denial.matched_relations(), ["manager".to_string()]);
            assert_eq!(denial.read_graph_digest(), family.read_graph().digest());
        }
        other => panic!("expected unsupported shape denial, got {other:?}"),
    }
}

#[test]
fn ordinary_traversal_with_same_relations_does_not_become_domain_operation() {
    let harness = PublicBridgeRuntimeHarness::new();
    let runtime = harness.bridge_backed_runtime();
    let mut workspace = runtime
        .workspace("graph-read-access.phase-three.no-relation-magic")
        .expect("runtime should open workspace");
    let operation = visible_face_neighborhood_operation();
    let family = declared_manager_family(&mut workspace, "ordinary-manager-default");
    let registry = ForgeQueryGraphReadOperationRegistry::admit([
        ForgeQueryGraphReadOperationRegistration::for_declared_operation(&operation)
            .lowers_to_traversal_operator(ForgeQueryGraphReadTraversalOperator::SuccessorWalk),
    ])
    .expect("domain operation registration should admit");

    let explanation =
        explain_graph_read_access_shape_for_family_with_operation_registry(&family, &registry)
            .expect("plain declared traversal should still resolve");
    let operation = &explanation
        .access_shape()
        .operation_resolution()
        .operations()[0];

    assert_eq!(
        operation.family(),
        &ForgeQueryGraphReadResolvedOperationFamily::DeclaredTraversal
    );
    assert_eq!(
        operation.kind(),
        &ForgeQueryGraphReadResolvedOperationKind::DeclarationTraversal
    );
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
                manager_schema(),
                |query| {
                    query
                        .domain_graph_operation(operation)
                        .traverse(traversal("manager", 2))
                        .project(field("identity", "id"))
                },
                |shape| shape.field(result_field("identity", "id", "id")),
            )
        })
        .expect("domain operation family should be admitted")
}

fn declared_manager_family(
    workspace: &mut forge_query::facade::runtime::ForgeQueryWorkspace,
    family_name: &str,
) -> forge_query::facade::runtime::ForgeQueryReadFamily {
    workspace
        .define_read_family(family_name, |read| {
            read.anchored_collection(
                "user",
                manager_schema(),
                |query| {
                    query
                        .traverse(traversal("manager", 2))
                        .project(field("identity", "id"))
                },
                |shape| shape.field(result_field("identity", "id", "id")),
            )
        })
        .expect("declared manager family should be admitted")
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

fn manager_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "graph-read-access-phase-three-manager",
        [
            SchemaFieldView::new("identity", "id", SchemaFieldKind::String),
            SchemaFieldView::new("profile", "display_name", SchemaFieldKind::String),
        ],
        [SchemaRelationView::new("manager", 2)],
    )
}
