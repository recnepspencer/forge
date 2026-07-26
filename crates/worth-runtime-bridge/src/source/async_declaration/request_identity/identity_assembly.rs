use std::sync::Arc;

use sha2::{Digest, Sha256};
use worth_signal::facade::{AdmittedResourceRequest, InFlightResourceRequest};

use super::counters::BridgeAsyncRequestIdentityCounters;
use super::identity::{
    AdmittedBridgeAsyncRequestIdentity, BridgeAsyncInFlightRequestIdentity,
    BridgeAsyncRequestAdmissionRequest, BridgeAsyncRequestFamilyAdmission,
    BridgeAsyncRequestIdentity,
};
use super::subscription_instance::BridgeAsyncRequestSubscriptionInstance;

pub(crate) fn assemble_admitted_request_identity(
    runtime_key: u64,
    request: BridgeAsyncRequestAdmissionRequest,
    admitted_request: AdmittedResourceRequest,
    lowered_node: String,
    payload_contract_digest: String,
    async_decision_digest: Option<Arc<str>>,
    in_flight: InFlightResourceRequest,
) -> AdmittedBridgeAsyncRequestIdentity {
    let counters = match request.family_admission() {
        BridgeAsyncRequestFamilyAdmission::RequestResponse => {
            BridgeAsyncRequestIdentityCounters::request_response_admitted()
        }
        BridgeAsyncRequestFamilyAdmission::SubscriptionBacked { .. } => {
            BridgeAsyncRequestIdentityCounters::subscription_backed_admitted()
        }
    };
    let family_admission = request.family_admission().clone();
    let canonical_basis = Arc::<str>::from(format!(
        "bridge-async-request-identity|declaration={}|lowering={}|basis-binding={}|truth-view-basis={}|family={:?}|lowering-family={:?}|subscription-instance={}|signal-node={}|signal-request-handle={}#{}|attempt={}|branch-epoch={}#{}|payload-contract={}|request-intent={}|async-decision={}",
        request.lowered().declaration_identity().as_str(),
        request.lowered().lowering_identity().as_str(),
        request.basis_binding().binding_identity().as_str(),
        request.basis_binding().truth_view_basis().digest(),
        request.basis_binding().family_kind(),
        request.basis_binding().lowering_family_kind(),
        family_admission
            .subscription_instance()
            .map(BridgeAsyncRequestSubscriptionInstance::digest)
            .unwrap_or("-"),
        lowered_node,
        admitted_request.handle().request_id().get(),
        admitted_request.handle().generation().get(),
        admitted_request.attempt().get(),
        admitted_request.handle().branch_epoch().branch_id().0,
        admitted_request.handle().branch_epoch().restore_epoch(),
        payload_contract_digest,
        in_flight.request_intent_digest().as_str(),
        async_decision_digest.as_deref().unwrap_or("-"),
    ));
    let digest = Sha256::digest(canonical_basis.as_bytes());
    let request_identity = BridgeAsyncRequestIdentity::admit_bridge_owned(format!(
        "bridge-async-request-identity-id:sha256:{digest:x}"
    ));
    let in_flight_identity =
        BridgeAsyncInFlightRequestIdentity::new(&request_identity, in_flight.clone(), counters);

    AdmittedBridgeAsyncRequestIdentity::new(
        runtime_key,
        request_identity,
        request.lowered().clone(),
        request.basis_binding().clone(),
        family_admission,
        admitted_request,
        Arc::from(in_flight.request_intent_digest().as_str().to_owned()),
        in_flight_identity,
        counters,
        canonical_basis,
        Arc::from(format!("bridge-async-request-identity:sha256:{digest:x}")),
    )
}
