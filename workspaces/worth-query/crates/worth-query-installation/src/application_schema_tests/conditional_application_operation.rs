use worth_query_declaration::facade::application_schema::{
    ApplicationOperationRef, ApplicationSchema,
};

use super::{TestInput, TestOperation, TestSchema};
use crate::facade::*;

struct TestDomain;
struct TestDomainOperation;
struct TestDomainFamily;

crate::worth_query_conditional_node!(
    TestConditionalNode in TestDomain, TestDomainOperation, TestDomainFamily
    => operation "ready-gate"
);

fn application_operation() -> ApplicationOperationRef<TestSchema, TestOperation, TestInput> {
    ApplicationOperationRef::from_schema_identifier("TestOperation")
}

fn installed_world() -> (
    WorthQueryInstalledPackageIndex,
    WorthQueryApplicationConditionalOperationBinding<
        TestSchema,
        TestOperation,
        TestInput,
        TestDomain,
        TestDomainOperation,
        TestDomainFamily,
    >,
) {
    let definition = crate::conditional_application_operation_test_fixture::definition::<
        TestDomain,
        TestDomainOperation,
        TestDomainFamily,
    >();
    let binding = WorthQueryApplicationConditionalOperationBinding::declare(
        application_operation(),
        definition.reference(),
    );
    let package = WorthQueryPortableDomainPackage::new(WorthQueryPortableDomainIdentity::new(
        TestSchema::OWNER,
        1,
        0,
    ))
    .application_schema(TestSchema::declaration().unwrap())
    .domain_operation(definition.into_portable())
    .conditional_application_operation(binding.clone())
    .validate()
    .unwrap();
    let admitted = WorthQueryInstallationAdmissionProfile::new("support", "configuration")
        .admit(package)
        .unwrap();
    let index = WorthQueryInstalledPackageIndex::build(
        WorthQueryInstallationRuntimeIdentity::fresh(),
        WorthQueryInstallationGeneration::initial(),
        [admitted],
    )
    .unwrap();
    (index, binding)
}

#[test]
fn package_declared_binding_resolves_an_exact_installed_conditional_node() {
    let (index, binding) = installed_world();
    let schema = index
        .bind_application_schema(TestSchema::declaration().unwrap())
        .unwrap();
    let application_operation = schema.installed_operation(application_operation()).unwrap();
    let operation = index
        .bind_conditional_application_operation(application_operation, &binding)
        .unwrap();
    let node = operation
        .bind_node(TestConditionalNode::reference())
        .unwrap();

    assert_eq!(node.location().node_identity(), "ready-gate");
    assert_eq!(node.declaration().identity(), "ready-gate");
    assert_eq!(
        node.operation().domain_operation().operation_slot(),
        "conditional-operation:1"
    );
    assert_ne!(
        node.authority_identity(),
        node.operation().authority_identity()
    );
    assert_eq!(
        index
            .counters()
            .installed_conditional_application_operation_count,
        1
    );
}

#[test]
fn successor_generation_rejects_the_prior_installed_application_operation() {
    let (index, binding) = installed_world();
    let schema = index
        .bind_application_schema(TestSchema::declaration().unwrap())
        .unwrap();
    let application_operation = schema.installed_operation(application_operation()).unwrap();
    let successor = index.successor_generation();

    let Err(denial) =
        successor.bind_conditional_application_operation(application_operation, &binding)
    else {
        panic!("a successor cannot accept a prior-generation application operation");
    };
    assert_eq!(
        denial.kind(),
        WorthQueryConditionalApplicationOperationDenialKind::StaleGeneration
    );
}

#[test]
fn package_rejects_a_conditional_binding_without_its_application_schema() {
    let definition = crate::conditional_application_operation_test_fixture::definition::<
        TestDomain,
        TestDomainOperation,
        TestDomainFamily,
    >();
    let binding = WorthQueryApplicationConditionalOperationBinding::declare(
        application_operation(),
        definition.reference(),
    );
    let denial = WorthQueryPortableDomainPackage::new(WorthQueryPortableDomainIdentity::new(
        TestSchema::OWNER,
        1,
        0,
    ))
    .domain_operation(definition.into_portable())
    .conditional_application_operation(binding)
    .validate()
    .unwrap_err();

    assert_eq!(
        denial.kind(),
        WorthQueryPortablePackageValidationDenialKind::ConditionalApplicationSchemaMissing
    );
}
