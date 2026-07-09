use crate::{WorthServerOperationRequest, WorthServerProductOperationPayload};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthServerProductIdempotencyBinding {
    storage_key: String,
    request_digest: String,
}

impl WorthServerProductIdempotencyBinding {
    pub(crate) fn derive(
        request: &WorthServerOperationRequest,
        payload: &WorthServerProductOperationPayload,
    ) -> Self {
        let request_context = request.resolved_request_context().request_context();
        let storage_key = format!(
            "worth-server-product-idempotency-scope-v1|tenant={}|workspace={}|key={}",
            request_context.workspace_target().tenant_id(),
            request_context.workspace_target().workspace_id(),
            request.identity().idempotency_key().unwrap_or("none"),
        );
        let request_digest = format!(
            "worth-server-product-idempotency-binding-v1|tenant={}|workspace={}|branch={}|session={}|family={}|operation={}|base={}|payload={}",
            request_context.workspace_target().tenant_id(),
            request_context.workspace_target().workspace_id(),
            request_context.branch_target().canonical_label(),
            request.identity().product_session_identity().unwrap_or("none"),
            request.identity().operation_family().as_str(),
            request.identity().operation_name(),
            request.identity().basis_digest().unwrap_or("none"),
            payload.envelope().canonical_digest(),
        );
        Self {
            storage_key,
            request_digest,
        }
    }

    pub(crate) fn storage_key(&self) -> &str {
        &self.storage_key
    }

    pub(crate) fn request_digest(&self) -> &str {
        &self.request_digest
    }
}
