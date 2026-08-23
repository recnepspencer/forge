use std::sync::Arc;

use crate::facade::AdmittedBridgeAsyncRequestIdentity;

pub struct BridgeOwnedAsyncRequestAdmission {
    request: AdmittedBridgeAsyncRequestIdentity,
    effects_indeterminate: BridgeOwnedAsyncEffectsIndeterminateIssuer,
}

pub struct BridgeOwnedAsyncEffectsIndeterminateIssuer {
    authority: Arc<()>,
    request: AdmittedBridgeAsyncRequestIdentity,
}

pub struct BridgeAsyncEffectsIndeterminateObservation {
    authority: Arc<()>,
    request: AdmittedBridgeAsyncRequestIdentity,
    envelope: worth_signal::facade::RawCompletionEnvelope,
}

impl BridgeOwnedAsyncRequestAdmission {
    pub(super) fn new(authority: &Arc<()>, request: AdmittedBridgeAsyncRequestIdentity) -> Self {
        Self {
            effects_indeterminate: BridgeOwnedAsyncEffectsIndeterminateIssuer {
                authority: Arc::clone(authority),
                request: request.clone(),
            },
            request,
        }
    }

    pub fn into_parts(
        self,
    ) -> (
        AdmittedBridgeAsyncRequestIdentity,
        BridgeOwnedAsyncEffectsIndeterminateIssuer,
    ) {
        (self.request, self.effects_indeterminate)
    }
}

impl BridgeOwnedAsyncEffectsIndeterminateIssuer {
    pub fn certify(&self, payload_byte_len: u64) -> BridgeAsyncEffectsIndeterminateObservation {
        let descriptor = self
            .request
            .lowered()
            .resource_descriptor()
            .expect("owner-issued async request retains its resource descriptor");
        BridgeAsyncEffectsIndeterminateObservation {
            authority: Arc::clone(&self.authority),
            request: self.request.clone(),
            envelope: worth_signal::facade::RawCompletionEnvelope::new(
                self.request.request_handle().request_id(),
                self.request.request_handle().generation(),
                self.request.request_handle().branch_epoch(),
                self.request.attempt(),
                descriptor.payload_contract_digest().clone(),
                payload_byte_len,
            ),
        }
    }
}

impl BridgeAsyncEffectsIndeterminateObservation {
    pub(super) fn into_parts(
        self,
    ) -> (
        Arc<()>,
        AdmittedBridgeAsyncRequestIdentity,
        worth_signal::facade::RawCompletionEnvelope,
    ) {
        (self.authority, self.request, self.envelope)
    }
}
