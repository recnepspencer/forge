use worth_query_installation::facade::{
    WorthQueryAdmittedPortableDomainPackage, WorthQueryArtifactProtocolVersion,
    WorthQueryArtifactSchemaVersion, WorthQueryArtifactVersionSupport,
    WorthQueryExecutionResourceContract, WorthQueryInstallationAdmissionProfile,
    WorthQueryInstallationSupportStatus, WorthQueryOperationGraphAccess,
    WorthQueryPortableDomainIdentity, WorthQueryPortableDomainPackage,
    WorthQueryValidatedPortableDomainPackage,
};

use super::candidate_contract::{candidate_contract, FixtureConvergenceContract};
use super::fixture_identity::{
    CandidateFamily, ComparatorFamily, OPERATION_SLOT, OWNER, WORKFLOW_STAGE,
};
use super::operation_definition::{direct_operation, workflow_operation};

pub(crate) fn admitted_package_with_contract(
    resources: WorthQueryExecutionResourceContract,
    convergence: FixtureConvergenceContract,
) -> WorthQueryAdmittedPortableDomainPackage {
    let contract = candidate_contract(OPERATION_SLOT, convergence);
    let reference = contract.reference();
    let package =
        WorthQueryPortableDomainPackage::new(WorthQueryPortableDomainIdentity::new(OWNER, 1, 0))
            .artifact_contract(contract)
            .domain_operation(direct_operation(
                reference,
                resources,
                WorthQueryOperationGraphAccess::Observe,
            ))
            .validate()
            .expect("convergence fixture package must validate");
    admit_package(package)
}

pub(super) fn admitted_workflow_package(
    operation_resources: WorthQueryExecutionResourceContract,
    stage_resources: WorthQueryExecutionResourceContract,
    graph_access: WorthQueryOperationGraphAccess,
) -> WorthQueryAdmittedPortableDomainPackage {
    let contract = candidate_contract(WORKFLOW_STAGE, FixtureConvergenceContract::Bounded);
    let reference = contract.reference();
    let package =
        WorthQueryPortableDomainPackage::new(WorthQueryPortableDomainIdentity::new(OWNER, 1, 0))
            .artifact_contract(contract)
            .domain_operation(workflow_operation(
                reference,
                operation_resources,
                stage_resources,
                graph_access,
            ))
            .validate()
            .expect("workflow convergence fixture package must validate");
    admit_package(package)
}

fn admit_package(
    package: WorthQueryValidatedPortableDomainPackage,
) -> WorthQueryAdmittedPortableDomainPackage {
    WorthQueryInstallationAdmissionProfile::new("convergence-support-v1", "convergence-config-v1")
        .artifact_version::<CandidateFamily>(
            WorthQueryArtifactSchemaVersion::new(1),
            WorthQueryArtifactProtocolVersion::new(1),
            WorthQueryArtifactVersionSupport::Admitted,
        )
        .artifact_comparator::<ComparatorFamily>(WorthQueryInstallationSupportStatus::Admitted)
        .admit(package)
        .expect("convergence fixture package must admit")
}
