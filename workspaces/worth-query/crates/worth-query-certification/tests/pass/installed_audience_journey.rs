use worth_query_host::facade::{
    domain::{
        WorthQueryArtifactChunkRequest, WorthQueryArtifactNativeAccessCounters,
        WorthQueryArtifactNativeAccessDenial, WorthQueryTransferredArtifactHandle,
        WorthQueryWorkflowStageWorkspace,
    },
    installed::{
        self,
        collection::{WorthQueryCollectionCursor, WorthQueryCollectionPatch},
        operation::{
            WorthQueryAdmittedExecutionResourcePlan, WorthQueryExecutionProviderSession,
            WorthQueryExecutionResourceAdmissionDenial, WorthQueryExecutionResourceRequest,
        },
    },
    runtime::WorthQueryWorkspace,
};
use worth_query_replay::facade::WorthQueryCertificationReplayCounters;
use worth_query::facade::{
    certification as query_certification, domain as query_domain, foundation as query_foundation,
    runtime as query_runtime,
};

struct ExampleFamily;

fn ordinary_entry(workspace: &mut WorthQueryWorkspace) {
    let root = workspace.observe_operating_world().unwrap();
    let _family = root.family(ExampleFamily);
    let _ = installed::operation::project_facts().entity_identities();
}

fn inspect_opaque_collection_artifacts(
    cursor: &WorthQueryCollectionCursor,
    patch: &WorthQueryCollectionPatch,
) {
    let _ = cursor.is_beginning();
    let _ = patch.maintenance_ordinal();
    let _ = patch.authority();
}

fn inspect_resource_admission(
    plan: &WorthQueryAdmittedExecutionResourcePlan,
    denial: &WorthQueryExecutionResourceAdmissionDenial,
    session: &WorthQueryExecutionProviderSession,
    transition: installed::transition::WorthQueryResourceAdmissionTransition<()>,
) {
    let _: &WorthQueryExecutionResourceRequest = plan.request();
    let _ = plan.request_identity();
    let _ = plan.strategy();
    let _ = plan.envelope();
    let _ = denial.kind();
    let _ = session.identity();
    let _ = session.attempt_identity();
    let _ = transition.into_result();
}

fn certification_entry(counters: WorthQueryCertificationReplayCounters) {
    let _ = counters;
}

fn carry_domain_evidence_without_promoting_authority(
    authority: &query_foundation::WorthQueryConsumedProjectionAuthority,
    admitted: &query_domain::WorthQueryAdmittedDomainEvidence,
    inspection: &query_runtime::WorthQueryDomainEvidenceInspectionCopy,
    certification: &query_certification::WorthQueryDomainEvidenceCertificationBundle,
) {
    let _ = authority;
    let _ = admitted.authority_posture();
    let _ = inspection.authority_posture();
    let _ = certification.authority_posture();
}

fn consume_native_artifact<'a>(
    workspace: &'a WorthQueryWorkflowStageWorkspace<'a>,
    artifact: &'a WorthQueryTransferredArtifactHandle,
    request: WorthQueryArtifactChunkRequest,
) -> Result<(usize, WorthQueryArtifactNativeAccessCounters), WorthQueryArtifactNativeAccessDenial> {
    let mut cursor = workspace.artifact_reader(artifact)?.chunks(request)?;
    let mut rows = 0;
    while cursor
        .next(|batch| rows += batch.row_count())?
        .is_some()
    {}
    Ok((rows, cursor.evidence().counters()))
}

fn main() {}
