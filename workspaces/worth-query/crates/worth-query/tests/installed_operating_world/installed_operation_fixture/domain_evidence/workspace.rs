use worth_query::facade::consumer_kit::{
    in_memory_test_runtime, WorthQueryTestBackendError, WorthQueryTestBackendSchema,
};
use worth_query::facade::{domain, runtime};

use super::super::{identity_contract, GeometryDomain};
use super::contract::{
    artifact_support, direct_package, direct_package_with_governance, EvidenceGovernance,
};
use super::direct_executor::EvidenceDirectExecutor;
use super::{EvidenceFamily, EvidenceRead, EvidenceScenario};

pub fn evidence_workspace(
    name: &str,
    scenario: EvidenceScenario,
    redaction: domain::WorthQueryArtifactRedactionPosture,
) -> Result<runtime::WorthQueryWorkspace, WorthQueryTestBackendError> {
    build_workspace(name, scenario, direct_package(redaction))
}

pub fn evidence_workspace_with_governance(
    name: &str,
    scenario: EvidenceScenario,
    governance: EvidenceGovernance,
) -> Result<runtime::WorthQueryWorkspace, WorthQueryTestBackendError> {
    build_workspace(name, scenario, direct_package_with_governance(governance))
}

fn build_workspace(
    name: &str,
    scenario: EvidenceScenario,
    package: domain::WorthQueryDomainPackage<GeometryDomain>,
) -> Result<runtime::WorthQueryWorkspace, WorthQueryTestBackendError> {
    let schema = WorthQueryTestBackendSchema::single_collection("Vertex")
        .aspect_contract(identity_contract())
        .unwrap()
        .aspect("identity.id", "identity.id")
        .unwrap();
    in_memory_test_runtime()
        .with_schema(schema)
        .domain_package_with_artifact_support(package, artifact_support())
        .domain_operation_executor(
            GeometryDomain,
            EvidenceRead,
            EvidenceFamily,
            EvidenceDirectExecutor::new(scenario),
        )
        .workspace(name)
}
