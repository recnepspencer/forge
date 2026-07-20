use crate::WorthServerOperationRequest;

use super::{WorthServerOperationAuthorityKind, WorthServerOperationScope};

pub(super) fn admit_durable_product_mutation(
    operation_request: &WorthServerOperationRequest,
    tenant_id: &str,
    workspace_id: &str,
    authority_scope: &str,
    expected_basis_digest: &str,
    idempotency_key: &str,
    durability_contract_digest: &str,
) -> Result<(WorthServerOperationAuthorityKind, WorthServerOperationScope), String> {
    if authority_scope.trim().is_empty() || durability_contract_digest.trim().is_empty() {
        return Err(
            "durable product mutation authority requires product scope and durability contract identity"
                .to_string(),
        );
    }
    super::admission_logic::validate_shared_read_basis_digest(
        operation_request,
        expected_basis_digest,
    )?;
    match operation_request.identity().idempotency_key() {
        Some(expected) if expected == idempotency_key => {}
        Some(expected) => {
            return Err(format!(
                "durable authority idempotency key `{idempotency_key}` does not match admitted operation key `{expected}`"
            ));
        }
        None => {
            return Err(
                "durable product mutation authority requires an admitted idempotency key"
                    .to_string(),
            );
        }
    }
    Ok((
        WorthServerOperationAuthorityKind::DurableProductMutation,
        WorthServerOperationScope::durable_product_authority(
            tenant_id,
            workspace_id,
            authority_scope,
        ),
    ))
}
