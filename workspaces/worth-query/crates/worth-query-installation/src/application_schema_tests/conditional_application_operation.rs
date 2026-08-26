use worth_query_declaration::facade::application_schema::{
    ApplicationOperationRef, ApplicationSchema,
};

use super::portable_record_assertions::assert_domain_operation_projection;
use super::{TestInput, TestOperation, TestSchema};
use crate::facade::*;

struct TestDomain;
struct TestDomainOperation;
struct TestDomainFamily;

crate::worth_query_conditional_node!(
    TestConditionalNode in TestDomain, TestDomainOperation, TestDomainFamily
    => operation "ready-gate"
);

fn application_operation(
) -> ApplicationOperationRef<TestSchema, TestOperation<TestSchema>, TestInput> {
    ApplicationOperationRef::from_declaration()
}

fn installed_world() -> (
    WorthQueryInstalledPackageIndex,
    WorthQueryApplicationConditionalOperationBinding<
        TestSchema,
        TestOperation<TestSchema>,
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
fn typed_export_closes_every_family_in_one_exact_canonical_inventory() {
    let definition = crate::conditional_application_operation_test_fixture::definition::<
        TestDomain,
        TestDomainOperation,
        TestDomainFamily,
    >();
    let binding = WorthQueryApplicationConditionalOperationBinding::declare(
        application_operation(),
        definition.reference(),
    );
    let artifact = crate::domain_computation_artifact_fixture::valid_contract(
        false,
        WorthQueryArtifactLifecycleContract::Retained,
        crate::domain_computation_artifact_fixture::domain_reproducibility(),
    );
    let package = WorthQueryPortableDomainPackage::new(WorthQueryPortableDomainIdentity::new(
        TestSchema::OWNER,
        1,
        0,
    ))
    .requires_capability("conditional-read")
    .requires_configuration("conditional-runtime")
    .requires_operating_posture("bounded")
    .definition(WorthQueryPortableDefinition::invariant(
        "conditional-ready",
        "ready-gate-required",
    ))
    .domain_operation(definition.into_portable())
    .artifact_contract(artifact)
    .application_schema(TestSchema::declaration().unwrap())
    .conditional_application_operation(binding)
    .permits_contribution("conditional-index")
    .validate()
    .unwrap();
    let export = package.export_typed_records().unwrap();
    let mut emitted_families = Vec::new();
    for record in export.records() {
        if emitted_families.last() != Some(&record.family()) {
            emitted_families.push(record.family());
        }
    }
    assert_eq!(emitted_families, WorthQueryPortablePackageRecordFamily::ALL);

    assert!(matches!(
        &export.records()[0],
        WorthQueryPortablePackageRecord::DomainIdentity(identity)
            if identity == package.domain_identity()
    ));
    macro_rules! assert_exact_family {
        ($variant:ident, $source:expr) => {{
            let actual = export
                .records()
                .iter()
                .filter_map(|record| match record {
                    WorthQueryPortablePackageRecord::$variant(value) => Some(value),
                    _ => None,
                })
                .collect::<Vec<_>>();
            assert_eq!(actual, $source.iter().collect::<Vec<_>>());
        }};
    }
    assert_exact_family!(CapabilityRequirement, package.capabilities());
    assert_exact_family!(ConfigurationRequirement, package.configuration());
    assert_exact_family!(OperatingRequirement, package.operating_requirements());
    assert_exact_family!(Definition, package.definitions());
    let exported_domain_operations = export
        .records()
        .iter()
        .filter_map(|record| match record {
            WorthQueryPortablePackageRecord::DomainOperation(value) => Some(value),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        exported_domain_operations.len(),
        package.domain_operations().len()
    );
    for (actual, source) in exported_domain_operations
        .into_iter()
        .zip(package.domain_operations())
    {
        assert_domain_operation_projection(actual, source);
    }
    assert_exact_family!(ArtifactContract, package.artifact_contracts());
    assert_exact_family!(ApplicationSchema, package.application_schemas());
    assert_exact_family!(
        ConditionalApplicationOperation,
        package.conditional_application_operations()
    );
    assert_exact_family!(ContributionPolicy, package.contribution_policy());
    assert_exact_family!(
        NativeAspectContract,
        package.application_contract_spine().native_aspects()
    );
    assert_exact_family!(
        ApplicationOperationContract,
        package.application_contract_spine().operations()
    );

    for family in [
        WorthQueryPortablePackageRecordFamily::DomainOperation,
        WorthQueryPortablePackageRecordFamily::NativeAspectContract,
        WorthQueryPortablePackageRecordFamily::ApplicationOperationContract,
    ] {
        let position = export
            .records()
            .iter()
            .position(|record| record.family() == family)
            .unwrap();
        let mut omitted = export.records().to_vec();
        omitted.remove(position);
        assert_eq!(
            crate::package::verify_source_closure_for_test(&package, export.manifest(), &omitted,)
                .unwrap_err()
                .kind(),
            WorthQueryPortablePackageExportDenialKind::IncompleteRecordClosure,
            "omitted {family:?}",
        );
        let mut duplicated = export.records().to_vec();
        duplicated.insert(position, duplicated[position].clone());
        assert_eq!(
            crate::package::verify_source_closure_for_test(
                &package,
                export.manifest(),
                &duplicated,
            )
            .unwrap_err()
            .kind(),
            WorthQueryPortablePackageExportDenialKind::IncompleteRecordClosure,
            "duplicated {family:?}",
        );
    }
}

#[test]
fn logical_export_budget_counts_authority_free_query_payload() {
    let source = crate::conditional_application_operation_test_fixture::definition::<
        TestDomain,
        TestDomainOperation,
        TestDomainFamily,
    >();
    let baseline_operation = WorthQueryDomainOperationDefinition::<
        TestDomain,
        TestDomainOperation,
        TestDomainFamily,
    >::new(source.identity().clone(), source.semantics().clone())
    .into_portable();
    let baseline_package = WorthQueryPortableDomainPackage::new(
        WorthQueryPortableDomainIdentity::new("logical-export-width", 1, 0),
    )
    .domain_operation(baseline_operation)
    .validate()
    .unwrap();
    let baseline = baseline_package.export_typed_records().unwrap();

    let wide_root = "wide-query-root".repeat(1_024);
    let mut wide_semantics = source.semantics().clone();
    wide_semantics.canonical_query =
        crate::conditional_application_operation_test_fixture::canonical_query_for_root(&wide_root);
    let wide_operation = WorthQueryDomainOperationDefinition::<
        TestDomain,
        TestDomainOperation,
        TestDomainFamily,
    >::new(source.identity().clone(), wide_semantics)
    .into_portable();
    let wide_package = WorthQueryPortableDomainPackage::new(WorthQueryPortableDomainIdentity::new(
        "logical-export-width",
        1,
        0,
    ))
    .domain_operation(wide_operation)
    .validate()
    .unwrap();
    let wide = wide_package.export_typed_records().unwrap();

    assert_eq!(
        wide.manifest().canonical_source_bytes(),
        baseline.manifest().canonical_source_bytes()
    );
    assert_eq!(
        wide.manifest().logical_export_bytes() - baseline.manifest().logical_export_bytes(),
        u64::try_from(wide_root.len() - "ConditionalEntity".len()).unwrap()
    );
    let denial = wide_package
        .export_typed_records_with_limits(WorthQueryPortablePackageExportLimits::new(
            u32::MAX,
            wide.manifest().logical_export_bytes() - 1,
        ))
        .unwrap_err();
    assert_eq!(
        denial.kind(),
        WorthQueryPortablePackageExportDenialKind::LogicalExportBytesExceeded
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
