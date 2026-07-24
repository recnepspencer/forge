use crate::domain_installation::{
    WorthQueryBoundGraphExecutionReceipt, WorthQueryExecutionProviderSession,
    WorthQueryExecutionResourceAttemptEvidence, WorthQueryGraphCallScope,
    WorthQueryGraphCommitCallSpec, WorthQueryGraphProviderFailure,
};

pub(super) fn contact_commit_provider(
    scope_identity: &str,
    operation_identity: &str,
    binding_identity: &str,
    authority: &super::super::graph_participation::WorthQueryInstalledGraphCommitAuthority,
    graph_roles: Vec<String>,
    resources: &super::WorthQueryAdmittedExecutionResourcePlan,
    resource_evidence: &WorthQueryExecutionResourceAttemptEvidence,
    provider_session: &WorthQueryExecutionProviderSession,
) -> Result<WorthQueryBoundGraphExecutionReceipt, WorthQueryGraphProviderFailure> {
    let call = provider_session
        .bind_graph_commit_call(
            WorthQueryGraphCommitCallSpec::new(
                WorthQueryGraphCallScope::new(scope_identity, operation_identity, binding_identity),
                graph_roles,
                authority.identity(),
            ),
            resource_evidence,
            resources.shared_envelope(),
        )
        .map_err(|denial| WorthQueryGraphProviderFailure::new(denial.detail()))?;
    let receipt = authority.provider.admit_commit(&call)?;
    call.admit_receipt(receipt)
        .map_err(|denial| WorthQueryGraphProviderFailure::new(denial.detail()))
}
