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

#[test]
fn direct_and_stage_evidence_references_close_against_installed_producer_roles() {
    let direct_contract = base_builder()
        .produced_by(["rank-candidates:1"])
        .compatibility(active_compatibility())
        .finish()
        .unwrap();
    let direct = direct_evidence_operation(direct_contract.reference());
    let direct_package = package_with(direct_contract, direct).unwrap();
    assert_eq!(direct_package.domain_operations().len(), 1);

    let stage_contract = valid_contract(
        false,
        WorthQueryArtifactLifecycleContract::Retained,
        domain_reproducibility(),
    );
    let stage = stage_evidence_workflow(stage_contract.reference());
    let stage_package = package_with(stage_contract, stage).unwrap();
    assert_eq!(stage_package.domain_operations().len(), 1);
}

#[test]
fn undeclared_and_wrong_role_evidence_references_are_denied() {
    let contract = base_builder()
        .produced_by(["rank-candidates:1"])
        .compatibility(active_compatibility())
        .finish()
        .unwrap();
    let undeclared = WorthQueryPortableDomainPackage::new(WorthQueryPortableDomainIdentity::new(
        "worth.routing",
        1,
        0,
    ))
    .domain_operation(direct_evidence_operation(contract.reference()))
    .validate()
    .unwrap_err();
    assert!(undeclared
        .slot()
        .contains("rank-candidates:1:undeclared-artifact-contract"));

    let wrong_role = base_builder()
        .produced_by(["different-operation"])
        .compatibility(active_compatibility())
        .finish()
        .unwrap();
    let denial = package_with(
        wrong_role.clone(),
        direct_evidence_operation(wrong_role.reference()),
    )
    .unwrap_err();
    assert!(denial
        .slot()
        .contains("rank-candidates:1:evidence-producer-role-not-permitted"));
}

#[test]
fn workflow_operation_evidence_must_be_declared_on_the_producing_stage() {
    let contract = valid_contract(
        false,
        WorthQueryArtifactLifecycleContract::Retained,
        domain_reproducibility(),
    );
    let denial = package_with(
        contract.clone(),
        workflow_operation_evidence(contract.reference()),
    )
    .unwrap_err();

    assert_eq!(
        denial.kind(),
        WorthQueryPortablePackageValidationDenialKind::InvalidDomainOperation
    );
    assert!(denial
        .slot()
        .contains("rank-candidates:1:workflow-operation-evidence-requires-stage-declaration"));
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

fn package_with(
    contract: WorthQueryPortableArtifactContract,
    operation: WorthQueryPortableDomainOperationDefinition,
) -> Result<WorthQueryValidatedPortableDomainPackage, WorthQueryPortablePackageValidationDenial> {
    WorthQueryPortableDomainPackage::new(WorthQueryPortableDomainIdentity::new(
        "worth.routing",
        1,
        0,
    ))
    .artifact_contract(contract)
    .domain_operation(operation)
    .validate()
}

fn direct_evidence_operation(
    reference: WorthQueryArtifactContractReference,
) -> WorthQueryPortableDomainOperationDefinition {
    let base = artifact_workflow(reference.clone());
    let mut semantics = base.semantics().clone();
    semantics.workflow = WorthQueryOperationWorkflowContract::NotRequired;
    semantics.evidence = WorthQueryDomainEvidenceContract::installed_artifact(reference);
    WorthQueryDomainOperationDefinition::<(), (), ()>::new(base.identity().clone(), semantics)
        .into_portable()
}

fn workflow_operation_evidence(
    reference: WorthQueryArtifactContractReference,
) -> WorthQueryPortableDomainOperationDefinition {
    let base = artifact_workflow(reference.clone());
    let mut semantics = base.semantics().clone();
    semantics.evidence = WorthQueryDomainEvidenceContract::installed_artifact(reference);
    WorthQueryDomainOperationDefinition::<(), (), ()>::new(base.identity().clone(), semantics)
        .into_portable()
}

fn stage_evidence_workflow(
    reference: WorthQueryArtifactContractReference,
) -> WorthQueryPortableDomainOperationDefinition {
    let base = artifact_workflow(reference.clone());
    let mut semantics = base.semantics().clone();
    let WorthQueryOperationWorkflowContract::Declared(workflow) = &semantics.workflow else {
        unreachable!("artifact workflow fixture is declared")
    };
    let stages = workflow
        .stages()
        .iter()
        .cloned()
        .map(|stage| {
            if stage.identity() != "collect" {
                return stage;
            }
            let mut stage_semantics = stage.semantics().clone();
            stage_semantics.evidence =
                WorthQueryDomainEvidenceContract::installed_artifact(reference.clone());
            stage.with_semantics(stage_semantics)
        })
        .collect::<Vec<_>>();
    semantics.workflow = WorthQueryOperationWorkflowContract::Declared(
        WorthQueryPortableWorkflowDefinition::new(workflow.entry_stage(), stages),
    );
    WorthQueryDomainOperationDefinition::<(), (), ()>::new(base.identity().clone(), semantics)
        .into_portable()
}
