use super::*;

pub(crate) fn retained_temporal_resume_basis(
    runtime: &crate::facade::RuntimeBridge,
    branch_identity: TruthBranchIdentity,
    snapshot_identity: TruthSnapshotIdentity,
    wake_posture: BridgeRetainedTemporalWakePosture,
    retention_complete: bool,
) -> crate::facade::BridgeRetainedTemporalResumeBasis {
    let truth_basis = BridgeTemporalTruthViewBasis::authoritative(
        branch_identity,
        TruthCommitIdentity::new("temporal-commit"),
        snapshot_identity,
    );
    let admitted = admitted_temporal_basis(truth_basis);
    runtime.capture_temporal_subscription_resume_basis(
        &admitted,
        wake_posture,
        None,
        retention_complete,
    )
}

pub(crate) fn admitted_async_request_identity(
    runtime: &crate::facade::RuntimeBridge,
    branch_identity: TruthBranchIdentity,
    snapshot_identity: TruthSnapshotIdentity,
    node: u64,
) -> AdmittedBridgeAsyncRequestIdentity {
    let declaration = BridgeAsyncSourceDeclarationDraft::request_response(
        crate::facade::BridgeAsyncSourceDeclarationIdentity::new(format!(
            "bridge-async:resume-basis:{node}"
        )),
        crate::facade::BridgeAsyncSourceLegacyDeclarationIdentity::new(format!(
            "bridge-async-legacy:resume-basis:{node}"
        )),
        ResourceNodeDeclaration::new(
            ResourceNodeId::from_node(NodeId::new(node as u32, 0)),
            ResourcePayloadContract::new(ResourcePayloadContractId::new(node))
                .with_max_payload_bytes(128),
        )
        .with_observation_policy(ResourceObservationPolicyDeclaration::LifecycleOnly)
        .with_retry_max_attempts(2),
    );
    let validated = runtime
        .validate_async_source_declaration(declaration)
        .expect("request-response declaration should validate");
    let lowered = runtime
        .lower_async_source_declaration(&validated)
        .expect("request-response declaration should lower");
    let truth_basis = BridgeAsyncRequestTruthViewBasis::authoritative(
        branch_identity,
        TruthCommitIdentity::new(format!("commit:{node}")),
        snapshot_identity,
    );
    let binding = runtime.bind_async_request_basis(&lowered, truth_basis);
    let request = BridgeAsyncRequestAdmissionRequest::request_response(&lowered, &binding)
        .expect("request-response basis binding should construct");
    runtime
        .admit_async_request_identity(request)
        .expect("request identity should admit")
}

pub(crate) fn retained_inflight_async_resume_basis(
    runtime: &crate::facade::RuntimeBridge,
    request_identity: &AdmittedBridgeAsyncRequestIdentity,
    retention_complete: bool,
) -> crate::facade::BridgeRetainedInflightAsyncResumeBasis {
    runtime.capture_inflight_async_subscription_resume_basis(request_identity, retention_complete)
}

pub(crate) fn retained_inflight_async_resume_basis_without_generation(
    request_identity: &AdmittedBridgeAsyncRequestIdentity,
    retention_complete: bool,
) -> crate::facade::BridgeRetainedInflightAsyncResumeBasis {
    crate::facade::BridgeRetainedInflightAsyncResumeBasis::capture_without_generation_for_test(
        request_identity,
        retention_complete,
    )
}

pub(crate) fn retained_shared_delivery_resume_basis(
    runtime: &crate::facade::RuntimeBridge,
    bundle: &crate::facade::BridgeSharedConsumerDeliveryBundleSealed,
) -> (
    crate::facade::BridgeSharedConsumerDeliveryProjection,
    crate::facade::BridgeSharedDeliveryAcknowledgementFrontier,
    crate::facade::BridgeRetainedDeliveryResumeBasis,
) {
    let projection = runtime
        .project_shared_delivery_consumer(bundle, 0)
        .expect("projection should admit");
    let acknowledgement = runtime
        .admit_shared_delivery_acknowledgement_frontier(bundle, &projection, 0)
        .expect("acknowledgement frontier should admit");
    let retained = runtime
        .capture_shared_delivery_subscription_resume_basis(
            bundle,
            &projection,
            &acknowledgement,
            true,
        )
        .expect("retained shared delivery basis should capture");
    (projection, acknowledgement, retained)
}

pub(crate) fn retained_subscription_resume_basis(
    runtime: &crate::facade::RuntimeBridge,
    active: &crate::facade::BridgeActiveSubscription,
    checkpoint: &crate::facade::BridgeSubscriptionCheckpoint,
    temporal_resume_basis: Option<crate::facade::BridgeRetainedTemporalResumeBasis>,
    inflight_async_resume_basis: Option<crate::facade::BridgeRetainedInflightAsyncResumeBasis>,
    delivery_resume_basis: Option<crate::facade::BridgeRetainedDeliveryResumeBasis>,
    retention_complete: bool,
) -> crate::facade::BridgeRetainedSubscriptionResumeBasis {
    runtime.capture_subscription_resume_basis(
        active,
        checkpoint,
        temporal_resume_basis,
        inflight_async_resume_basis,
        delivery_resume_basis,
        retention_complete,
    )
}
