use crate::domain_installation::{
    WorthQueryGraphCommitCall, WorthQueryGraphProviderFailure, WorthQueryGraphProviderReceipt,
};

pub(super) fn contact_commit_provider(
    scope_identity: &str,
    operation_identity: &str,
    binding_identity: &str,
    authority: &super::super::graph_participation::WorthQueryInstalledGraphCommitAuthority,
    graph_roles: Vec<String>,
) -> Result<WorthQueryGraphProviderReceipt, WorthQueryGraphProviderFailure> {
    let call = WorthQueryGraphCommitCall::new(
        scope_identity.to_string(),
        operation_identity.to_string(),
        binding_identity.to_string(),
        graph_roles,
    );
    let receipt = authority.provider.admit_commit(&call)?;
    if !receipt.binds_call(call.call_identity()) {
        return Err(WorthQueryGraphProviderFailure::new(
            "commit provider returned a receipt minted for another Query call",
        ));
    }
    Ok(receipt)
}
