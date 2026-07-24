use crate::domain_installation::{
    graph_participation::WorthQueryGraphCommitCallParts,
    WorthQueryExecutionResourceAttemptEvidence, WorthQueryGraphCommitCall,
    WorthQueryGraphProviderFailure, WorthQueryGraphProviderReceipt,
};

pub(super) fn contact_commit_provider(
    scope_identity: &str,
    operation_identity: &str,
    binding_identity: &str,
    authority: &super::super::graph_participation::WorthQueryInstalledGraphCommitAuthority,
    graph_roles: Vec<String>,
    resources: &super::WorthQueryAdmittedExecutionResourcePlan,
    resource_evidence: &WorthQueryExecutionResourceAttemptEvidence,
) -> Result<WorthQueryGraphProviderReceipt, WorthQueryGraphProviderFailure> {
    let call = WorthQueryGraphCommitCall::new(WorthQueryGraphCommitCallParts {
        scope_identity: scope_identity.to_string(),
        operation_identity: operation_identity.to_string(),
        binding_identity: binding_identity.to_string(),
        graph_roles,
        execution_resources: resource_evidence.clone(),
        resource_envelope: resources.shared_envelope(),
    });
    let receipt = authority.provider.admit_commit(&call)?;
    if !receipt.binds_call(call.call_identity()) {
        return Err(WorthQueryGraphProviderFailure::new(
            "commit provider returned a receipt minted for another Query call",
        ));
    }
    Ok(receipt)
}
