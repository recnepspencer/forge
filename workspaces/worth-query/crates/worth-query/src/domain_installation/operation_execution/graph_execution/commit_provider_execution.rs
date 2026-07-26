use crate::domain_installation::{
    WorthQueryBoundGraphExecutionReceipt, WorthQueryExecutionProviderSession,
    WorthQueryExecutionResourceAttemptEvidence, WorthQueryGraphCommitCallRequest,
    WorthQueryGraphProviderFailure,
};

pub(super) fn contact_commit_provider(
    scope_identity: &str,
    stage_identity: Option<&str>,
    authority: &super::super::graph_participation::WorthQueryInstalledGraphCommitAuthority,
    graph_authorities: &[&worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority],
    resources: &super::WorthQueryAdmittedExecutionResourcePlan,
    resource_evidence: &WorthQueryExecutionResourceAttemptEvidence,
    provider_session: &WorthQueryExecutionProviderSession,
) -> Result<WorthQueryBoundGraphExecutionReceipt, WorthQueryGraphProviderFailure> {
    let request = match stage_identity {
        Some(stage_identity) => WorthQueryGraphCommitCallRequest::workflow_stage(
            scope_identity,
            stage_identity,
            authority.identity(),
        ),
        None => WorthQueryGraphCommitCallRequest::direct(scope_identity, authority.identity()),
    };
    let call = provider_session
        .bind_graph_commit_call(
            graph_authorities,
            request,
            resource_evidence,
            resources.shared_envelope(),
        )
        .map_err(|denial| WorthQueryGraphProviderFailure::new(denial.detail()))?;
    let receipt = authority.provider.admit_commit(&call)?;
    call.admit_receipt(receipt)
        .map_err(|denial| WorthQueryGraphProviderFailure::new(denial.detail()))
}
