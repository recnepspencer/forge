use crate::domain_computation_artifact_fixture::*;
use crate::domain_computation_workflow_test_support::artifact_workflow;
use crate::facade::*;

#[test]
fn package_validation_closes_artifact_references_and_stage_roles() {
    let contract = valid_contract(
        false,
        WorthQueryArtifactLifecycleContract::Retained,
        domain_reproducibility(),
    );
    let operation = artifact_workflow(contract.reference());
    let package = WorthQueryPortableDomainPackage::new(WorthQueryPortableDomainIdentity::new(
        "worth.routing",
        1,
        0,
    ))
    .artifact_contract(contract)
    .domain_operation(operation)
    .validate()
    .unwrap();

    assert_eq!(package.artifact_contracts().len(), 1);
    assert_eq!(package.domain_operations().len(), 1);
}

#[test]
fn undeclared_artifact_reference_is_denied_at_package_validation() {
    let contract = valid_contract(
        false,
        WorthQueryArtifactLifecycleContract::Retained,
        domain_reproducibility(),
    );
    let denial = WorthQueryPortableDomainPackage::new(WorthQueryPortableDomainIdentity::new(
        "worth.routing",
        1,
        0,
    ))
    .domain_operation(artifact_workflow(contract.reference()))
    .validate()
    .unwrap_err();

    assert_eq!(
        denial.kind(),
        WorthQueryPortablePackageValidationDenialKind::InvalidDomainOperation
    );
    assert!(denial
        .slot()
        .contains("collect:undeclared-artifact-contract"));
}

#[test]
fn artifact_reference_does_not_bypass_producer_role_policy() {
    let contract = base_builder()
        .consumed_by(["rank"])
        .compatibility(active_compatibility())
        .finish()
        .unwrap();
    let denial = workflow_denial(contract);

    assert_eq!(
        denial.kind(),
        WorthQueryPortablePackageValidationDenialKind::InvalidDomainOperation
    );
    assert!(denial
        .slot()
        .contains("collect:artifact-producer-role-not-permitted"));
}

#[test]
fn artifact_reference_does_not_bypass_consumer_role_policy() {
    let contract = base_builder()
        .produced_by(["collect"])
        .compatibility(active_compatibility())
        .finish()
        .unwrap();
    let denial = workflow_denial(contract);

    assert_eq!(
        denial.kind(),
        WorthQueryPortablePackageValidationDenialKind::InvalidDomainOperation
    );
    assert!(denial
        .slot()
        .contains("rank:artifact-consumer-role-not-permitted"));
}

fn workflow_denial(
    contract: WorthQueryPortableArtifactContract,
) -> WorthQueryPortablePackageValidationDenial {
    let operation = artifact_workflow(contract.reference());
    WorthQueryPortableDomainPackage::new(WorthQueryPortableDomainIdentity::new(
        "worth.routing",
        1,
        0,
    ))
    .artifact_contract(contract)
    .domain_operation(operation)
    .validate()
    .unwrap_err()
}
