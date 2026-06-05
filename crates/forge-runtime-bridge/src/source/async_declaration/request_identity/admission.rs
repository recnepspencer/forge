use std::sync::Arc;

use sha2::{Digest, Sha256};

use forge_signal::facade::{
    AdmittedResourceRequest, InFlightResourceRequest, ResourceRequestIntent,
};

use super::super::{BridgeAsyncSourceDeclarationFamilyKind, LoweredBridgeAsyncSourceDeclaration};
use super::binding::ValidatedBridgeAsyncRequestBasisBinding;
use super::counters::BridgeAsyncRequestIdentityCounters;
use super::identity::{
    AdmittedBridgeAsyncRequestIdentity, BridgeAsyncInFlightRequestIdentity,
    BridgeAsyncRequestAdmissionRequest, BridgeAsyncRequestFamilyAdmission,
    BridgeAsyncRequestIdentity,
};
use super::rejection::{
    BridgeAsyncRequestIdentityRejection, BridgeAsyncRequestIdentityRejectionKind,
};
use super::state::{
    live_async_declaration_for_lowering, live_resource_declaration_for_lowering,
    BridgeSignalRuntime,
};
use super::subscription_instance::{
    BridgeAsyncRequestSubscriptionInstance, BridgeAsyncRequestSubscriptionInstanceKind,
};
use super::truth_basis::BridgeAsyncRequestTruthViewBasisKind;

impl BridgeAsyncRequestAdmissionRequest {
    pub fn request_response(
        lowered: &LoweredBridgeAsyncSourceDeclaration,
        basis_binding: &ValidatedBridgeAsyncRequestBasisBinding,
    ) -> Result<Self, BridgeAsyncRequestIdentityRejection> {
        validate_request_response(lowered, basis_binding)?;
        Ok(Self::new(
            lowered.clone(),
            basis_binding.clone(),
            BridgeAsyncRequestFamilyAdmission::RequestResponse,
        ))
    }

    pub fn subscription_backed(
        lowered: &LoweredBridgeAsyncSourceDeclaration,
        basis_binding: &ValidatedBridgeAsyncRequestBasisBinding,
        subscription_instance: BridgeAsyncRequestSubscriptionInstance,
    ) -> Result<Self, BridgeAsyncRequestIdentityRejection> {
        validate_subscription_backed(lowered, basis_binding, &subscription_instance)?;
        Ok(Self::new(
            lowered.clone(),
            basis_binding.clone(),
            BridgeAsyncRequestFamilyAdmission::SubscriptionBacked {
                subscription_instance,
            },
        ))
    }
}

impl AdmittedBridgeAsyncRequestIdentity {
    pub fn admit(
        runtime_key: u64,
        signal_runtime: &mut BridgeSignalRuntime,
        request: BridgeAsyncRequestAdmissionRequest,
    ) -> Result<Self, BridgeAsyncRequestIdentityRejection> {
        match request.family_admission() {
            BridgeAsyncRequestFamilyAdmission::RequestResponse => {
                admit_request_response(runtime_key, signal_runtime, request)
            }
            BridgeAsyncRequestFamilyAdmission::SubscriptionBacked { .. } => {
                admit_subscription_backed(runtime_key, signal_runtime, request)
            }
        }
    }
}

fn admit_request_response(
    runtime_key: u64,
    runtime: &mut BridgeSignalRuntime,
    request: BridgeAsyncRequestAdmissionRequest,
) -> Result<AdmittedBridgeAsyncRequestIdentity, BridgeAsyncRequestIdentityRejection> {
    let declaration = request
        .lowered()
        .request_response_declaration()
        .cloned()
        .ok_or_else(|| family_kind_mismatch("request-response lowered declaration"))?;
    let descriptor = request
        .lowered()
        .resource_descriptor()
        .ok_or_else(|| family_kind_mismatch("lowered resource descriptor"))?;
    let lowered_node = descriptor.node().node().to_string();
    let payload_contract_digest = descriptor.payload_contract_digest().as_str().to_owned();
    let declaration = live_resource_declaration_for_lowering(
        runtime_key,
        runtime,
        request.lowered().lowering_identity().as_str(),
        &declaration,
    )
    .map_err(signal_request_rejected)?;
    let report = runtime
        .admit_resource_request(ResourceRequestIntent::new(declaration.node()))
        .map_err(signal_request_rejected)?;
    let in_flight = admitted_in_flight(
        runtime,
        report.admitted_request(),
        request.lowered().declaration_identity().as_str(),
    )?;
    Ok(build_identity(
        request,
        report.admitted_request(),
        lowered_node,
        payload_contract_digest,
        None,
        in_flight,
    ))
}

fn admit_subscription_backed(
    runtime_key: u64,
    runtime: &mut BridgeSignalRuntime,
    request: BridgeAsyncRequestAdmissionRequest,
) -> Result<AdmittedBridgeAsyncRequestIdentity, BridgeAsyncRequestIdentityRejection> {
    let declaration = request
        .lowered()
        .subscription_backed_declaration()
        .cloned()
        .ok_or_else(|| family_kind_mismatch("subscription-backed lowered declaration"))?;
    let bundle = request
        .lowered()
        .async_node_capability_bundle()
        .ok_or_else(|| family_kind_mismatch("lowered async capability bundle"))?;
    let lowered_node = bundle.node().to_string();
    let payload_contract_digest = bundle.payload_contract_digest().as_str().to_owned();
    let declaration = live_async_declaration_for_lowering(
        runtime_key,
        runtime,
        request.lowered().lowering_identity().as_str(),
        &declaration,
    )
    .map_err(signal_request_rejected)?;
    let capable = runtime.async_capable_node(declaration.node()).ok_or_else(|| {
        BridgeAsyncRequestIdentityRejection::new(
            BridgeAsyncRequestIdentityRejectionKind::SignalRequestAdmissionRejected,
            format!(
                "signal runtime declared bridge async subscription-backed source `{}` but did not retain an attachable async capability handle for node {}",
                request.lowered().declaration_identity().as_str(),
                declaration.node(),
            ),
        )
    })?;
    let async_report = runtime
        .admit_async_node_request(capable.request_intent())
        .map_err(signal_request_rejected)?;
    let resource_report = async_report.resource_admission().cloned().ok_or_else(|| {
        BridgeAsyncRequestIdentityRejection::new(
            BridgeAsyncRequestIdentityRejectionKind::SignalAsyncRequestBlocked,
            format!(
                "signal runtime blocked bridge async subscription-backed request identity `{}` for node {}",
                request.lowered().declaration_identity().as_str(),
                declaration.node(),
            ),
        )
    })?;
    let in_flight = admitted_in_flight(
        runtime,
        resource_report.admitted_request(),
        request.lowered().declaration_identity().as_str(),
    )?;
    Ok(build_identity(
        request,
        resource_report.admitted_request(),
        lowered_node,
        payload_contract_digest,
        Some(Arc::from(
            async_report
                .classification()
                .decision_digest()
                .as_str()
                .to_owned(),
        )),
        in_flight,
    ))
}

pub(crate) fn admit_from_existing_signal_request(
    runtime: &mut BridgeSignalRuntime,
    request: BridgeAsyncRequestAdmissionRequest,
    admitted_request: AdmittedResourceRequest,
    async_decision_digest: Option<Arc<str>>,
) -> Result<AdmittedBridgeAsyncRequestIdentity, BridgeAsyncRequestIdentityRejection> {
    let (lowered_node, payload_contract_digest) = match request.family_admission() {
        BridgeAsyncRequestFamilyAdmission::RequestResponse => {
            let descriptor = request
                .lowered()
                .resource_descriptor()
                .ok_or_else(|| family_kind_mismatch("lowered resource descriptor"))?;
            (
                descriptor.node().node().to_string(),
                descriptor.payload_contract_digest().as_str().to_owned(),
            )
        }
        BridgeAsyncRequestFamilyAdmission::SubscriptionBacked { .. } => {
            let bundle = request
                .lowered()
                .async_node_capability_bundle()
                .ok_or_else(|| family_kind_mismatch("lowered async capability bundle"))?;
            (
                bundle.node().to_string(),
                bundle.payload_contract_digest().as_str().to_owned(),
            )
        }
    };
    let in_flight = admitted_in_flight(
        runtime,
        admitted_request,
        request.lowered().declaration_identity().as_str(),
    )?;
    Ok(build_identity(
        request,
        admitted_request,
        lowered_node,
        payload_contract_digest,
        async_decision_digest,
        in_flight,
    ))
}

fn build_identity(
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
        async_decision_digest
            .as_deref()
            .unwrap_or("-"),
    ));
    let digest = Sha256::digest(canonical_basis.as_bytes());
    let request_identity = BridgeAsyncRequestIdentity::new(format!(
        "bridge-async-request-identity-id:sha256:{digest:x}"
    ));
    let in_flight_identity = BridgeAsyncInFlightRequestIdentity::new(
        &request_identity,
        in_flight.clone(),
        counters.clone(),
    );

    AdmittedBridgeAsyncRequestIdentity::new(
        request_identity,
        request.lowered().clone(),
        request.basis_binding().clone(),
        family_admission,
        admitted_request.handle(),
        admitted_request.attempt(),
        Arc::from(in_flight.request_intent_digest().as_str().to_owned()),
        in_flight_identity,
        counters,
        canonical_basis,
        Arc::from(format!("bridge-async-request-identity:sha256:{digest:x}")),
    )
}

fn admitted_in_flight(
    runtime: &mut BridgeSignalRuntime,
    admitted_request: AdmittedResourceRequest,
    declaration_identity: &str,
) -> Result<InFlightResourceRequest, BridgeAsyncRequestIdentityRejection> {
    runtime
        .in_flight_resource_request(admitted_request.handle())
        .cloned()
        .ok_or_else(|| {
            BridgeAsyncRequestIdentityRejection::new(
                BridgeAsyncRequestIdentityRejectionKind::InFlightRequestMissing,
                format!(
                    "signal runtime admitted bridge async request identity `{declaration_identity}` but did not retain an in-flight request for handle {}#{}",
                    admitted_request.handle().request_id().get(),
                    admitted_request.handle().generation().get(),
                ),
            )
        })
}

fn validate_request_response(
    lowered: &LoweredBridgeAsyncSourceDeclaration,
    basis_binding: &ValidatedBridgeAsyncRequestBasisBinding,
) -> Result<(), BridgeAsyncRequestIdentityRejection> {
    if lowered.family_kind() != BridgeAsyncSourceDeclarationFamilyKind::RequestResponse
        || basis_binding.family_kind() != BridgeAsyncSourceDeclarationFamilyKind::RequestResponse
    {
        return Err(family_kind_mismatch(
            "request-response family artifacts for request identity admission",
        ));
    }
    validate_shared_binding(lowered, basis_binding)
}

fn validate_subscription_backed(
    lowered: &LoweredBridgeAsyncSourceDeclaration,
    basis_binding: &ValidatedBridgeAsyncRequestBasisBinding,
    subscription_instance: &BridgeAsyncRequestSubscriptionInstance,
) -> Result<(), BridgeAsyncRequestIdentityRejection> {
    if lowered.family_kind() != BridgeAsyncSourceDeclarationFamilyKind::SubscriptionBacked
        || basis_binding.family_kind() != BridgeAsyncSourceDeclarationFamilyKind::SubscriptionBacked
    {
        return Err(family_kind_mismatch(
            "subscription-backed family artifacts for request identity admission",
        ));
    }
    validate_shared_binding(lowered, basis_binding)?;
    match (
        basis_binding.truth_view_basis_kind(),
        subscription_instance.kind(),
        basis_binding
            .truth_view_basis()
            .preview_active_subscription_identity(),
        subscription_instance.preview_active_subscription_identity(),
    ) {
        (
            BridgeAsyncRequestTruthViewBasisKind::Preview,
            BridgeAsyncRequestSubscriptionInstanceKind::Preview,
            Some(left),
            Some(right),
        ) if left == right
            && basis_binding.truth_view_basis().preview_parent_truth_view_basis_digest()
                == subscription_instance.parent_truth_view_basis_digest() => Ok(()),
        (BridgeAsyncRequestTruthViewBasisKind::Preview, _, _, _)
        | (_, BridgeAsyncRequestSubscriptionInstanceKind::Preview, _, _) => Err(
            BridgeAsyncRequestIdentityRejection::new(
                BridgeAsyncRequestIdentityRejectionKind::PreviewBasisSubscriptionInstanceMismatch,
                "bridge async preview truth-view basis must bind to the exact matching preview subscription instance",
            ),
        ),
        _ => Ok(()),
    }
}

fn validate_shared_binding(
    lowered: &LoweredBridgeAsyncSourceDeclaration,
    basis_binding: &ValidatedBridgeAsyncRequestBasisBinding,
) -> Result<(), BridgeAsyncRequestIdentityRejection> {
    if lowered.declaration_identity() != basis_binding.declaration_identity()
        || lowered.lowering_identity() != basis_binding.lowering_identity()
    {
        return Err(BridgeAsyncRequestIdentityRejection::new(
            BridgeAsyncRequestIdentityRejectionKind::LoweringIdentityMismatch,
            "bridge async request identity binding requires one exact lowered declaration and matching request-basis binding",
        ));
    }
    Ok(())
}

fn family_kind_mismatch(expected: &str) -> BridgeAsyncRequestIdentityRejection {
    BridgeAsyncRequestIdentityRejection::new(
        BridgeAsyncRequestIdentityRejectionKind::FamilyKindMismatch,
        format!("bridge async request identity binding requires {expected}"),
    )
}

fn signal_request_rejected(
    error: forge_signal::facade::SignalError,
) -> BridgeAsyncRequestIdentityRejection {
    BridgeAsyncRequestIdentityRejection::new(
        BridgeAsyncRequestIdentityRejectionKind::SignalRequestAdmissionRejected,
        error.to_string(),
    )
}
