use worth_query_host::facade::{
    admission::resource_admission::{
        WorthQueryAdmittedExecutionResourcePlan, WorthQueryExecutionResourceAdmissionDenial,
    },
    declaration::domain_computation::WorthQueryExecutionResourceRequest,
    domain::{WorthQueryInstalledDomainOperationAuthority, WorthQueryPortableDomainPackage},
    installed::{
        domain_computation::{
        WorthQueryArtifactChunkRequest, WorthQueryArtifactNativeAccessCounters,
        WorthQueryArtifactNativeAccessDenial, WorthQueryTransferredArtifactHandle,
        },
        provider_session::WorthQueryExecutionProviderSession,
    },
    publication::domain_computation::WorthQueryDomainEvidenceMaterial,
    runtime::{WorthQueryExecutionRuntime, WorthQueryExecutionRuntimeInstaller},
};
use worth_query_replay::facade::WorthQueryCertificationReplayCounters;

fn install_and_inspect(
    installer: WorthQueryExecutionRuntimeInstaller,
    package: WorthQueryPortableDomainPackage,
) {
    let _ = (installer, package);
}

fn inspect_runtime_and_installed_operation(
    runtime: &WorthQueryExecutionRuntime,
    operation: &WorthQueryInstalledDomainOperationAuthority,
) {
    let _ = runtime.authority_identity();
    let _ = runtime.installed_packages().validate_domain_operation(operation);
}

fn inspect_resource_admission(
    plan: &WorthQueryAdmittedExecutionResourcePlan,
    denial: &WorthQueryExecutionResourceAdmissionDenial,
    session: &WorthQueryExecutionProviderSession,
) {
    let _: &WorthQueryExecutionResourceRequest = plan.request();
    let _ = plan.request_identity();
    let _ = plan.strategy();
    let _ = plan.envelope();
    let _ = denial.kind();
    let _ = session.identity();
    let _ = session.attempt_identity();
}

fn carry_artifact_and_publication(
    artifact: WorthQueryTransferredArtifactHandle,
    request: WorthQueryArtifactChunkRequest,
    counters: WorthQueryArtifactNativeAccessCounters,
    denial: WorthQueryArtifactNativeAccessDenial,
    evidence: WorthQueryDomainEvidenceMaterial,
) {
    let _ = (artifact, request, counters, denial, evidence);
}

fn certification_entry(counters: WorthQueryCertificationReplayCounters) {
    let _ = counters;
}

fn main() {}
