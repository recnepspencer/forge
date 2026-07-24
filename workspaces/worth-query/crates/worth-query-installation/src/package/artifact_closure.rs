use crate::domain_computation::WorthQueryPortableArtifactContract;
use crate::domain_operation::{
    WorthQueryDomainEvidenceContract, WorthQueryOperationWorkflowContract,
    WorthQueryPortableDomainOperationDefinition, WorthQueryWorkflowValueContract,
};

use super::WorthQueryPortablePackageValidationDenial;

pub(super) fn reject_artifact_contract_conflicts(
    contracts: &[WorthQueryPortableArtifactContract],
) -> Result<(), WorthQueryPortablePackageValidationDenial> {
    for pair in contracts.windows(2) {
        let left_slot = (
            pair[0].family(),
            pair[0].schema_version(),
            pair[0].protocol_version(),
        );
        let right_slot = (
            pair[1].family(),
            pair[1].schema_version(),
            pair[1].protocol_version(),
        );
        if left_slot != right_slot {
            continue;
        }
        return Err(if pair[0] == pair[1] {
            WorthQueryPortablePackageValidationDenial::duplicate_artifact_contract(
                pair[1].family().as_str(),
            )
        } else {
            WorthQueryPortablePackageValidationDenial::conflicting_artifact_contract(
                pair[1].family().as_str(),
            )
        });
    }
    Ok(())
}

pub(super) fn validate_workflow_artifact_closure(
    operations: &[WorthQueryPortableDomainOperationDefinition],
    contracts: &[WorthQueryPortableArtifactContract],
) -> Result<(), WorthQueryPortablePackageValidationDenial> {
    for operation in operations {
        let operation_slot = operation.identity().slot();
        if matches!(
            &operation.semantics().workflow,
            WorthQueryOperationWorkflowContract::Declared(_)
        ) && !matches!(
            &operation.semantics().evidence,
            WorthQueryDomainEvidenceContract::NotRequired
        ) {
            return Err(
                WorthQueryPortablePackageValidationDenial::invalid_domain_operation(format!(
                    "{operation_slot}:workflow-operation-evidence-requires-stage-declaration"
                )),
            );
        }
        validate_evidence_reference(
            contracts,
            &operation_slot,
            &operation_slot,
            &operation.semantics().evidence,
        )?;
        let WorthQueryOperationWorkflowContract::Declared(workflow) =
            &operation.semantics().workflow
        else {
            continue;
        };
        for stage in workflow.stages() {
            validate_evidence_reference(
                contracts,
                stage.identity(),
                stage.identity(),
                &stage.semantics().evidence,
            )?;
            validate_stage_value(
                contracts,
                stage.identity(),
                "consumer",
                &stage.semantics().input,
            )?;
            validate_stage_value(
                contracts,
                stage.identity(),
                "producer",
                &stage.semantics().output,
            )?;
        }
    }
    Ok(())
}

fn validate_evidence_reference(
    contracts: &[WorthQueryPortableArtifactContract],
    subject: &str,
    producer_role: &str,
    evidence: &WorthQueryDomainEvidenceContract,
) -> Result<(), WorthQueryPortablePackageValidationDenial> {
    let WorthQueryDomainEvidenceContract::InstalledArtifact(reference) = evidence else {
        return Ok(());
    };
    let contract = installed_contract(contracts, subject, reference)?;
    contract
        .producer_roles()
        .iter()
        .any(|role| role == producer_role)
        .then_some(())
        .ok_or_else(|| {
            WorthQueryPortablePackageValidationDenial::invalid_domain_operation(format!(
                "{subject}:evidence-producer-role-not-permitted:{}",
                reference.family().as_str()
            ))
        })
}

fn validate_stage_value(
    contracts: &[WorthQueryPortableArtifactContract],
    stage_identity: &str,
    direction: &str,
    value: &WorthQueryWorkflowValueContract,
) -> Result<(), WorthQueryPortablePackageValidationDenial> {
    let WorthQueryWorkflowValueContract::InstalledArtifact(reference) = value else {
        return Ok(());
    };
    let contract = installed_contract(contracts, stage_identity, reference)?;
    let admitted_roles = if direction == "producer" {
        contract.producer_roles()
    } else {
        contract.consumer_roles()
    };
    admitted_roles
        .iter()
        .any(|role| role == stage_identity)
        .then_some(())
        .ok_or_else(|| {
            WorthQueryPortablePackageValidationDenial::invalid_domain_operation(format!(
                "{stage_identity}:artifact-{direction}-role-not-permitted:{}",
                reference.family().as_str()
            ))
        })
}

fn installed_contract<'a>(
    contracts: &'a [WorthQueryPortableArtifactContract],
    subject: &str,
    reference: &crate::domain_computation::WorthQueryArtifactContractReference,
) -> Result<&'a WorthQueryPortableArtifactContract, WorthQueryPortablePackageValidationDenial> {
    contracts
        .iter()
        .find(|contract| {
            contract.family() == reference.family()
                && contract.schema_version() == reference.schema_version()
                && contract.protocol_version() == reference.protocol_version()
        })
        .ok_or_else(|| {
            WorthQueryPortablePackageValidationDenial::invalid_domain_operation(format!(
                "{subject}:undeclared-artifact-contract:{}",
                reference.family().as_str()
            ))
        })
}
