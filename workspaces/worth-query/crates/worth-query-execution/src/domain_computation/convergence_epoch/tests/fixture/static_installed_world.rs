use worth_query_installation::facade::{
    WorthQueryArtifactFamily, WorthQueryInstallationGeneration,
    WorthQueryInstalledArtifactContractAuthority, WorthQueryInstalledDomainOperationAuthority,
};

use crate::domain_computation::{WorthQueryExecutionRuntime, WorthQueryExecutionRuntimeInstaller};

use super::candidate_contract::FixtureConvergenceContract;
use super::fixture_identity::{CandidateFamily, OPERATION_SLOT, OWNER};
use super::package::admitted_package_with_contract;
use super::provider::execution_support;
use super::resource_contract::resource_contract;

pub(crate) struct StaticConvergenceAdmissionFixture {
    pub operation: WorthQueryInstalledDomainOperationAuthority,
    pub artifact: WorthQueryInstalledArtifactContractAuthority,
    _runtime: WorthQueryExecutionRuntime,
}

pub(crate) fn static_convergence_admission_fixture(
    convergence: FixtureConvergenceContract,
) -> StaticConvergenceAdmissionFixture {
    let resources = resource_contract(&execution_support(false));
    let (runtime, _) = WorthQueryExecutionRuntimeInstaller::new()
        .install(
            WorthQueryInstallationGeneration::initial(),
            [admitted_package_with_contract(resources, convergence)],
        )
        .expect("static convergence fixture package must install")
        .into_parts();
    let operation = runtime
        .installed_packages()
        .domain_operation(OWNER, OPERATION_SLOT)
        .expect("static fixture operation must be installed");
    let artifact = runtime
        .installed_packages()
        .artifact_contract(
            OWNER,
            CandidateFamily::SEMANTIC_FAMILY,
            worth_query_installation::facade::WorthQueryArtifactSchemaVersion::new(1),
            worth_query_installation::facade::WorthQueryArtifactProtocolVersion::new(1),
        )
        .expect("static fixture artifact must be installed");
    StaticConvergenceAdmissionFixture {
        operation,
        artifact,
        _runtime: runtime,
    }
}
