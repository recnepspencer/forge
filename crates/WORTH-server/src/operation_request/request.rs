use crate::WorthServerResolvedRequestContext;

use super::{
    WorthServerOperationIdentity, WorthServerOperationInputEnvelope,
    WorthServerOperationRequestReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerOperationRequest {
    resolved_request_context: WorthServerResolvedRequestContext,
    identity: WorthServerOperationIdentity,
    payload_envelope: Option<WorthServerOperationInputEnvelope>,
    receipt: WorthServerOperationRequestReceipt,
    canonical_digest: String,
}

impl WorthServerOperationRequest {
    pub(crate) fn new(
        resolved_request_context: WorthServerResolvedRequestContext,
        identity: WorthServerOperationIdentity,
        payload_envelope: Option<WorthServerOperationInputEnvelope>,
        receipt: WorthServerOperationRequestReceipt,
    ) -> Self {
        let canonical_digest = format!(
            "worth-server-operation-request-v1|identity={}|receipt={}|payload={}",
            identity.canonical_digest(),
            receipt.canonical_digest(),
            payload_envelope
                .as_ref()
                .map(WorthServerOperationInputEnvelope::canonical_digest)
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

    pub fn resolved_request_context(&self) -> &WorthServerResolvedRequestContext {
        &self.resolved_request_context
    }

    pub fn identity(&self) -> &WorthServerOperationIdentity {
        &self.identity
    }

    pub fn payload_envelope(&self) -> Option<&WorthServerOperationInputEnvelope> {
        self.payload_envelope.as_ref()
    }

    pub fn receipt(&self) -> &WorthServerOperationRequestReceipt {
        &self.receipt
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
