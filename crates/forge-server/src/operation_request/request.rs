use crate::ForgeServerResolvedRequestContext;

use super::{
    ForgeServerOperationIdentity, ForgeServerOperationInputEnvelope,
    ForgeServerOperationRequestReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerOperationRequest {
    resolved_request_context: ForgeServerResolvedRequestContext,
    identity: ForgeServerOperationIdentity,
    payload_envelope: Option<ForgeServerOperationInputEnvelope>,
    receipt: ForgeServerOperationRequestReceipt,
    canonical_digest: String,
}

impl ForgeServerOperationRequest {
    pub(crate) fn new(
        resolved_request_context: ForgeServerResolvedRequestContext,
        identity: ForgeServerOperationIdentity,
        payload_envelope: Option<ForgeServerOperationInputEnvelope>,
        receipt: ForgeServerOperationRequestReceipt,
    ) -> Self {
        let canonical_digest = format!(
            "forge-server-operation-request-v1|identity={}|receipt={}|payload={}",
            identity.canonical_digest(),
            receipt.canonical_digest(),
            payload_envelope
                .as_ref()
                .map(ForgeServerOperationInputEnvelope::canonical_digest)
                .unwrap_or("none"),
        );
        Self {
            resolved_request_context,
            identity,
            payload_envelope,
            receipt,
            canonical_digest,
        }
    }

    pub fn resolved_request_context(&self) -> &ForgeServerResolvedRequestContext {
        &self.resolved_request_context
    }

    pub fn identity(&self) -> &ForgeServerOperationIdentity {
        &self.identity
    }

    pub fn payload_envelope(&self) -> Option<&ForgeServerOperationInputEnvelope> {
        self.payload_envelope.as_ref()
    }

    pub fn receipt(&self) -> &ForgeServerOperationRequestReceipt {
        &self.receipt
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
